# Deterministic Upgrade Plan

Status: **in progress, not closed yet**.

Progress as of the current implementation pass:

- D0 gate visibility is in place: `make clippy-strict`, `make clippy-warning-count`,
  `make determinism`, a non-blocking strict-clippy preview in CI and a blocking
  determinism job in CI.
- D1 started: `uninlined_format_args` and several rustc visibility/
  qualification suggestions were applied mechanically. The diagnostic inventory
  dropped from 11,231 to 9,471.
- D2/T1 started: retry jitter no longer reads `SystemTime::now`; backoff is now
  stable for a fixed attempt/config.
- D6/T2 started: alias list output is sorted for both JSON and human formats,
  recursive local copy traversal is sorted, persisted multipart completed parts
  are canonicalized by part number, pending upload lookup is sorted by upload id.
- T3/T5 started: `SOURCE_DATE_EPOCH` is honored by retention timestamp parsing.
- D2/T1 continued: `Clock`, `Entropy` and `Env` ports live in
  `crates/core/src/ports.rs`. Transfer pacing, KMS diagnostics, admin
  health/devnull diagnostics and SigV4 signing (admin and XML paths) take the
  clock from a port; config resolution and alias environment lookup accept an
  injected environment; IAM and bucket-metadata ZIP exports honor
  `SOURCE_DATE_EPOCH`.
- D3 continued: CI runs the determinism contract twice, with
  `TOKIO_WORKER_THREADS=1` and `4`, under `SOURCE_DATE_EPOCH=0`. The strict
  clippy command remains an explicitly non-blocking preview until the ratchet is
  paid down.
- The determinism contract test suite exists and passes locally:
  `crates/cli/tests/determinism_contract.rs`.
- The full workspace suite passes after these changes: 2,721 tests, 0 failures.

This document defines what must be done before rc can claim, objectively:

1. deterministic observable behavior end to end; and
2. a clippy gate that has no ignored warnings.

The performance/rigor migration documented in `bench/results/SUMMARY.md` is the
starting point, not the finish line. It improved toolchain policy, release
codegen and supply-chain checks. It did not prove determinism and it did not
close clippy.

---

## 1. Definition of done

The upgrade is only complete when all four gates below pass reproducibly in CI.

### D1 — Output determinism

Given the same:

- command line and stdin,
- config files and environment variables,
- mocked server responses or fixture files,
- logical clock and random seed,

running the same command twice must produce **byte-identical**:

- stdout,
- stderr (excluding explicitly separate log streams),
- exit code,
- files written to the output directory (including file names, contents and
  metadata that is part of the contract).

This must hold across repeated runs, different process start times, different
Tokio worker counts, different `TZ`, `LANG`, `COLUMNS`, `HOME`, `umask`, and
different filesystem enumeration orders.

### D2 — Clippy closure

This command must exit 0 with **zero warnings**:

```sh
cargo clippy --workspace --all-targets -- -D warnings
```

Also:

- `[workspace.lints]` must not contain a warning-level ratchet for lints that
  are expected to be fixed.
- `clippy.toml` must not exempt APIs that are part of the determinism problem
  (`HashMap`, `HashSet`, `SystemTime::now`, `Instant::now`, environment reads,
  process exit, etc.).
- Test-only blanket allowances must be removed or narrowed to a documented,
  lint-specific exception that still passes `allow_attributes_without_reason`.
- `cargo fmt --all -- --check` must pass.
- CI must run the strict command above, not `cargo clippy` without `-D warnings`.

### D3 — Test determinism

A new determinism suite must pass in CI:

```sh
make determinism
```

It must run the same command matrix repeatedly with fixed inputs and compare
bytes, not just exit codes or parsed JSON.

### D4 — Build reproducibility baseline

At minimum:

- `rust-toolchain.toml` and `Cargo.lock` are committed and CI builds with
  `--locked`;
- the build does not read wall-clock time or network state outside Cargo;
- archive/export outputs honor `SOURCE_DATE_EPOCH` where timestamps are part of
  the artifact;
- a documented command reproduces the same binary hash on the same
  OS/toolchain, or the unavoidable variance is explicitly listed and excluded
  from the claim.

