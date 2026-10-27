# rc performance baseline/run

- label: `total-after`
- rc: `/home/one/projects/one/cli-rustfs/target/release/rc`
- rustc: `rustc 1.99.0 (b940084d7 2026-09-28)`
- git: `25f1660d2a989e4802829d7e4e2b84f846d3d60e`
- started_unix_s: 1793064877

| grupo | operação | amostras | min | mediana | p95 | máx | desvio |
|---|---|---:|---:|---:|---:|---:|---:|
| micro/path | parse remote shallow | 15 | 49 ns | 53 ns | 76 ns | 77 ns | 11 ns |
| micro/path | parse remote deep | 15 | 65 ns | 67 ns | 70 ns | 72 ns | 2 ns |
| micro/path | parse local absolute | 15 | 11 ns | 11 ns | 12 ns | 18 ns | 2 ns |
| micro/path | parse invalid | 15 | 44 ns | 44 ns | 45 ns | 48 ns | 1 ns |
| micro/object-key | normalize logical deep | 15 | 227 ns | 231 ns | 239 ns | 239 ns | 4 ns |
| micro/object-key | normalize logical colon | 15 | 98 ns | 109 ns | 135 ns | 138 ns | 12 ns |
| micro/object-key | normalize windows portable | 15 | 457 ns | 468 ns | 475 ns | 527 ns | 16 ns |
| micro/object-key | normalize windows reject | 15 | 71 ns | 76 ns | 84 ns | 86 ns | 5 ns |
| micro/object-key | relative local path | 15 | 157 ns | 161 ns | 167 ns | 167 ns | 3 ns |
| micro/alias | header parse valid | 15 | 57 ns | 59 ns | 61 ns | 63 ns | 1 ns |
| micro/alias | header parse invalid | 15 | 41 ns | 42 ns | 44 ns | 57 ns | 4 ns |
| micro/alias | endpoint validate | 15 | 365 ns | 373 ns | 378 ns | 444 ns | 19 ns |
| micro/alias | endpoint reject credentials | 15 | 15 ns | 16 ns | 17 ns | 25 ns | 2 ns |
| micro/retry | classify retryable | 15 | 28 ns | 28 ns | 30 ns | 34 ns | 2 ns |
| micro/retry | classify terminal | 15 | 3 ns | 3 ns | 3 ns | 3 ns | 0 ns |
| micro/options | read options version | 15 | 7 ns | 7 ns | 7 ns | 7 ns | 0 ns |
| micro/options | metadata validate | 15 | 190 ns | 197 ns | 222 ns | 230 ns | 11 ns |
| micro/config | serialize 128 aliases | 15 | 190.15 µs | 191.43 µs | 201.69 µs | 206.89 µs | 4.92 µs |
| micro/config | parse 128 aliases | 15 | 327.83 µs | 329.72 µs | 338.92 µs | 339.45 µs | 4.35 µs |
| e2e/startup | --version | 15 | 4.681 ms | 5.663 ms | 5.874 ms | 6.307 ms | 428.27 µs |
| e2e/startup | --help | 15 | 4.871 ms | 5.408 ms | 5.939 ms | 6.071 ms | 360.96 µs |
| e2e/startup | completions bash | 15 | 25.853 ms | 26.941 ms | 28.510 ms | 29.313 ms | 944.62 µs |
| e2e/alias | set anonymous | 15 | 6.020 ms | 6.502 ms | 7.164 ms | 7.293 ms | 411.39 µs |
| e2e/alias | remove | 15 | 5.730 ms | 6.198 ms | 6.895 ms | 6.930 ms | 354.50 µs |
| e2e/alias | list 128 aliases | 15 | 7.376 ms | 8.135 ms | 8.442 ms | 8.661 ms | 302.95 µs |
| e2e/local | cp local-to-local error | 15 | 5.874 ms | 6.420 ms | 6.752 ms | 6.944 ms | 288.92 µs |
| e2e/s3 | ls buckets json | 15 | 15.299 ms | 16.555 ms | 17.611 ms | 20.392 ms | 1.146 ms |
| e2e/s3 | ls 1000 objects json | 15 | 20.693 ms | 22.203 ms | 23.232 ms | 23.389 ms | 796.01 µs |
