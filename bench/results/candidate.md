# rc performance baseline/run

- label: `candidate-post-rigor`
- rc: `/home/one/projects/one/cli-rustfs/target/release/rc`
- rustc: `rustc 1.99.0 (b940084d7 2026-09-28)`
- git: `25f1660d2a989e4802829d7e4e2b84f846d3d60e`
- started_unix_s: 1793061083

| grupo | operação | amostras | min | mediana | p95 | máx | desvio |
|---|---|---:|---:|---:|---:|---:|---:|
| micro/path | parse remote shallow | 15 | 55 ns | 67 ns | 101 ns | 135 ns | 22 ns |
| micro/path | parse remote deep | 15 | 64 ns | 66 ns | 69 ns | 73 ns | 2 ns |
| micro/path | parse local absolute | 15 | 11 ns | 11 ns | 11 ns | 12 ns | 0 ns |
| micro/path | parse invalid | 15 | 44 ns | 45 ns | 46 ns | 49 ns | 1 ns |
| micro/object-key | normalize logical deep | 15 | 260 ns | 266 ns | 271 ns | 272 ns | 4 ns |
| micro/object-key | normalize logical colon | 15 | 96 ns | 105 ns | 124 ns | 132 ns | 10 ns |
| micro/object-key | normalize windows portable | 15 | 459 ns | 469 ns | 473 ns | 480 ns | 5 ns |
| micro/object-key | normalize windows reject | 15 | 78 ns | 79 ns | 86 ns | 86 ns | 3 ns |
| micro/object-key | relative local path | 15 | 150 ns | 153 ns | 160 ns | 160 ns | 4 ns |
| micro/alias | header parse valid | 15 | 58 ns | 58 ns | 67 ns | 67 ns | 3 ns |
| micro/alias | header parse invalid | 15 | 41 ns | 42 ns | 50 ns | 51 ns | 3 ns |
| micro/alias | endpoint validate | 15 | 385 ns | 392 ns | 410 ns | 414 ns | 8 ns |
| micro/alias | endpoint reject credentials | 15 | 15 ns | 16 ns | 20 ns | 21 ns | 2 ns |
| micro/retry | classify retryable | 15 | 28 ns | 29 ns | 29 ns | 33 ns | 1 ns |
| micro/retry | classify terminal | 15 | 3 ns | 3 ns | 3 ns | 4 ns | 0 ns |
| micro/options | read options version | 15 | 7 ns | 7 ns | 8 ns | 8 ns | 0 ns |
| micro/options | metadata validate | 15 | 184 ns | 191 ns | 234 ns | 247 ns | 19 ns |
| micro/config | serialize 128 aliases | 15 | 184.97 µs | 190.57 µs | 194.22 µs | 247.09 µs | 15.09 µs |
| micro/config | parse 128 aliases | 15 | 326.64 µs | 334.76 µs | 338.99 µs | 339.97 µs | 3.95 µs |
| e2e/startup | --version | 15 | 4.591 ms | 5.205 ms | 6.075 ms | 6.993 ms | 626.98 µs |
| e2e/startup | --help | 15 | 4.753 ms | 5.457 ms | 6.173 ms | 6.458 ms | 521.15 µs |
| e2e/startup | completions bash | 15 | 27.135 ms | 28.950 ms | 30.407 ms | 32.141 ms | 1.432 ms |
| e2e/alias | set anonymous | 15 | 5.830 ms | 6.455 ms | 6.920 ms | 7.431 ms | 433.78 µs |
| e2e/alias | remove | 15 | 5.666 ms | 6.159 ms | 7.304 ms | 7.398 ms | 517.13 µs |
| e2e/alias | list 128 aliases | 15 | 7.209 ms | 7.957 ms | 8.924 ms | 9.157 ms | 536.32 µs |
| e2e/local | cp local-to-local error | 15 | 5.609 ms | 5.971 ms | 6.936 ms | 8.319 ms | 694.69 µs |
| e2e/s3 | ls buckets json | 15 | 19.390 ms | 20.682 ms | 21.955 ms | 21.978 ms | 886.01 µs |
| e2e/s3 | ls 1000 objects json | 15 | 25.118 ms | 26.968 ms | 29.247 ms | 29.459 ms | 1.399 ms |
