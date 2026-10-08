//! A disposable, self-contained view of the canonical map. No server or asset bundle.
use crate::catalog;
use clap::Args;
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Args)]
pub struct Options {
    /// Generate the file and print its path without opening a browser.
    #[arg(long)]
    pub no_open: bool,
    /// Write to a new file instead of a unique temporary HTML file.
    #[arg(long)]
    pub output: Option<PathBuf>,
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn render(root: &Path) -> Result<String, String> {
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let mut rows = String::new();
    for feature in catalog::read(&root)? {
        let goals = feature
            .sections
            .iter()
            .find(|(heading, _)| heading.starts_with("Sub-features"))
            .map(|(_, body)| body.as_str())
            .unwrap_or_default();
        rows.push_str(&format!(
            "<tr data-source=\"{}\"><th scope=\"row\"><b>{}</b><small>{}</small></th><td class=\"description\"><div tabindex=\"0\">{}</div></td><td class=\"goals\"><div tabindex=\"0\">{}</div></td><td class=\"status\">{}</td><td><button type=\"button\" aria-label=\"Copy {}\" title=\"Copy {}\"><svg aria-hidden=\"true\" viewBox=\"0 0 16 16\"><path d=\"M6 3V1h9v10h-2M1 5h10v10H1z\"/></svg></button></td></tr>",
            escape(&feature.path), escape(&feature.title), escape(&feature.id),
            escape(&feature.description), escape(goals), feature.status,
            escape(&feature.title), escape(&feature.title)
        ));
    }
    Ok(include_str!("feature_view.html").replace("<!--rows-->", &rows))
}

fn generate(root: &Path, output: Option<PathBuf>) -> Result<PathBuf, String> {
    let html = render(root)?;
    let path = match output {
        Some(path) => path,
        None => {
            let stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_nanos();
            std::env::temp_dir().join(format!(
                "wstack-features-{}-{stamp}.html",
                std::process::id()
            ))
        }
    };
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| format!("create {} (output must be new): {e}", path.display()))?;
    if let Err(error) = file.write_all(html.as_bytes()) {
        drop(file);
        let _ = std::fs::remove_file(&path);
        return Err(format!("write {}: {error}", path.display()));
    }
    path.canonicalize().map_err(|e| e.to_string())
}

fn open(path: &Path) -> Result<(), String> {
    let program = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "windows") {
        "explorer"
    } else {
        "xdg-open"
    };
    let status = Command::new(program)
        .arg(path)
        .status()
        .map_err(|e| format!("{program}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program}: {status}"))
    }
}

pub fn run(root: &Path, options: Options) -> ExitCode {
    let result = generate(root, options.output).and_then(|path| {
        println!("{}", path.display());
        if !options.no_open {
            open(&path).map_err(|e| {
                format!(
                    "open {}: {e}; open it manually or use --no-open",
                    path.display()
                )
            })?;
        }
        Ok(())
    });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
