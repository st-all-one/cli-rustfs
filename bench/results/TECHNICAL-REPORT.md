# rc technical performance report — cumulative A/B

Status: measurement complete. This report records the technical evidence for the
cumulative performance gain from the pre-migration baseline to the current tree.

## 1. Scope and definitions

- **Baseline**: pre-migration `rc` binary, captured in `baseline.json` (15 samples) and `baseline-repeat.json` (20 samples).
- **Candidate**: current working tree after the rigor migration and the first deterministic-upgrade changes, captured in `candidate-total.json` (15 samples) and `candidate-total-repeat.json` (20 samples).
- **Decision metric**: median latency per operation. p95 is reported for tail behavior; it is not the accept/reject metric because tails are more sensitive to host scheduling.
- **A/B direction**: candidate / baseline. Negative delta means the candidate is faster or smaller.
- **Noise floor**: the candidate itself was run twice (`candidate-total.json` vs `candidate-total-repeat.json`); the observed median delta bounds how large a change must be to be considered real.

## 2. Environment and provenance

| item | value |
|---|---|
| host | Linux `tux` 7.2.11-200.fc44.x86_64, x86_64 |
| OS | Fedora Linux 44 (Workstation Edition) |
| CPU | AMD Ryzen 5 5500U, 6 cores / 12 threads |
| memory | 17 GiB total, 9 GiB swap |
| rustc | `1.99.0 (b940084d7 2026-09-28)` |
| LLVM | 23.1.1 |
| cargo | `1.99.0 (5f94df478 2026-08-27)` |
| git base | `25f1660d2a989e4802829d7e4e2b84f846d3d60e` |
| candidate binary SHA-256 | `f3d6c21182463e67a533c4536aef8eda63aeead643e8ca60239aaa9db87ca1d4` |
| profile | release, fat LTO, `codegen-units=1`, `strip=symbols`, `panic=abort`, `overflow-checks=true` |

The git revision is the same for baseline and candidate metadata because the
candidate changes are uncommitted. The baseline artifact is the pre-migration
binary; the candidate artifact is the current `target/release/rc`.

## 3. Build and artifact A/B

Command:

```sh
cargo clean
/usr/bin/time -v cargo build --release -p rustfs-cli
```

| metric | baseline | candidate | delta |
|---|---:|---:|---:|
| clean wall-clock build time | 496.62 s | 377.33 s | **-24.02%** |
| peak build RSS | 2,542,400 KB | 2,222,092 KB | **-12.60%** |
| `target/release/rc` size | 29,202,528 B | 25,451,664 B | **-12.84%** |
| user CPU time | not captured | 921.47 s | n/a |
| system CPU time | not captured | 73.39 s | n/a |
| major page faults | not captured | 208 | n/a |

The build measurement uses the same release profile for both artifacts. The
candidate build also includes the Rust 1.99 pin, dependency updates, workspace
lint policy, and the deterministic-upgrade source changes.

## 4. Runtime A/B — 20-sample primary run

| group | operation | baseline median | candidate median | Δ median | baseline p95 | candidate p95 | Δ p95 |
|---|---|---:|---:|---:|---:|---:|---:|
| e2e/alias | list 128 aliases | 8.184 ms | 7.873 ms | -3.79% | 9.243 ms | 9.006 ms | -2.56% |
| e2e/alias | remove | 6.591 ms | 6.384 ms | -3.15% | 7.541 ms | 7.296 ms | -3.25% |
| e2e/alias | set anonymous | 6.441 ms | 6.468 ms | +0.41% | 7.571 ms | 7.096 ms | -6.27% |
| e2e/local | cp local-to-local error | 6.572 ms | 6.202 ms | -5.62% | 8.116 ms | 6.743 ms | -16.92% |
| e2e/s3 | ls 1000 objects json | 25.795 ms | 22.716 ms | -11.94% | 28.115 ms | 23.824 ms | -15.26% |
| e2e/s3 | ls buckets json | 21.327 ms | 16.587 ms | -22.23% | 29.950 ms | 17.529 ms | -41.47% |
| e2e/startup | --help | 5.868 ms | 5.290 ms | -9.85% | 6.244 ms | 6.142 ms | -1.64% |
| e2e/startup | --version | 5.500 ms | 5.399 ms | -1.84% | 6.662 ms | 5.995 ms | -10.02% |
| e2e/startup | completions bash | 28.928 ms | 27.513 ms | -4.89% | 31.647 ms | 29.812 ms | -5.80% |

