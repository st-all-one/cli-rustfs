# rc A/B performance report

- baseline: `bench/results/baseline.json`
- candidate: `bench/results/candidate-total.json`

Latency ratios are candidate/baseline for the median and p95. Lower is better. A negative delta is an improvement.

| grupo | operação | baseline mediana | candidate mediana | Δ mediana | baseline p95 | candidate p95 | Δ p95 |
|---|---|---:|---:|---:|---:|---:|---:|
| e2e/alias | list 128 aliases | 8.077 ms | 8.135 ms | +0.72% | 8.789 ms | 8.442 ms | -3.95% |
| e2e/alias | remove | 6.476 ms | 6.198 ms | -4.30% | 7.106 ms | 6.895 ms | -2.96% |
| e2e/alias | set anonymous | 6.384 ms | 6.502 ms | +1.86% | 7.481 ms | 7.164 ms | -4.24% |
| e2e/local | cp local-to-local error | 6.323 ms | 6.420 ms | +1.54% | 6.920 ms | 6.752 ms | -2.43% |
| e2e/s3 | ls 1000 objects json | 26.062 ms | 22.203 ms | -14.81% | 27.980 ms | 23.232 ms | -16.97% |
| e2e/s3 | ls buckets json | 20.913 ms | 16.555 ms | -20.84% | 21.798 ms | 17.611 ms | -19.21% |
| e2e/startup | --help | 5.817 ms | 5.408 ms | -7.04% | 6.715 ms | 5.939 ms | -11.56% |
| e2e/startup | --version | 5.576 ms | 5.663 ms | +1.55% | 6.692 ms | 5.874 ms | -12.22% |
| e2e/startup | completions bash | 30.537 ms | 26.941 ms | -11.78% | 32.853 ms | 28.510 ms | -13.22% |
| micro/alias | endpoint reject credentials | 16 ns | 16 ns | +0.65% | 16 ns | 17 ns | +10.83% |
| micro/alias | endpoint validate | 396 ns | 373 ns | -5.80% | 403 ns | 378 ns | -6.23% |
| micro/alias | header parse invalid | 42 ns | 42 ns | -0.48% | 43 ns | 44 ns | +1.62% |
| micro/alias | header parse valid | 61 ns | 59 ns | -3.59% | 68 ns | 61 ns | -9.93% |
| micro/config | parse 128 aliases | 329.09 µs | 329.72 µs | +0.19% | 331.43 µs | 338.92 µs | +2.26% |
| micro/config | serialize 128 aliases | 183.56 µs | 191.43 µs | +4.28% | 197.94 µs | 201.69 µs | +1.90% |
| micro/object-key | normalize logical colon | 105 ns | 109 ns | +3.23% | 125 ns | 136 ns | +8.31% |
| micro/object-key | normalize logical deep | 268 ns | 231 ns | -13.60% | 286 ns | 238 ns | -16.67% |
| micro/object-key | normalize windows portable | 470 ns | 468 ns | -0.51% | 491 ns | 474 ns | -3.40% |
| micro/object-key | normalize windows reject | 75 ns | 76 ns | +1.87% | 88 ns | 84 ns | -4.32% |
| micro/object-key | relative local path | 155 ns | 161 ns | +3.60% | 163 ns | 167 ns | +2.15% |
| micro/options | metadata validate | 188 ns | 197 ns | +4.67% | 202 ns | 222 ns | +10.11% |
| micro/options | read options version | 7 ns | 7 ns | -1.35% | 8 ns | 7 ns | -9.76% |
| micro/path | parse invalid | 45 ns | 44 ns | -2.21% | 49 ns | 45 ns | -8.32% |
| micro/path | parse local absolute | 11 ns | 11 ns | +0.88% | 12 ns | 12 ns | -6.45% |
| micro/path | parse remote deep | 77 ns | 67 ns | -12.91% | 108 ns | 70 ns | -35.06% |
| micro/path | parse remote shallow | 56 ns | 53 ns | -6.02% | 170 ns | 76 ns | -55.01% |
| micro/retry | classify retryable | 28 ns | 28 ns | +1.06% | 28 ns | 30 ns | +3.87% |
| micro/retry | classify terminal | 3 ns | 3 ns | +0.00% | 3 ns | 3 ns | +0.00% |

Comparable cells: 28. Baselines with no candidate match: 0.
