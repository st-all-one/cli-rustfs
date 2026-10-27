//! rc performance bench and A/B comparator.
//!
//! ```sh
//! cargo run --release --manifest-path bench/Cargo.toml -- all \
//!   --rc target/release/rc --samples 10 \
//!   --out bench/results/baseline.md --json bench/results/baseline.json \
//!   --label baseline
//!
//! cargo run --release --manifest-path bench/Cargo.toml -- compare \
//!   --baseline bench/results/baseline.json \
//!   --candidate bench/results/candidate.json \
//!   --out bench/results/ab-report.md
//! ```

mod e2e;
mod harness;
mod micro;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use harness::{Harness, Meta, fmt_ns};

fn main() {
    let args = parse_args();
    match args.mode.as_str() {
        "micro" | "e2e" | "all" => run_bench(&args),
        "compare" => run_compare(&args),
        other => {
            eprintln!("unknown mode: {other}");
            std::process::exit(2);
        }
    }
}

struct Args {
    mode: String,
    rc: PathBuf,
    samples: usize,
    out: Option<PathBuf>,
    json: Option<PathBuf>,
    baseline: Option<PathBuf>,
    candidate: Option<PathBuf>,
    label: String,
}

fn parse_args() -> Args {
    let mut args = Args {
        mode: "all".to_string(),
        rc: PathBuf::from("target/release/rc"),
        samples: 10,
        out: None,
        json: None,
        baseline: None,
        candidate: None,
        label: "unlabeled".to_string(),
    };

    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut index = 0;
    while index < raw.len() {
        let arg = raw[index].as_str();
        let next = |index: &mut usize| -> String {
            *index += 1;
            raw.get(*index).cloned().unwrap_or_default()
        };
        match arg {
            "micro" | "e2e" | "all" | "compare" => args.mode = arg.to_string(),
            "--rc" => args.rc = PathBuf::from(next(&mut index)),
            "--samples" => {
                args.samples = next(&mut index).parse().unwrap_or(args.samples);
            }
            "--out" => args.out = Some(PathBuf::from(next(&mut index))),
            "--json" => args.json = Some(PathBuf::from(next(&mut index))),
            "--baseline" => args.baseline = Some(PathBuf::from(next(&mut index))),
            "--candidate" => args.candidate = Some(PathBuf::from(next(&mut index))),
            "--label" => args.label = next(&mut index),
            "--help" | "-h" => {
                println!(
                    "usage: rc-bench [micro|e2e|all|compare] [--rc PATH] [--samples N] [--label NAME] [--out report.md] [--json out.json] [--baseline before.json] [--candidate after.json]"
                );
                std::process::exit(0);
            }
            other => {
                eprintln!("unknown argument: {other}");
                std::process::exit(2);
            }
        }
        index += 1;
    }
    args
}

fn run_bench(args: &Args) {
    let rc = if args.rc.is_absolute() {
        args.rc.clone()
    } else {
        std::env::current_dir()
            .unwrap_or_default()
            .join(&args.rc)
    };
    if !rc.exists() {
        eprintln!(
            "rc binary not found at {}; build it with `cargo build --release -p rustfs-cli`",
            rc.display()
        );
        std::process::exit(2);
    }

    let meta = Meta::capture(args.label.clone(), rc.display().to_string());
    let mut harness = Harness::new(meta);
    let samples = args.samples.max(1);

    if args.mode == "micro" || args.mode == "all" {
        eprintln!("== microbenchmarks ({} samples) ==", samples);
        micro::run(&mut harness, samples);
    }
    if args.mode == "e2e" || args.mode == "all" {
        eprintln!("== end-to-end ({} samples) ==", samples);
        e2e::run(&mut harness, &rc, samples);
    }

    let markdown = harness.render_markdown();
    print!("{markdown}");
    if let Some(path) = &args.out {
        write_file(path, &markdown);
    }
    if let Some(path) = &args.json {
        write_file(path, &harness.render_json());
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(path, contents).expect("write benchmark output");
}

fn run_compare(args: &Args) {
    let baseline_path = args
        .baseline
        .as_ref()
        .expect("compare requires --baseline before.json");
    let candidate_path = args
        .candidate
        .as_ref()
        .expect("compare requires --candidate after.json");

    let baseline = load_cells(baseline_path);
    let candidate = load_cells(candidate_path);

    let mut report = String::new();
    report.push_str("# rc A/B performance report\n\n");
    report.push_str(&format!(
        "- baseline: `{}`\n- candidate: `{}`\n\n",
        baseline_path.display(),
        candidate_path.display()
    ));
    report.push_str(
        "Latency ratios are candidate/baseline for the median and p95. Lower is better. \
         A negative delta is an improvement.\n\n",
    );
    report.push_str(
        "| grupo | operação | baseline mediana | candidate mediana | Δ mediana | baseline p95 | candidate p95 | Δ p95 |\n",
    );
    report.push_str("|---|---|---:|---:|---:|---:|---:|---:|\n");

    let mut shared = 0_usize;
    for (key, before) in &baseline {
        let Some(after) = candidate.get(key) else {
            continue;
        };
        shared += 1;
        let median_delta = percent_delta(before.median_ns, after.median_ns);
        let p95_delta = percent_delta(before.p95_ns, after.p95_ns);
        report.push_str(&format!(
            "| {} | {} | {} | {} | {:+.2}% | {} | {} | {:+.2}% |\n",
            key.0,
            key.1,
            fmt_ns(before.median_ns),
            fmt_ns(after.median_ns),
            median_delta,
            fmt_ns(before.p95_ns),
            fmt_ns(after.p95_ns),
            p95_delta,
        ));
    }
    report.push_str(&format!(
        "\nComparable cells: {shared}. Baselines with no candidate match: {}.\n",
        baseline.len().saturating_sub(shared)
    ));

    print!("{report}");
    if let Some(path) = &args.out {
        write_file(path, &report);
    }
}

type CellKey = (String, String);

struct Stats {
    median_ns: f64,
    p95_ns: f64,
}

fn load_cells(path: &Path) -> BTreeMap<CellKey, Stats> {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let document: serde_json::Value = serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
    let cells = document
        .get("cells")
        .and_then(serde_json::Value::as_array)
        .unwrap_or_else(|| panic!("{} has no cells array", path.display()));

    let mut result = BTreeMap::new();
    for cell in cells {
        let group = cell
            .get("group")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string();
        let name = cell
            .get("name")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string();
        let median_ns = cell
            .get("median_ns")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or_default();
        let p95_ns = cell
            .get("p95_ns")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or_default();
        result.insert((group, name), Stats { median_ns, p95_ns });
    }
    result
}

fn percent_delta(baseline: f64, candidate: f64) -> f64 {
    if baseline == 0.0 {
        return 0.0;
    }
    (candidate / baseline - 1.0) * 100.0
}
