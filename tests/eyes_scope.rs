mod common;
use common::scratch;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const MAP: &str = ".agents/skills/verify-drafts/features";

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wstack"))
        .args(["eyes-and-hands", "check", "--json", "--run", "--root"])
        .arg(root)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn a_standard_map_target_drives_commands_from_its_owning_project() {
    let root = scratch("eyes-project");
    let out = run(&root.join(MAP), &[]);
    assert!(out.status.success(), "{out:?}");
    let value: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(
        value["cwd"],
        fs::canonicalize(&root).unwrap().to_string_lossy().as_ref()
    );
    assert!(!root.join(".state").exists());
}

#[test]
fn scoped_work_is_not_blocked_by_an_unrelated_map_without_recipes() {
    let root = scratch("eyes-project");
    let unrelated = root.join(".agents/skills/verify-unrelated/features");
    fs::create_dir_all(&unrelated).unwrap();
    let text = fs::read_to_string(root.join(MAP).join("save.md")).unwrap();
    fs::write(unrelated.join("other.md"), text).unwrap();
    fs::write(
        unrelated.join("README.md"),
        "# Other\n\n- [Other](./other.md) Unrelated feature.\n",
    )
    .unwrap();
    let full = run(&root, &[]);
    assert_eq!(full.status.code(), Some(1));
    let scoped = run(&root, &["--feature", "save.md"]);
    assert!(scoped.status.success(), "{scoped:?}");
    let value: Value = serde_json::from_slice(&scoped.stdout).unwrap();
    assert_eq!(value["capabilities"].as_array().unwrap().len(), 2);
}

#[test]
fn invalid_unselected_features_and_probes_do_not_block_selected_work() {
    let root = scratch("eyes-project");
    let map = root.join(MAP);
    fs::write(map.join("other.md"), "# Malformed unrelated feature\n").unwrap();
    let index = fs::read_to_string(map.join("README.md")).unwrap();
    fs::write(
        map.join("README.md"),
        index + "\n- [Other](./other.md) Unrelated.\n",
    )
    .unwrap();
    let path = map.join("capabilities.json");
    let mut value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let mut other = value["probes"][0].clone();
    other["feature"] = serde_json::json!("other.md");
    other["act"] = serde_json::json!([]);
    value["probes"].as_array_mut().unwrap().push(other);
    fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert_eq!(run(&root, &[]).status.code(), Some(1));
    let scoped = run(&root, &["--feature", "save.md"]);
    assert!(scoped.status.success(), "{scoped:?}");
}

#[test]
fn strict_record_deserialization_follows_feature_scope() {
    for invalid in [
        serde_json::json!([]),
        serde_json::json!("not-an-argv-array"),
    ] {
        let root = scratch("eyes-project");
        let map = root.join(MAP);
        let path = map.join("capabilities.json");
        let mut value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let mut other = value["probes"][0].clone();
        other["feature"] = serde_json::json!("unrelated.md");
        other["act"] = invalid;
        other["misspelled-assertion"] = serde_json::json!([]);
        value["probes"].as_array_mut().unwrap().push(other);
        fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert_eq!(run(&root, &[]).status.code(), Some(1));
        let scoped = run(&root, &["--feature", "save.md"]);
        assert!(scoped.status.success(), "{scoped:?}");
    }
}

#[test]
fn duplicate_fields_in_selected_records_prevent_execution() {
    for (field, duplicate) in [
        ("assert", "[\"python3\", \"driver.py\", \"assert\"]"),
        ("feature", "\"save.md\""),
        ("next", "\"add validation probe\""),
    ] {
        let root = scratch("eyes-project");
        let path = root.join(MAP).join("capabilities.json");
        let mut text = fs::read_to_string(&path).unwrap();
        if field == "next" {
            text = text.replace(
                "\"gaps\": []",
                r#""gaps": [{"feature":"save.md","capability":"reject-empty","reason":"missing driver","owner":"editor","next":"add validation probe"}]"#,
            );
        }
        let key = format!("\"{field}\":");
        fs::write(
            &path,
            text.replacen(&key, &format!("{key} {duplicate}, {key}"), 1),
        )
        .unwrap();
        for args in [vec![], vec!["--feature", "save.md"]] {
            let out = run(&root, &args);
            assert_eq!(out.status.code(), Some(1), "{out:?}");
            let value: Value = serde_json::from_slice(&out.stdout).unwrap();
            assert_eq!(value["executed"], false);
            assert!(value["problems"][0]["message"]
                .as_str()
                .unwrap()
                .contains(&format!("duplicate field `{field}`")));
            assert!(!root.join(".state").exists());
        }
    }
}
