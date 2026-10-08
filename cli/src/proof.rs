//! Real CLI boundary proof; disposable fixture state is separate from retained artifacts.
use crate::operations;
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct OwnedState(PathBuf);
impl Drop for OwnedState {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn copy_fixture(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir(to).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(from).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        if kind.is_symlink() {
            return Err("fixture symlinks are not owned state".into());
        }
        if kind.is_dir() {
            copy_fixture(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.join(entry.file_name())).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn write_json(path: &Path, value: &Value) -> Result<(), String> {
    fs::write(
        path,
        serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

fn observe(evidence: &Path, state: &Path, fixture: &str, as_json: bool) -> Result<Value, String> {
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let target = state.join(fixture);
    let mut command = Command::new(&executable);
    command.args(["features", "check", "--root"]).arg(&target);
    if as_json {
        command.arg("--json");
    }
    let output = command.output().map_err(|e| e.to_string())?;
    let stem = format!("{fixture}-{}", if as_json { "json" } else { "text" });
    fs::write(evidence.join(format!("{stem}.stdout")), &output.stdout)
        .map_err(|e| e.to_string())?;
    fs::write(evidence.join(format!("{stem}.stderr")), &output.stderr)
        .map_err(|e| e.to_string())?;
    let good = fixture == "features-good";
    let code_matches = output.status.code() == Some(if good { 0 } else { 1 });
    let diagnostic = match fixture {
        "features-good" => "ok: 2 features in 1 map(s), 1 planned",
        "features-dead" => "dead entry `gone.md`",
        "features-orphan" => "orphan file `list.md`",
        "features-sections" => "missing H2 `Proof`",
        _ => return Err("unknown proof fixture".into()),
    };
    let observed = if as_json {
        serde_json::from_slice::<Value>(&output.stdout).is_ok_and(|v| {
            v["ok"] == good
                && v["maps"] == 1
                && if good {
                    v["features"] == 2 && v["planned"] == 1
                } else {
                    v["problems"].as_array().is_some_and(|p| {
                        p.iter().any(|p| {
                            p["message"]
                                .as_str()
                                .is_some_and(|m| m.contains(diagnostic))
                        })
                    })
                }
        })
    } else if good {
        output.stderr.is_empty() && String::from_utf8_lossy(&output.stdout).contains(diagnostic)
    } else {
        String::from_utf8_lossy(&output.stderr).contains(diagnostic)
    };
    let mut arguments = vec![
        executable.display().to_string(),
        "features".to_string(),
        "check".to_string(),
        "--root".to_string(),
        target.display().to_string(),
    ];
    if as_json {
        arguments.push("--json".to_string());
    }
    Ok(
        json!({"case": fixture, "entry_point": if as_json {"features check --json"} else {"features check"},
        "command": arguments,
        "json": as_json, "exit_code": output.status.code(), "status": if code_matches && observed {"pass"} else {"failed"},
        "expected_diagnostic": diagnostic, "artifacts": [format!("{stem}.stdout"), format!("{stem}.stderr")]}),
    )
}

pub fn run(root: &Path, base: &str, destination: Option<PathBuf>) -> Result<Value, String> {
    let identity = operations::identity(root, base)?;
    let id = format!(
        "feature-map-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos()
    );
    let evidence = destination.unwrap_or_else(|| root.join(".evidence").join(&id));
    let evidence = if evidence.is_absolute() {
        evidence
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(evidence)
    };
    if let Some(parent) = evidence.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::create_dir(&evidence).map_err(|e| format!("evidence directory must be new: {e}"))?;
    let mut report = json!({"run_id": id, "feature": "feature-map", "surface": "cli", "identity": identity,
        "status": "incomplete", "evidence": evidence, "coverage": [], "cleanup": false});
    write_json(&evidence.join("report.json"), &report)?;
    let state = OwnedState(evidence.join("state"));
    let outcome = (|| {
        fs::create_dir(&state.0).map_err(|e| e.to_string())?;
        for fixture in [
            "features-good",
            "features-dead",
            "features-orphan",
            "features-sections",
        ] {
            copy_fixture(
                &root.join("tests/fixtures").join(fixture),
                &state.0.join(fixture),
            )?;
            for as_json in [false, true] {
                let observation = observe(&evidence, &state.0, fixture, as_json)?;
                report["coverage"]
                    .as_array_mut()
                    .ok_or("missing coverage array")?
                    .push(observation);
                write_json(&evidence.join("report.json"), &report)?;
            }
        }
        let final_identity = operations::identity(root, base)?;
        let dirty = report["identity"]["dirty"] == true;
        let unchanged = final_identity == report["identity"];
        report["candidate_unchanged"] = if dirty { Value::Null } else { json!(unchanged) };
        let observations_pass = report["coverage"]
            .as_array()
            .ok_or("missing coverage array")?
            .iter()
            .all(|c| c["status"] == "pass");
        report["status"] = json!(if !observations_pass || !unchanged {
            "failed"
        } else if dirty {
            "development"
        } else {
            "pass"
        });
        Ok::<(), String>(())
    })();
    if let Err(error) = outcome {
        report["status"] = json!("blocked");
        report["error"] = json!(error);
    }
    drop(state);
    report["cleanup"] = json!(!evidence.join("state").exists());
    if report["cleanup"] != true {
        report["status"] = json!("failed");
    }
    write_json(&evidence.join("report.json"), &report)?;
    // Read the retained report after teardown, so cleanup cannot erase its proof.
    let retained = fs::read(evidence.join("report.json")).map_err(|e| e.to_string())?;
    serde_json::from_slice(&retained).map_err(|e| e.to_string())
}

pub fn browser(
    root: &Path,
    base: &str,
    evidence: Option<PathBuf>,
    feature: &str,
) -> Result<serde_json::Value, String> {
    let mut command = Command::new("node");
    command.env(
        "WSTACK_PROOF_BIN",
        std::env::current_exe().map_err(|e| e.to_string())?,
    );
    command
        .arg(root.join(format!(
            ".agents/skills/verify-wstack/scripts/{feature}-browser.cjs"
        )))
        .arg(root)
        .arg(base);
    if let Some(path) = evidence {
        command.arg(path);
    }
    let output = command.output().map_err(|e| {
        format!("{feature} browser proof needs Node.js and Playwright; see verify-wstack: {e}")
    })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.trim().is_empty() {
            return Err(stderr.trim().into());
        }
        let report: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|_| format!("{feature} browser proof failed without a readable report"))?;
        let error = report["error"]
            .as_str()
            .unwrap_or("incomplete browser proof");
        let evidence = report["evidence"].as_str().unwrap_or("see verify-wstack");
        return Err(format!(
            "{}; retained evidence: {evidence}",
            error.lines().next().unwrap_or(error)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| format!("{feature} proof report: {e}"))
}
