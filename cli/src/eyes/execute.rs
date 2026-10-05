use super::schema::{Execution, Probe, Step};
use std::fs::{self, File};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

static CANCELLED: AtomicBool = AtomicBool::new(false);

pub fn install_handler() -> Result<(), String> {
    ctrlc::set_handler(|| CANCELLED.store(true, Ordering::Relaxed)).map_err(|e| e.to_string())
}

pub fn cancelled() -> bool {
    CANCELLED.load(Ordering::Relaxed)
}

#[cfg(unix)]
fn stop_group(pid: u32) {
    let _ = Command::new("/bin/kill")
        .args(["-KILL", &format!("-{pid}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

#[cfg(not(unix))]
fn stop_group(pid: u32) {
    #[cfg(windows)]
    let _ = Command::new("taskkill")
        .args(["/F", "/T", "/PID", &pid.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    #[cfg(not(windows))]
    let _ = pid;
}

fn step(
    root: &Path,
    dir: &Path,
    stage: &'static str,
    argv: &[String],
    timeout: u64,
    groups: &mut Vec<u32>,
) -> Step {
    let mut result = Step {
        stage,
        argv: argv.to_vec(),
        exit: None,
        timed_out: false,
        error: None,
        stdout: dir.join(format!("{stage}.stdout.log")),
        stderr: dir.join(format!("{stage}.stderr.log")),
    };
    let mut run = || -> Result<(), String> {
        let stdout = File::create(&result.stdout).map_err(|e| e.to_string())?;
        let stderr = File::create(&result.stderr).map_err(|e| e.to_string())?;
        let mut command = Command::new(&argv[0]);
        command
            .args(&argv[1..])
            .current_dir(root)
            .env("WSTACK_EVIDENCE_DIR", dir)
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr);
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        let mut child = command
            .spawn()
            .map_err(|e| format!("cannot start {}: {e}", argv[0]))?;
        groups.push(child.id());
        let start = Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    result.exit = status.code();
                    return Ok(());
                }
                Ok(None) => {}
                Err(error) => {
                    stop_group(child.id());
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(error.to_string());
                }
            }
            let cancelled = stage != "cleanup" && CANCELLED.load(Ordering::Relaxed);
            if cancelled || start.elapsed() >= Duration::from_millis(timeout) {
                result.timed_out = !cancelled;
                if cancelled {
                    result.error = Some("audit interrupted".into());
                }
                stop_group(child.id());
                let _ = child.kill();
                result.exit = child.wait().ok().and_then(|s| s.code());
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    };
    if let Err(error) = run() {
        result.error = Some(error);
    }
    result
}

pub fn run(root: &Path, map: &Path, probe: &Probe, dir: &Path) -> Result<Execution, String> {
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut steps = Vec::new();
    let mut groups = Vec::new();
    for (stage, argv) in probe.commands() {
        if stage != "cleanup"
            && (CANCELLED.load(Ordering::Relaxed) || steps.iter().any(|s: &Step| !s.passed()))
        {
            continue;
        }
        steps.push(step(root, dir, stage, argv, probe.timeout_ms, &mut groups));
    }
    for pid in groups {
        stop_group(pid);
    }
    if steps
        .iter()
        .any(|s| !s.stdout.is_file() || !s.stderr.is_file())
    {
        if let Some(cleanup) = steps.last_mut() {
            cleanup.error = Some("cleanup removed command evidence".into());
        }
    }
    let passed =
        steps.len() == 5 && steps.iter().all(Step::passed) && !CANCELLED.load(Ordering::Relaxed);
    Ok(Execution {
        map: map.to_path_buf(),
        feature: probe.feature.clone(),
        probe: probe.name.clone(),
        passed,
        steps,
    })
}
