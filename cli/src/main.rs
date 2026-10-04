use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use wstack::{checks, skills, sync};

#[derive(Parser)]
#[command(
    name = "wstack",
    version,
    about = "Lint and inspect a suite of agent skills"
)]
struct Cli {
    /// Repository root containing `skills/` and `shared/` (default: nearest ancestor of the current directory with `skills/`).
    #[arg(long, global = true)]
    root: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Lint skills: frontmatter, links, README listing, length, vendored copies, inline blocks.
    Check {
        #[arg(long)]
        json: bool,
    },
    /// List skills with their descriptions.
    List {
        #[arg(long)]
        json: bool,
    },
    /// Refresh vendored copies and inline blocks of `shared/` files inside skills.
    Sync,
}

fn check(root: &Path, as_json: bool) -> ExitCode {
    let problems = checks::run(root);
    let count = skills::discover(root).len();
    if as_json {
        println!(
            "{}",
            json!({ "ok": problems.is_empty(), "skills": count, "problems": problems })
        );
    } else if problems.is_empty() {
        println!("ok: {count} skills checked");
    } else {
        problems.iter().for_each(|p| eprintln!("{p}"));
        eprintln!("{} problem(s) in {count} skills", problems.len());
    }
    if problems.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn list(root: &Path, as_json: bool) -> ExitCode {
    let found = skills::discover(root);
    if as_json {
        let rows: Vec<_> = found
            .iter()
            .map(|s| json!({ "name": s.folder, "description": s.description() }))
            .collect();
        println!("{}", json!({ "skills": rows }));
    } else {
        found
            .iter()
            .for_each(|s| println!("{}\t{}", s.folder, s.description()));
    }
    ExitCode::SUCCESS
}

fn sync_command(root: &Path) -> ExitCode {
    match sync::run(root) {
        Ok(written) if written.is_empty() => {
            println!("vendored copies and inline blocks already up to date")
        }
        Ok(written) => written
            .iter()
            .for_each(|p| println!("wrote {}", p.display())),
        Err(message) => {
            eprintln!("error: {message}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let root = cli.root.unwrap_or_else(|| {
        let here = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        skills::find_root(&here)
    });
    match cli.command {
        Command::Check { json } => check(&root, json),
        Command::List { json } => list(&root, json),
        Command::Sync => sync_command(&root),
    }
}
