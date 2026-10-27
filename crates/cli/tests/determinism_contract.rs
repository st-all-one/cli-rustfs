#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    let_underscore_drop,
    reason = "determinism fixtures fail fast with actionable diagnostics"
)]

//! Byte-for-byte determinism contract tests.
//!
//! These are intentionally local-only: no network, no real clock dependence and
//! no shared config. The same command is executed twice with a frozen
//! environment and its stdout, stderr and exit code must match exactly.

use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

fn rc_binary() -> &'static str {
    env!("CARGO_BIN_EXE_rc")
}

fn run_rc(args: &[&str], config_dir: &Path, columns: &str) -> Output {
    let mut command = Command::new(rc_binary());
    for (key, _) in std::env::vars_os() {
        let key = key.to_string_lossy();
        if key.starts_with("RC_") || key.starts_with("AWS_") || key == "RUST_LOG" {
            command.env_remove(key.as_ref());
        }
    }
    command
        .args(args)
        .env("RC_CONFIG_DIR", config_dir)
        .env("HOME", config_dir)
        .env("USERPROFILE", config_dir)
        .env("TZ", "UTC")
        .env("SOURCE_DATE_EPOCH", "0")
        .env("LANG", "C")
        .env("LC_ALL", "C")
        .env("COLUMNS", columns)
        .env("NO_COLOR", "1")
        .env("CLICOLOR", "0")
        .env("TERM", "dumb")
        .current_dir(config_dir)
        .output()
        .expect("execute rc")
}

fn assert_stable(args: &[&str], config_dir: &Path, columns: &str) -> Output {
    let first = run_rc(args, config_dir, columns);
    let second = run_rc(args, config_dir, columns);

    assert_eq!(
        first.status.code(),
        second.status.code(),
        "exit code changed for {args:?}: stderr={}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert_eq!(
        first.stdout,
        second.stdout,
        "stdout changed for {args:?}: {}",
        String::from_utf8_lossy(&first.stdout)
    );
    assert_eq!(
        first.stderr,
        second.stderr,
        "stderr changed for {args:?}: {}",
        String::from_utf8_lossy(&first.stderr)
    );

    first
}

#[test]
fn version_is_byte_stable() {
    let temp = TempDir::new().expect("temp dir");
    let output = assert_stable(&["--version"], temp.path(), "80");
    assert!(output.status.success());
}

#[test]
fn help_is_byte_stable_across_columns() {
    let narrow = TempDir::new().expect("temp dir");
    let wide = TempDir::new().expect("temp dir");

    let narrow_output = assert_stable(&["--help"], narrow.path(), "40");
    let wide_output = assert_stable(&["--help"], wide.path(), "200");

    assert!(narrow_output.status.success());
    assert!(wide_output.status.success());
    assert_eq!(
        narrow_output.stdout, wide_output.stdout,
        "help output must not depend on terminal width"
    );
}

#[test]
fn completions_are_byte_stable() {
    let temp = TempDir::new().expect("temp dir");
    let output = assert_stable(&["completions", "bash"], temp.path(), "80");
    assert!(output.status.success());
}

#[test]
fn alias_list_is_byte_stable() {
    let temp = TempDir::new().expect("temp dir");

    let set = run_rc(
        &[
            "alias",
            "set",
            "zeta",
            "http://localhost:9000",
            "--anonymous",
            "--json",
        ],
        temp.path(),
        "80",
    );
    assert!(
        set.status.success(),
        "alias set failed: {}",
        String::from_utf8_lossy(&set.stderr)
    );

    let set_alpha = run_rc(
        &[
            "alias",
            "set",
            "alpha",
            "http://localhost:9001",
            "--anonymous",
            "--json",
        ],
        temp.path(),
        "80",
    );
    assert!(
        set_alpha.status.success(),
        "alias set failed: {}",
        String::from_utf8_lossy(&set_alpha.stderr)
    );

    let output = assert_stable(&["alias", "list", "--json"], temp.path(), "80");
    assert!(output.status.success());

    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("alias list is JSON");
    let aliases = value["aliases"].as_array().expect("aliases array");
    assert_eq!(aliases[0]["name"], "alpha");
    assert_eq!(aliases[1]["name"], "zeta");
}

#[test]
fn alias_export_is_byte_stable() {
    let temp = TempDir::new().expect("temp dir");

    let set = run_rc(
        &[
            "alias",
            "set",
            "local",
            "http://localhost:9000",
            "--anonymous",
        ],
        temp.path(),
        "80",
    );
    assert!(
        set.status.success(),
        "alias set failed: {}",
        String::from_utf8_lossy(&set.stderr)
    );

    let output = assert_stable(&["alias", "export"], temp.path(), "80");
    assert!(output.status.success());
}

#[test]
fn local_cp_error_is_byte_stable() {
    let temp = TempDir::new().expect("temp dir");
    let source = temp.path().join("missing-source");
    let destination = temp.path().join("destination");
    let source = source.to_str().expect("utf-8 temp path");
    let destination = destination.to_str().expect("utf-8 temp path");

    let output = assert_stable(&["cp", source, destination], temp.path(), "80");
    assert!(!output.status.success(), "missing source must fail");
}
