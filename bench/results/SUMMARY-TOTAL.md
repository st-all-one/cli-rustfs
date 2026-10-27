# rc total performance gain — final A/B

This is the final A/B of the **pre-migration baseline** against the current
working tree after the rigor migration and the first deterministic-upgrade
changes.

- baseline binary: pre-migration `rc`
- candidate binary: current `target/release/rc`
- host: Linux x86_64
- rustc: `1.99.0 (b940084d7 2026-09-28)`
- decision metric: median; p95 used for tail validation
- runtime samples: baseline 20, candidate 20 (main comparison)
- detailed report: `TECHNICAL-REPORT.md`
- raw data:
  - `baseline-repeat.json`
  - `candidate-total-repeat.json`
  - `ab-report-total-repeat.md`
  - `candidate-total-noise-floor.md`

## Build and artifact gain

Clean release build (`cargo clean` followed by
`cargo build --release -p rustfs-cli`):

| metric | baseline | current | delta |
|---|---:|---:|---:|
| wall-clock build time | 496.62 s | 377.33 s | **-24.02%** |
| peak RSS | 2,542,400 KB | 2,222,092 KB | **-12.60%** |
| `target/release/rc` size | 29,202,528 B | 25,451,664 B | **-12.84%** |

The build gain comes from the cumulative change set: Rust 1.99 pin, workspace
lint policy, release profile (`fat` LTO, one codegen unit, `panic=abort`,
symbol stripping), dependency upgrades and the subsequent cleanup changes.

## Runtime A/B (20-sample run)

Candidate / baseline; negative is faster.

| group | operation | baseline median | candidate median | Δ median | Δ p95 |
|---|---|---:|---:|---:|---:|
| e2e/startup | `--help` | 5.868 ms | 5.290 ms | **-9.85%** | -1.64% |
| e2e/startup | `--version` | 5.500 ms | 5.399 ms | -1.84% | -10.02% |
| e2e/startup | `completions bash` | 28.928 ms | 27.513 ms | -4.89% | -5.80% |
| e2e/alias | set anonymous | 6.441 ms | 6.468 ms | +0.41% | -6.27% |
| e2e/alias | remove | 6.591 ms | 6.384 ms | -3.15% | -3.25% |
| e2e/alias | list 128 aliases | 8.184 ms | 7.873 ms | -3.79% | -2.56% |
| e2e/local | cp local-to-local error | 6.572 ms | 6.202 ms | **-5.62%** | -16.92% |
| e2e/s3 | ls buckets json | 21.327 ms | 16.587 ms | **-22.23%** | -41.47% |
| e2e/s3 | ls 1000 objects json | 25.795 ms | 22.716 ms | **-11.94%** | -15.26% |

Average e2e median delta: **-6.99%**.

The current run-to-run noise floor (`candidate-total-noise-floor.md`) is roughly
±4.65% on e2e medians, so the startup, local-error, object-list and bucket-list
gains are outside noise. The alias-set delta is neutral within noise.

The 15-sample run (`ab-report-total.md`) independently shows the same direction:
`ls buckets` -20.84%, `ls 1000 objects` -14.81%, `--help` -7.04%,
`completions` -11.78%, with an average e2e median delta of -5.90%.

## Microbenchmarks

Pure `rc-core` paths remain effectively unchanged within measurement noise:
config parse/serialize is within ±1%, option validation within a few percent,
and sub-100 ns cells fluctuate by double-digit percentages between identical
runs. The improvement is therefore in build/artifact cost and process-level
e2e paths, not in these pure CPU functions.

## Total result

- clean build: **-24.0%**
- peak build memory: **-12.6%**
- release binary: **-12.8%**
- e2e median performance: **~7% faster on average**
- largest runtime wins: S3 bucket listing **-22%**, S3 object listing **-12%**,
  `--help` **-10%**
- no measured e2e regression outside the noise floor

This is the total observed gain from the pre-migration baseline to the current
tree. Determinism work is not claimed complete here; this report measures the
cumulative performance effect of the changes made so far.
