# rc performance baseline/run

- label: `total-after-repeat`
- rc: `/home/one/projects/one/cli-rustfs/target/release/rc`
- rustc: `rustc 1.99.0 (b940084d7 2026-09-28)`
- git: `25f1660d2a989e4802829d7e4e2b84f846d3d60e`
- started_unix_s: 1793064883

| grupo | operação | amostras | min | mediana | p95 | máx | desvio |
|---|---|---:|---:|---:|---:|---:|---:|
| micro/path | parse remote shallow | 20 | 49 ns | 54 ns | 57 ns | 58 ns | 3 ns |
| micro/path | parse remote deep | 20 | 64 ns | 64 ns | 70 ns | 71 ns | 2 ns |
| micro/path | parse local absolute | 20 | 11 ns | 11 ns | 11 ns | 11 ns | 0 ns |
| micro/path | parse invalid | 20 | 44 ns | 45 ns | 46 ns | 49 ns | 1 ns |
| micro/object-key | normalize logical deep | 20 | 221 ns | 226 ns | 235 ns | 237 ns | 4 ns |
| micro/object-key | normalize logical colon | 20 | 96 ns | 98 ns | 105 ns | 108 ns | 4 ns |
| micro/object-key | normalize windows portable | 20 | 449 ns | 471 ns | 597 ns | 1.07 µs | 138 ns |
| micro/object-key | normalize windows reject | 20 | 70 ns | 72 ns | 83 ns | 84 ns | 5 ns |
| micro/object-key | relative local path | 20 | 154 ns | 157 ns | 163 ns | 163 ns | 3 ns |
| micro/alias | header parse valid | 20 | 58 ns | 61 ns | 70 ns | 73 ns | 4 ns |
| micro/alias | header parse invalid | 20 | 43 ns | 43 ns | 49 ns | 51 ns | 2 ns |
| micro/alias | endpoint validate | 20 | 386 ns | 392 ns | 403 ns | 406 ns | 5 ns |
| micro/alias | endpoint reject credentials | 20 | 16 ns | 16 ns | 16 ns | 16 ns | 0 ns |
| micro/retry | classify retryable | 20 | 29 ns | 29 ns | 30 ns | 35 ns | 1 ns |
| micro/retry | classify terminal | 20 | 2 ns | 3 ns | 3 ns | 3 ns | 0 ns |
| micro/options | read options version | 20 | 7 ns | 7 ns | 8 ns | 8 ns | 0 ns |
| micro/options | metadata validate | 20 | 189 ns | 193 ns | 202 ns | 203 ns | 4 ns |
| micro/config | serialize 128 aliases | 20 | 189.77 µs | 191.19 µs | 195.25 µs | 212.03 µs | 4.85 µs |
| micro/config | parse 128 aliases | 20 | 328.42 µs | 331.19 µs | 335.66 µs | 339.24 µs | 2.79 µs |
| e2e/startup | --version | 20 | 4.587 ms | 5.399 ms | 5.995 ms | 6.585 ms | 472.08 µs |
| e2e/startup | --help | 20 | 4.814 ms | 5.290 ms | 6.142 ms | 6.274 ms | 463.02 µs |
| e2e/startup | completions bash | 20 | 25.967 ms | 27.513 ms | 29.812 ms | 30.501 ms | 1.221 ms |
| e2e/alias | set anonymous | 20 | 5.608 ms | 6.468 ms | 7.096 ms | 7.181 ms | 419.44 µs |
| e2e/alias | remove | 20 | 5.903 ms | 6.384 ms | 7.296 ms | 8.099 ms | 535.32 µs |
| e2e/alias | list 128 aliases | 20 | 7.128 ms | 7.873 ms | 9.006 ms | 9.175 ms | 572.64 µs |
| e2e/local | cp local-to-local error | 20 | 5.670 ms | 6.202 ms | 6.743 ms | 6.845 ms | 349.09 µs |
| e2e/s3 | ls buckets json | 20 | 15.722 ms | 16.587 ms | 17.529 ms | 19.841 ms | 894.43 µs |
| e2e/s3 | ls 1000 objects json | 20 | 21.240 ms | 22.716 ms | 23.824 ms | 23.834 ms | 744.22 µs |
