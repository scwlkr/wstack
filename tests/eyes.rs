mod common;
use common::scratch;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

const MAP: &str = ".agents/skills/verify-drafts/features";

fn run(root: &Path, args: &[&str], mode: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wstack"))
        .args(["eyes-and-hands", "check", "--json", "--root"])
        .arg(root)
        .args(args)
        .env("EYES_TEST_MODE", mode)
        .output()
        .unwrap()
}

fn report(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|e| panic!("{e}: {output:?}"))
}

fn edit(root: &Path, change: impl FnOnce(&mut Value)) {
    let path = root.join(MAP).join("capabilities.json");
    let mut value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    change(&mut value);
    fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}

#[test]
fn inspection_is_read_only_and_does_not_claim_unrun_commands_passed() {
    let root = scratch("eyes-project");
    let out = run(&root, &[], "");
    assert_eq!(out.status.code(), Some(1));
    let value = report(&out);
    assert_eq!(value["executed"], false);
    assert_eq!(value["capabilities"][0]["status"], "unproven");
    assert!(value["receipt"].is_null());
    assert!(!root.join(".state").exists());
}

#[test]
fn live_probe_drives_app_reads_persistence_and_retains_receipt_after_cleanup() {
    let root = scratch("eyes-project");
    let out = run(&root, &["--run"], "");
    assert!(out.status.success(), "{out:?}");
    let value = report(&out);
    assert_eq!(value["ok"], true);
    assert_eq!(value["capabilities"].as_array().unwrap().len(), 2);
    assert_eq!(value["executions"][0]["steps"].as_array().unwrap().len(), 5);
    let receipt = Path::new(value["receipt"].as_str().unwrap());
    assert!(receipt.is_file());
    let assertion = value["executions"][0]["steps"][3]["stdout"]
        .as_str()
        .unwrap();
    assert!(fs::read_to_string(assertion).unwrap().contains("Plan trip"));
    assert!(!root.join(".state").exists());
}