Bit-identical binaries across different linkers/OSes are not required, but the
claim must state exactly what is reproducible.

---

## 2. Current objective snapshot

Snapshot command:

```sh
cargo clippy --workspace --all-targets --message-format=json
```

Current result at the time of writing:

- clippy exits 0, but emits **9,471 compiler diagnostics** across all targets;
- CI runs the strict clippy command but keeps it non-blocking, and publishes the
  per-lint inventory to the job summary;
- `pedantic` and `nursery` are currently `warn`;
- several restrictions are currently `warn` instead of `deny`;
- tests have a shared allow header for `expect_used`, `unwrap_used`, `panic`,
  `indexing_slicing`, `let_underscore_drop` and `large_stack_arrays`.

Top current diagnostics (all targets):

| count | diagnostic |
|---:|---|
| 3,325 | `missing_docs` |
| 1,091 | `clippy::large_futures` |
| 914 | `unreachable_pub` |
| 783 | `clippy::absolute_paths` |
| 648 | `clippy::arithmetic_side_effects` |
| 484 | `clippy::indexing_slicing` |
| 468 | `clippy::fn_params_excessive_bools` |
| 448 | `clippy::as_conversions` |
| 310 | `clippy::struct_excessive_bools` |
| 167 | `clippy::cast_precision_loss` |
| 160 | `clippy::manual_let_else` |
| 104 | `clippy::cast_sign_loss` |
| 98 | `clippy::needless_pass_by_value` |
| 42 | `clippy::format_push_string` |
| 41 | `clippy::let_underscore_untyped` |
| 40 | `clippy::cast_possible_truncation` |
| 39 | `clippy::assigning_clones` |
| 36 | `clippy::cast_possible_wrap` |
| 32 | `clippy::unused_self` |
| 32 | `clippy::let_underscore_must_use` |
| 26 | `clippy::unreachable` |
| 24 | `clippy::redundant_pub_crate` |
| 21 | `clippy::unreadable_literal` |
| 16 | `clippy::unnecessary_wraps` |
| 16 | `clippy::unused_async_trait_impl` |
| 16 | `clippy::unused_async` |

This table is a snapshot, not a permanent budget. Re-run the command after every
remediation PR and keep the count moving monotonically to zero.

---

## 3. Current nondeterminism inventory

These are concrete sources that prevent an end-to-end determinism claim.

### 3.1 Wall-clock time and random values

| location | issue |
|---|---|
| `crates/core/src/retry.rs` | retry jitter uses `SystemTime::now()` |
| `crates/cli/src/commands/ilm/rule.rs` | random-ish suffix from `SystemTime::now()` |
| `crates/cli/src/commands/retention.rs` | `Timestamp::try_from(SystemTime::now())` |
| `crates/core/src/admin/inspect_archive.rs` | `rand::rngs::OsRng` |
| `crates/core/src/admin/kms_diagnostic.rs` | `Instant::now()` in measurement/reporting |
| `crates/core/src/transfer.rs` | `Instant::now()` used for pacing/order |
| `crates/s3/src/admin.rs` | request `SystemTime::now()` |
| `crates/s3/src/client.rs` | request `SystemTime::now()` |
| `crates/s3/src/admin/diagnostics.rs` | `Instant::now()` in diagnostic output |
| `crates/s3/src/ops.rs` | `Instant::now()` latency values |

Required work:

- introduce a `Clock` port (`now_utc`, `now_monotonic`);
- introduce a `Random`/`Jitter` port with a seedable implementation;
- inject both at the CLI/adapter boundary;
- keep real implementations only at the process boundary;
- tests use a fixed clock and fixed seed;
- remove `SystemTime::now`, `Instant::now` and `OsRng` from decision/output code;
- make jitter deterministic: derive it from attempt + seed, or disable it in
  deterministic mode;
- define whether timestamps are omitted, fixed to `SOURCE_DATE_EPOCH`, or
  explicitly marked non-contractual.

### 3.2 Unordered collections

Current counts in `crates/*/src`:

- 134 `HashMap` references;
- 20 `HashSet` references;
- 177 `BTreeMap` references;
- 31 `BTreeSet` references;
- 0 `IndexMap` references.

