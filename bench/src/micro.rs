//! Microbenchmarks for pure `rc-core` paths that sit on every command.
//!
//! These are intentionally allocation-stable workloads: the same input is
//! reused across samples so changes in allocator noise are visible in the
//! stddev instead of being hidden by fixture generation.

use std::hint::black_box;

use rc_core::config::Defaults;
use rc_core::{
    Alias, Config, Error, ObjectAttributes, ObjectKeyPolicy, ObjectReadOptions, RequestHeader, is_retryable_error, normalize_relative_key, parse_path,
    relative_local_path_from_key, validate_alias_endpoint,
};

use crate::harness::Harness;

pub fn run(harness: &mut Harness, samples: usize) {
    const OPS: u64 = 2_000;
    const HEAVY_OPS: u64 = 200;

    let remote_shallow = "local/bucket";
    let remote_deep = "local/bucket/a/b/c/d/e/f/object.tar.zst";
    let local_absolute = "/var/tmp/archive/object.bin";
    let invalid_path = "local//";
    let colon_key = "loki/fake/deadbeef/19f6abd9af4:19f6abe0e77:499628ff";
    let deep_key = "logs/2026/10/26/host-01/application/service/worker/part-00042.jsonl";
    let windows_unsafe = "CON.txt";
    let valid_header = "x-amz-meta-tenant:acme";
    let invalid_header = "authorization:secret";
    let endpoint = "https://s3.example.com:9443";
    let bad_endpoint = "https://user:pass@example.com/{}";
    let retryable = Error::Network("connection reset by peer".to_string());
    let non_retryable = Error::InvalidPath("../escape".to_string());

    harness.measure("micro/path", "parse remote shallow", samples, OPS, || {
        let parsed = parse_path(black_box(remote_shallow));
        black_box(parsed.is_ok());
    });
    harness.measure("micro/path", "parse remote deep", samples, OPS, || {
        let parsed = parse_path(black_box(remote_deep));
        black_box(parsed.is_ok());
    });
    harness.measure("micro/path", "parse local absolute", samples, OPS, || {
        let parsed = parse_path(black_box(local_absolute));
        black_box(parsed.is_ok());
    });
    harness.measure("micro/path", "parse invalid", samples, OPS, || {
        let parsed = parse_path(black_box(invalid_path));
        black_box(parsed.is_err());
    });

    harness.measure(
        "micro/object-key",
        "normalize logical deep",
        samples,
        OPS,
        || {
            let key = normalize_relative_key(black_box(deep_key), ObjectKeyPolicy::Logical);
            black_box(key.is_ok());
        },
    );
    harness.measure(
        "micro/object-key",
        "normalize logical colon",
        samples,
        OPS,
        || {
            let key = normalize_relative_key(black_box(colon_key), ObjectKeyPolicy::Logical);
            black_box(key.is_ok());
        },
    );
    harness.measure(
        "micro/object-key",
        "normalize windows portable",
        samples,
        OPS,
        || {
            let key = normalize_relative_key(black_box(deep_key), ObjectKeyPolicy::WindowsPortable);
            black_box(key.is_ok());
        },
    );
    harness.measure(
        "micro/object-key",
        "normalize windows reject",
        samples,
        OPS,
        || {
            let key =
                normalize_relative_key(black_box(windows_unsafe), ObjectKeyPolicy::WindowsPortable);
            black_box(key.is_err());
        },
    );
    harness.measure(
        "micro/object-key",
        "relative local path",
        samples,
        OPS,
        || {
            let path = relative_local_path_from_key(
                black_box("local/bucket/a/b/c/object.bin"),
                black_box("local/bucket/"),
                ObjectKeyPolicy::Logical,
            );
            black_box(path.is_ok());
        },
    );

    harness.measure("micro/alias", "header parse valid", samples, OPS, || {
        let header = RequestHeader::parse(black_box(valid_header));
        black_box(header.is_ok());
    });
    harness.measure("micro/alias", "header parse invalid", samples, OPS, || {
        let header = RequestHeader::parse(black_box(invalid_header));
        black_box(header.is_err());
    });
    harness.measure("micro/alias", "endpoint validate", samples, OPS, || {
        let endpoint = validate_alias_endpoint(black_box(endpoint));
        black_box(endpoint.is_ok());
    });
    harness.measure(
        "micro/alias",
        "endpoint reject credentials",
        samples,
        OPS,
        || {
            let endpoint = validate_alias_endpoint(black_box(bad_endpoint));
            black_box(endpoint.is_err());
        },
    );

    harness.measure("micro/retry", "classify retryable", samples, OPS, || {
        black_box(is_retryable_error(black_box(&retryable)));
    });
    harness.measure("micro/retry", "classify terminal", samples, OPS, || {
        black_box(is_retryable_error(black_box(&non_retryable)));
    });

    harness.measure("micro/options", "read options version", samples, OPS, || {
        let options = ObjectReadOptions::for_version(Some(black_box("v-123".to_string())));
        black_box(options.is_ok());
    });

    let attributes = ObjectAttributes {
        content_type: Some("application/json".to_string()),
        cache_control: Some("max-age=60".to_string()),
        user_metadata: [
            ("tenant".to_string(), "acme".to_string()),
            ("trace-id".to_string(), "abc123".to_string()),
        ]
        .into_iter()
        .collect(),
        ..ObjectAttributes::default()
    };
    harness.measure(
        "micro/options",
        "metadata validate",
        samples,
        OPS,
        || {
            black_box(black_box(&attributes).validate().is_ok());
        },
    );

    let mut config = Config {
        schema_version: 1,
        defaults: Defaults::default(),
        aliases: Vec::with_capacity(128),
    };
    for index in 0..128 {
        let mut alias = Alias::new(
            format!("alias-{index:03}"),
            format!("http://127.0.0.1:{}/", 9000 + index),
            "access",
            "secret",
        );
        alias.anonymous = true;
        config.aliases.push(alias);
    }
    let config_toml = toml::to_string(&config).expect("bench config should serialize");
    let config_text = config_toml.as_str();

    harness.measure("micro/config", "serialize 128 aliases", samples, HEAVY_OPS, || {
        let text = toml::to_string(black_box(&config));
        black_box(text.is_ok());
    });
    harness.measure("micro/config", "parse 128 aliases", samples, HEAVY_OPS, || {
        let parsed: Result<Config, _> = toml::from_str(black_box(config_text));
        black_box(parsed.is_ok());
    });
}
