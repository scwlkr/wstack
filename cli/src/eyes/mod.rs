//! Executable capability audits using the existing verification feature map.
mod execute;
mod inventory;
mod schema;

use crate::problem::Problem;
use schema::{Capability, Execution};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct Report {
    ok: bool,
    executed: bool,
    root: PathBuf,
    cwd: PathBuf,
    selected_features: Vec<String>,
    git_sha: Option<String>,
    git_dirty: Option<bool>,
    timestamp_ms: u128,
    receipt: Option<PathBuf>,
    capabilities: Vec<Capability>,
    executions: Vec<Execution>,
    problems: Vec<Problem>,
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn workdir(root: &Path) -> PathBuf {
    let project = || {
        let skill = root.parent()?;
        let skills = skill.parent()?;
        let agent = skills.parent()?;
        (root.file_name()? == "features"
            && skill.file_name()?.to_str()?.starts_with("verify-")
            && skills.file_name()? == "skills"
            && [".agents", ".cursor", ".claude"]
                .iter()
                .any(|name| agent.file_name().is_some_and(|n| n == *name)))
        .then(|| agent.parent().map(Path::to_path_buf))
        .flatten()
    };
    project().unwrap_or_else(|| root.to_path_buf())
}

fn audit(root: &Path, run: bool, selected: &[String], output: Option<&Path>) -> Report {
    let root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    let cwd = workdir(&root);
    let inventory = inventory::inspect(&root, selected);
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let mut report = Report {
        ok: false,
        executed: false,
        git_sha: git(&cwd, &["rev-parse", "HEAD"]),
        git_dirty: git(&cwd, &["status", "--porcelain"]).map(|s| !s.is_empty()),
        cwd,
        root,
        selected_features: selected.to_vec(),
        timestamp_ms: stamp.as_millis(),
        receipt: None,
        capabilities: inventory.capabilities,
        executions: Vec::new(),
        problems: inventory.problems,
    };
    if !run || !report.problems.is_empty() {
        return report;
    }
    let dir = output
        .map(Path::to_path_buf)
        .unwrap_or_else(std::env::temp_dir)
        .join(format!(
            "wstack-eyes-{}-{}",
            std::process::id(),
            stamp.as_nanos()
        ));
    let prepare = || -> Result<PathBuf, String> {
        execute::install_handler()?;
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        fs::canonicalize(&dir).map_err(|e| e.to_string())
    };
    let dir = match prepare() {
        Ok(dir) => dir,
        Err(message) => {
            report.problems.push(Problem::new(
                &report.root,
                &dir,
                1,
                "eyes-and-hands",
                message,
            ));
            return report;
        }
    };
    report.executed = !inventory.probes.is_empty();
    report.receipt = Some(dir.join("receipt.json"));
    for (index, (map, probe)) in inventory.probes.iter().enumerate() {
        if execute::cancelled() {
            break;
        }
        let execution = execute::run(&report.cwd, map, probe, &dir.join(format!("probe-{index}")));
        let (status, detail) = match &execution {
            Ok(e) if e.passed => ("passed", "all five commands passed in this audit".into()),
            Ok(e) => {
                let failed = e.steps.iter().find(|s| !s.passed());
                (
                    "failed",
                    failed
                        .map(|s| {
                            format!(
                                "{} failed: exit={:?}, timeout={}, error={:?}; read {}",
                                s.stage,
                                s.exit,
                                s.timed_out,
                                s.error,
                                s.stderr.display()
                            )
                        })
                        .unwrap_or_else(|| "audit interrupted; cleanup attempted".into()),
                )
            }
            Err(message) => ("failed", message.clone()),
        };
        for row in report
            .capabilities
            .iter_mut()
            .filter(|r| &r.map == map && r.probe.as_deref() == Some(&probe.name))
        {
            row.status = status;
            row.detail = detail.clone();
            row.next = (status != "passed")
                .then(|| "repair the failed boundary and rerun the probe".into());
        }
        match execution {
            Ok(e) => report.executions.push(e),
            Err(message) => report.problems.push(Problem::new(
                &report.root,
                map,
                1,
                "eyes-and-hands",
                message,
            )),
        }
    }
    report.ok = report.problems.is_empty()
        && !report.capabilities.is_empty()
        && report.capabilities.iter().all(|r| r.status == "passed");
    let receipt = report.receipt.as_ref().unwrap();
    let save = serde_json::to_vec_pretty(&report)
        .map_err(|e| e.to_string())
        .and_then(|bytes| fs::write(receipt, bytes).map_err(|e| e.to_string()));
    if let Err(message) = save {
        report.ok = false;
        report.problems.push(Problem::new(
            &report.root,
            receipt,
            1,
            "eyes-and-hands",
            message,
        ));
    }
    report
}

pub fn command(
    root: &Path,
    json: bool,
    run: bool,
    selected: &[String],
    output: Option<&Path>,
) -> ExitCode {
    let report = audit(root, run, selected, output);
    if json {
        println!("{}", serde_json::to_string(&report).unwrap());
    } else {
        for row in &report.capabilities {
            println!(
                "{}:{} {}: {}",
                row.feature, row.capability, row.status, row.detail
            );
            if let Some(next) = &row.next {
                println!("  next: {next}");
            }
        }
        for problem in &report.problems {
            eprintln!("{problem}");
        }
        if let Some(receipt) = &report.receipt {
            println!("receipt: {}", receipt.display());
        }
    }
    if report.ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
