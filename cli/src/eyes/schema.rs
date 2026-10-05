use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::fs;
use std::path::{Path, PathBuf};

pub const MANIFEST: &str = "capabilities.json";

pub struct Manifest {
    pub probes: Vec<Probe>,
    pub gaps: Vec<Gap>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    version: u32,
    probes: Vec<Box<RawValue>>,
    #[serde(default)]
    gaps: Vec<Box<RawValue>>,
}

#[derive(Deserialize)]
struct FeatureScope {
    feature: String,
}

pub fn load(path: &Path, selected: &[String]) -> Result<Manifest, String> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("cannot read {MANIFEST}: {e}; register probes or explicit gaps"))?;
    let raw: RawManifest = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    if raw.version != 1 {
        return Err(format!("unsupported capabilities version {}", raw.version));
    }
    let wanted = |entry: &RawValue| {
        selected.is_empty()
            || serde_json::from_str::<FeatureScope>(entry.get())
                .map(|scope| selected.contains(&scope.feature))
                .unwrap_or(true)
    };
    let probes = raw
        .probes
        .into_iter()
        .filter(|entry| wanted(entry))
        .map(|entry| serde_json::from_str(entry.get()))
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    let gaps = raw
        .gaps
        .into_iter()
        .filter(|entry| wanted(entry))
        .map(|entry| serde_json::from_str(entry.get()))
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    Ok(Manifest { probes, gaps })
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Probe {
    pub feature: String,
    pub name: String,
    pub covers: Vec<String>,
    pub setup: Vec<String>,
    pub observe: Vec<String>,
    pub act: Vec<String>,
    pub assert: Vec<String>,
    pub cleanup: Vec<String>,
    pub timeout_ms: u64,
}

impl Probe {
    pub fn commands(&self) -> [(&'static str, &Vec<String>); 5] {
        [
            ("setup", &self.setup),
            ("observe", &self.observe),
            ("act", &self.act),
            ("assert", &self.assert),
            ("cleanup", &self.cleanup),
        ]
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub feature: String,
    pub capability: String,
    pub reason: String,
    pub owner: String,
    pub next: String,
}

#[derive(Serialize)]
pub struct Capability {
    pub map: PathBuf,
    pub feature: String,
    pub capability: String,
    pub status: &'static str,
    pub detail: String,
    pub owner: Option<String>,
    pub next: Option<String>,
    pub probe: Option<String>,
}

#[derive(Serialize)]
pub struct Step {
    pub stage: &'static str,
    pub argv: Vec<String>,
    pub exit: Option<i32>,
    pub timed_out: bool,
    pub error: Option<String>,
    pub stdout: PathBuf,
    pub stderr: PathBuf,
}

impl Step {
    pub fn passed(&self) -> bool {
        self.exit == Some(0) && !self.timed_out && self.error.is_none()
    }
}

#[derive(Serialize)]
pub struct Execution {
    pub map: PathBuf,
    pub feature: String,
    pub probe: String,
    pub passed: bool,
    pub steps: Vec<Step>,
}
