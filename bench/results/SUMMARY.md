# rc rigor migration — performance A/B and baseline report

Environment: `rustc 1.99.0 (b940084d7 2026-09-28)`, Linux x86_64,
git base `25f1660d2a989e4802829d7e4e2b84f846d3d60e`.
All runtime numbers are on the same host, same release binary layout
(`opt-level=3`, fat LTO, one codegen unit), with isolated config dirs and a
loopback mock S3 server. Raw data: `baseline*.json`, `candidate*.json`.

## Methodology

- **micro**: pure `rc-core` paths, 2,000 ops/sample (200 for 128-alias TOML).
- **e2e**: real `rc` process, 15–20 samples/command.
- **Decision metric**: median latency. p95 is shown for tail regressions.
- **Noise floor**: the same pre-change binary was run twice
  (`baseline.json` vs `baseline-repeat.json`, `noise-floor.md`). Median
  e2e deltas were roughly ±1–6%; sub-100 ns micro cells showed up to ~40%
  run-to-run noise and are not used for accept/reject decisions.

## Build and artifact A/B (clean release build)

| metric | baseline | after | delta |
|---|---:|---:|---:|
| clean `cargo build --release -p rustfs-cli` | 496.62 s | 392.14 s | **-21.0%** |
| peak RSS | 2,542,400 KB | 2,219,148 KB | **-12.7%** |
| `target/release/rc` size | 29,202,528 B | 25,422,800 B | **-12.9%** |

The after-build includes the Rust 1.99 toolchain pin, strict lint policy,
release `panic=abort`, `overflow-checks=true`, fat LTO, and dependency updates
(`zip` 9.1.2, tokio 1.54.1, rustls 0.23.46, rustls-webpki 0.103.16,
webpki-roots 1.0.11).

## Runtime A/B (baseline repeat → candidate repeat)

| group | operation | baseline median | candidate median | Δ median | Δ p95 |
|---|---|---:|---:|---:|---:|
| e2e/startup | `--help` | 5.868 ms | 5.444 ms | -7.22% | -4.45% |
| e2e/startup | `--version` | 5.500 ms | 5.372 ms | -2.33% | -12.54% |
| e2e/startup | `completions bash` | 28.928 ms | 28.557 ms | -1.28% | -1.94% |
| e2e/alias | set anonymous | 6.441 ms | 6.202 ms | -3.71% | -7.35% |
| e2e/alias | remove | 6.591 ms | 6.251 ms | -5.16% | -9.02% |
| e2e/alias | list 128 aliases | 8.184 ms | 8.307 ms | +1.50% | -0.31% |
| e2e/local | cp local-to-local error | 6.572 ms | 6.191 ms | -5.80% | -12.04% |
| e2e/s3 | ls buckets json | 21.327 ms | 20.044 ms | -6.02% | -26.44% |
| e2e/s3 | ls 1000 objects json | 25.795 ms | 25.480 ms | -1.22% | -2.00% |

Result: no measured regression on the ms-scale e2e paths; startup, alias
writes, the local error path and S3 bucket listing improved. Micro-path
differences are inside the measured run-to-run noise floor, which is expected:
the rigor migration changes compile-time policy and release codegen, not the
algorithms on those paths.

## Rigor / determinism changes adopted

- `rust-toolchain.toml` pinned to Rust 1.99; `rust-version` and `clippy.toml`
  MSRV aligned.
- `rustfmt.toml` with Edition/Style Edition 2024, 100-column width, Unix
  newlines, lowercase hex literals, field-init and `?` shorthands.
- `[workspace.lints]` with deny-first correctness/suspicious/perf/style groups,
  panic/unwrap/expect bans, unsafe documentation, disallowed macros/methods,
  and a documented visible ratchet for pedantic/nursery findings.
- `clippy.toml` adopted from knudge and adapted to rc: test panic allowances,
  async-friendly lock guards, project vocabulary, and disallowed APIs.
- Release profile: fat LTO, one codegen unit, `strip="symbols"`,
  `overflow-checks=true`, `panic="abort"`; dev keeps debug assertions and
  overflow checks.
- `deny.toml` expanded to advisories + licenses + bans + sources. `zip` upgraded
  to 9.1.2; remaining transitive advisories are explicitly documented.
- `Makefile` quality gate (`fmt`, `clippy`, `test`, file-length) plus
  `deny`/`audit`/`machete`/`typos`/`bench` targets.
- CI pinned to 1.99 across fmt/clippy/test/build/doc/MSRV; clippy still fails on
  denied lints but keeps pedantic findings as a visible ratchet.
- `.gitattributes` for deterministic LF, `.gitignore` for the isolated bench
  tree, `_typos.toml` for domain vocabulary, `bench/` harness and A/B comparator.

## Verification after migration

| check | result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --workspace --all-targets` | exit 0 (denied lints clean; pedantic warnings remain visible) |
| `cargo test --workspace` | 2,686 passed, 0 failed (66 test binaries) |
| `cargo doc --workspace --no-deps` | pass |
| `cargo deny check` | advisories/bans/licenses/sources ok |
| `cargo machete crates/cli crates/core crates/s3` | no unused dependencies |
| `typos` | pass |
