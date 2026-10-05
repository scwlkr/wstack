use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use wstack::{checks, eyes, features, skills, sync};

#[derive(Parser)]
#[command(
    name = "wstack",
    version,
    about = "Inspect agent skills and audit development capabilities"
)]
struct Cli {
    /// Repository root containing `skills/` and `shared/` (default: nearest ancestor of the current directory with `skills/`).
    /// For features or eyes-and-hands: a project root or feature map directory (default: current directory).
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
    /// Work with a verification skill's feature map.
    Features {
        #[command(subcommand)]
        action: FeaturesAction,
    },
    /// Audit scriptable observation and interaction against the feature map.
    EyesAndHands {
        #[command(subcommand)]
        action: EyesAction,
    },
}

#[derive(Subcommand)]
enum EyesAction {
    /// Find capability gaps; --run executes registered probes and writes a receipt.
    Check {
        #[arg(long)]
        json: bool,
        /// Execute setup/observe/act/assert/cleanup commands, including their side effects.
        #[arg(long)]
        run: bool,
        /// Limit the audit to named feature files (repeatable, e.g. --feature save.md).
        #[arg(long)]
        feature: Vec<String>,
        /// Parent for a fresh evidence directory (default: system temporary directory).
        #[arg(long, requires = "run")]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum FeaturesAction {
    /// Validate feature maps: README index, required sections, no orphan files.
    Check {
        #[arg(long)]
        json: bool,
    },
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

fn features_check(root: &Path, as_json: bool) -> ExitCode {
    let root = wstack::links::normalize(root);
    let reports: Vec<_> = features::locate(&root)
        .iter()
        .map(|dir| features::check(&root, dir))
        .collect();
    let problems: Vec<_> = reports.iter().flat_map(|r| &r.problems).collect();
    let count: usize = reports.iter().map(|r| r.features).sum();
    let planned: usize = reports.iter().map(|r| r.planned).sum();
    let none = reports.is_empty();
    if as_json {
        println!(
            "{}",
            json!({ "ok": !none && problems.is_empty(), "maps": reports.len(), "features": count, "planned": planned, "problems": problems })
        );
    } else if none {
        eprintln!("no feature map found under {}", root.display());
    } else if problems.is_empty() {
        println!(
            "ok: {count} features in {} map(s), {planned} planned",
            reports.len()
        );
    } else {
        problems.iter().for_each(|p| eprintln!("{p}"));
        eprintln!("{} problem(s) in {count} features", problems.len());
    }
    if none || !problems.is_empty() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let here = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let root = cli.root.unwrap_or_else(|| match cli.command {
        Command::Features { .. } | Command::EyesAndHands { .. } => here.clone(),
        _ => skills::find_root(&here),
    });
    match cli.command {
        Command::EyesAndHands {
            action:
                EyesAction::Check {
                    json,
                    run,
                    feature,
                    output,
                },
        } => eyes::command(&root, json, run, &feature, output.as_deref()),
        Command::Features {
            action: FeaturesAction::Check { json },
        } => features_check(&root, json),
        Command::Check { json } => check(&root, json),
        Command::List { json } => list(&root, json),
        Command::Sync => sync_command(&root),
    }
}
