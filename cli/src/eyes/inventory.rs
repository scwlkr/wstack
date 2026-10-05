use super::schema::{self, Capability, Probe, MANIFEST};
use crate::{features, problem::Problem};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Inventory {
    pub capabilities: Vec<Capability>,
    pub probes: Vec<(PathBuf, Probe)>,
    pub problems: Vec<Problem>,
}

fn subfeatures(file: &Path) -> Result<(Vec<String>, bool), String> {
    let text = fs::read_to_string(file).map_err(|e| e.to_string())?;
    let mut inside = false;
    let mut fenced = false;
    let mut ids = BTreeSet::new();
    for line in text.lines().map(str::trim) {
        if line.starts_with("```") || line.starts_with("~~~") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        if line.starts_with("## ") {
            inside = line.starts_with("## Sub-features");
        } else if inside && !line.is_empty() {
            let id = line
                .strip_prefix("- `")
                .and_then(|s| s.split_once('`'))
                .map(|p| p.0);
            let Some(id) = id.filter(|s| {
                !s.is_empty()
                    && s.chars()
                        .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
            }) else {
                return Err("Sub-features must use bullet lines starting with a backtick-quoted capability ID".into());
            };
            if !ids.insert(id.to_string()) {
                return Err(format!("duplicate capability `{id}`"));
            }
        }
    }
    if ids.is_empty() {
        return Err("no sub-feature capability IDs".into());
    }
    Ok((
        ids.into_iter().collect(),
        text.lines().any(|l| l.trim() == "Status: planned"),
    ))
}

fn valid_probe(probe: &Probe) -> bool {
    !probe.name.trim().is_empty()
        && !probe.covers.is_empty()
        && (1..=300_000).contains(&probe.timeout_ms)
        && probe.commands().iter().all(|(_, argv)| {
            argv.first().is_some_and(|s| !s.trim().is_empty())
                && argv.iter().all(|s| !s.contains('\0'))
        })
}

fn register_map(root: &Path, dir: &Path, selected: &[String], out: &mut Inventory) {
    let report = features::check_scope(root, dir, selected);
    out.problems.extend(report.problems);
    let mut known = BTreeMap::new();
    let mut files: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension().is_some_and(|e| e == "md")
                && p.file_name().is_some_and(|n| n != "README.md")
        })
        .collect();
    files.sort();
    for file in files {
        if !selected.is_empty()
            && !file
                .file_name()
                .is_some_and(|n| selected.iter().any(|s| n == s.as_str()))
        {
            continue;
        }
        match subfeatures(&file) {
            Ok((ids, planned)) => {
                let feature = file.file_name().unwrap().to_string_lossy().to_string();
                for id in ids {
                    known.insert((feature.clone(), id), planned);
                }
            }
            Err(message) => {
                out.problems
                    .push(Problem::new(root, &file, 1, "eyes-and-hands", message))
            }
        }
    }
    let path = dir.join(MANIFEST);
    let manifest = match schema::load(&path, selected) {
        Ok(manifest) => Some(manifest),
        Err(message) => {
            out.problems
                .push(Problem::new(root, &path, 1, "eyes-and-hands", message));
            None
        }
    };
    let mut owners = BTreeMap::new();
    if let Some(manifest) = &manifest {
        let mut names = BTreeSet::new();
        let mut claim_problems = Vec::new();
        let mut claim = |key: (String, String), value: (Option<String>, Option<usize>)| {
            let message = if !known.contains_key(&key) {
                Some(format!("unknown capability {}:{}", key.0, key.1))
            } else if owners.insert(key.clone(), value).is_some() {
                Some(format!("duplicate coverage for {}:{}", key.0, key.1))
            } else {
                None
            };
            if let Some(message) = message {
                claim_problems.push(Problem::new(root, &path, 1, "eyes-and-hands", message));
            }
        };
        for probe in &manifest.probes {
            if !valid_probe(probe) || !names.insert(&probe.name) {
                out.problems.push(Problem::new(root, &path, 1, "eyes-and-hands", format!("invalid or duplicate probe `{}`: require covers, five argv commands and timeout_ms 1..300000", probe.name)));
            }
            for id in &probe.covers {
                claim(
                    (probe.feature.clone(), id.clone()),
                    (Some(probe.name.clone()), None),
                );
            }
        }
        for (index, gap) in manifest.gaps.iter().enumerate() {
            if [&gap.reason, &gap.owner, &gap.next]
                .iter()
                .any(|s| s.trim().is_empty())
            {
                out.problems.push(Problem::new(
                    root,
                    &path,
                    1,
                    "eyes-and-hands",
                    "gap requires reason, owner and next action".into(),
                ));
            }
            claim(
                (gap.feature.clone(), gap.capability.clone()),
                (None, Some(index)),
            );
        }
        out.problems.extend(claim_problems);
    }
    for ((feature, id), planned) in known {
        if !selected.is_empty() && !selected.contains(&feature) {
            continue;
        }
        let mut row = Capability {
            map: dir.to_path_buf(),
            feature: feature.clone(),
            capability: id.clone(),
            status: "missing",
            detail: "no registered CLI recipe".into(),
            owner: None,
            next: Some("add a probe or explicit gap to capabilities.json".into()),
            probe: None,
        };
        match owners.get(&(feature, id)) {
            Some((Some(name), _)) => {
                row.probe = Some(name.clone());
                row.status = "unproven";
                row.detail = "registered commands have not run in this audit".into();
                row.next = Some("run the audit with --run within authorized scope".into());
            }
            Some((_, Some(index))) => {
                let gap = &manifest.as_ref().unwrap().gaps[*index];
                row.status = "blocked";
                row.detail = gap.reason.clone();
                row.owner = Some(gap.owner.clone());
                row.next = Some(gap.next.clone());
            }
            _ => {}
        }
        if planned {
            row.status = "planned";
            row.detail = "feature is marked Status: planned; live access is not proven".into();
        }
        out.capabilities.push(row);
    }
    if let Some(manifest) = manifest {
        out.probes.extend(
            manifest
                .probes
                .into_iter()
                .filter(|p| {
                    out.capabilities.iter().any(|r| {
                        r.map == dir
                            && r.probe.as_deref() == Some(&p.name)
                            && r.status == "unproven"
                    })
                })
                .map(|p| (dir.to_path_buf(), p)),
        );
    }
}

pub fn inspect(root: &Path, selected: &[String]) -> Inventory {
    let mut out = Inventory {
        capabilities: Vec::new(),
        probes: Vec::new(),
        problems: Vec::new(),
    };
    let maps = features::locate(root);
    if maps.is_empty() {
        out.problems.push(Problem::new(
            root,
            root,
            1,
            "eyes-and-hands",
            "no feature map; create a verify-<project>/features inventory first".into(),
        ));
    }
    for dir in maps {
        if !selected.is_empty()
            && !fs::read_dir(&dir)
                .into_iter()
                .flatten()
                .flatten()
                .any(|e| selected.iter().any(|s| e.file_name() == s.as_str()))
        {
            continue;
        }
        register_map(root, &dir, selected, &mut out);
    }
    for name in selected {
        if !out.capabilities.iter().any(|r| &r.feature == name) {
            out.problems.push(Problem::new(
                root,
                root,
                1,
                "eyes-and-hands",
                format!("unknown selected feature `{name}`"),
            ));
        }
    }
    out
}
