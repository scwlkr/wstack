mod commands;
mod json;
mod metadata;
mod operational;
mod routes;

use std::{collections::BTreeSet, env, path::PathBuf, process::Command};

pub struct Route {
    pub name: &'static str,
    pub program: &'static str,
    pub args: &'static [&'static str],
}

fn help() {
    println!("Project CLI\n\nUsage: ./project <command> [args...]\n");
    for (_, name, arguments, description) in commands::COMMANDS {
        println!(
            "  {} {arguments}    {description} [id:{name}]",
            commands::public_name(name)
        );
    }
    for route in routes::ROUTES {
        println!(
            "  {}    {} {}",
            route.name,
            route.program,
            route.args.join(" ")
        );
    }
    println!(
        "\nExamples:\n  ./project {}\n  ./project <command> --help",
        commands::public_name("doctor")
    );
    println!(
        "\nAdd automation in tools/project-cli/src/routes.rs; route through real app interfaces."
    );
}

fn doctor() -> i32 {
    let mut programs = BTreeSet::from(["cargo", "rustc"]);
    programs.extend(routes::ROUTES.iter().map(|route| route.program));
    let mut missing = false;
    for program in programs {
        match Command::new(program).arg("--version").output() {
            Ok(output) if output.status.success() => println!("{program}=ok"),
            _ => {
                eprintln!("{program}=missing-or-unavailable");
                missing = true;
            }
        }
    }
    println!("app_commands={}", routes::ROUTES.len());
    if routes::ROUTES.is_empty() {
        println!("next=connect app commands in tools/project-cli/src/routes.rs");
    }
    i32::from(missing)
}

fn run() -> i32 {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    if let Err(error) = env::set_current_dir(&root) {
        eprintln!("Cannot enter project {}: {error}", root.display());
        return 1;
    }
    let mut args = env::args().skip(1);
    let name = args.next().unwrap_or_else(|| "--help".into());
    let remaining: Vec<_> = args.collect();
    if let Some(command) = commands::find(&name) {
        if remaining.iter().any(|arg| arg == "--help" || arg == "-h") {
            for (_, id, arguments, _) in commands::COMMANDS {
                if commands::public_name(id) == name {
                    println!("Usage: ./project {name} {arguments}");
                }
            }
            return 0;
        }
        let result = match command {
            commands::Builtin::Brand => operational::brand(&root, &remaining),
            commands::Builtin::Info => operational::info(&root, &remaining),
            commands::Builtin::Doctor if remaining == ["--json"] => Ok(operational::doctor(&root)),
            commands::Builtin::Doctor if remaining.is_empty() => Ok(doctor()),
            commands::Builtin::Doctor => {
                eprintln!("Usage: {name} [--json]");
                return 2;
            }
            commands::Builtin::FeaturesList => operational::features(&root, "list", &remaining),
            commands::Builtin::FeaturesShow => operational::features(&root, "show", &remaining),
            commands::Builtin::FeaturesCheck => operational::features(&root, "check", &remaining),
            commands::Builtin::FeaturesView => operational::features(&root, "view", &remaining),
        };
        return match result {
            Ok(code) => code,
            Err(error) => {
                eprintln!("{error}");
                1
            }
        };
    }
    match name.as_str() {
        "help" | "--help" | "-h" => {
            help();
            0
        }
        "--version" => {
            println!("project-cli {}", env!("CARGO_PKG_VERSION"));
            0
        }
        _ => {
            let Some(route) = routes::ROUTES.iter().find(|route| route.name == name) else {
                eprintln!("Unknown command: {name}. Run ./project --help.");
                return 2;
            };
            if remaining
                .first()
                .is_some_and(|arg| arg == "--help" || arg == "-h")
            {
                println!("Usage: ./project {} [args...]\nRuns: {} {}\nExample: ./project {}\nPass -- before --help to request the underlying tool's help.",
                    route.name, route.program, route.args.join(" "), route.name);
                return 0;
            }
            let forwarded = if remaining.first().is_some_and(|arg| arg == "--") {
                &remaining[1..]
            } else {
                &remaining[..]
            };
            let mut command = Command::new(route.program);
            command.args(route.args).args(forwarded);
            // Replace ourselves so cancellation and exit status reach the real tool.
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                let error = command.exec();
                eprintln!(
                    "Cannot run {}: {error}. Run ./project {}.",
                    route.program,
                    commands::public_name("doctor")
                );
                127
            }
            #[cfg(not(unix))]
            {
                match command.status() {
                    Ok(status) => status.code().unwrap_or(1),
                    Err(error) => {
                        eprintln!(
                            "Cannot run {}: {error}. Run ./project {}.",
                            route.program,
                            commands::public_name("doctor")
                        );
                        127
                    }
                }
            }
        }
    }
}

fn main() {
    std::process::exit(run());
}