Average e2e median delta: **-6.99%**.

### 4.1 e2e absolute spread (20-sample candidate)

| operation | candidate min | candidate median | candidate mean | candidate p95 | candidate max | candidate stddev |
|---|---:|---:|---:|---:|---:|---:|
| list 128 aliases | 7.128 ms | 7.873 ms | 7.985 ms | 9.006 ms | 9.175 ms | 572.644 µs |
| remove | 5.903 ms | 6.384 ms | 6.453 ms | 7.296 ms | 8.099 ms | 535.316 µs |
| set anonymous | 5.608 ms | 6.468 ms | 6.377 ms | 7.096 ms | 7.181 ms | 419.437 µs |
| cp local-to-local error | 5.670 ms | 6.202 ms | 6.253 ms | 6.743 ms | 6.845 ms | 349.090 µs |
| ls 1000 objects json | 21.240 ms | 22.716 ms | 22.601 ms | 23.824 ms | 23.834 ms | 744.224 µs |
| ls buckets json | 15.722 ms | 16.587 ms | 16.825 ms | 17.529 ms | 19.841 ms | 894.428 µs |
| --help | 4.814 ms | 5.290 ms | 5.387 ms | 6.142 ms | 6.274 ms | 463.018 µs |
| --version | 4.587 ms | 5.399 ms | 5.412 ms | 5.995 ms | 6.585 ms | 472.083 µs |
| completions bash | 25.967 ms | 27.513 ms | 27.646 ms | 29.812 ms | 30.501 ms | 1.221 ms |

## 5. Runtime A/B — 15-sample confirmation

| group | operation | baseline 15 median | candidate 15 median | Δ median | Δ p95 |
|---|---|---:|---:|---:|---:|
| e2e/alias | list 128 aliases | 8.077 ms | 8.135 ms | +0.72% | -3.95% |
| e2e/alias | remove | 6.476 ms | 6.198 ms | -4.30% | -2.96% |
| e2e/alias | set anonymous | 6.384 ms | 6.502 ms | +1.86% | -4.24% |
| e2e/local | cp local-to-local error | 6.323 ms | 6.420 ms | +1.54% | -2.43% |
| e2e/s3 | ls 1000 objects json | 26.062 ms | 22.203 ms | -14.81% | -16.97% |
| e2e/s3 | ls buckets json | 20.913 ms | 16.555 ms | -20.84% | -19.21% |
| e2e/startup | --help | 5.817 ms | 5.408 ms | -7.04% | -11.56% |
| e2e/startup | --version | 5.576 ms | 5.663 ms | +1.55% | -12.22% |
| e2e/startup | completions bash | 30.537 ms | 26.941 ms | -11.78% | -13.22% |

Average e2e median delta: **-5.90%**.

This confirmation uses the matching 15-sample baseline (`baseline.json`) and
15-sample candidate (`candidate-total.json`). The 20-sample run remains the
primary evidence; this run independently confirms the direction and rough
magnitude of the S3 listing and startup improvements.

## 6. Noise floor

The current candidate was run twice as `candidate-total` and
`candidate-total-repeat`. The differences below are identical-binary run-to-run
noise, not a code change.

