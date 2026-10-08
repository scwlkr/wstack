use crate::{commands, json, metadata};
use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn maps(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for base in [".agents/skills", ".cursor/skills", ".claude/skills"] {
        if let Ok(entries) = fs::read_dir(root.join(base)) {
            for entry in entries.flatten() {
                if entry.file_name().to_string_lossy().starts_with("verify-")
                    && entry.path().join("features").is_dir()
                {
                    found.push(entry.path().join("features"));
                }
            }
        }
    }
    found.sort();
    found
}

pub fn info(root: &Path, args: &[String]) -> Result<i32, String> {
    let base = match args {
        [flag] if flag == "--json" => "HEAD",
        [flag, option, value] if flag == "--json" && option == "--base" => value,
        _ => return Err("Usage: info --json [--base REF]".into()),
    };
    let owns_git = git(root, &["rev-parse", "--show-toplevel"])
        .is_some_and(|path| Path::new(&path).canonicalize().ok() == root.canonicalize().ok());
    let commit = owns_git
        .then(|| git(root, &["rev-parse", "HEAD"]))
        .flatten();
    let comparison = if commit.is_some() {
        Some(
            git(
                root,
                &["rev-parse", "--verify", &format!("{base}^{{commit}}")],
            )
            .ok_or("comparison base does not resolve")?,
        )
    } else {
        None
    };
    let dirty = owns_git
        .then(|| git(root, &["status", "--porcelain"]))
        .flatten()
        .map(|s| (!s.is_empty()).to_string())
        .unwrap_or_else(|| "null".into());
    let found = maps(root);
    let map = (found.len() == 1).then(|| found[0].to_string_lossy().into_owned());
    let skill = (found.len() == 1).then(|| {
        found[0]
            .parent()
            .unwrap()
            .join("SKILL.md")
            .to_string_lossy()
            .into_owned()
    });
    let capabilities = commands::COMMANDS
        .iter()
        .map(|(_, id, args, description)| {
            format!(
                "{{\"id\":{},\"name\":{},\"arguments\":{},\"description\":{}}}",
                json::string(id),
                json::string(&commands::public_name(id)),
                json::string(args),
                json::string(description)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let routes = crate::routes::ROUTES
        .iter()
        .map(|route| {
            format!(
                "{{\"name\":{},\"program\":{},\"args\":{}}}",
                json::string(route.name),
                json::string(route.program),
                json::strings(route.args.iter().map(|a| a.to_string()))
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    println!("{{\"schema\":1,\"project\":{},\"root\":{},\"tracker\":{},\"team\":{},\"commit\":{},\"comparison_base\":{},\"dirty\":{},\"feature_map\":{},\"verification_skill\":{},\"map_count\":{},\"capabilities\":[{}],\"routes\":[{}],\"scope\":\"discovery; application readiness and proof require project adapters\"}}",
        json::string(metadata::NAME),json::string(&root.to_string_lossy()),json::string(metadata::TRACKER),json::string(metadata::TEAM),json::optional(commit),json::optional(comparison),dirty,json::optional(map),json::optional(skill),found.len(),capabilities,routes);
    Ok(0)
}

fn wstack() -> String {
    env::var("WSTACK_BIN").unwrap_or_else(|_| "wstack".into())
}

pub fn brand(root: &Path, args: &[String]) -> Result<i32, String> {
    let mut command = Command::new(wstack());
    command.arg("brand").args(args).arg("--root").arg(root);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        Err(format!(
            "wstack unavailable: {}; set WSTACK_BIN to its installed executable",
            command.exec()
        ))
    }
    #[cfg(not(unix))]
    {
        command
            .status()
            .map(|s| s.code().unwrap_or(1))
            .map_err(|e| e.to_string())
    }
}

pub fn doctor(root: &Path) -> i32 {
    let mut tools = BTreeSet::from(["cargo", "rustc", "git"]);
    tools.extend(crate::routes::ROUTES.iter().map(|route| route.program));
    let mut checks = Vec::new();
    let mut ready = true;
    for tool in tools {
        let ok = Command::new(tool)
            .arg("--version")
            .output()
            .is_ok_and(|o| o.status.success());
        ready &= ok;
        checks.push(format!(
            "{{\"id\":{},\"ready\":{ok},\"remediation\":{}}}",
            json::string(tool),
            json::string(&format!("install {tool} on PATH"))
        ));
    }
    let found = maps(root);
    ready &= found.len() <= 1;
    let map_ready = found.len() == 1
        && Command::new(wstack())
            .args(["features", "check", "--root"])
            .arg(root)
            .output()
            .is_ok_and(|o| o.status.success());
    if !found.is_empty() {
        ready &= map_ready;
    }
    println!("{{\"ready\":{ready},\"checks\":[{}],\"map_count\":{},\"feature_map_ready\":{map_ready},\"app_ready\":null,\"scope\":\"tools and map only; project-owned app readiness and live proof remain required\",\"next\":\"create one verification map; configure WSTACK_BIN if unavailable; reuse project app/harness routes\"}}",checks.join(","),found.len());
    i32::from(!ready)
}

pub fn features(root: &Path, action: &str, args: &[String]) -> Result<i32, String> {
    if maps(root).len() != 1 {
        return Err("create or reconcile one canonical verification map before discovery; see project verification skill".into());
    }
    let mut command = Command::new(wstack());
    command
        .args(["features", action, "--root"])
        .arg(root)
        .args(args);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        Err(format!(
            "wstack unavailable: {}; set WSTACK_BIN to its installed executable",
            command.exec()
        ))
    }
    #[cfg(not(unix))]
    {
        command
            .status()
            .map(|s| s.code().unwrap_or(1))
            .map_err(|e| e.to_string())
    }
}
