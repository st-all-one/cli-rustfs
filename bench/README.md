# rc performance bench and A/B harness

An isolated benchmark harness (outside the Cargo workspace, like `fuzz/`) with
**no benchmark framework** and no async runtime of its own. It measures both the
pure `rc-core` paths and the real `rc` binary so a rigor/toolchain change can be
accepted or rejected on evidence.

## What is measured

- **micro** — pure hot paths: path parsing, object-key normalization, alias
  header/endpoint validation, retry classification, read/transfer option
  validation, and TOML config parse/serialize with 128 aliases.
- **e2e** — real process startup (`--version`, `--help`, completions), local
  alias config writes/reads, a local error path, and S3 list operations against
  a deterministic loopback mock S3 server (32 buckets and 1,000 objects).
- The harness records `min`, median, p95, max and standard deviation per cell.
  **Median is the A/B decision metric**; p95 is reported for tail regressions.

## Baseline discipline

A baseline is only useful if its noise floor is known. The repository keeps:

| File | Role |
|---|---|
| `results/baseline.json` | pre-migration baseline |
| `results/baseline-repeat.json` | same binary, second run |
| `results/noise-floor.md` | baseline vs repeat (measurement noise) |
| `results/candidate.json` | post-migration run |
| `results/ab-report.md` | baseline vs candidate |

Accept a change only when the delta exceeds the observed noise floor (roughly
±5% on the median for these workloads). Tail-only changes should be inspected
with p95 rather than hidden by a median.

## Running

```sh
# Build the real binary first.
cargo build --release -p rustfs-cli

# Full run, baseline label.
cargo run --release --manifest-path bench/Cargo.toml -- all \
  --rc target/release/rc --samples 15 \
  --label baseline --out bench/results/baseline.md --json bench/results/baseline.json

# Post-change run.
cargo run --release --manifest-path bench/Cargo.toml -- all \
  --rc target/release/rc --samples 15 \
  --label candidate --out bench/results/candidate.md --json bench/results/candidate.json

# A/B report (candidate / baseline; negative delta is faster).
cargo run --release --manifest-path bench/Cargo.toml -- compare \
  --baseline bench/results/baseline.json \
  --candidate bench/results/candidate.json \
  --out bench/results/ab-report.md
```

`make bench` runs the full path and writes `bench/results/latest.*`.
`make bench-quick` is the 5-sample smoke variant.

## Controls

The e2e commands run with an isolated `RC_CONFIG_DIR`, `NO_COLOR=1`, no
`RC_HOST_*` variables, and a fresh temp directory. The mock S3 server is
loopback-only and returns fixed XML, so results are comparable across runs and
machines without credentials or a live object store.

The bench is observational: it is not part of `make check` or the release gate.