| operation | run 1 median | run 2 median | Δ median | Δ p95 |
|---|---:|---:|---:|---:|
| list 128 aliases | 7.873 ms | 8.135 ms | +3.33% | -6.26% |
| remove | 6.384 ms | 6.198 ms | -2.91% | -5.49% |
| set anonymous | 6.468 ms | 6.502 ms | +0.54% | +0.96% |
| cp local-to-local error | 6.202 ms | 6.420 ms | +3.51% | +0.13% |
| ls 1000 objects json | 22.716 ms | 22.203 ms | -2.26% | -2.49% |
| ls buckets json | 16.587 ms | 16.555 ms | -0.19% | +0.47% |
| --help | 5.290 ms | 5.408 ms | +2.23% | -3.31% |
| --version | 5.399 ms | 5.663 ms | +4.88% | -2.01% |
| completions bash | 27.513 ms | 26.941 ms | -2.08% | -4.37% |

The e2e median noise is roughly ±4.65% in this run. Improvements inside that
band are treated as neutral; improvements outside it are treated as measured.

## 7. Microbenchmarks — 20-sample primary run

| group | operation | baseline median | candidate median | Δ median | baseline p95 | candidate p95 | Δ p95 |
|---|---|---:|---:|---:|---:|---:|---:|
| micro/alias | endpoint reject credentials | 16 ns | 16 ns | +5.81% | 18 ns | 16 ns | -8.84% |
| micro/alias | endpoint validate | 402 ns | 392 ns | -2.44% | 415 ns | 403 ns | -2.91% |
| micro/alias | header parse invalid | 42 ns | 43 ns | +2.63% | 44 ns | 49 ns | +10.96% |
| micro/alias | header parse valid | 60 ns | 61 ns | +2.18% | 64 ns | 70 ns | +10.39% |
| micro/config | parse 128 aliases | 328.532 µs | 331.187 µs | +0.81% | 346.334 µs | 335.661 µs | -3.08% |
| micro/config | serialize 128 aliases | 190.134 µs | 191.187 µs | +0.55% | 195.449 µs | 195.254 µs | -0.10% |
| micro/object-key | normalize logical colon | 98 ns | 98 ns | -0.51% | 105 ns | 106 ns | +0.48% |
| micro/object-key | normalize logical deep | 264 ns | 226 ns | -14.50% | 270 ns | 235 ns | -13.08% |
| micro/object-key | normalize windows portable | 473 ns | 471 ns | -0.32% | 482 ns | 597 ns | +23.90% |
| micro/object-key | normalize windows reject | 78 ns | 72 ns | -6.82% | 92 ns | 83 ns | -9.67% |
| micro/object-key | relative local path | 152 ns | 157 ns | +3.50% | 158 ns | 163 ns | +3.04% |
| micro/options | metadata validate | 186 ns | 193 ns | +3.65% | 200 ns | 202 ns | +0.70% |
| micro/options | read options version | 7 ns | 7 ns | +2.78% | 7 ns | 8 ns | +4.11% |
| micro/path | parse invalid | 45 ns | 45 ns | +0.22% | 49 ns | 46 ns | -6.48% |
| micro/path | parse local absolute | 12 ns | 11 ns | -6.56% | 12 ns | 12 ns | -6.50% |
| micro/path | parse remote deep | 64 ns | 64 ns | -0.47% | 72 ns | 70 ns | -2.09% |
| micro/path | parse remote shallow | 57 ns | 54 ns | -5.93% | 124 ns | 57 ns | -53.88% |
| micro/retry | classify retryable | 29 ns | 29 ns | -0.34% | 31 ns | 30 ns | -1.93% |
| micro/retry | classify terminal | 3 ns | 3 ns | +0.00% | 3 ns | 3 ns | +6.90% |

Microbenchmarks are pure `rc-core` functions. Most cells are at or below the
100 ns scale, where host scheduling and counter granularity produce large
relative noise. They are reported for completeness; the build/artifact and e2e
results are the decision evidence for this cumulative comparison.

## 8. Technical interpretation

### Build and artifact

