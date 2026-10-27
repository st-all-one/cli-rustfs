# rc A/B performance report

- baseline: `bench/results/baseline.json`
- candidate: `bench/results/baseline-repeat.json`

Latency ratios are candidate/baseline for the median and p95. Lower is better. A negative delta is an improvement.

| grupo | operação | baseline mediana | candidate mediana | Δ mediana | baseline p95 | candidate p95 | Δ p95 |
|---|---|---:|---:|---:|---:|---:|---:|
| e2e/alias | list 128 aliases | 8.077 ms | 8.184 ms | +1.32% | 8.789 ms | 9.243 ms | +5.16% |
| e2e/alias | remove | 6.476 ms | 6.591 ms | +1.78% | 7.106 ms | 7.541 ms | +6.13% |
| e2e/alias | set anonymous | 6.384 ms | 6.441 ms | +0.89% | 7.481 ms | 7.571 ms | +1.20% |
| e2e/local | cp local-to-local error | 6.323 ms | 6.572 ms | +3.93% | 6.920 ms | 8.116 ms | +17.29% |
| e2e/s3 | ls 1000 objects json | 26.062 ms | 25.795 ms | -1.02% | 27.980 ms | 28.115 ms | +0.48% |
| e2e/s3 | ls buckets json | 20.913 ms | 21.327 ms | +1.98% | 21.798 ms | 29.950 ms | +37.40% |
| e2e/startup | --help | 5.817 ms | 5.868 ms | +0.87% | 6.715 ms | 6.244 ms | -7.01% |
| e2e/startup | --version | 5.576 ms | 5.500 ms | -1.37% | 6.692 ms | 6.662 ms | -0.44% |
| e2e/startup | completions bash | 30.537 ms | 28.928 ms | -5.27% | 32.853 ms | 31.647 ms | -3.67% |
| micro/alias | endpoint reject credentials | 16 ns | 16 ns | +0.00% | 16 ns | 18 ns | +15.29% |
| micro/alias | endpoint validate | 396 ns | 402 ns | +1.44% | 403 ns | 415 ns | +3.00% |
| micro/alias | header parse invalid | 42 ns | 42 ns | -0.24% | 43 ns | 44 ns | +1.39% |
| micro/alias | header parse valid | 61 ns | 60 ns | -2.77% | 68 ns | 64 ns | -5.93% |
| micro/config | parse 128 aliases | 329.09 µs | 328.53 µs | -0.17% | 331.43 µs | 346.33 µs | +4.50% |
| micro/config | serialize 128 aliases | 183.56 µs | 190.13 µs | +3.58% | 197.94 µs | 195.45 µs | -1.26% |
| micro/object-key | normalize logical colon | 105 ns | 98 ns | -6.46% | 125 ns | 105 ns | -16.07% |
| micro/object-key | normalize logical deep | 268 ns | 264 ns | -1.27% | 286 ns | 270 ns | -5.70% |
| micro/object-key | normalize windows portable | 470 ns | 473 ns | +0.49% | 491 ns | 482 ns | -1.97% |
| micro/object-key | normalize windows reject | 75 ns | 78 ns | +3.74% | 88 ns | 92 ns | +4.66% |
| micro/object-key | relative local path | 155 ns | 152 ns | -2.51% | 163 ns | 158 ns | -3.19% |
| micro/options | metadata validate | 188 ns | 186 ns | -1.22% | 202 ns | 200 ns | -0.59% |
| micro/options | read options version | 7 ns | 7 ns | -2.70% | 8 ns | 7 ns | -10.98% |
| micro/path | parse invalid | 45 ns | 45 ns | -1.55% | 49 ns | 49 ns | +0.20% |
| micro/path | parse local absolute | 11 ns | 12 ns | +7.96% | 12 ns | 12 ns | -0.81% |
| micro/path | parse remote deep | 77 ns | 64 ns | -16.04% | 108 ns | 72 ns | -33.30% |
| micro/path | parse remote shallow | 56 ns | 57 ns | +1.42% | 170 ns | 124 ns | -27.12% |
| micro/retry | classify retryable | 28 ns | 29 ns | +3.19% | 28 ns | 31 ns | +9.51% |
| micro/retry | classify terminal | 3 ns | 3 ns | +0.00% | 3 ns | 3 ns | -6.45% |

Comparable cells: 28. Baselines with no candidate match: 0.
