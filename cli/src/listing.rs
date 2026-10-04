//! Skills must be listed in the README table and, when present, `skills.sh.json`.

use crate::problem::Problem;
use crate::skills::Skill;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn table_names(text: &str) -> BTreeMap<String, usize> {
    let mut names = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
        let Some(row) = line.trim_start().strip_prefix('|') else {
            continue;
        };
        let cell = row.split('|').next().unwrap_or("").trim();
        let name = cell.trim_matches('`');
        if cell.starts_with('`') && cell.ends_with('`') {
            names.entry(name.to_string()).or_insert(index + 1);
        }
    }
    names
}

/// Every skill folder appears as `` `name` `` in the first column of a README table row.
pub fn readme(root: &Path, skills: &[Skill]) -> Vec<Problem> {
    let path = root.join("README.md");
    let Ok(text) = fs::read_to_string(&path) else {
        return vec![Problem::new(
            root,
            &path,
            1,
            "readme",
            "README.md not found".into(),
        )];
    };
    let listed = table_names(&text);
    let fallback = listed.values().min().copied().unwrap_or(1);
    let mut out: Vec<Problem> = skills
        .iter()
        .filter(|s| !listed.contains_key(&s.folder))
        .map(|s| {
            Problem::new(
                root,
                &path,
                fallback,
                "readme",
                format!("skill `{}` is not listed in the README table", s.folder),
            )
        })
        .collect();
    let known: Vec<&str> = skills.iter().map(|s| s.folder.as_str()).collect();
    for (name, line) in &listed {
        if name.starts_with("wstack") && !known.contains(&name.as_str()) {
            out.push(Problem::new(
                root,
                &path,
                *line,
                "readme",
                format!("README lists `{name}` but no such skill folder exists"),
            ));
        }
    }
    out
}

fn grouped_names(value: &Value) -> Vec<String> {
    let groups = value["groupings"].as_array().cloned().unwrap_or_default();
    groups
        .iter()
        .flat_map(|g| g["skills"].as_array().cloned().unwrap_or_default())
        .filter_map(|v| v.as_str().map(String::from))
        .collect()
}

/// Every skill is grouped in `skills.sh.json`, and every grouped name exists.
pub fn skills_sh(root: &Path, skills: &[Skill]) -> Vec<Problem> {
    let path = root.join("skills.sh.json");
    let Ok(raw) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_str::<Value>(&raw) else {
        return vec![Problem::new(
            root,
            &path,
            1,
            "skills.sh",
            "invalid JSON".into(),
        )];
    };
    let grouped = grouped_names(&value);
    let missing = skills
        .iter()
        .filter(|s| !grouped.contains(&s.folder))
        .map(|s| format!("skill `{}` is not in any grouping", s.folder));
    let unknown = grouped
        .iter()
        .filter(|g| !skills.iter().any(|s| &s.folder == *g))
        .map(|g| format!("grouping lists unknown skill `{g}`"));
    missing
        .chain(unknown)
        .map(|m| Problem::new(root, &path, 1, "skills.sh", m))
        .collect()
}
