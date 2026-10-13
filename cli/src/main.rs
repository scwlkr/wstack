use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use wstack::{checks, skills, sync};

const HELP: &str = "wstack [--root PATH] <check|list|sync|ci> [--json]
  check   Validate skill metadata, links and shared copies
  list    Show skill names/descriptions
  sync    Refresh copies from shared/
  ci      Run this repository's required checks";

struct Args {
    command: String,
    root: PathBuf,
    json: bool,
}

fn parse() -> Result<Args, String> {
    let mut args = std::env::args().skip(1);
    let mut root = None;
    let mut command = None;
    let mut json = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = Some(PathBuf::from(args.next().ok_or("--root needs a path")?)),
            "--json" => json = true,
            "-h" | "--help" => command = Some("help".into()),
            "-V" | "--version" => command = Some("version".into()),
            "check" | "list" | "sync" | "ci" if command.is_none() => command = Some(arg),
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }
    let command = command.unwrap_or_else(|| "help".into());
    if json && !matches!(command.as_str(), "check" | "list") {
        return Err("--json supports check/list".into());
    }
    let here = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(Args {
        command,
        root: root.unwrap_or_else(|| skills::find_root(&here)),
        json,
    })
}

fn check(root: &Path, as_json: bool) -> ExitCode {
    let problems = checks::run(root);
    let warnings = checks::line_limits(root);
    let count = skills::discover(root).len();
    if as_json {
        println!(
            "{}",
            json!({"ok": problems.is_empty(), "skills": count,
            "problems": problems, "warnings": warnings})
        );
    } else {
        for warning in warnings {
            eprintln!("warning: {warning}; split when useful");
        }
        if problems.is_empty() {
            println!("ok: {count} skills checked");
        } else {
            problems.iter().for_each(|p| eprintln!("{p}"));
        }
    }
    if problems.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn list(root: &Path, as_json: bool) {
    let found = skills::discover(root);
    if as_json {
        let rows: Vec<_> = found
            .iter()
            .map(|s| json!({"name": s.folder, "description": s.description()}))
            .collect();
        println!("{}", json!({"skills": rows}));
    } else {
        found
            .iter()
            .for_each(|s| println!("{}\t{}", s.folder, s.description()));
    }
}

fn ci(root: &Path) -> Result<(), String> {
    for args in [
        vec!["build"],
        vec!["test"],
        vec!["clippy", "--all-targets", "--", "-D", "warnings"],
        vec!["run", "--", "check"],
    ] {
        let status = Command::new("cargo")
            .args(&args)
            .current_dir(root.join("cli"))
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!("cargo {} failed", args.join(" ")));
        }
    }
    let status = Command::new("python3")
        .args([
            "-m",
            "unittest",
            "discover",
            "-s",
            "tests",
            "-p",
            "test_setup.py",
        ])
        .current_dir(root)
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("setup checks failed".into());
    }
    Ok(())
}

fn run(args: Args) -> Result<ExitCode, String> {
    match args.command.as_str() {
        "help" => println!("{HELP}"),
        "version" => println!("wstack {}", env!("CARGO_PKG_VERSION")),
        "check" => return Ok(check(&args.root, args.json)),
        "list" => list(&args.root, args.json),
        "sync" => {
            for path in sync::run(&args.root)? {
                println!("wrote {}", path.display());
            }
        }
        "ci" => ci(&args.root)?,
        _ => unreachable!(),
    }
    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    match parse().and_then(run) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
