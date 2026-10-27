//! Deterministic measurement harness with no external benchmark framework.
//!
//! Each sample runs `ops_per_sample` operations and records ns/op. Samples are
//! sorted before percentiles are extracted; medians are the primary A/B metric
//! because they are robust to scheduler and allocator noise.

use std::fmt::Write as _;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Statistics for one benchmark cell.
pub struct Cell {
    pub group: String,
    pub name: String,
    pub ops_per_sample: u64,
    per_op_ns: Vec<f64>,
}

impl Cell {
    pub fn min_ns(&self) -> f64 {
        self.per_op_ns.first().copied().unwrap_or(0.0)
    }

    pub fn median_ns(&self) -> f64 {
        percentile(&self.per_op_ns, 0.5)
    }

    pub fn mean_ns(&self) -> f64 {
        if self.per_op_ns.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.per_op_ns.iter().sum();
        sum / self.per_op_ns.len() as f64
    }

    pub fn p95_ns(&self) -> f64 {
        percentile(&self.per_op_ns, 0.95)
    }

    pub fn max_ns(&self) -> f64 {
        self.per_op_ns.last().copied().unwrap_or(0.0)
    }

    pub fn stddev_ns(&self) -> f64 {
        if self.per_op_ns.len() < 2 {
            return 0.0;
        }
        let mean = self.mean_ns();
        let variance: f64 = self
            .per_op_ns
            .iter()
            .map(|sample| (sample - mean).powi(2))
            .sum::<f64>()
            / (self.per_op_ns.len() - 1) as f64;
        variance.sqrt()
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let index = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted.get(index).copied().unwrap_or(0.0)
}

/// Metadata recorded alongside every run so A/B reports are reproducible.
pub struct Meta {
    pub label: String,
    pub rc_binary: String,
    pub rustc: String,
    pub git_rev: String,
    pub started_unix_s: u64,
}

impl Meta {
    pub fn capture(label: String, rc_binary: String) -> Self {
        Self {
            label,
            rc_binary,
            rustc: command_output("rustc", &["--version"]),
            git_rev: command_output("git", &["rev-parse", "HEAD"]),
            started_unix_s: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

fn command_output(program: &str, args: &[&str]) -> String {
    std::process::Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Aggregate benchmark run.
pub struct Harness {
    meta: Meta,
    cells: Vec<Cell>,
}

impl Harness {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta,
            cells: Vec::new(),
        }
    }

    /// Measure `samples` samples of `ops_per_sample` executions of `f`.
    pub fn measure<F: FnMut()>(
        &mut self,
        group: &str,
        name: &str,
        samples: usize,
        ops_per_sample: u64,
        mut f: F,
    ) {
        for _ in 0..3 {
            for _ in 0..ops_per_sample {
                f();
            }
        }

        let mut per_op_ns = Vec::with_capacity(samples);
        for _ in 0..samples {
            let start = Instant::now();
            for _ in 0..ops_per_sample {
                f();
            }
            let elapsed = start.elapsed();
            per_op_ns.push(elapsed.as_nanos() as f64 / ops_per_sample as f64);
        }
        per_op_ns.sort_by(|left, right| {
            left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
        });
        self.cells.push(Cell {
            group: group.to_string(),
            name: name.to_string(),
            ops_per_sample,
            per_op_ns,
        });
    }

    /// Measure a command or scenario that reports its own duration.
    pub fn measure_cmd<F: FnMut() -> Duration>(
        &mut self,
        group: &str,
        name: &str,
        samples: usize,
        mut f: F,
    ) {
        let _ = f();
        let mut per_op_ns = Vec::with_capacity(samples);
        for _ in 0..samples {
            let duration = f();
            per_op_ns.push(duration.as_nanos() as f64);
        }
        per_op_ns.sort_by(|left, right| {
            left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
        });
        self.cells.push(Cell {
            group: group.to_string(),
            name: name.to_string(),
            ops_per_sample: 1,
            per_op_ns,
        });
    }

    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "# rc performance baseline/run");
        let _ = writeln!(out);
        let _ = writeln!(out, "- label: `{}`", self.meta.label);
        let _ = writeln!(out, "- rc: `{}`", self.meta.rc_binary);
        let _ = writeln!(out, "- rustc: `{}`", self.meta.rustc);
        let _ = writeln!(out, "- git: `{}`", self.meta.git_rev);
        let _ = writeln!(out, "- started_unix_s: {}", self.meta.started_unix_s);
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "| grupo | operação | amostras | min | mediana | p95 | máx | desvio |"
        );
        let _ = writeln!(out, "|---|---|---:|---:|---:|---:|---:|---:|");
        for cell in &self.cells {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                cell.group,
                cell.name,
                cell.per_op_ns.len(),
                fmt_ns(cell.min_ns()),
                fmt_ns(cell.median_ns()),
                fmt_ns(cell.p95_ns()),
                fmt_ns(cell.max_ns()),
                fmt_ns(cell.stddev_ns()),
            );
        }
        out
    }

    pub fn render_json(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "{{");
        let _ = writeln!(out, "  \"meta\": {{");
        let _ = writeln!(out, "    \"label\": {},", json_str(&self.meta.label));
        let _ = writeln!(
            out,
            "    \"rc_binary\": {},",
            json_str(&self.meta.rc_binary)
        );
        let _ = writeln!(out, "    \"rustc\": {},", json_str(&self.meta.rustc));
        let _ = writeln!(out, "    \"git_rev\": {},", json_str(&self.meta.git_rev));
        let _ = writeln!(
            out,
            "    \"started_unix_s\": {}",
            self.meta.started_unix_s
        );
        let _ = writeln!(out, "  }},");
        let _ = writeln!(out, "  \"cells\": [");
        for (index, cell) in self.cells.iter().enumerate() {
            let comma = if index + 1 == self.cells.len() {
                ""
            } else {
                ","
            };
            let _ = writeln!(
                out,
                "    {{\"group\":{},\"name\":{},\"n\":{},\"min_ns\":{:.1},\"median_ns\":{:.1},\"mean_ns\":{:.1},\"p95_ns\":{:.1},\"max_ns\":{:.1},\"stddev_ns\":{:.1}}}{comma}",
                json_str(&cell.group),
                json_str(&cell.name),
                cell.per_op_ns.len(),
                cell.min_ns(),
                cell.median_ns(),
                cell.mean_ns(),
                cell.p95_ns(),
                cell.max_ns(),
                cell.stddev_ns(),
            );
        }
        let _ = writeln!(out, "  ]");
        let _ = writeln!(out, "}}");
        out
    }
}

/// Human-readable duration.
pub fn fmt_ns(ns: f64) -> String {
    if ns >= 1e9 {
        format!("{:.3} s", ns / 1e9)
    } else if ns >= 1e6 {
        format!("{:.3} ms", ns / 1e6)
    } else if ns >= 1e3 {
        format!("{:.2} µs", ns / 1e3)
    } else {
        format!("{ns:.0} ns")
    }
}

fn json_str(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"<invalid>\"".to_string())
}
