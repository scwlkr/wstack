//! Validate a verification skill's feature map: README index plus one file per feature.

use crate::links;
use crate::problem::Problem;
use std::fs;
use std::path::{Path, PathBuf};

/// Required H2 headings, in order; a heading matches when it starts with the entry.
pub const REQUIRED: [&str; 5] = [
    "Sub-features",
    "How to get to it",
    "Driving it with",
    "Proof",
    "Gotchas",
];
const SKILL_DIRS: [&str; 3] = [".agents/skills", ".cursor/skills", ".claude/skills"];
const CHECK: &str = "features";

#[derive(Debug, Default)]
pub struct Report {
    pub dir: PathBuf,
    pub features: usize,
    pub planned: usize,
    pub problems: Vec<Problem>,
}

pub fn locate(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for base in SKILL_DIRS {
        let Ok(entries) = fs::read_dir(root.join(base)) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let features = entry.path().join("features");
            if name.to_string_lossy().starts_with("verify-") && features.is_dir() {
                found.push(features);
            }
        }
    }
    found.sort();
    if found.is_empty() && root.join("README.md").is_file() {
        found.push(root.to_path_buf());
    }
    found
}

struct Headings {
    h1: bool,
    /// Line, title and whether the section has any body text.
    h2: Vec<(usize, String, bool)>,
    planned: bool,
}

fn headings(text: &str) -> Headings {
    let mut found = Headings {
        h1: false,
        h2: Vec::new(),
        planned: false,
    };
    let mut fenced = false;
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        let is_fence = trimmed.starts_with("```") || trimmed.starts_with("~~~");
        let heading = !fenced && !is_fence && trimmed.starts_with('#');
        fenced ^= is_fence;
        if let (true, Some(title)) = (heading, trimmed.strip_prefix("## ")) {
            found.h2.push((index + 1, title.trim().to_string(), false));
        } else if heading && trimmed.starts_with("# ") {
            found.h1 = true;
        } else if !trimmed.is_empty() {
            found.planned |= trimmed == "Status: planned";
            if let Some(last) = found.h2.last_mut() {
                last.2 = true;
            }
        }
    }
    found
}

fn file_problems(root: &Path, file: &Path, report: &mut Report) {
    let text = fs::read_to_string(file).unwrap_or_default();
    let found = headings(&text);
    let mut add = |line: usize, message: String| {
        report
            .problems
            .push(Problem::new(root, file, line, CHECK, message));
    };
    if !found.h1 {
        add(1, "missing H1 title".into());
    }
    let mut from = 0;
    for required in REQUIRED {
        let at = found.h2[from..]
            .iter()
            .position(|(_, title, _)| title.starts_with(required));
        match at {
            Some(offset) => from += offset + 1,
            None => add(1, format!("missing H2 `{required}` (or out of order)")),
        }
    }
    for (line, title, has_body) in &found.h2 {
        if !has_body && REQUIRED.iter().any(|r| title.starts_with(r)) {
            add(*line, format!("empty section `{title}`"));
        }
    }
    report.planned += usize::from(found.planned);
}

fn indexed(readme: &str) -> Vec<(usize, String)> {
    links::extract(readme)
        .into_iter()
        .filter_map(|link| {
            let target = link.target.split('#').next()?.trim_start_matches("./");
            let local = target.ends_with(".md") && !target.contains('/');
            (local && target != "README.md").then(|| (link.line, target.to_string()))
        })
        .collect()
}

fn markdown_files(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".md") && n != "README.md")
        .collect();
    names.sort();
    names
}

fn index_problems(root: &Path, dir: &Path, files: &[String], report: &mut Report) {
    let readme = dir.join("README.md");
    let Ok(text) = fs::read_to_string(&readme) else {
        let message = "missing README.md index".to_string();
        report
            .problems
            .push(Problem::new(root, dir, 1, CHECK, message));
        return;
    };
    let listed = indexed(&text);
    let mut add = |line: usize, message: String| {
        report
            .problems
            .push(Problem::new(root, &readme, line, CHECK, message));
    };
    for (position, (line, name)) in listed.iter().enumerate() {
        if !files.contains(name) {
            add(*line, format!("dead entry `{name}`: no such file"));
        } else if listed[..position].iter().any(|(_, seen)| seen == name) {
            add(*line, format!("duplicate entry `{name}`"));
        }
    }
    for name in files {
        if !listed.iter().any(|(_, listed)| listed == name) {
            add(
                1,
                format!("orphan file `{name}` is not listed in the index"),
            );
        }
    }
}

/// Check one feature map directory; problem paths are relative to `root`.
pub fn check(root: &Path, dir: &Path) -> Report {
    let files = markdown_files(dir);
    let mut report = Report {
        dir: dir.to_path_buf(),
        features: files.len(),
        ..Report::default()
    };
    index_problems(root, dir, &files, &mut report);
    if files.is_empty() {
        let message = "no feature files".to_string();
        report
            .problems
            .push(Problem::new(root, dir, 1, CHECK, message));
    }
    for name in &files {
        file_problems(root, &dir.join(name), &mut report);
    }
    report
}