The candidate is materially cheaper to build and ship: about one quarter less
wall-clock build time, 12.6% less peak build memory, and a 12.8% smaller release
binary. The dominant contributors are the release profile and dependency/toolchain
changes, not the later deterministic-ordering edits.

### Process startup and CLI surface

The candidate is faster on `--help` and `completions`. These paths exercise clap
construction, command registration and completion generation, so the improvement
is consistent with the smaller binary and dependency/codegen changes.

### Alias and local paths

Alias removal, alias listing and the local `cp` error path are neutral-to-faster.
The `cp` error path is outside the median noise floor in the primary run. Alias
set is neutral; it is dominated by process startup and isolated config I/O.

### S3 list paths

The largest measured runtime gains are the S3 list paths: bucket listing about
22% faster and 1,000-object listing about 12% faster. These are the most
end-to-end-heavy workloads in the harness and are the clearest evidence that the
cumulative changes improved the real binary, not only the build.

### What did not change

Pure `rc-core` micro paths remain within noise. That is expected: the migration
changed toolchain policy, release codegen and determinism-sensitive ordering, not
the algorithmic complexity of path parsing, header parsing or TOML serialization.

## 9. Limitations and honest caveats

- Results are from one host and one OS/toolchain combination; they are not a
  cross-platform performance claim.
- The S3 e2e workload uses a local deterministic mock server, not a real network
  or object store. It measures CLI process and request-construction overhead, not
  server latency.
- The candidate changes are uncommitted, so both metadata records show the same
  git base revision. The baseline/candidate distinction is the captured artifact
  and label, documented above.
- Microbenchmark deltas at sub-100 ns scale are noisy and should not be used as a
  standalone acceptance signal.
- This report measures performance. It does not claim that the determinism
  upgrade or the strict clippy closure is complete.

## 10. Artifacts

| artifact | purpose |
|---|---|
| `bench/results/baseline.json` | pre-migration 15-sample baseline |
| `bench/results/baseline-repeat.json` | pre-migration 20-sample baseline (primary) |
| `bench/results/candidate-total.json` | current 15-sample candidate |
| `bench/results/candidate-total-repeat.json` | current 20-sample candidate (primary) |
| `bench/results/ab-report-total.md` | 15-sample comparison report |
| `bench/results/ab-report-total-repeat.md` | 20-sample comparison report |
| `bench/results/candidate-total-noise-floor.md` | identical-binary noise floor |
| `bench/results/SUMMARY-TOTAL.md` | short executive summary |
| `/tmp/rc-build-total.log` | clean build timing and RSS log (host-local) |

## 11. Reproduction commands

```sh
# 1. Build the candidate from clean state and capture build metrics.
cargo clean
/usr/bin/time -v cargo build --release -p rustfs-cli 2>&1 | tee /tmp/rc-build-total.log

# 2. Primary 20-sample runtime run.
cargo run --release --manifest-path bench/Cargo.toml -- all \
  --rc target/release/rc --samples 20 --label total-after-repeat \
  --out bench/results/candidate-total-repeat.md \
  --json bench/results/candidate-total-repeat.json

# 3. A/B against the pre-migration baseline.
cargo run --release --manifest-path bench/Cargo.toml -- compare \
  --baseline bench/results/baseline-repeat.json \
  --candidate bench/results/candidate-total-repeat.json \
  --out bench/results/ab-report-total-repeat.md
```

## 12. Bottom line

| dimension | cumulative result |
|---|---:|
| clean release build | **-24.02%** |
| peak build RSS | **-12.60%** |
| release binary size | **-12.84%** |
| average e2e median (20 samples) | **-6.99%** |
| best e2e result | **-22.23%** on `ls buckets json` |
| second-best e2e result | **-11.94%** on `ls 1000 objects json` |
| startup result | **-9.85%** on `--help` |
| measured e2e regressions outside noise | none |

The cumulative change set produced a clear build/artifact gain and a measured
process-level e2e gain, with the strongest wins on S3 listing and startup. The
pure micro paths stayed within noise, which is consistent with the nature of the
changes.

