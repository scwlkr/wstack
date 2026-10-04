//! Discovery of skill folders under `<root>/skills/`.

use crate::frontmatter::{self, Frontmatter};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Skill {
    pub folder: String,
    pub dir: PathBuf,
    pub file: PathBuf,
    pub text: Option<String>,
}

impl Skill {
    pub fn frontmatter(&self) -> Frontmatter {
        self.text
            .as_deref()
            .map(frontmatter::parse)
            .unwrap_or_default()
    }

    pub fn description(&self) -> String {
        self.frontmatter()
            .description
            .map(|(_, d)| d)
            .unwrap_or_default()
    }
}

/// Every directory directly under `skills/`, sorted by name.
pub fn discover(root: &Path) -> Vec<Skill> {
    let Ok(entries) = fs::read_dir(root.join("skills")) else {
        return Vec::new();
    };
    let mut skills: Vec<Skill> = entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| {
            let dir = entry.path();
            let file = dir.join("SKILL.md");
            let text = fs::read_to_string(&file).ok();
            let folder = entry.file_name().to_string_lossy().into_owned();
            Skill {
                folder,
                dir,
                file,
                text,
            }
        })
        .collect();
    skills.sort_by(|a, b| a.folder.cmp(&b.folder));
    skills
}

/// All regular files below `dir`, skipping build output and VCS data.
pub fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        if matches!(
            name.to_str(),
            Some(".git" | "target" | "node_modules" | "__pycache__")
        ) {
            continue;
        }
        if path.is_dir() {
            files.extend(walk(&path));
        } else {
            files.push(path);
        }
    }
    files.sort();
    files
}

/// Nearest ancestor of `start` (or `start` itself) that contains a `skills/` folder.
pub fn find_root(start: &Path) -> PathBuf {
    let found = start.ancestors().find(|dir| dir.join("skills").is_dir());
    found.unwrap_or(start).to_path_buf()
}
