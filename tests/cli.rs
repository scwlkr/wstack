mod common;
use common::{fixture, scratch};
use serde_json::Value;
use std::process::{Command, Output};

fn wstack(args: &[&str], root: &std::path::Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wstack"))
        .args(args)
        .arg("--root")
        .arg(root)
        .output()
        .unwrap()
}

#[test]
fn check_succeeds_on_a_good_repo() {
    let out = wstack(&["check"], &fixture("good"));
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("ok: 1 skills checked"));
}

#[test]
fn check_fails_with_file_and_line_on_stderr() {
    let out = wstack(&["check"], &fixture("bad-link"));
    assert_eq!(out.status.code(), Some(1));
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("skills/alpha/SKILL.md:5: [links] broken link `missing.md`"),
        "{err}"
    );
}

#[test]
fn check_json_reports_problems_and_exit_code() {
    let out = wstack(&["check", "--json"], &fixture("bad-name"));
    assert_eq!(out.status.code(), Some(1));
    let value: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["ok"], false);
    assert_eq!(value["problems"][0]["file"], "skills/alpha/SKILL.md");
    assert_eq!(value["problems"][0]["line"], 2);
}

#[test]
fn check_json_ok_on_good_repo() {
    let out = wstack(&["check", "--json"], &fixture("good"));
    let value: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(out.status.success());
    assert_eq!(value["ok"], true);
    assert_eq!(value["skills"], 1);
}

#[test]
fn list_prints_names_and_descriptions() {
    let out = wstack(&["list"], &fixture("good"));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "alpha\tDo alpha things.\n"
    );
    let json = wstack(&["list", "--json"], &fixture("good"));
    let value: Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(value["skills"][0]["name"], "alpha");
}

#[test]
fn sync_command_fixes_what_check_reports() {
    let root = scratch("bad-drift");
    assert_eq!(wstack(&["check"], &root).status.code(), Some(1));
    assert!(wstack(&["sync"], &root).status.success());
    assert!(wstack(&["check"], &root).status.success());
}
