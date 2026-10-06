mod common;
use common::{fixture, scratch};
use serde_json::Value;
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wstack"))
        .args(args)
        .arg("--root")
        .arg(root)
        .output()
        .unwrap()
}

#[test]
fn discovery_reads_the_checked_map_and_rejects_unknown_features() {
    let root = scratch("features-good");
    let listed = run(&root, &["features", "list", "--json"]);
    assert!(listed.status.success());
    let catalog: Value = serde_json::from_slice(&listed.stdout).unwrap();
    assert_eq!(catalog["features"].as_array().unwrap().len(), 2);
    fs::write(root.join("README.md"), "# Changed index\n- [Add](add.md)\n").unwrap();
    let stale = run(&root, &["features", "list", "--json"]);
    assert_eq!(stale.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&stale.stderr).contains("orphan file"));
    let missing = run(
        &fixture("features-good"),
        &["features", "show", "unknown", "--json"],
    );
    assert_eq!(missing.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("unknown feature"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn proof_retains_evidence_and_fails_when_invalid_fixture_behavior_changes() {
    let root = scratch("features-good");
    let fixtures = root.join("tests/fixtures");
    fs::write(root.join(".gitignore"), ".evidence/\n").unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    for name in [
        "features-good",
        "features-dead",
        "features-orphan",
        "features-sections",
    ] {
        fs::rename(scratch(name), fixtures.join(name)).unwrap();
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
    let good = run(&root, &["verify", "feature-map"]);
    assert!(good.status.success(), "{good:?}");
    let report: Value = serde_json::from_slice(&good.stdout).unwrap();
    assert_eq!(report["coverage"].as_array().unwrap().len(), 8);
    assert_eq!(report["cleanup"], true);
    let evidence = Path::new(report["evidence"].as_str().unwrap());
    assert!(!evidence.join("state").exists());
    assert!(evidence.join("features-dead-json.stdout").is_file());
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(evidence.join("report.json")).unwrap()).unwrap(),
        report
    );
    assert!(report["coverage"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["json"] == true)
        .all(|c| c["command"].as_array().unwrap().last().unwrap() == "--json"));
    fs::remove_dir_all(fixtures.join("features-sections")).unwrap();
    let blocked = run(&root, &["verify", "feature-map"]);
    assert_eq!(blocked.status.code(), Some(1));
    let partial: Value = serde_json::from_slice(&blocked.stdout).unwrap();
    assert_eq!(partial["status"], "blocked");
    assert_eq!(partial["coverage"].as_array().unwrap().len(), 6);
    assert_eq!(partial["cleanup"], true);
    fs::rename(
        scratch("features-sections"),
        fixtures.join("features-sections"),
    )
    .unwrap();
    fs::write(root.join("development-note"), "dirty candidate\n").unwrap();
    let development = run(&root, &["verify", "feature-map"]);
    let dev_report: Value = serde_json::from_slice(&development.stdout).unwrap();
    assert_eq!(dev_report["status"], "development");
    assert!(dev_report["candidate_unchanged"].is_null());
    // Replace the invalid map with a valid one: acceptance must stay unchanged.
    fs::remove_dir_all(fixtures.join("features-dead")).unwrap();
    fs::rename(scratch("features-good"), fixtures.join("features-dead")).unwrap();
    let bad = run(&root, &["verify", "feature-map"]);
    assert_eq!(bad.status.code(), Some(1));
    let failed: Value = serde_json::from_slice(&bad.stdout).unwrap();
    assert_eq!(failed["status"], "failed");
    assert_eq!(failed["cleanup"], true);
    fs::remove_dir_all(root).unwrap();
}
