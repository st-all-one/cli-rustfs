# rc A/B performance report

- baseline: `bench/results/baseline.json`
- candidate: `bench/results/candidate.json`

Latency ratios are candidate/baseline for the median and p95. Lower is better. A negative delta is an improvement.

| grupo | operação | baseline mediana | candidate mediana | Δ mediana | baseline p95 | candidate p95 | Δ p95 |
|---|---|---:|---:|---:|---:|---:|---:|
| e2e/alias | list 128 aliases | 8.077 ms | 7.957 ms | -1.48% | 8.789 ms | 8.924 ms | +1.54% |
| e2e/alias | remove | 6.476 ms | 6.159 ms | -4.89% | 7.106 ms | 7.304 ms | +2.79% |
| e2e/alias | set anonymous | 6.384 ms | 6.455 ms | +1.11% | 7.481 ms | 6.920 ms | -7.50% |
| e2e/local | cp local-to-local error | 6.323 ms | 5.971 ms | -5.57% | 6.920 ms | 6.936 ms | +0.23% |
| e2e/s3 | ls 1000 objects json | 26.062 ms | 26.968 ms | +3.48% | 27.980 ms | 29.247 ms | +4.53% |
| e2e/s3 | ls buckets json | 20.913 ms | 20.682 ms | -1.10% | 21.798 ms | 21.955 ms | +0.72% |
| e2e/startup | --help | 5.817 ms | 5.457 ms | -6.19% | 6.715 ms | 6.173 ms | -8.06% |
| e2e/startup | --version | 5.576 ms | 5.205 ms | -6.65% | 6.692 ms | 6.075 ms | -9.22% |
| e2e/startup | completions bash | 30.537 ms | 28.950 ms | -5.20% | 32.853 ms | 30.407 ms | -7.44% |
| micro/alias | endpoint reject credentials | 16 ns | 16 ns | +1.29% | 16 ns | 20 ns | +29.30% |
| micro/alias | endpoint validate | 396 ns | 392 ns | -1.11% | 403 ns | 410 ns | +1.64% |
| micro/alias | header parse invalid | 42 ns | 42 ns | -1.19% | 43 ns | 50 ns | +14.81% |
| micro/alias | header parse valid | 61 ns | 58 ns | -5.38% | 68 ns | 67 ns | -1.04% |
| micro/config | parse 128 aliases | 329.09 µs | 334.76 µs | +1.72% | 331.43 µs | 339.00 µs | +2.28% |
| micro/config | serialize 128 aliases | 183.56 µs | 190.57 µs | +3.82% | 197.94 µs | 194.22 µs | -1.88% |
| micro/object-key | normalize logical colon | 105 ns | 105 ns | -0.19% | 125 ns | 124 ns | -0.56% |
| micro/object-key | normalize logical deep | 268 ns | 266 ns | -0.52% | 286 ns | 271 ns | -5.24% |
| micro/object-key | normalize windows portable | 470 ns | 469 ns | -0.30% | 491 ns | 473 ns | -3.64% |
| micro/object-key | normalize windows reject | 75 ns | 79 ns | +5.34% | 88 ns | 86 ns | -2.28% |
| micro/object-key | relative local path | 155 ns | 153 ns | -1.80% | 163 ns | 160 ns | -1.72% |
| micro/options | metadata validate | 188 ns | 191 ns | +1.27% | 202 ns | 234 ns | +15.91% |
| micro/options | read options version | 7 ns | 7 ns | -1.35% | 8 ns | 8 ns | -8.54% |
| micro/path | parse invalid | 45 ns | 45 ns | -1.55% | 49 ns | 46 ns | -6.29% |
| micro/path | parse local absolute | 11 ns | 11 ns | +0.00% | 12 ns | 12 ns | -7.26% |
| micro/path | parse remote deep | 77 ns | 66 ns | -14.21% | 108 ns | 69 ns | -35.71% |
| micro/path | parse remote shallow | 56 ns | 67 ns | +17.88% | 170 ns | 101 ns | -40.33% |
| micro/retry | classify retryable | 28 ns | 29 ns | +1.42% | 28 ns | 29 ns | +2.46% |
| micro/retry | classify terminal | 3 ns | 3 ns | +0.00% | 3 ns | 3 ns | +9.68% |

Comparable cells: 28. Baselines with no candidate match: 0.
