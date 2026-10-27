# rc A/B performance report

- baseline: `bench/results/baseline-repeat.json`
- candidate: `bench/results/candidate-total-repeat.json`

Latency ratios are candidate/baseline for the median and p95. Lower is better. A negative delta is an improvement.

| grupo | operação | baseline mediana | candidate mediana | Δ mediana | baseline p95 | candidate p95 | Δ p95 |
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
| micro/alias | endpoint reject credentials | 16 ns | 16 ns | +5.81% | 18 ns | 16 ns | -8.84% |
| micro/alias | endpoint validate | 402 ns | 392 ns | -2.44% | 415 ns | 403 ns | -2.91% |
| micro/alias | header parse invalid | 42 ns | 43 ns | +2.63% | 44 ns | 49 ns | +10.96% |
| micro/alias | header parse valid | 60 ns | 61 ns | +2.18% | 64 ns | 70 ns | +10.39% |
| micro/config | parse 128 aliases | 328.53 µs | 331.19 µs | +0.81% | 346.33 µs | 335.66 µs | -3.08% |
| micro/config | serialize 128 aliases | 190.13 µs | 191.19 µs | +0.55% | 195.45 µs | 195.25 µs | -0.10% |
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

Comparable cells: 28. Baselines with no candidate match: 0.
