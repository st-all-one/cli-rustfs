# rc A/B performance report

- baseline: `bench/results/candidate-total.json`
- candidate: `bench/results/candidate-total-repeat.json`

Latency ratios are candidate/baseline for the median and p95. Lower is better. A negative delta is an improvement.

| grupo | operação | baseline mediana | candidate mediana | Δ mediana | baseline p95 | candidate p95 | Δ p95 |
|---|---|---:|---:|---:|---:|---:|---:|
| e2e/alias | list 128 aliases | 8.135 ms | 7.873 ms | -3.22% | 8.442 ms | 9.006 ms | +6.68% |
| e2e/alias | remove | 6.198 ms | 6.384 ms | +2.99% | 6.895 ms | 7.296 ms | +5.81% |
| e2e/alias | set anonymous | 6.502 ms | 6.468 ms | -0.54% | 7.164 ms | 7.096 ms | -0.95% |
| e2e/local | cp local-to-local error | 6.420 ms | 6.202 ms | -3.39% | 6.752 ms | 6.743 ms | -0.13% |
| e2e/s3 | ls 1000 objects json | 22.203 ms | 22.716 ms | +2.31% | 23.232 ms | 23.824 ms | +2.55% |
| e2e/s3 | ls buckets json | 16.555 ms | 16.587 ms | +0.19% | 17.611 ms | 17.529 ms | -0.47% |
| e2e/startup | --help | 5.408 ms | 5.290 ms | -2.18% | 5.939 ms | 6.142 ms | +3.42% |
| e2e/startup | --version | 5.663 ms | 5.399 ms | -4.65% | 5.874 ms | 5.995 ms | +2.05% |
| e2e/startup | completions bash | 26.941 ms | 27.513 ms | +2.12% | 28.510 ms | 29.812 ms | +4.57% |
| micro/alias | endpoint reject credentials | 16 ns | 16 ns | +5.13% | 17 ns | 16 ns | -5.17% |
| micro/alias | endpoint validate | 373 ns | 392 ns | +5.06% | 378 ns | 403 ns | +6.64% |
| micro/alias | header parse invalid | 42 ns | 43 ns | +2.87% | 44 ns | 49 ns | +10.71% |
| micro/alias | header parse valid | 59 ns | 61 ns | +3.05% | 61 ns | 70 ns | +15.30% |
| micro/config | parse 128 aliases | 329.72 µs | 331.19 µs | +0.44% | 338.92 µs | 335.66 µs | -0.96% |
| micro/config | serialize 128 aliases | 191.43 µs | 191.19 µs | -0.13% | 201.69 µs | 195.25 µs | -3.19% |
| micro/object-key | normalize logical colon | 109 ns | 98 ns | -9.85% | 136 ns | 106 ns | -22.14% |
| micro/object-key | normalize logical deep | 231 ns | 226 ns | -2.29% | 238 ns | 235 ns | -1.64% |
| micro/object-key | normalize windows portable | 468 ns | 471 ns | +0.68% | 474 ns | 597 ns | +25.73% |
| micro/object-key | normalize windows reject | 76 ns | 72 ns | -5.11% | 84 ns | 83 ns | -1.19% |
| micro/object-key | relative local path | 161 ns | 157 ns | -2.61% | 167 ns | 163 ns | -2.34% |
| micro/options | metadata validate | 197 ns | 193 ns | -2.18% | 222 ns | 202 ns | -9.10% |
| micro/options | read options version | 7 ns | 7 ns | +1.37% | 7 ns | 8 ns | +2.70% |
| micro/path | parse invalid | 44 ns | 45 ns | +0.90% | 45 ns | 46 ns | +2.21% |
| micro/path | parse local absolute | 11 ns | 11 ns | +0.00% | 12 ns | 12 ns | -0.86% |
| micro/path | parse remote deep | 67 ns | 64 ns | -4.04% | 70 ns | 70 ns | +0.57% |
| micro/path | parse remote shallow | 53 ns | 54 ns | +1.51% | 76 ns | 57 ns | -25.29% |
| micro/retry | classify retryable | 28 ns | 29 ns | +1.75% | 30 ns | 30 ns | +3.39% |
| micro/retry | classify terminal | 3 ns | 3 ns | +0.00% | 3 ns | 3 ns | +0.00% |

Comparable cells: 28. Baselines with no candidate match: 0.
