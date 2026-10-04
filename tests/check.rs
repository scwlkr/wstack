mod common;
use common::{fixture, scratch};
use std::fs;
use wstack::checks;
use wstack::problem::Problem;

fn run(name: &str) -> Vec<Problem> {
    checks::run(&fixture(name))
}

fn only(problems: &[Problem], check: &str) -> Vec<String> {
    problems
        .iter()
        .filter(|p| p.check == check)
        .map(|p| p.to_string())
        .collect()
}

#[test]
fn good_fixture_passes() {
    assert_eq!(run("good"), Vec::new());
}

#[test]
fn name_must_match_folder() {
    let found = only(&run("bad-name"), "frontmatter");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].starts_with("skills/alpha/SKILL.md:2:"),
        "{found:?}"
    );
    assert!(found[0].contains("does not match folder `alpha`"));
}

#[test]
fn description_is_required() {
    let found = only(&run("bad-description"), "frontmatter");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("missing `description`"));
}

#[test]
fn unclosed_frontmatter_is_reported() {
    let found = only(&run("bad-unclosed"), "frontmatter");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("unclosed"));
}

#[test]
fn broken_links_are_reported_with_lines() {
    let found = only(&run("bad-link"), "links");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].starts_with("skills/alpha/SKILL.md:5:"),
        "{found:?}"
    );
    assert!(found[0].contains("missing.md"));
}

#[test]
fn skill_links_may_not_leave_the_skill_folder() {
    let found = only(&run("bad-escape"), "links");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("leaves the skill folder"));
}

#[test]
fn skills_must_be_listed_in_readme() {
    let found = only(&run("bad-readme"), "readme");
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found
        .iter()
        .any(|m| m.contains("skill `gamma` is not listed")));
    assert!(found
        .iter()
        .any(|m| m.starts_with("README.md:6:") && m.contains("wstack-ghost")));
}

#[test]
fn drifted_vendored_copy_is_reported() {
    let found = only(&run("bad-drift"), "sync");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].starts_with("skills/alpha/references/rules.md:1:"));
}

#[test]
fn skills_sh_grouping_must_match_skills() {
    let found = only(&run("bad-grouping"), "skills.sh");
    assert_eq!(found.len(), 2, "{found:?}");
}

#[test]
fn long_files_are_reported_at_the_first_excess_line() {
    let root = scratch("good");
    let body = "line\n".repeat(301);
    fs::write(root.join("skills/alpha/notes.md"), &body).unwrap();
    fs::write(root.join("skills/alpha/edge.md"), "line\n".repeat(300)).unwrap();
    let found = only(&checks::run(&root), "length");
    assert_eq!(
        found,
        vec!["skills/alpha/notes.md:301: [length] 301 lines; limit is 300".to_string()]
    );
}

#[test]
fn sync_repairs_drift_and_check_then_passes() {
    let root = scratch("bad-drift");
    assert_eq!(wstack::sync::run(&root).unwrap().len(), 1);
    assert_eq!(checks::run(&root), Vec::new());
    assert!(wstack::sync::run(&root).unwrap().is_empty());
}

#[test]
fn inline_block_in_sync_passes() {
    assert_eq!(run("good-inline"), Vec::new());
}

#[test]
fn drifted_inline_block_is_reported_at_its_begin_marker() {
    let found = only(&run("bad-inline"), "sync");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].starts_with("skills/alpha/SKILL.md:7:"),
        "{found:?}"
    );
    assert!(found[0].contains("inline block of shared/rules.md"));
}

#[test]
fn missing_inline_markers_are_reported() {
    let found = only(&run("bad-inline-markers"), "sync");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("missing markers"), "{found:?}");
}

#[test]
fn sync_repairs_inline_block_and_keeps_surrounding_text() {
    let root = scratch("bad-inline");
    assert_eq!(wstack::sync::run(&root).unwrap().len(), 1);
    assert_eq!(checks::run(&root), Vec::new());
    let fixed = fs::read_to_string(root.join("skills/alpha/SKILL.md")).unwrap();
    let good = fs::read_to_string(fixture("good-inline").join("skills/alpha/SKILL.md")).unwrap();
    assert_eq!(fixed, good);
    assert!(wstack::sync::run(&root).unwrap().is_empty());
}

#[test]
fn sync_refuses_to_guess_where_a_missing_inline_block_goes() {
    let root = scratch("bad-inline-markers");
    let before = fs::read_to_string(root.join("skills/alpha/SKILL.md")).unwrap();
    assert!(wstack::sync::run(&root)
        .unwrap_err()
        .contains("missing markers"));
    assert_eq!(
        fs::read_to_string(root.join("skills/alpha/SKILL.md")).unwrap(),
        before
    );
}
