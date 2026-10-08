//! Discovery derived from the same Markdown index validated by the checker.
use crate::{features, links};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Serialize)]
pub struct Feature {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: &'static str,
    pub path: String,
    pub sections: BTreeMap<String, String>,
    pub required_observations: Vec<features::Observation>,
}

pub fn read(root: &Path) -> Result<Vec<Feature>, String> {
    let dirs = features::locate(root);
    if dirs.is_empty() {
        return Err("no feature map found; create a verify-*/features map".into());
    }
    let mut result = Vec::new();
    for dir in dirs {
        let report = features::check(root, &dir);
        if !report.problems.is_empty() {
            return Err(report
                .problems
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n"));
        }
        let index = fs::read_to_string(dir.join("README.md")).map_err(|e| e.to_string())?;
        for (_, name) in features::indexed(&index) {
            let path = dir.join(&name);
            let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let id = name.trim_end_matches(".md").to_string();
            if result.iter().any(|f: &Feature| f.id == id) {
                return Err(format!(
                    "ambiguous feature ID `{id}`; keep one canonical map"
                ));
            }
            let mut title = String::new();
            let mut description = String::new();
            let mut section = String::new();
            let mut sections = BTreeMap::<String, String>::new();
            let mut fenced = false;
            let mut planned = false;
            for line in text.lines() {
                let trimmed = line.trim();
                let fence = trimmed.starts_with("```") || trimmed.starts_with("~~~");
                if !fenced && !fence {
                    if let Some(h) = trimmed.strip_prefix("# ") {
                        title = h.to_string();
                        continue;
                    }
                    if let Some(h) = trimmed.strip_prefix("## ") {
                        section = h.to_string();
                        continue;
                    }
                    planned |= trimmed == "Status: planned";
                }
                fenced ^= fence;
                if !section.is_empty() {
                    let body = sections.entry(section.clone()).or_default();
                    body.push_str(line);
                    body.push('\n');
                } else if trimmed != "Status: planned" {
                    description.push_str(line);
                    description.push('\n');
                }
            }
            sections
                .values_mut()
                .for_each(|s| *s = s.trim().to_string());
            result.push(Feature {
                id,
                title,
                description: description.trim().to_string(),
                status: if planned { "planned" } else { "implemented" },
                path: links::normalize(&path).display().to_string(),
                sections,
                required_observations: features::observations(&text)?,
            });
        }
    }
    Ok(result)
}
