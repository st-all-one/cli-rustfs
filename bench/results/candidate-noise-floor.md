# rc A/B performance report

- baseline: `bench/results/candidate.json`
- candidate: `bench/results/candidate-repeat.json`

Latency ratios are candidate/baseline for the median and p95. Lower is better. A negative delta is an improvement.

| grupo | operação | baseline mediana | candidate mediana | Δ mediana | baseline p95 | candidate p95 | Δ p95 |
|---|---|---:|---:|---:|---:|---:|---:|
| e2e/alias | list 128 aliases | 7.957 ms | 8.307 ms | +4.40% | 8.924 ms | 9.214 ms | +3.25% |
| e2e/alias | remove | 6.159 ms | 6.251 ms | +1.49% | 7.304 ms | 6.861 ms | -6.07% |
| e2e/alias | set anonymous | 6.455 ms | 6.202 ms | -3.92% | 6.920 ms | 7.014 ms | +1.37% |
| e2e/local | cp local-to-local error | 5.971 ms | 6.191 ms | +3.68% | 6.936 ms | 7.139 ms | +2.93% |
| e2e/s3 | ls 1000 objects json | 26.968 ms | 25.480 ms | -5.52% | 29.247 ms | 27.552 ms | -5.80% |
| e2e/s3 | ls buckets json | 20.682 ms | 20.044 ms | -3.09% | 21.955 ms | 22.033 ms | +0.35% |
| e2e/startup | --help | 5.457 ms | 5.444 ms | -0.24% | 6.173 ms | 5.966 ms | -3.36% |
| e2e/startup | --version | 5.205 ms | 5.372 ms | +3.21% | 6.075 ms | 5.827 ms | -4.09% |
| e2e/startup | completions bash | 28.950 ms | 28.557 ms | -1.36% | 30.407 ms | 31.033 ms | +2.06% |
| micro/alias | endpoint reject credentials | 16 ns | 17 ns | +10.83% | 20 ns | 23 ns | +11.82% |
| micro/alias | endpoint validate | 392 ns | 401 ns | +2.24% | 410 ns | 464 ns | +13.18% |
| micro/alias | header parse invalid | 42 ns | 54 ns | +29.40% | 50 ns | 64 ns | +28.43% |
| micro/alias | header parse valid | 58 ns | 58 ns | +0.00% | 67 ns | 76 ns | +14.07% |
| micro/config | parse 128 aliases | 334.76 µs | 335.65 µs | +0.26% | 339.00 µs | 355.91 µs | +4.99% |
| micro/config | serialize 128 aliases | 190.57 µs | 195.84 µs | +2.77% | 194.22 µs | 200.25 µs | +3.10% |
| micro/object-key | normalize logical colon | 105 ns | 110 ns | +5.24% | 124 ns | 129 ns | +3.38% |
| micro/object-key | normalize logical deep | 266 ns | 282 ns | +5.79% | 271 ns | 303 ns | +11.69% |
| micro/object-key | normalize windows portable | 469 ns | 476 ns | +1.51% | 473 ns | 592 ns | +25.04% |
| micro/object-key | normalize windows reject | 79 ns | 78 ns | -0.89% | 86 ns | 88 ns | +2.10% |
| micro/object-key | relative local path | 153 ns | 153 ns | +0.26% | 160 ns | 161 ns | +0.62% |
| micro/options | metadata validate | 191 ns | 198 ns | +3.67% | 234 ns | 287 ns | +22.63% |
| micro/options | read options version | 7 ns | 7 ns | -1.37% | 8 ns | 7 ns | -1.33% |
| micro/path | parse invalid | 45 ns | 57 ns | +27.80% | 46 ns | 74 ns | +59.09% |
| micro/path | parse local absolute | 11 ns | 13 ns | +11.50% | 12 ns | 16 ns | +41.74% |
| micro/path | parse remote deep | 66 ns | 91 ns | +38.45% | 69 ns | 100 ns | +44.73% |
| micro/path | parse remote shallow | 67 ns | 63 ns | -6.01% | 101 ns | 80 ns | -21.05% |
| micro/retry | classify retryable | 29 ns | 34 ns | +19.93% | 29 ns | 43 ns | +46.74% |
| micro/retry | classify terminal | 3 ns | 3 ns | +0.00% | 3 ns | 3 ns | -14.71% |

Comparable cells: 28. Baselines with no candidate match: 0.
