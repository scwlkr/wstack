//! Checkout identity, pilot readiness, and local repository gates.
use crate::catalog;
use serde_json::{json, Value};
use std::{path::Path, process::Command};

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().into())
}

pub fn identity(root: &Path, base: &str) -> Result<Value, String> {
    Ok(
        json!({"project": "wstack", "root": root, "commit": git(root, &["rev-parse", "HEAD"] )?,
        "comparison_base": git(root, &["rev-parse", "--verify", &format!("{base}^{{commit}}")])?,
        "dirty": !git(root, &["status", "--porcelain"])?.is_empty(), "surface": "cli",
        "verification_skill": root.join(".agents/skills/verify-wstack/SKILL.md"),
        "feature_maps": crate::features::locate(root), "tracker": "https://github.com/scwlkr/wstack/issues",
        "executable": std::env::current_exe().map_err(|e| e.to_string())? }),
    )
}

pub fn doctor(root: &Path) -> Value {
    let mut checks = Vec::new();
    for tool in ["git", "cargo", "rustc", "python3"] {
        let ready = Command::new(tool)
            .arg("--version")
            .output()
            .is_ok_and(|o| o.status.success());
        checks.push(json!({"id": tool, "ready": ready, "remediation": format!("install {tool} and put it on PATH")}));
    }
    for fixture in [
        "features-good",
        "features-dead",
        "features-orphan",
        "features-sections",
    ] {
        checks.push(json!({"id": fixture, "ready": root.join("tests/fixtures").join(fixture).join("README.md").is_file(),
            "remediation": "restore tracked tests/fixtures from this candidate"}));
    }
    let maps = catalog::read(root);
    checks.push(json!({"id": "canonical-map", "ready": maps.is_ok(),
        "remediation": maps.err().unwrap_or_else(|| "./project features check".into())}));
    checks.push(
        json!({"id": "revision", "ready": identity(root, "HEAD").is_ok(),
        "remediation": "run in a Git checkout with a committed HEAD"}),
    );
    json!({"ready": checks.iter().all(|c| c["ready"] == true), "scope": "feature-map CLI fixture proof; no service/browser qualification", "checks": checks})
}

pub fn ci(root: &Path) -> Result<(), String> {
    for args in [
        vec!["build", "--locked"],
        vec!["test", "--locked"],
        vec![
            "clippy",
            "--locked",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        vec!["run", "--locked", "--", "check"],
    ] {
        eprintln!("cargo {}", args.join(" "));
        let status = Command::new("cargo")
            .current_dir(root.join("cli"))
            .args(&args)
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!("cargo {} failed: {status}", args.join(" ")));
        }
    }
    let status = Command::new("python3")
        .current_dir(root)
        .args([
            "-m",
            "unittest",
            "discover",
            "-s",
            "skills/wstack-setup/tests",
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("setup checks failed: {status}"));
    }
    Ok(())
}