Largest `HashMap`/`HashSet` concentrations:

| count | file |
|---:|---|
| 28 | `crates/s3/src/client.rs` |
| 18 | `crates/cli/src/commands/cp.rs` |
| 13 | `crates/cli/src/commands/diff.rs` |
| 9 | `crates/core/src/traits.rs` |
| 7 | `crates/s3/src/lifecycle_xml.rs` |
| 7 | `crates/cli/src/commands/object_identity.rs` |
| 6 | `crates/s3/src/admin.rs` |
| 6 | `crates/core/src/lifecycle.rs` |
| 6 | `crates/cli/src/commands/tag.rs` |
| 6 | `crates/cli/src/commands/replicate.rs` |
| 4 | `crates/s3/src/watch.rs` |
| 4 | `crates/core/src/transfer_options.rs` |
| 4 | `crates/cli/src/commands/rb.rs` |
| 4 | `crates/cli/src/commands/multipart.rs` |

Rules:

- a `HashMap` used only as a lookup table is acceptable if its iteration order
  never reaches output, logs, error text, file order or request order;
- every iteration that can reach an observable surface must be replaced by
  `BTreeMap`/`BTreeSet`/`IndexMap` or sorted at the boundary;
- define comparator rules explicitly (byte order vs Unicode order vs
  locale-independent order);
- restore `HashMap`/`HashSet` to `clippy.toml` `disallowed-types` after the
  adapter boundary has a documented `allow` with a reason;
- add a static audit script that flags new `HashMap`/`HashSet` iteration in
  output modules.

### 3.3 Filesystem enumeration

`read_dir` occurrences:

- `crates/cli/src/commands/cp.rs` (two sites, one already sorts by file name);
- `crates/cli/src/commands/mirror.rs` (sorts);
- `crates/core/src/admin/inspect_archive.rs` (test assertions);
- `crates/s3/src/multipart.rs`.

Rules:

- collect and sort every traversal before planning, output, upload order or
  deletion order;
- use one canonical comparator shared by copy, mirror, rm, diff and undo;
- cover symlinks, case sensitivity and non-UTF-8 names explicitly;
- test on filesystems that return entries in different physical orders.

### 3.4 Concurrency and scheduling

There are 13 `tokio::spawn` sites in `src`. Some are tests, but production code
uses parallel transfers, multipart copy, signal handling and diagnostics.

Rules:

- never emit output in task-completion order;
- collect results keyed by a stable identity, then sort before rendering;
- define stable merge rules for partial failures;
- keep logs separate from stdout/stderr contracts;
- run the determinism suite with `TOKIO_WORKER_THREADS=1` and
  `TOKIO_WORKER_THREADS=N`;
- test cancellation/timeout paths with a fake clock, not real sleeps.

### 3.5 Terminal and formatting state

Output can vary with:

- `COLUMNS` / terminal width;
- TTY vs pipe;
- color support;
- locale;
- progress bar timing;
- table auto-arrangement.

Rules:

- JSON output must be width-independent;
- human output must either use a fixed width in tests or declare dynamic width
  outside the determinism contract;
- progress/spinner output must be suppressed or normalized for comparison;
- numbers, dates and sizes must have one canonical formatter;
- JSON object key order and newline policy must be specified.

### 3.6 Environment and global process state

Current `std::env::var` usage remains in several modules. Required:

- read environment once at the boundary and pass typed config down;
- do not read environment inside library code paths that claim determinism;
- normalize `HOME`, `XDG_CONFIG_HOME`, `RC_CONFIG_DIR`, `TZ`, `LANG`, `LC_*`;
- make config resolution independent of current working directory changes;
- avoid mutable global state and `OnceLock` values that depend on first call
  order; if unavoidable, initialize explicitly in `main`.

---

## 4. Clippy closure workstream

Do not mix all of this into one PR. The order below keeps review tractable and
keeps the warning count monotonically decreasing.

### C0 — Make the gate visible

