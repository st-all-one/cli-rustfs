# rc performance baseline/run

- label: `baseline-pre-rigor`
- rc: `/home/one/projects/one/cli-rustfs/target/release/rc`
- rustc: `rustc 1.99.0 (b940084d7 2026-09-28)`
- git: `25f1660d2a989e4802829d7e4e2b84f846d3d60e`
- started_unix_s: 1793059436

| grupo | operação | amostras | min | mediana | p95 | máx | desvio |
|---|---|---:|---:|---:|---:|---:|---:|
| micro/path | parse remote shallow | 15 | 54 ns | 56 ns | 170 ns | 185 ns | 44 ns |
| micro/path | parse remote deep | 15 | 64 ns | 77 ns | 108 ns | 110 ns | 16 ns |
| micro/path | parse local absolute | 15 | 11 ns | 11 ns | 12 ns | 14 ns | 1 ns |
| micro/path | parse invalid | 15 | 44 ns | 45 ns | 49 ns | 51 ns | 2 ns |
| micro/object-key | normalize logical deep | 15 | 259 ns | 268 ns | 286 ns | 317 ns | 15 ns |
| micro/object-key | normalize logical colon | 15 | 97 ns | 105 ns | 125 ns | 132 ns | 9 ns |
| micro/object-key | normalize windows portable | 15 | 461 ns | 470 ns | 491 ns | 505 ns | 12 ns |
| micro/object-key | normalize windows reject | 15 | 73 ns | 75 ns | 88 ns | 102 ns | 8 ns |
| micro/object-key | relative local path | 15 | 146 ns | 155 ns | 163 ns | 167 ns | 5 ns |
| micro/alias | header parse valid | 15 | 59 ns | 61 ns | 68 ns | 70 ns | 3 ns |
| micro/alias | header parse invalid | 15 | 41 ns | 42 ns | 43 ns | 47 ns | 1 ns |
| micro/alias | endpoint validate | 15 | 380 ns | 396 ns | 403 ns | 403 ns | 6 ns |
| micro/alias | endpoint reject credentials | 15 | 15 ns | 16 ns | 16 ns | 16 ns | 0 ns |
| micro/retry | classify retryable | 15 | 28 ns | 28 ns | 28 ns | 33 ns | 1 ns |
| micro/retry | classify terminal | 15 | 3 ns | 3 ns | 3 ns | 3 ns | 0 ns |
| micro/options | read options version | 15 | 7 ns | 7 ns | 8 ns | 8 ns | 0 ns |
| micro/options | metadata validate | 15 | 178 ns | 188 ns | 202 ns | 224 ns | 11 ns |
| micro/config | serialize 128 aliases | 15 | 178.87 µs | 183.56 µs | 197.94 µs | 204.98 µs | 7.64 µs |
| micro/config | parse 128 aliases | 15 | 325.37 µs | 329.09 µs | 331.43 µs | 331.94 µs | 2.05 µs |
| e2e/startup | --version | 15 | 4.905 ms | 5.576 ms | 6.692 ms | 7.223 ms | 629.51 µs |
| e2e/startup | --help | 15 | 5.233 ms | 5.817 ms | 6.715 ms | 7.176 ms | 561.16 µs |
| e2e/startup | completions bash | 15 | 27.092 ms | 30.537 ms | 32.853 ms | 32.935 ms | 1.654 ms |
| e2e/alias | set anonymous | 15 | 5.909 ms | 6.384 ms | 7.481 ms | 7.881 ms | 629.08 µs |
| e2e/alias | remove | 15 | 5.807 ms | 6.476 ms | 7.106 ms | 7.253 ms | 465.55 µs |
| e2e/alias | list 128 aliases | 15 | 7.366 ms | 8.077 ms | 8.789 ms | 9.325 ms | 548.05 µs |
| e2e/local | cp local-to-local error | 15 | 5.694 ms | 6.323 ms | 6.920 ms | 7.596 ms | 506.34 µs |
| e2e/s3 | ls buckets json | 15 | 19.559 ms | 20.913 ms | 21.798 ms | 21.916 ms | 777.87 µs |
| e2e/s3 | ls 1000 objects json | 15 | 24.407 ms | 26.062 ms | 27.980 ms | 28.182 ms | 1.185 ms |
