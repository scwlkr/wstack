//! Relative markdown link extraction and resolution.

use std::path::{Component, Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    pub line: usize,
    pub target: String,
}

fn strip_code_spans(line: &str) -> String {
    let mut out = String::new();
    let mut in_code = false;
    for c in line.chars() {
        if c == '`' {
            in_code = !in_code;
        } else if !in_code {
            out.push(c);
        }
    }
    out
}

fn is_fence(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("```") || trimmed.starts_with("~~~")
}

fn targets_in(line: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find("](") {
        rest = &rest[at + 2..];
        let Some(end) = rest.find(')') else { break };
        let raw = rest[..end].trim();
        let target = raw.split_whitespace().next().unwrap_or("");
        found.push(target.trim_matches(|c| c == '<' || c == '>').to_string());
        rest = &rest[end..];
    }
    found
}

fn is_local(target: &str) -> bool {
    let external = target.contains("://") || target.starts_with("mailto:");
    !target.is_empty() && !target.starts_with('#') && !external
}

/// Inline links to local paths, ignoring fenced blocks and inline code.
pub fn extract(text: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let mut fenced = false;
    for (index, line) in text.lines().enumerate() {
        if is_fence(line) {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        for target in targets_in(&strip_code_spans(line))
            .into_iter()
            .filter(|t| is_local(t))
        {
            links.push(Link {
                line: index + 1,
                target,
            });
        }
    }
    links
}

/// Link path without `#fragment` or `?query`.
pub fn path_part(target: &str) -> &str {
    target.split(['#', '?']).next().unwrap_or("")
}

/// Lexically collapse `.` and `..` components.
pub fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for part in path.components() {
        match part {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Lexically resolve `target` against `base`.
pub fn resolve(base: &Path, target: &str) -> PathBuf {
    normalize(&base.join(path_part(target)))
}