- [x] Add `clippy-strict` target:
  `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Change CI clippy job to the strict target.
- [ ] Add `RUSTFLAGS="-D warnings"` to the clippy CI job only.
- [x] Keep the existing `clippy` ratchet target temporarily for local work.
- [x] Add a CI job summary with the current warning count per lint, so progress
  is measurable.

### C1 — Mechanical lints (auto-fixable or low risk)

Suggested order:

- [ ] `uninlined_format_args` (623)
- [ ] `manual_let_else` (160)
- [ ] `single_match_else` (118)
- [ ] `unnested_or_patterns` (64)
- [ ] `redundant_closure_for_method_calls` (52)
- [ ] `format_push_string` (42)
- [ ] `assigning_clones` (39)
- [ ] `let_underscore_untyped` (41)
- [ ] `let_underscore_must_use` (32)
- [ ] `unnecessary_wraps` (16)
- [ ] `unused_self` (32)
- [ ] `semicolon_if_nothing_returned` (18)
- [ ] `explicit_iter_loop` (16)
- [ ] `unreadable_literal` (21)
- [ ] `needless_raw_string_hashes` (12)
- [ ] `trivially_copy_pass_by_ref` (9)
- [ ] `if_not_else` (8)
- [ ] `items_after_statements` (5)
- [ ] `needless_collect` (5)
- [ ] `filter_map_next` (4)
- [ ] `collapsible_else_if` (4)
- [ ] `useless_let_if_seq` (4)

Method:

```sh
cargo clippy --fix --workspace --all-targets --allow-dirty --allow-staged
cargo fmt --all
cargo test --workspace
```

Review the auto-fix diff; do not accept semantic rewrites blindly.

### C2 — API-shape lints

- [ ] `fn_params_excessive_bools` (468) and `struct_excessive_bools` (310):
  replace flag arguments with two-variant enums or option structs where they
  are not wire-contract DTOs.
- [ ] `needless_pass_by_value` (98): pass references where ownership is not
  required.
- [ ] `large_futures` (1,089): box at spawn boundaries or refactor command
  futures. This is also a stack-safety issue.
- [ ] `redundant_pub_crate` (98), `unreachable_pub` (1,293): tighten visibility
  to `pub(crate)`/`pub(super)` where the item is not part of a public contract.
- [ ] `absolute_paths` (833): import at module scope or justify the path.
- [ ] `unused_qualifications` (273): remove redundant qualification.
- [ ] `unused_async` / `unused_async_trait_impl` (32): remove needless async.

### C3 — Numeric and panic-safety lints

- [ ] `as_conversions` (454): replace with `TryFrom`/`From` or explicit checked
  helpers.
- [ ] `cast_possible_truncation` (40), `cast_possible_wrap` (36),
  `cast_sign_loss` (104), `cast_precision_loss` (167): use checked conversion
  helpers and assert the invariant once.
- [ ] `indexing_slicing` (484): use `.get()`, pattern matching or a validated
  cursor; add tests for every edge.
- [ ] `arithmetic_side_effects` (648): use `checked_*`/`saturating_*` where the
  overflow is not a panic-by-design invariant.
- [ ] `unreachable` (26): replace with an explicit error or a proven-exhaustive
  match.
- [ ] `unused_qualifications` and `unreachable_pub` must not be silenced
  globally.

### C4 — Documentation and public surface

- [ ] `missing_docs` (3,343): document public items or reduce visibility.
  For a CLI application, most of these should become `pub(crate)`, not
  `#[allow(missing_docs)]`.
- [ ] `missing_errors_doc`, `missing_panics_doc`, `missing_safety_doc`: either
  document or forbid the API.
- [ ] `allow_attributes_without_reason`: every remaining `#[allow]` needs a
  reason.
- [ ] Remove the shared integration-test allow header and replace it with
  narrow, per-file exceptions only where the test harness genuinely needs them.
- [ ] Restore `clippy.toml` bans for `HashMap`, `HashSet`, `SystemTime::now`,
  `Instant::now`, `std::env::var`, and `std::process::exit` once the ports in
  §3 exist.

### C5 — Remove the ratchet

