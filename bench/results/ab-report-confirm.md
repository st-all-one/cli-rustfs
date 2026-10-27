# rc A/B performance report

- baseline: `bench/results/baseline-repeat.json`
- candidate: `bench/results/candidate-repeat.json`

Latency ratios are candidate/baseline for the median and p95. Lower is better. A negative delta is an improvement.

| grupo | operação | baseline mediana | candidate mediana | Δ mediana | baseline p95 | candidate p95 | Δ p95 |
|---|---|---:|---:|---:|---:|---:|---:|
| e2e/alias | list 128 aliases | 8.184 ms | 8.307 ms | +1.50% | 9.243 ms | 9.214 ms | -0.31% |
| e2e/alias | remove | 6.591 ms | 6.251 ms | -5.16% | 7.541 ms | 6.861 ms | -9.02% |
| e2e/alias | set anonymous | 6.441 ms | 6.202 ms | -3.71% | 7.571 ms | 7.014 ms | -7.35% |
| e2e/local | cp local-to-local error | 6.572 ms | 6.191 ms | -5.80% | 8.116 ms | 7.139 ms | -12.04% |
| e2e/s3 | ls 1000 objects json | 25.795 ms | 25.480 ms | -1.22% | 28.115 ms | 27.552 ms | -2.00% |
| e2e/s3 | ls buckets json | 21.327 ms | 20.044 ms | -6.02% | 29.950 ms | 22.033 ms | -26.44% |
| e2e/startup | --help | 5.868 ms | 5.444 ms | -7.22% | 6.244 ms | 5.966 ms | -4.45% |
| e2e/startup | --version | 5.500 ms | 5.372 ms | -2.33% | 6.662 ms | 5.827 ms | -12.54% |
| e2e/startup | completions bash | 28.928 ms | 28.557 ms | -1.28% | 31.647 ms | 31.033 ms | -1.94% |
| micro/alias | endpoint reject credentials | 16 ns | 17 ns | +12.26% | 18 ns | 23 ns | +25.41% |
| micro/alias | endpoint validate | 402 ns | 401 ns | -0.32% | 415 ns | 464 ns | +11.68% |
| micro/alias | header parse invalid | 42 ns | 54 ns | +28.16% | 44 ns | 64 ns | +45.43% |
| micro/alias | header parse valid | 60 ns | 58 ns | -2.68% | 64 ns | 76 ns | +20.00% |
| micro/config | parse 128 aliases | 328.53 µs | 335.65 µs | +2.17% | 346.33 µs | 355.91 µs | +2.77% |
| micro/config | serialize 128 aliases | 190.13 µs | 195.84 µs | +3.00% | 195.45 µs | 200.25 µs | +2.46% |
| micro/object-key | normalize logical colon | 98 ns | 110 ns | +12.30% | 105 ns | 129 ns | +22.48% |
| micro/object-key | normalize logical deep | 264 ns | 282 ns | +6.59% | 270 ns | 303 ns | +12.23% |
| micro/object-key | normalize windows portable | 473 ns | 476 ns | +0.72% | 482 ns | 592 ns | +22.91% |
| micro/object-key | normalize windows reject | 78 ns | 78 ns | +0.64% | 92 ns | 88 ns | -4.67% |
| micro/object-key | relative local path | 152 ns | 153 ns | +0.99% | 158 ns | 161 ns | +2.15% |
| micro/options | metadata validate | 186 ns | 198 ns | +6.29% | 200 ns | 287 ns | +42.99% |
| micro/options | read options version | 7 ns | 7 ns | +0.00% | 7 ns | 7 ns | +1.37% |
| micro/path | parse invalid | 45 ns | 57 ns | +27.80% | 49 ns | 74 ns | +48.79% |
| micro/path | parse local absolute | 12 ns | 13 ns | +3.28% | 12 ns | 16 ns | +32.52% |
| micro/path | parse remote deep | 64 ns | 91 ns | +41.46% | 72 ns | 100 ns | +39.50% |
| micro/path | parse remote shallow | 57 ns | 63 ns | +9.25% | 124 ns | 80 ns | -35.36% |
| micro/retry | classify retryable | 29 ns | 34 ns | +17.87% | 31 ns | 43 ns | +37.30% |
| micro/retry | classify terminal | 3 ns | 3 ns | +0.00% | 3 ns | 3 ns | +0.00% |

Comparable cells: 28. Baselines with no candidate match: 0.
