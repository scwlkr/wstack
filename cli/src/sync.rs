//! Vendored copies: `shared/` is canonical, skills carry identical copies so each installs alone.
//! A destination is either a whole-file copy, or `{"inline": "<file>"}` for a block between markers.

use crate::inline;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const MANIFEST: &str = "shared/sync.json";

#[derive(Deserialize)]
#[serde(untagged)]
enum Target {
    File(String),
    Inline { inline: String },
}

pub struct Copy {
    /// Source path as written in the manifest, also the inline marker label.
    pub label: String,
    pub source: PathBuf,
    pub dest: PathBuf,
    pub inline: bool,
}

/// Parse `shared/sync.json`: `{ "<source>": ["<dest>", ...] }`. `Ok(None)` when absent.
pub fn load(root: &Path) -> Result<Option<Vec<Copy>>, String> {
    let Ok(raw) = fs::read_to_string(root.join(MANIFEST)) else {
        return Ok(None);
    };
    let map: BTreeMap<String, Vec<Target>> =
        serde_json::from_str(&raw).map_err(|e| format!("invalid {MANIFEST}: {e}"))?;
    let copies = map
        .into_iter()
        .flat_map(|(label, targets)| {
            targets.into_iter().map(move |target| {
                let (dest, inline) = match target {
                    Target::File(dest) => (dest, false),
                    Target::Inline { inline } => (inline, true),
                };
                Copy {
                    source: root.join(&label),
                    dest: root.join(dest),
                    label: label.clone(),
                    inline,
                }
            })
        })
        .collect();
    Ok(Some(copies))
}

fn block(copy: &Copy) -> Result<String, String> {
    let text =
        fs::read_to_string(&copy.source).map_err(|e| format!("{}: {e}", copy.source.display()))?;
    Ok(text.trim_end().to_string())
}

/// The destination text this copy should have, or why it cannot be produced.
fn expected(copy: &Copy) -> Result<String, String> {
    if !copy.inline {
        return fs::read_to_string(&copy.source)
            .map_err(|e| format!("{}: {e}", copy.source.display()));
    }
    let text = fs::read_to_string(&copy.dest).map_err(|e| format!("missing destination: {e}"))?;
    inline::render(&text, &copy.label, &block(copy)?)
}

/// `None` when the copy is current; otherwise the 1-based line and reason it is not.
pub fn problem(copy: &Copy) -> Option<(usize, String)> {
    let line = fs::read_to_string(&copy.dest)
        .ok()
        .and_then(|text| inline::locate(&text, &copy.label).ok())
        .filter(|_| copy.inline)
        .map_or(1, |(open, _)| open + 1);
    match expected(copy) {
        Ok(want) if fs::read_to_string(&copy.dest).is_ok_and(|have| have == want) => None,
        Ok(_) => Some((line, "differs from or is missing".into())),
        Err(message) => Some((line, message)),
    }
}

pub fn in_sync(copy: &Copy) -> bool {
    problem(copy).is_none()
}

/// Rewrite every out-of-date destination; returns the destinations written.
pub fn run(root: &Path) -> Result<Vec<PathBuf>, String> {
    let copies = load(root)?.ok_or_else(|| format!("missing {MANIFEST}"))?;
    let mut written = Vec::new();
    for copy in copies.iter().filter(|c| !in_sync(c)) {
        let want = expected(copy).map_err(|e| format!("{}: {e}", copy.dest.display()))?;
        if let Some(parent) = copy.dest.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&copy.dest, want).map_err(|e| format!("{}: {e}", copy.dest.display()))?;
        written.push(copy.dest.clone());
    }
    Ok(written)
}
