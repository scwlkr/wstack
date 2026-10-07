mod common;
use common::scratch;
use serde_json::{json, Value};
use std::{fs, path::PathBuf, process::Command};

struct Fixture {
    root: PathBuf,
    evidence: PathBuf,
    report: Value,
}

impl Fixture {
    fn new(surface: &str) -> Self {
        let root = scratch("features-good");
        let map = root.join("add.md");
        let required = if surface == "browser" {
            "- Required observation: reader | Browser\n- Required observation: routes | HTTP"
        } else {
            "- Required observation: routes | HTTP"
        };
        fs::write(
            &map,
            fs::read_to_string(&map)
                .unwrap()
                .replace("## Proof", &format!("## Proof\n\n{required}")),
        )
        .unwrap();
        fs::write(root.join(".gitignore"), ".evidence/\n").unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["add", "."],
            vec![
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "-qm",
                "mixed receipt fixture",
            ],
        ] {
            assert!(Command::new("git")
                .current_dir(&root)
                .args(args)
                .status()
                .unwrap()
                .success());
        }
        let head = Command::new("git")
            .current_dir(&root)
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        let head = String::from_utf8(head.stdout).unwrap().trim().to_string();
        let evidence = root.join(".evidence");
        fs::create_dir(&evidence).unwrap();
        for (name, bytes) in [
            ("metadata.json", "{\"requests\":3}"),
            ("reader.html", "retained DOM"),
            ("reader.png", "retained screenshot"),
            ("get.body", "approved object"),
            ("head.body", ""),
            ("redirect.body", ""),
        ] {
            fs::write(evidence.join(name), bytes).unwrap();
        }
        let report = json!({"feature": "add", "surface": surface, "status": "pass", "cleanup": true,
            "candidate_unchanged": true,
            "identity": {"commit": head, "comparison_base": head, "dirty": false,
                "binary": "fixture-browser", "binary_sha256": "a".repeat(64), "assets_sha256": "b".repeat(64)},
            "instance": {"pid": 100, "endpoint": "http://127.0.0.1:54321", "startup": {
                "preview": "http://127.0.0.1:54321", "assets": "/tmp/owned-fixture-assets",
                "preparedAssets": false, "preparedWorker": false, "publication": false}},
            "browser": {"pid": 101, "executable": "fixture-browser", "version": "fixture-version", "profile": "/tmp/owned-fixture-profile"},
            "cleanup_receipt": {"preview_pid_absent": true, "port_closed": true, "browser_pid_absent": true,
                "profile_removed": true, "state_removed": true, "errors": []},
            "coverage": [{"case": "reader", "entry_point": "Browser", "status": "pass",
                "action": {"description": "open Reader", "url": "https://example.invalid/", "steps": ["open Reader"]},
                "dom_artifact": "reader.html", "screenshot": "reader.png"},
                {"case": "routes", "entry_point": "HTTP", "status": "pass", "artifacts": ["metadata.json"],
                    "http_observations": [
                        {"action": {"method": "GET", "path": "/approved.json"}, "http_status": 200, "raw_body": "get.body"},
                        {"action": {"method": "HEAD", "path": "/approved.json"}, "http_status": 200, "raw_body": "head.body"},
                        {"action": {"method": "GET", "path": "/"}, "http_status": 308, "raw_body": "redirect.body"}]}]});
        Self {
            root,
            evidence,
            report,
        }
    }

    fn check(&self, report: &Value) -> std::process::Output {
        let path = self.evidence.join("report.json");
        fs::write(&path, serde_json::to_vec(report).unwrap()).unwrap();
        Command::new(env!("CARGO_BIN_EXE_wstack"))
            .args([
                "evidence",
                "check",
                path.to_str().unwrap(),
                "--json",
                "--root",
                self.root.to_str().unwrap(),
            ])
            .output()
            .unwrap()
    }

    fn rejects(&self, report: &Value, reason: &str) {
        let output = self.check(report);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(reason),
            "{output:?}"
        );
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn browser_receipt_accepts_grouped_http_readbacks_including_empty_head_and_redirect_bodies() {
    let fixture = Fixture::new("browser");
    let output = fixture.check(&fixture.report);
    assert!(output.status.success(), "{output:?}");
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["observations"], 2);
}

