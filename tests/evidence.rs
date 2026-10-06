mod common;
use common::scratch;
use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn run(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_wstack"))
        .args(args)
        .arg("--root")
        .arg(root)
        .output()
        .unwrap()
}

fn pilot() -> (std::path::PathBuf, Value) {
    let root = scratch("features-good");
    fs::remove_file(root.join("add.md")).unwrap();
    fs::remove_file(root.join("list.md")).unwrap();
    fs::write(root.join("README.md"), "# Pilot\n- [Map](feature-map.md)\n").unwrap();
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../.agents/skills/verify-wstack/features/feature-map.md"),
        root.join("feature-map.md"),
    )
    .unwrap();
    fs::write(root.join(".gitignore"), ".evidence/\n").unwrap();
    fs::create_dir_all(root.join("tests/fixtures")).unwrap();
    for name in [
        "features-good",
        "features-dead",
        "features-orphan",
        "features-sections",
    ] {
        fs::rename(scratch(name), root.join("tests/fixtures").join(name)).unwrap();
    }
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
            "fixture",
        ],
    ] {
        assert!(Command::new("git")
            .current_dir(&root)
            .args(args)
            .status()
            .unwrap()
            .success());
    }
    let output = run(&root, &["verify", "feature-map"]);
    assert!(output.status.success(), "{output:?}");
    (root, serde_json::from_slice(&output.stdout).unwrap())
}