#[test]
fn missing_manifest_and_uncovered_subfeatures_are_actionable_failures() {
    let root = scratch("eyes-project");
    edit(&root, |v| v["probes"][0]["covers"] = json!(["save-title"]));
    let out = run(&root, &["--run"], "");
    assert_eq!(out.status.code(), Some(1));
    let value = report(&out);
    assert!(value["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r["capability"] == "reject-empty" && r["status"] == "missing"));
    fs::remove_file(root.join(MAP).join("capabilities.json")).unwrap();
    let value = report(&run(&root, &["--run"], ""));
    assert_eq!(value["executed"], false);
    assert!(value["problems"][0]["message"]
        .as_str()
        .unwrap()
        .contains("register probes"));
}

#[test]
fn explicit_gap_keeps_owner_and_next_action_visible() {
    let root = scratch("eyes-project");
    edit(&root, |v| {
        v["probes"][0]["covers"] = json!(["save-title"]);
        v["gaps"] = json!([{"feature":"save.md", "capability":"reject-empty", "reason":"no validation driver", "owner":"editor", "next":"add invalid-input scenario"}]);
    });
    let out = run(&root, &["--run"], "");
    assert_eq!(out.status.code(), Some(1));
    let value = report(&out);
    let blocked = value["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["status"] == "blocked")
        .unwrap();
    assert_eq!(blocked["owner"], "editor");
    assert_eq!(blocked["next"], "add invalid-input scenario");
}

#[test]
fn invalid_schema_or_unknown_coverage_prevents_any_execution() {
    for mode in [
        "unknown-field",
        "unknown-capability",
        "duplicate",
        "empty-command",
    ] {
        let root = scratch("eyes-project");
        edit(&root, |v| match mode {
            "unknown-field" => v["probes"][0]["assertion"] = json!(["echo", "passed"]),
            "unknown-capability" => v["probes"][0]["covers"] = json!(["invented"]),
            "duplicate" => v["probes"][0]["covers"] = json!(["save-title", "save-title"]),
            _ => v["probes"][0]["act"] = json!([]),
        });
        let value = report(&run(&root, &["--run"], ""));
        assert_eq!(value["executed"], false, "{mode}: {value}");
        assert!(!root.join(".state").exists());
    }
}

#[test]
fn failed_action_skips_assertion_runs_cleanup_and_exposes_error_log() {
    let root = scratch("eyes-project");
    let out = run(&root, &["--run"], "fail");
    assert_eq!(out.status.code(), Some(1));
    let value = report(&out);
    let steps = value["executions"][0]["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[2]["exit"], 9);
    assert_eq!(steps[3]["stage"], "cleanup");
    assert!(fs::read_to_string(steps[2]["stderr"].as_str().unwrap())
        .unwrap()
        .contains("cannot reach"));
    assert!(!root.join(".state").exists());
}

#[test]
fn cleanup_failure_cannot_be_reported_as_a_pass() {
    let root = scratch("eyes-project");
    let out = run(&root, &["--run"], "cleanup-fail");
    assert_eq!(out.status.code(), Some(1));
    let value = report(&out);
    assert_eq!(value["executions"][0]["steps"][4]["exit"], 17);
    assert_eq!(value["capabilities"][0]["status"], "failed");
    assert!(Path::new(value["receipt"].as_str().unwrap()).is_file());
}

#[test]
fn erased_evidence_fails_the_probe_and_unrelated_state_survives() {
    let root = scratch("eyes-project");
    fs::create_dir_all(root.join(".state/owner")).unwrap();
    fs::write(root.join(".state/owner/keep"), "unrelated").unwrap();
    let out = run(&root, &["--run"], "erase-evidence");
    assert_eq!(out.status.code(), Some(1));
    assert!(report(&out)["capabilities"][0]["detail"]
        .as_str()
        .unwrap()
        .contains("removed command evidence"));
    assert_eq!(
        fs::read_to_string(root.join(".state/owner/keep")).unwrap(),
        "unrelated"
    );
}

#[test]
fn argv_is_literal_and_never_implicitly_evaluated_as_shell_code() {
    let root = scratch("eyes-project");
    edit(&root, |v| {
        for key in ["act", "assert"] {
            v["probes"][0][key]
                .as_array_mut()
                .unwrap()
                .push(json!("$(touch INJECTED); two words"));
        }
    });
    let out = run(&root, &["--run"], "");
    assert!(out.status.success(), "{out:?}");
    assert!(!root.join("INJECTED").exists());
}

#[test]
fn planned_features_and_unknown_selection_never_claim_live_access() {
    let root = scratch("eyes-project");
    let out = run(&root, &["--run", "--feature", "absent.md"], "");
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(report(&out)["executed"], false);
    let path = root.join(MAP).join("save.md");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text + "\nStatus: planned\n").unwrap();
    let out = run(&root, &["--run"], "");
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(report(&out)["capabilities"][0]["status"], "planned");
    assert!(!root.join(".state").exists());
}

#[test]
#[cfg(unix)]
fn timeout_kills_descendants_and_preserves_cleanup_and_evidence() {
    let root = scratch("eyes-project");
    edit(&root, |v| v["probes"][0]["timeout_ms"] = json!(300));
    let start = Instant::now();
    let out = run(&root, &["--run"], "hang");
    assert!(start.elapsed() < Duration::from_secs(5));
    assert_eq!(out.status.code(), Some(1));
    let value = report(&out);
    assert_eq!(value["executions"][0]["steps"][2]["timed_out"], true);
    let dir = Path::new(value["receipt"].as_str().unwrap())
        .parent()
        .unwrap();
    std::thread::sleep(Duration::from_millis(800));
    assert!(!dir.join("probe-0/orphan").exists());
    assert!(!root.join(".state").exists());
}

#[test]
#[cfg(unix)]
fn interrupt_attempts_cleanup_and_writes_a_failed_receipt() {
    let root = scratch("eyes-project");
    let child = Command::new(env!("CARGO_BIN_EXE_wstack"))
        .args(["eyes-and-hands", "check", "--json", "--run", "--root"])
        .arg(&root)
        .env("EYES_TEST_MODE", "hang")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let start = Instant::now();
    while !root.join(".state").is_dir()
        || !fs::read_dir(root.join(".state"))
            .unwrap()
            .flatten()
            .any(|e| e.path().join("acting").exists())
    {
        assert!(start.elapsed() < Duration::from_secs(5));
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(Command::new("/bin/kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap()
        .success());
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    let value = report(&out);
    assert_eq!(
        value["executions"][0]["steps"][2]["error"],
        "audit interrupted"
    );
    assert_eq!(value["executions"][0]["steps"][3]["stage"], "cleanup");
    assert!(Path::new(value["receipt"].as_str().unwrap()).is_file());
    assert!(!root.join(".state").exists());
}
