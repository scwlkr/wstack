use clap::{CommandFactory, Parser, Subcommand};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use wstack::{brand, catalog, checks, evidence, features, operations, proof, skills, sync};

#[derive(Parser)]
#[command(
    name = "wstack",
    version,
    about = "Lint and inspect a suite of agent skills",
    after_help = "Start with ./project info --json, then ./project doctor --json.\nUse ./project ci for all required local repository gates."
)]
struct Cli {
    /// Repository root containing `skills/` and `shared/` (default: nearest ancestor of the current directory with `skills/`).
    /// For features/evidence/brand: a project root (default: current directory).
    #[arg(long, global = true)]
    root: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Refresh a portable local brand guide, search assets, or retrieve prompt style JSON.
    Brand {
        /// Existing brand folder, relative to --root; no suite setup required.
        #[arg(long, global = true, default_value = "brand")]
        brand_dir: PathBuf,
        #[command(subcommand)]
        action: brand::Action,
    },
    /// Describe this checkout, revision, paths and supported commands.
    Info {
        #[arg(long)]
        json: bool,
        #[arg(long, default_value = "HEAD")]
        base: String,
    },
    /// Check read-only prerequisites for the feature-map CLI pilot.
    Doctor {
        #[arg(long)]
        json: bool,
    },
    /// Run all required local Rust and setup gates.
    Ci,
    /// Check retained receipt consistency; application assertions belong to its harness.
    Evidence {
        #[command(subcommand)]
        action: EvidenceAction,
    },
    /// Prove a mapped feature through the real CLI; retain artifacts after cleanup.
    Verify {
        #[arg(value_parser = ["feature-map", "brand"])]
        id: String,
        #[arg(long, default_value = "HEAD")]
        base: String,
        #[arg(long)]
        evidence_dir: Option<PathBuf>,
    },
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
}

#[derive(Subcommand)]
enum EvidenceAction {
    /// Require mapped coverage, current clean candidate/base, cleanup and readable artifacts.
    Check {
        report: PathBuf,
        #[arg(long, default_value = "HEAD")]
        base: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum FeaturesAction {
    /// Discover feature IDs and recipes from the canonical Markdown map.
    List {
        #[arg(long)]
        json: bool,
    },
    /// Read a feature's acceptance, entry points, prerequisites and proof recipe.
    Show {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Validate feature maps: README index, required sections, no orphan files.
    Check {
        #[arg(long)]
        json: bool,
    },
}

fn capabilities(command: clap::Command) -> serde_json::Value {
    json!({"name": command.get_name(), "description": command.get_about().map(ToString::to_string),
        "arguments": command.get_arguments().map(|a| json!({"id": a.get_id().as_str(), "long": a.get_long(), "required": a.is_required_set()})).collect::<Vec<_>>(),
        "commands": command.get_subcommands().cloned().map(capabilities).collect::<Vec<_>>()})
}

fn output(result: Result<serde_json::Value, String>, as_json: bool) -> ExitCode {
    match result {
        Ok(value) => {
            if as_json {
                println!("{value}");
            } else {
                println!("{}", serde_json::to_string_pretty(&value).unwrap());
            }
            if value["ready"] == false
                || matches!(value["status"].as_str(), Some("failed" | "blocked"))
            {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
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
        Command::Features { .. } | Command::Evidence { .. } | Command::Brand { .. } => here.clone(),
        _ => skills::find_root(&here),
    });
    match cli.command {
        Command::Brand { brand_dir, action } => brand::run(&root, &brand_dir, action),
        Command::Info { json, base } => output(
            operations::identity(&root, &base).map(|mut v| {
                v["capabilities"] = capabilities(Cli::command());
                v
            }),
            json,
        ),
        Command::Doctor { json } => output(Ok(operations::doctor(&root)), json),
        Command::Ci => output(
            operations::ci(&root).map(|()| json!({"status": "pass"})),
            false,
        ),
        Command::Evidence {
            action: EvidenceAction::Check { report, base, json },
        } => output(evidence::check(&root, &base, &report), json),
        Command::Verify {
            id,
            base,
            evidence_dir,
        } => output(
            if id == "brand" {
                brand::verify(&root, &base, evidence_dir)
            } else {
                proof::run(&root, &base, evidence_dir)
            },
            true,
        ),
        Command::Features {
            action: FeaturesAction::List { json },
        } => output(catalog::read(&root).map(|f| json!({"features": f})), json),
        Command::Features {
            action: FeaturesAction::Show { id, json },
        } => output(
            catalog::read(&root).and_then(|f| {
                f.into_iter()
                    .find(|f| f.id == id)
                    .map(|f| json!({"feature": f}))
                    .ok_or_else(|| format!("unknown feature `{id}`; run features list"))
            }),
            json,
        ),
        Command::Features {
            action: FeaturesAction::Check { json },
        } => features_check(&root, json),
        Command::Check { json } => check(&root, json),
        Command::List { json } => list(&root, json),
        Command::Sync => sync_command(&root),
    }
}