#[test]
fn current_real_receipt_passes_and_unsafe_or_incomplete_claims_reject() {
    let (root, report) = pilot();
    let evidence = Path::new(report["evidence"].as_str().unwrap());
    let file = evidence.join("candidate.json");
    let check = |value: &Value| {
        fs::write(&file, serde_json::to_vec(value).unwrap()).unwrap();
        run(
            &root,
            &["evidence", "check", file.to_str().unwrap(), "--json"],
        )
    };
    assert!(check(&report).status.success());
    for key in ["cleanup", "candidate_unchanged"] {
        let mut value = report.clone();
        value[key] = false.into();
        assert_eq!(check(&value).status.code(), Some(1), "{key}");
    }
    for status in [
        "development",
        "blocked",
        "failed",
        "incomplete",
        "cancelled",
    ] {
        let mut value = report.clone();
        value["status"] = status.into();
        assert_eq!(check(&value).status.code(), Some(1), "{status}");
    }
    for key in ["commit", "comparison_base"] {
        let mut value = report.clone();
        value["identity"][key] = "stale-revision".into();
        assert_eq!(check(&value).status.code(), Some(1), "{key}");
    }
    let mut value = report.clone();
    value["identity"]["dirty"] = true.into();
    assert_eq!(check(&value).status.code(), Some(1));
    let mut value = report.clone();
    value["coverage"].as_array_mut().unwrap().pop();
    assert_eq!(check(&value).status.code(), Some(1));
    let mut value = report.clone();
    value["coverage"][0]["status"] = "skipped".into();
    assert_eq!(check(&value).status.code(), Some(1));
    let mut value = report.clone();
    let duplicate = value["coverage"][0].clone();
    value["coverage"].as_array_mut().unwrap().push(duplicate);
    assert_eq!(check(&value).status.code(), Some(1));
    for path in ["missing.stdout", "../../README.md", "/etc/hosts"] {
        let mut value = report.clone();
        value["coverage"][0]["artifacts"][0] = path.into();
        assert_eq!(check(&value).status.code(), Some(1), "{path}");
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root.join("README.md"), evidence.join("linked.stdout")).unwrap();
        let mut value = report.clone();
        value["coverage"][0]["artifacts"][0] = "linked.stdout".into();
        assert_eq!(check(&value).status.code(), Some(1));
    }
    let mut value = report.clone();
    value["identity"]["executable"] = "wrong-binary".into();
    assert_eq!(check(&value).status.code(), Some(1));
    fs::write(root.join("dirty-source"), "changed").unwrap();
    assert_eq!(check(&report).status.code(), Some(1));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn malformed_map_observations_are_structural_errors() {
    let root = scratch("features-good");
    let file = root.join("add.md");
    let original = fs::read_to_string(&file).unwrap();
    for declaration in [
        "- Required observation: missing-entrypoint",
        "- Required observation: bad id | CLI",
        "- Required observation: case | CLI\n- Required observation: case | CLI",
    ] {
        fs::write(
            &file,
            original.replace("## Proof", &format!("## Proof\n\n{declaration}")),
        )
        .unwrap();
        assert_eq!(run(&root, &["features", "check"]).status.code(), Some(1));
    }
    fs::write(
        &file,
        original.replace(
            "## Proof",
            "## Proof (retained artifacts)\n\n- Required observation: case | CLI",
        ),
    )
    .unwrap();
    assert!(run(&root, &["features", "check"]).status.success());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn qualified_browser_shape_requires_owned_identity_actions_artifacts_and_cleanup() {
    let (root, mut report) = pilot();
    let map = root.join("feature-map.md");
    let text = fs::read_to_string(&map).unwrap();
    let text = text
        .lines()
        .filter(|line| !line.starts_with("- Required observation:"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        &map,
        text.replace(
            "## Proof",
            "## Proof\n\n- Required observation: features-good | Browser",
        ),
    )
    .unwrap();
    assert!(Command::new("git")
        .current_dir(&root)
        .args(["add", "."])
        .status()
        .unwrap()
        .success());
    assert!(Command::new("git")
        .current_dir(&root)
        .args([
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "-qm",
            "browser receipt fixture"
        ])
        .status()
        .unwrap()
        .success());
    let sha = Command::new("git")
        .current_dir(&root)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let sha = String::from_utf8(sha.stdout).unwrap().trim().to_string();
    report["identity"] = serde_json::json!({"commit": sha, "comparison_base": sha, "dirty": false,
        "binary": "fixture-browser", "binary_sha256": "a".repeat(64), "assets_sha256": "b".repeat(64)});
    report["surface"] = "browser".into();
    report["instance"] = serde_json::json!({"pid": 100, "endpoint": "http://127.0.0.1:54321",
        "startup": {"preview": "http://127.0.0.1:54321", "assets": "/tmp/owned-fixture-assets",
        "preparedAssets": false, "preparedWorker": false, "publication": false}});
    report["browser"] = serde_json::json!({"pid": 101, "executable": "fixture-browser", "version": "fixture-version", "profile": "/tmp/owned-fixture-profile"});
    report["cleanup_receipt"] = serde_json::json!({"preview_pid_absent": true, "port_closed": true, "browser_pid_absent": true, "profile_removed": true, "state_removed": true, "errors": []});
    report["coverage"] = serde_json::json!([{"case": "features-good", "entry_point": "Browser", "status": "pass",
        "action": {"description": "fixture action", "url": "https://example.invalid/", "steps": ["fixture step"]},
        "dom_artifact": "fixture.html", "screenshot": "fixture.png"}]);
    let evidence = Path::new(report["evidence"].as_str().unwrap());
    fs::write(evidence.join("fixture.html"), "fixture DOM").unwrap();
    fs::write(evidence.join("fixture.png"), "fixture screenshot bytes").unwrap();
    let file = evidence.join("browser.json");
    let check = |value: &Value| {
        fs::write(&file, serde_json::to_vec(value).unwrap()).unwrap();
        run(
            &root,
            &["evidence", "check", file.to_str().unwrap(), "--json"],
        )
    };
    assert!(check(&report).status.success());
    for pointer in [
        "/identity/assets_sha256",
        "/identity/binary_sha256",
        "/browser/pid",
        "/browser/executable",
        "/browser/version",
        "/browser/profile",
        "/instance/pid",
        "/instance/endpoint",
        "/instance/startup/preview",
        "/instance/startup/assets",
        "/coverage/0/action/steps",
        "/coverage/0/action/url",
        "/coverage/0/dom_artifact",
        "/coverage/0/screenshot",
        "/cleanup_receipt/errors",
    ] {
        let mut value = report.clone();
        *value.pointer_mut(pointer).unwrap() = Value::Null;
        assert_eq!(check(&value).status.code(), Some(1), "{pointer}");
    }
    for key in [
        "preview_pid_absent",
        "port_closed",
        "browser_pid_absent",
        "profile_removed",
        "state_removed",
    ] {
        let mut value = report.clone();
        value["cleanup_receipt"][key] = false.into();
        assert_eq!(check(&value).status.code(), Some(1), "{key}");
    }
    for path in ["missing.png", "../README.md", "/etc/hosts"] {
        let mut value = report.clone();
        value["coverage"][0]["screenshot"] = path.into();
        assert_eq!(check(&value).status.code(), Some(1), "{path}");
    }
    let mut value = report.clone();
    value["browser"]["pid"] = report["instance"]["pid"].clone();
    assert_eq!(check(&value).status.code(), Some(1));
    let mut value = report.clone();
    value["instance"]["startup"]["preview"] = "http://127.0.0.1:54322".into();
    assert_eq!(check(&value).status.code(), Some(1));
    for key in ["preparedAssets", "preparedWorker", "publication"] {
        let mut value = report.clone();
        value["instance"]["startup"][key] = true.into();
        assert_eq!(check(&value).status.code(), Some(1), "{key}");
    }
    let mut value = report.clone();
    value["cleanup_receipt"]["errors"] = serde_json::json!(["cleanup failed"]);
    assert_eq!(check(&value).status.code(), Some(1));
    fs::write(evidence.join("fixture.html"), "").unwrap();
    assert_eq!(check(&report).status.code(), Some(1));
    fs::remove_dir_all(root).unwrap();
}
