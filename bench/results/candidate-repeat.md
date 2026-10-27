# rc performance baseline/run

- label: `candidate-post-rigor-repeat`
- rc: `/home/one/projects/one/cli-rustfs/target/release/rc`
- rustc: `rustc 1.99.0 (b940084d7 2026-09-28)`
- git: `25f1660d2a989e4802829d7e4e2b84f846d3d60e`
- started_unix_s: 1793061119

| grupo | operação | amostras | min | mediana | p95 | máx | desvio |
|---|---|---:|---:|---:|---:|---:|---:|
| micro/path | parse remote shallow | 20 | 58 ns | 63 ns | 80 ns | 82 ns | 9 ns |
| micro/path | parse remote deep | 20 | 66 ns | 91 ns | 100 ns | 111 ns | 12 ns |
| micro/path | parse local absolute | 20 | 12 ns | 13 ns | 16 ns | 33 ns | 5 ns |
| micro/path | parse invalid | 20 | 46 ns | 57 ns | 73 ns | 75 ns | 9 ns |
| micro/object-key | normalize logical deep | 20 | 262 ns | 282 ns | 303 ns | 335 ns | 18 ns |
| micro/object-key | normalize logical colon | 20 | 95 ns | 111 ns | 129 ns | 131 ns | 11 ns |
| micro/object-key | normalize windows portable | 20 | 462 ns | 476 ns | 592 ns | 623 ns | 42 ns |
| micro/object-key | normalize windows reject | 20 | 75 ns | 78 ns | 88 ns | 93 ns | 5 ns |
| micro/object-key | relative local path | 20 | 146 ns | 153 ns | 161 ns | 165 ns | 5 ns |
| micro/alias | header parse valid | 20 | 57 ns | 58 ns | 76 ns | 78 ns | 7 ns |
| micro/alias | header parse invalid | 20 | 39 ns | 54 ns | 64 ns | 69 ns | 10 ns |
| micro/alias | endpoint validate | 20 | 379 ns | 401 ns | 464 ns | 546 ns | 38 ns |
| micro/alias | endpoint reject credentials | 20 | 17 ns | 17 ns | 23 ns | 29 ns | 3 ns |
| micro/retry | classify retryable | 20 | 28 ns | 34 ns | 43 ns | 49 ns | 5 ns |
| micro/retry | classify terminal | 20 | 3 ns | 3 ns | 3 ns | 3 ns | 0 ns |
| micro/options | read options version | 20 | 7 ns | 7 ns | 7 ns | 17 ns | 2 ns |
| micro/options | metadata validate | 20 | 180 ns | 198 ns | 287 ns | 293 ns | 35 ns |
| micro/config | serialize 128 aliases | 20 | 183.75 µs | 195.84 µs | 200.25 µs | 208.66 µs | 5.83 µs |
| micro/config | parse 128 aliases | 20 | 328.06 µs | 335.65 µs | 355.91 µs | 369.69 µs | 10.32 µs |
| e2e/startup | --version | 20 | 4.726 ms | 5.372 ms | 5.827 ms | 6.107 ms | 356.11 µs |
| e2e/startup | --help | 20 | 4.980 ms | 5.444 ms | 5.966 ms | 6.096 ms | 290.69 µs |
| e2e/startup | completions bash | 20 | 26.692 ms | 28.557 ms | 31.033 ms | 32.425 ms | 1.528 ms |
| e2e/alias | set anonymous | 20 | 5.727 ms | 6.202 ms | 7.014 ms | 7.787 ms | 500.01 µs |
| e2e/alias | remove | 20 | 5.729 ms | 6.251 ms | 6.861 ms | 6.880 ms | 360.79 µs |
| e2e/alias | list 128 aliases | 20 | 7.286 ms | 8.307 ms | 9.214 ms | 9.506 ms | 586.12 µs |
| e2e/local | cp local-to-local error | 20 | 5.856 ms | 6.191 ms | 7.139 ms | 7.200 ms | 408.60 µs |
| e2e/s3 | ls buckets json | 20 | 18.973 ms | 20.044 ms | 22.033 ms | 22.349 ms | 900.26 µs |
| e2e/s3 | ls 1000 objects json | 20 | 24.165 ms | 25.480 ms | 27.552 ms | 27.628 ms | 870.17 µs |
