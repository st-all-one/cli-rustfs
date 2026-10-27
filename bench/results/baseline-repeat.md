# rc performance baseline/run

- label: `baseline-pre-rigor-repeat`
- rc: `/home/one/projects/one/cli-rustfs/target/release/rc`
- rustc: `rustc 1.99.0 (b940084d7 2026-09-28)`
- git: `25f1660d2a989e4802829d7e4e2b84f846d3d60e`
- started_unix_s: 1793059443

| grupo | operação | amostras | min | mediana | p95 | máx | desvio |
|---|---|---:|---:|---:|---:|---:|---:|
| micro/path | parse remote shallow | 20 | 55 ns | 57 ns | 124 ns | 125 ns | 21 ns |
| micro/path | parse remote deep | 20 | 64 ns | 64 ns | 72 ns | 73 ns | 3 ns |
| micro/path | parse local absolute | 20 | 12 ns | 12 ns | 12 ns | 12 ns | 0 ns |
| micro/path | parse invalid | 20 | 44 ns | 45 ns | 49 ns | 50 ns | 2 ns |
| micro/object-key | normalize logical deep | 20 | 255 ns | 264 ns | 270 ns | 280 ns | 6 ns |
| micro/object-key | normalize logical colon | 20 | 95 ns | 98 ns | 105 ns | 110 ns | 4 ns |
| micro/object-key | normalize windows portable | 20 | 461 ns | 473 ns | 482 ns | 483 ns | 5 ns |
| micro/object-key | normalize windows reject | 20 | 76 ns | 78 ns | 92 ns | 97 ns | 6 ns |
| micro/object-key | relative local path | 20 | 143 ns | 151 ns | 158 ns | 161 ns | 5 ns |
| micro/alias | header parse valid | 20 | 57 ns | 60 ns | 64 ns | 66 ns | 2 ns |
| micro/alias | header parse invalid | 20 | 41 ns | 42 ns | 44 ns | 48 ns | 1 ns |
| micro/alias | endpoint validate | 20 | 389 ns | 402 ns | 415 ns | 417 ns | 8 ns |
| micro/alias | endpoint reject credentials | 20 | 15 ns | 16 ns | 18 ns | 20 ns | 1 ns |
| micro/retry | classify retryable | 20 | 28 ns | 29 ns | 31 ns | 35 ns | 2 ns |
| micro/retry | classify terminal | 20 | 3 ns | 3 ns | 3 ns | 3 ns | 0 ns |
| micro/options | read options version | 20 | 7 ns | 7 ns | 7 ns | 13 ns | 1 ns |
| micro/options | metadata validate | 20 | 183 ns | 186 ns | 201 ns | 248 ns | 14 ns |
| micro/config | serialize 128 aliases | 20 | 181.65 µs | 190.13 µs | 195.45 µs | 201.64 µs | 4.86 µs |
| micro/config | parse 128 aliases | 20 | 325.90 µs | 328.53 µs | 346.33 µs | 351.05 µs | 6.53 µs |
| e2e/startup | --version | 20 | 4.920 ms | 5.500 ms | 6.662 ms | 6.749 ms | 482.86 µs |
| e2e/startup | --help | 20 | 5.302 ms | 5.868 ms | 6.244 ms | 6.695 ms | 323.56 µs |
| e2e/startup | completions bash | 20 | 27.403 ms | 28.928 ms | 31.647 ms | 32.843 ms | 1.552 ms |
| e2e/alias | set anonymous | 20 | 6.011 ms | 6.441 ms | 7.571 ms | 7.796 ms | 472.04 µs |
| e2e/alias | remove | 20 | 5.946 ms | 6.591 ms | 7.541 ms | 7.809 ms | 447.10 µs |
| e2e/alias | list 128 aliases | 20 | 7.347 ms | 8.184 ms | 9.243 ms | 10.141 ms | 680.11 µs |
| e2e/local | cp local-to-local error | 20 | 6.085 ms | 6.572 ms | 8.116 ms | 13.380 ms | 1.573 ms |
| e2e/s3 | ls buckets json | 20 | 19.601 ms | 21.327 ms | 29.950 ms | 40.581 ms | 4.897 ms |
| e2e/s3 | ls 1000 objects json | 20 | 24.209 ms | 25.795 ms | 28.115 ms | 29.746 ms | 1.318 ms |
