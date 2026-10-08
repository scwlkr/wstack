//! The standalone skill helper and installed binary use identical bundled resources.
use clap::Subcommand;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Subcommand)]
pub enum Action {
    /// Refresh derived catalog/prompt/downloads; preserve authored guides and legacy ownership.
    Refresh {
        #[arg(long)]
        json: bool,
    },
    /// List actual local assets; search names, categories, descriptions and tags.
    List {
        #[arg(long, default_value = "")]
        query: String,
        #[arg(long)]
        json: bool,
    },
    /// Print compact image-prompt JSON from style.json (diagnostics on stderr).
    Style,
}

const RESOURCES: &[(&str, &str)] = &[
    (
        "scripts/brand.py",
        include_str!("../../skills/wstack-brand/scripts/brand.py"),
    ),
    (
        "scripts/guide.py",
        include_str!("../../skills/wstack-brand/scripts/guide.py"),
    ),
    (
        "scripts/metadata.py",
        include_str!("../../skills/wstack-brand/scripts/metadata.py"),
    ),
    (
        "scripts/generated.py",
        include_str!("../../skills/wstack-brand/scripts/generated.py"),
    ),
    (
        "assets/integration.js",
        include_str!("../../skills/wstack-brand/assets/integration.js"),
    ),
    (
        "assets/guide.html",
        include_str!("../../skills/wstack-brand/assets/guide.html"),
    ),
    (
        "assets/guide.css",
        include_str!("../../skills/wstack-brand/assets/guide.css"),
    ),
    (
        "assets/guide.js",
        include_str!("../../skills/wstack-brand/assets/guide.js"),
    ),
];

struct Bundle(PathBuf);

impl Drop for Bundle {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn bundle() -> Result<Bundle, String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let path =
        std::env::temp_dir().join(format!("wstack-brand-{}-{timestamp}", std::process::id()));
    fs::create_dir(&path).map_err(|e| format!("create brand helper directory: {e}"))?;
    let bundle = Bundle(path);
    for (name, contents) in RESOURCES {
        let target = bundle.0.join(name);
        fs::create_dir_all(target.parent().unwrap()).map_err(|e| e.to_string())?;
        fs::write(target, contents).map_err(|e| e.to_string())?;
    }
    Ok(bundle)
}

pub fn run(root: &Path, folder: &Path, action: Action) -> ExitCode {
    let result = (|| {
        let bundle = bundle()?;
        let mut command = Command::new("python3");
        command.arg(bundle.0.join("scripts/brand.py"));
        match action {
            Action::Refresh { json } => {
                command.arg("refresh");
                if json {
                    command.arg("--json");
                }
            }
            Action::List { query, json } => {
                command.args(["list", "--query", &query]);
                if json {
                    command.arg("--json");
                }
            }
            Action::Style => {
                command.arg("style");
            }
        }
        command
            .arg("--root")
            .arg(root)
            .arg("--brand-dir")
            .arg(folder);
        let status = command
            .status()
            .map_err(|e| format!("brand helper needs python3 on PATH: {e}"))?;
        Ok::<_, String>(if status.success() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        })
    })();
    result.unwrap_or_else(|message| {
        eprintln!("error: {message}");
        ExitCode::FAILURE
    })
}

pub fn verify(
    root: &Path,
    base: &str,
    evidence: Option<PathBuf>,
) -> Result<serde_json::Value, String> {
    let mut command = Command::new("node");
    command.env(
        "WSTACK_PROOF_BIN",
        std::env::current_exe().map_err(|e| e.to_string())?,
    );
    command
        .arg(root.join(".agents/skills/verify-wstack/scripts/brand-browser.cjs"))
        .arg(root)
        .arg(base);
    if let Some(path) = evidence {
        command.arg(path);
    }
    let output = command.output().map_err(|e| {
        format!("brand browser proof needs Node.js and Playwright; see verify-wstack: {e}")
    })?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().into());
    }
    serde_json::from_slice(&output.stdout).map_err(|e| format!("brand proof report: {e}"))
}