#[test]
fn mixed_receipt_rejects_unknown_entries_missing_cases_and_nonpassing_rows() {
    let fixture = Fixture::new("browser");
    let mut report = fixture.report.clone();
    report["coverage"][1]["entry_point"] = "CLI".into();
    fixture.rejects(&report, "Browser observation lacks");
    let mut report = fixture.report.clone();
    report["coverage"].as_array_mut().unwrap().pop();
    fixture.rejects(&report, "missing required observation: routes | HTTP");
    for row in [0, 1] {
        for status in ["failed", "skipped"] {
            let mut report = fixture.report.clone();
            report["coverage"][row]["status"] = status.into();
            fixture.rejects(&report, "failed/skipped");
        }
    }
}

#[test]
fn mixed_http_groups_require_nonempty_arrays_and_each_actual_http_shape() {
    let fixture = Fixture::new("browser");
    for value in [
        Value::Null,
        json!([]),
        json!({}),
        json!([null]),
        json!([{}]),
    ] {
        let mut report = fixture.report.clone();
        report["coverage"][1]["http_observations"] = value;
        fixture.rejects(&report, "HTTP");
    }
    for pointer in [
        "/action/method",
        "/action/path",
        "/http_status",
        "/raw_body",
    ] {
        for index in 0..3 {
            let mut report = fixture.report.clone();
            *report["coverage"][1]["http_observations"][index]
                .pointer_mut(pointer)
                .unwrap() = Value::Null;
            fixture.rejects(&report, "HTTP observation lacks");
        }
    }
    let mut report = fixture.report.clone();
    report["coverage"][1]
        .as_object_mut()
        .unwrap()
        .remove("http_observations");
    fixture.rejects(&report, "requires nonempty http_observations");
}

#[test]
fn mixed_http_bodies_require_safe_readable_files_and_metadata_stays_nonempty() {
    let fixture = Fixture::new("browser");
    for path in ["missing.body", "../add.md", "/etc/hosts", ""] {
        let mut report = fixture.report.clone();
        report["coverage"][1]["http_observations"][1]["raw_body"] = path.into();
        fixture.rejects(&report, "artifact");
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(
            fixture.evidence.join("get.body"),
            fixture.evidence.join("linked.body"),
        )
        .unwrap();
        std::os::unix::fs::symlink(&fixture.evidence, fixture.evidence.join("linked-directory"))
            .unwrap();
        for path in ["linked.body", "linked-directory/get.body"] {
            let mut report = fixture.report.clone();
            report["coverage"][1]["http_observations"][0]["raw_body"] = path.into();
            fixture.rejects(&report, "artifact symlink");
        }
    }
    let mut report = fixture.report.clone();
    report["coverage"][1]["artifacts"] = json!([]);
    fixture.rejects(&report, "observation has no retained artifact");
    for name in ["metadata.json", "reader.html", "reader.png"] {
        let path = fixture.evidence.join(name);
        let original = fs::read(&path).unwrap();
        fs::write(&path, "").unwrap();
        fixture.rejects(&fixture.report, "empty browser artifact");
        fs::write(&path, original).unwrap();
    }
}

#[test]
fn mixed_receipt_keeps_browser_identity_actions_and_cleanup_constraints() {
    let fixture = Fixture::new("browser");
    for pointer in [
        "/identity/binary_sha256",
        "/identity/assets_sha256",
        "/browser/pid",
        "/browser/executable",
        "/browser/profile",
        "/instance/endpoint",
        "/coverage/0/action/steps",
        "/coverage/0/dom_artifact",
        "/coverage/0/screenshot",
        "/cleanup_receipt/port_closed",
        "/cleanup_receipt/profile_removed",
    ] {
        let mut report = fixture.report.clone();
        *report.pointer_mut(pointer).unwrap() = Value::Null;
        fixture.rejects(&report, "Browser");
    }
}

#[test]
fn standalone_http_keeps_its_ungrouped_shape_and_empty_body_semantics() {
    let fixture = Fixture::new("http");
    let mut report = fixture.report.clone();
    report["coverage"] = json!([{"case": "routes", "entry_point": "HTTP", "status": "pass",
        "action": {"method": "HEAD", "path": "/approved.json"}, "http_status": 200, "raw_body": "head.body"}]);
    assert!(fixture.check(&report).status.success());
    report["coverage"][0]["action"]["method"] = "GET".into();
    assert!(fixture.check(&report).status.success());
    report["coverage"][0]["raw_body"] = "missing.body".into();
    fixture.rejects(&report, "unreadable artifact");
}