- [ ] Change `pedantic` and `nursery` from `warn` to `deny`.
- [ ] Change `indexing_slicing`, `arithmetic_side_effects`, `as_conversions`,
  casts, `absolute_paths`, `fn_params_excessive_bools` and
  `struct_excessive_bools` from `warn` back to `deny`.
- [ ] Remove `double_must_use`, `doc_markdown`, `use_self`, `map_unwrap_or`,
  `derive_partial_eq_without_eq`, `too_many_lines`, `too_many_arguments`,
  `type_complexity`, `upper_case_acronyms` and similar global allows, fixing the
  code instead.
- [ ] Re-run the strict clippy gate with zero warnings.

---

## 5. Determinism implementation workstream

### T1 — Ports

- [ ] `Clock` trait: `now_utc()`, `now_monotonic()`, and test fake.
- [ ] `Jitter`/`Random` trait with a deterministic seeded implementation.
- [ ] `Env` port for process environment reads.
- [ ] `Output` port for stdout/stderr/table width/progress.
- [ ] Use these ports in `retry`, `transfer`, `multipart_copy`, admin
  diagnostics, archive export and backup/restore paths.

### T2 — Stable ordering

- [ ] Add a single `stable_sort`/`compare` module with documented ordering.
- [ ] Audit every `HashMap`/`HashSet` that is iterated into output.
- [ ] Replace with `BTreeMap`/`BTreeSet`/`IndexMap` or sort immediately before
  serialization.
- [ ] Sort `read_dir` always, even when a later stage sorts again.
- [ ] Sort server-returned arrays when the API does not guarantee order.
- [ ] Add ordering tests for empty, one-item, duplicate and non-UTF-8 inputs.

### T3 — Output contract

- [ ] Specify canonical JSON: key order, indentation, newline, number format,
  timestamp format, null/omitted-field rules.
- [ ] Specify canonical human output width and table behavior.
- [ ] Make stderr error payloads free of timestamps, temp paths and random IDs.
- [ ] Define log separation: logs never pollute stdout or the stable stderr
  contract.
- [ ] Add golden/snapshot tests that compare raw bytes.

### T4 — Concurrency

- [ ] Replace completion-order output with keyed collection + sort.
- [ ] Define stable partial-failure ordering.
- [ ] Make transfer concurrency deterministic under a fixed scheduler setting.
- [ ] Run the same scenario with different worker counts and compare output.
- [ ] Use fake time in timeout/cancellation tests.

### T5 — Filesystem and archive determinism

- [ ] Sort every directory traversal.
- [ ] Define symlink, hardlink and permission behavior.
- [ ] Honor `SOURCE_DATE_EPOCH` for generated ZIP/TAR/archive timestamps.
- [ ] Make temporary file names non-observable; never include them in output.
- [ ] Make archive entry order deterministic.
- [ ] Make export metadata order deterministic.

### T6 — Environment normalization

- [ ] Read env once in `main` and pass typed config.
- [ ] Remove env reads from core library paths.
- [ ] Normalize `TZ`, `LANG`, `LC_*`, `COLUMNS`, `HOME`, `XDG_*`,
  `RC_CONFIG_DIR`.
- [ ] Make tests independent of host locale, timezone and terminal.

---

## 6. Determinism test suite

Create `determinism/` or `crates/cli/tests/determinism/` with a fixture matrix.

For each scenario:

1. create a fresh temp workspace;
2. freeze config, environment, mock server and `SOURCE_DATE_EPOCH`;
3. run the command twice (or N times);
4. compare stdout, stderr, exit code and generated files byte for byte;
5. repeat with:
   - `TOKIO_WORKER_THREADS=1` and `TOKIO_WORKER_THREADS=8`;
   - `TZ=UTC` and `TZ=America/Sao_Paulo`;
   - `LANG=C` and `LANG=pt_BR.UTF-8`;
   - `COLUMNS=40` and `COLUMNS=200`;
   - different physical directory insertion orders.

Minimum scenarios:

