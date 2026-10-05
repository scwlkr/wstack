use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const MANIFEST: &str = "capabilities.json";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub version: u32,
    pub probes: Vec<Probe>,
    #[serde(default)]
    pub gaps: Vec<Gap>,
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
