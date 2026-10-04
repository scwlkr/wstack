//! Lint rules. Each rule is a small function returning problems; `run` combines them.

use crate::frontmatter::valid_name;
use crate::problem::Problem;
use crate::skills::{discover, walk, Skill};
use crate::{links, listing, sync};
use std::fs;
use std::path::Path;

pub const MAX_LINES: usize = 300;

fn missing_field(root: &Path, skill: &Skill, line: usize, field: &str) -> Problem {
    Problem::new(
        root,
        &skill.file,
        line,
        "frontmatter",
        format!("missing `{field}`"),
    )
}

fn name_problems(root: &Path, skill: &Skill) -> Vec<Problem> {
    let front = skill.frontmatter();
    let at = |line: usize, message: String| {
        Problem::new(root, &skill.file, line, "frontmatter", message)
    };
    let mut out = Vec::new();
    match &front.name {
        None => out.push(missing_field(root, skill, 1, "name")),
        Some((line, name)) => {
            if !valid_name(name) {
                out.push(at(*line, format!("invalid name `{name}`: use lowercase letters, digits, single hyphens, max 64")));
            }
            if *name != skill.folder {
                out.push(at(
                    *line,
                    format!("name `{name}` does not match folder `{}`", skill.folder),
                ));
            }
        }
    }
    out
}

fn description_problems(root: &Path, skill: &Skill) -> Vec<Problem> {
    let front = skill.frontmatter();
    match front.description {
        None => vec![missing_field(root, skill, 1, "description")],
        Some((line, text)) if text.is_empty() || text.len() > 1024 => vec![Problem::new(
            root,
            &skill.file,
            line,
            "frontmatter",
            "description must be 1 to 1024 characters".to_string(),
        )],
        Some(_) => Vec::new(),
    }
}

/// Valid `SKILL.md` frontmatter: name matches folder, description present.
pub fn frontmatter(root: &Path, skills: &[Skill]) -> Vec<Problem> {
    let mut out = Vec::new();
    for skill in skills {
        let front = skill.frontmatter();
        if skill.text.is_none() {
            out.push(Problem::new(
                root,
                &skill.dir,
                1,
                "frontmatter",
                "missing SKILL.md".into(),
            ));
        } else if front.start.is_none() || !front.closed {
            out.push(Problem::new(
                root,
                &skill.file,
                1,
                "frontmatter",
                "missing or unclosed `---` frontmatter".into(),
            ));
        } else {
            out.extend(name_problems(root, skill));
            out.extend(description_problems(root, skill));
        }
    }
    out
}

fn owning_skill<'a>(skills: &'a [Skill], file: &Path) -> Option<&'a Skill> {
    skills.iter().find(|skill| file.starts_with(&skill.dir))
}

fn link_problems(root: &Path, skills: &[Skill], file: &Path) -> Vec<Problem> {
    let Ok(text) = fs::read_to_string(file) else {
        return Vec::new();
    };
    let base = file.parent().unwrap_or(root);
    let owner = owning_skill(skills, file);
    let mut out = Vec::new();
    for link in links::extract(&text) {
        let target = links::resolve(base, &link.target);
        let report = |message: String| Problem::new(root, file, link.line, "links", message);
        if !target.exists() {
            out.push(report(format!("broken link `{}`", link.target)));
        } else if owner.is_some_and(|skill| !target.starts_with(&skill.dir)) {
            out.push(report(format!(
                "link `{}` leaves the skill folder; vendor it with `wstack sync`",
                link.target
            )));
        }
    }
    out
}

/// Every relative markdown link resolves; skill links stay inside their skill folder.
pub fn links(root: &Path, skills: &[Skill]) -> Vec<Problem> {
    walk(root)
        .into_iter()
        .filter(|f| f.extension().is_some_and(|e| e == "md"))
        .filter(|f| {
            !f.strip_prefix(root)
                .is_ok_and(|r| r.starts_with("tests/fixtures"))
        })
        .flat_map(|file| link_problems(root, skills, &file))
        .collect()
}

/// Skill and shared files stay within the line budget.
pub fn line_limits(root: &Path) -> Vec<Problem> {
    ["skills", "shared"]
        .iter()
        .flat_map(|dir| walk(&root.join(dir)))
        .filter(|f| f.file_name().is_some_and(|n| n != "Cargo.lock"))
        .filter_map(|file| {
            let count = fs::read_to_string(&file).ok()?.lines().count();
            let message = format!("{count} lines; limit is {MAX_LINES}");
            (count > MAX_LINES).then(|| Problem::new(root, &file, MAX_LINES + 1, "length", message))
        })
        .collect()
}

/// Vendored copies match their `shared/` source.
pub fn vendored(root: &Path) -> Vec<Problem> {
    let copies = match sync::load(root) {
        Ok(Some(copies)) => copies,
        Ok(None) => return Vec::new(),
        Err(message) => {
            return vec![Problem::new(
                root,
                &root.join(sync::MANIFEST),
                1,
                "sync",
                message,
            )]
        }
    };
    copies
        .iter()
        .filter_map(|copy| {
            let (line, reason) = sync::problem(copy)?;
            let how = if copy.inline {
                "inline block of"
            } else {
                "copy of"
            };
            let message = format!("{reason}; {how} {}; run `wstack sync`", copy.label);
            Some(Problem::new(root, &copy.dest, line, "sync", message))
        })
        .collect()
}

/// Run every rule against the repository at `root`.
pub fn run(root: &Path) -> Vec<Problem> {
    let normalized = links::normalize(root);
    let root = normalized.as_path();
    let skills = discover(root);
    let mut problems = frontmatter(root, &skills);
    problems.extend(links(root, &skills));
    problems.extend(listing::readme(root, &skills));
    problems.extend(listing::skills_sh(root, &skills));
    problems.extend(line_limits(root));
    problems.extend(vendored(root));
    problems
}