- [ ] `--help`, `--version`, completions;
- [ ] `alias set/list/remove/export/import`;
- [ ] local `cp` error and success;
- [ ] `ls` buckets/objects with 0, 1 and 1,000 items;
- [ ] `ls --json` and human output;
- [ ] `diff`, `mirror`, `rm --dry-run` with mixed files;
- [ ] multipart copy/multipart upload metadata;
- [ ] admin list commands with unordered mock responses;
- [ ] archive export/import with fixed `SOURCE_DATE_EPOCH`;
- [ ] retry with fake clock and deterministic jitter;
- [ ] diagnostics with fake clock;
- [ ] partial failures with multiple servers;
- [ ] non-UTF-8 and symlink filesystem fixtures (Unix).

Add a CI job that runs this suite on Linux, plus a smaller smoke subset on
macOS/Windows.

---

## 7. CI and Makefile changes

- [ ] `make check` runs `fmt`, `clippy-strict`, `test`, `file-length`.
- [ ] Add `make determinism`.
- [ ] Add `make clippy-warning-count` for the ratchet dashboard.
- [ ] CI clippy uses `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] CI determinism job runs with `--locked`.
- [ ] CI build job runs `cargo build --workspace --release --locked`.
- [ ] Add `SOURCE_DATE_EPOCH` to determinism/archive jobs.
- [ ] Add a check that `clippy.toml` does not contain the determinism-critical
  exemptions.
- [ ] Keep `cargo-deny` ignores separate and documented; they are not clippy
  exemptions.

---

## 8. File-level hotspot order

Start with the files that dominate both diagnostics and nondeterminism:

1. `crates/s3/src/client.rs` — 729 diagnostics, 28 hash collections, request
   time sources.
2. `crates/cli/src/commands/cp.rs` — 414 diagnostics, 18 hash collections,
   `read_dir`, parallel transfer ordering.
3. `crates/s3/src/admin.rs` — request time, admin diagnostics.
4. `crates/core/src/retry.rs` — wall-clock jitter.
5. `crates/core/src/transfer.rs` — pacing and completion order.
6. `crates/core/src/admin/inspect_archive.rs` — OS RNG and archive traversal.
7. `crates/cli/src/commands/{diff,mirror,rm,replicate}.rs` — ordering
   and partial-failure merge.
8. `crates/cli/src/output/*` — canonical formatting, widths, colors.
9. `crates/cli/src/commands/admin/*` — timestamps/latency in diagnostic output.
10. `crates/cli/tests/admin_support/mod.rs` — 589 diagnostics; replace blanket
    test allows with narrow helpers.

---

## 9. Suggested PR sequence

Each PR must keep `cargo test --workspace` green and the performance baseline
unchanged within the measured noise floor (`bench/results/noise-floor.md`).

1. **PR D0 — Gate visibility**
   Add `clippy-strict` and the determinism test skeleton; do not require zero
   yet.
2. **PR D1 — Mechanical clippy**
   Fix C1 categories; add warning-count artifact.
3. **PR D2 — Visibility and docs**
   Fix `unreachable_pub`, `missing_docs`, `redundant_pub_crate`, `absolute_paths`.
4. **PR D3 — Numeric safety**
   Fix casts, indexing, arithmetic, `unreachable`.
5. **PR D4 — API shape and futures**
   Fix bool flags, `large_futures`, pass-by-value.
6. **PR D5 — Time/random ports**
   Add `Clock`, `Jitter`, `Env`, `Output`; remove wall-clock/RNG from core.
7. **PR D6 — Ordering**
   Replace observable hash-map iteration, sort all traversals and server arrays.
8. **PR D7 — Output contract**
   Canonical JSON/human output, narrow test allows, byte snapshots.
9. **PR D8 — Determinism suite and CI**
   Matrix of env/thread/columns/TZ/locale, CI job.
10. **PR D9 — Close the gate**
    Remove the ratchet and test blanket allows; switch CI to
    `-D warnings`; update `DETERMINISTIC_UPGRADE.md` status to closed.
11. **PR D10 — Re-run performance A/B**
    Confirm no regression; record new baseline in `bench/results/`.

---

## 9.1 Implementation progress log

Completed in the first implementation pass:

- [x] Added `make clippy-strict`.
- [x] Added `make clippy-warning-count` and `scripts/clippy-warning-count.sh`.
- [x] Added `make determinism`.
- [x] Added `crates/cli/tests/determinism_contract.rs` with byte comparisons for
  version, help across terminal widths, completions, alias list/export and a
  local copy error path.
- [x] Added a non-blocking strict-clippy preview job and a blocking determinism
  job to CI.
- [x] Made retry jitter deterministic for a fixed attempt/config.
- [x] Sorted alias list output in JSON and human modes.
- [x] Sorted recursive local copy traversal.
- [x] Sorted pending multipart upload lookup and canonicalized completed parts.
- [x] Made lifecycle rule IDs deterministic from the target, state and rule body.
- [x] Added `SOURCE_DATE_EPOCH` support to retention timestamp parsing.
- [x] Applied `uninlined_format_args` and other machine-applicable cleanup.
- [x] Added `crates/core/src/ports.rs` with `Clock`, `Entropy` and `Env` ports,
  plus real (`SystemClock`, `OsEntropy`, `ProcessEnv`) and deterministic
  (`FixedClock`, `SeededEntropy`, `MapEnv`) implementations.
- [x] Injected the clock into transfer rate pacing, KMS diagnostics, admin
  health/devnull measurements and SigV4 request signing (admin and XML paths).
- [x] Injected entropy into the KMS diagnostic and injected the environment into
  config directory resolution and alias environment lookup.
- [x] Applied `SOURCE_DATE_EPOCH` to IAM and bucket-metadata ZIP exports.
- [x] Sorted admin user, policy, group, service-account and OIDC provider lists
  returned as unordered maps or arrays, including user group membership.
- [x] Injected the environment port into completion installation path
  resolution.
- [x] Added a CI determinism job that repeats the contract with
  `TOKIO_WORKER_THREADS=1` and `4` under `SOURCE_DATE_EPOCH=0`, and marked the
  strict clippy preview explicitly non-blocking.
- [x] Published the per-lint clippy inventory to the CI job summary.
- [x] Verified the full workspace test suite after the changes.

Still open:

- [ ] Strict clippy has zero warnings (`make clippy-strict` still fails).
- [ ] CI strict clippy is blocking rather than `continue-on-error`.
- [ ] The warning ratchet and test blanket allowances are removed.
- [ ] Clock ports still do not cover the AWS SDK native signing/presigning path,
  and ILM timeout polling uses process time by design.
- [ ] Jitter remains deterministic but has no injectable per-request seed.
- [ ] `std::env` reads remain in secret input and the platform default config
  directory fallback.
- [ ] Observable `HashMap`/`HashSet` iteration is audited and removed.
- [ ] All `read_dir` sites are covered by tests for non-canonical filesystem
  order.
- [ ] `InspectArchiveKey::generate` and archive key wrapping intentionally use
  OS entropy; encrypted diagnostic archive bytes are explicitly non-contractual.
- [ ] `SOURCE_DATE_EPOCH` applies to client-generated ZIP exports only; archive
  formats produced by the server depend on server support.
- [ ] Performance A/B is re-run after the determinism changes.

## 10. Acceptance checklist

- [ ] `cargo clippy --workspace --all-targets -- -D warnings` is clean.
- [ ] `cargo fmt --all -- --check` is clean.
- [ ] `cargo test --workspace` is green.
- [ ] `make determinism` is green in CI on repeated runs.
- [ ] Determinism matrix covers worker counts, TZ, locale, columns and
      filesystem order.
- [ ] No `SystemTime::now`/`Instant::now` in deterministic core/command logic.
- [ ] No `OsRng` in deterministic output paths.
- [ ] No observable `HashMap`/`HashSet` iteration.
- [ ] All `read_dir` traversals are sorted before use.
- [ ] Archive outputs honor `SOURCE_DATE_EPOCH`.
- [ ] JSON/human output contract is documented and byte-tested.
- [ ] The clippy ratchet and test blanket allowances are removed.
- [ ] `clippy.toml` determinism exemptions are removed.
- [ ] CI enforces the strict gate.
- [ ] Performance A/B shows no regression beyond the noise floor.
- [ ] `bench/results/SUMMARY.md` is updated with the new evidence.
- [ ] This document's status becomes **closed** only after the above.
