//! Receipt consistency only: live harnesses remain responsible for behavior assertions.
mod surface;
use crate::{catalog, features, operations};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path},
};

fn artifact(base: &Path, name: &str, nonempty: bool) -> Result<(), String> {
    let path = Path::new(name);
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(format!(
            "artifact must stay inside retained evidence: {name}"
        ));
    }
    let target = base.join(path);
    for part in target.ancestors().take_while(|p| *p != base) {
        if part.is_symlink() {
            return Err(format!(
                "artifact symlink is not retained owned evidence: {name}"
            ));
        }
    }
    let bytes = fs::read(&target).map_err(|e| format!("unreadable artifact {name}: {e}"))?;
    if nonempty && bytes.is_empty() {
        return Err(format!("empty browser artifact: {name}"));
    }
    Ok(())
}

fn artifacts(row: &Value) -> Result<Vec<&str>, String> {
    let mut names = Vec::new();
    if let Some(items) = row.get("artifacts") {
        for item in items.as_array().ok_or("artifacts must be an array")? {
            names.push(item.as_str().ok_or("artifact name must be a string")?);
        }
    }
    for key in ["artifact", "raw_body", "dom_artifact", "screenshot"] {
        if let Some(value) = row.get(key) {
            names.push(value.as_str().ok_or("artifact name must be a string")?);
        }
    }
    if names.is_empty() {
        return Err("observation has no retained artifact".into());
    }
    Ok(names)
}

fn inspect(root: &Path, base: &str, path: &Path, report: &Value) -> Result<usize, String> {
    let current = operations::identity(root, base)?;
    if current["dirty"] != false {
        return Err("current candidate is dirty".into());
    }
    for key in ["commit", "comparison_base"] {
        if report["identity"][key] != current[key] {
            return Err(format!(
                "report {key} does not match current resolved revision"
            ));
        }
    }
    if report["identity"]["dirty"] != false
        || report["candidate_unchanged"] != true
        || report["cleanup"] != true
        || report["status"] != "pass"
    {
        return Err(
            "receipt requires clean unchanged candidate, pass status and completed cleanup".into(),
        );
    }
    if features::locate(root).len() != 1 {
        return Err("receipt requires one canonical feature map".into());
    }
    let id = report["feature"]
        .as_str()
        .ok_or("receipt has no feature ID")?;
    let feature = catalog::read(root)?
        .into_iter()
        .find(|f| f.id == id)
        .ok_or("receipt feature is not mapped")?;
    if feature.status != "implemented" || feature.required_observations.is_empty() {
        return Err(
            "declare implemented feature's required observations in its Proof section".into(),
        );
    }
    let surface = report["surface"].as_str().ok_or("receipt has no surface")?;
    surface::identity(report, surface)?;
    let rows = report["coverage"]
        .as_array()
        .ok_or("receipt has no coverage array")?;
    let mut covered = BTreeSet::new();
    let directory = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()
        .map_err(|e| e.to_string())?;
    for row in rows {
        let case = row["case"].as_str().ok_or("coverage has no case")?;
        let entry = row["entry_point"]
            .as_str()
            .ok_or("coverage has no entrypoint")?;
        if row["status"] != "pass" || !covered.insert((case, entry)) {
            return Err(format!(
                "failed/skipped or duplicate observation: {case} | {entry}"
            ));
        }
        surface::observation(report, row, surface)?;
        for name in artifacts(row)? {
            artifact(&directory, name, surface == "browser")?;
        }
    }
    for required in &feature.required_observations {
        if !covered.contains(&(required.case.as_str(), required.entry_point.as_str())) {
            return Err(format!(
                "missing required observation: {} | {}",
                required.case, required.entry_point
            ));
        }
    }
    // Detect edits made while the receipt/map/artifacts were being inspected.
    if operations::identity(root, base)? != current {
        return Err("candidate changed during receipt checking".into());
    }
    Ok(covered.len())
}

pub fn check(root: &Path, base: &str, path: &Path) -> Result<Value, String> {
    let report: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let result = inspect(root, base, path, &report);
    Ok(
        json!({"ok": result.is_ok(), "status": if result.is_ok() {"pass"} else {"failed"},
        "observations": result.as_ref().ok(), "problems": result.err().into_iter().collect::<Vec<_>>(),
        "report": path, "scope": "receipt consistency; live harness owns behavior assertions and runtime identity qualification"}),
    )
}
