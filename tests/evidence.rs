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
    fs::remove_dir_all(root).unwrap();
}
