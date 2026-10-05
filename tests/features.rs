mod common;
use common::{fixture, scratch};
use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};
use wstack::features;

fn run(root: &Path, json: bool) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_wstack"));
    cmd.args(["features", "check", "--root"]).arg(root);
    if json {
        cmd.arg("--json");
    }
    cmd.output().unwrap()
}

fn messages(name: &str) -> Vec<String> {
    let root = fixture(name);
    let dirs = features::locate(&root);
    let problems = dirs
        .iter()
        .flat_map(|dir| features::check(&root, dir).problems);
    problems.map(|p| p.to_string()).collect()
}

fn assert_has(name: &str, expected: &str) {
    let found = messages(name);
    assert!(
        found.iter().any(|m| m.contains(expected)),
        "{expected:?} not in {found:#?}"
    );
}

#[test]
fn good_map_passes_and_counts_planned() {
    let out = run(&fixture("features-good"), false);
    assert!(out.status.success(), "{out:?}");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("ok: 2 features in 1 map(s), 1 planned"),
        "{text}"
    );
}

#[test]
fn orphan_file_is_reported() {
    assert_has(
        "features-orphan",
        "README.md:1: [features] orphan file `list.md`",
    );
}

#[test]
fn dead_and_duplicate_entries_are_reported() {
    assert_has(
        "features-dead",
        "README.md:4: [features] duplicate entry `add.md`",
    );
    assert_has(
        "features-dead",
        "README.md:5: [features] dead entry `gone.md`",
    );
}

#[test]
fn missing_and_misordered_sections_are_reported() {
    assert_has(
        "features-sections",
        "no-h1.md:1: [features] missing H1 title",
    );
    assert_has(
        "features-sections",
        "missing.md:1: [features] missing H2 `Proof`",
    );
    assert_has(
        "features-sections",
        "order.md:1: [features] missing H2 `Proof` (or out of order)",
    );
}

#[test]
fn gotchas_is_required_and_last() {
    assert_has(
        "features-sections",
        "no-gotchas.md:1: [features] missing H2 `Gotchas` (or out of order)",
    );
    assert_has(
        "features-sections",
        "gotchas-first.md:1: [features] missing H2 `Gotchas` (or out of order)",
    );
}

#[test]
fn empty_gotchas_is_reported() {
    assert_has(
        "features-sections",
        "empty-gotchas.md:25: [features] empty section `Gotchas`",
    );
}

#[test]
fn empty_sections_are_reported() {
    assert_has(
        "features-sections",
        "empty.md:21: [features] empty section `Proof`",
    );
}

#[test]
fn headings_inside_code_fences_do_not_count() {
    assert_has(
        "features-sections",
        "fenced.md:1: [features] missing H2 `Proof`",
    );
}

#[test]
fn project_root_finds_the_verify_skill_map() {
    let root = scratch("features-project");
    std::fs::write(root.join("README.md"), "# Ordinary project README\n").unwrap();
    let dirs = features::locate(&root);
    assert_eq!(dirs.len(), 1);
    assert!(dirs[0].ends_with("verify-demo/features"));
    assert!(run(&root, false).status.success());
}

#[test]
fn no_map_is_a_failure() {
    let out = run(&fixture("features-empty"), false);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("no feature map found"));
}

#[test]
fn json_reports_problems_and_exit_code() {
    let out = run(&fixture("features-dead"), true);
    assert_eq!(out.status.code(), Some(1));
    let value: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["ok"], false);
    assert_eq!(value["problems"][0]["check"], "features");
}

#[test]
fn shipped_example_map_is_valid() {
    let example = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../skills/wstack-verify-create/references/feature-map-example");
    let out = run(&example, false);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
