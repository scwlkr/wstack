//! Qualified receipt shapes; these fields do not independently establish runtime truth.
use serde_json::Value;
use std::path::Path;

fn text(value: &Value) -> bool {
    value.as_str().is_some_and(|s| !s.is_empty())
}

fn hash(value: &Value) -> bool {
    value
        .as_str()
        .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
}

fn instance(report: &Value) -> bool {
    text(&report["identity"]["binary"])
        && hash(&report["identity"]["binary_sha256"])
        && report["instance"]["pid"].as_u64().is_some_and(|p| p > 0)
        && text(&report["instance"]["endpoint"])
}

pub(super) fn identity(report: &Value, surface: &str) -> Result<(), String> {
    match surface {
        "cli" if text(&report["identity"]["executable"]) => Ok(()),
        "cli" => Err("CLI receipt has no actual executable identity".into()),
        "http" if instance(report) => Ok(()),
        "http" => Err("HTTP receipt lacks binary/owned instance identity".into()),
        "browser" => {
            let browser = &report["browser"];
            let cleanup = &report["cleanup_receipt"];
            let startup = &report["instance"]["startup"];
            let endpoint = report["instance"]["endpoint"].as_str().unwrap_or("");
            let loopback = endpoint
                .strip_prefix("http://127.0.0.1:")
                .is_some_and(|s| s.parse::<u16>().is_ok_and(|port| port > 0));
            if !instance(report)
                || !hash(&report["identity"]["assets_sha256"])
                || browser["pid"].as_u64().is_none_or(|p| p == 0)
                || browser["pid"] == report["instance"]["pid"]
                || !text(&browser["executable"])
                || browser["executable"] != report["identity"]["binary"]
                || !text(&browser["version"])
                || !browser["profile"]
                    .as_str()
                    .is_some_and(|s| Path::new(s).is_absolute())
                || !loopback
                || startup["preview"] != report["instance"]["endpoint"]
                || !startup["assets"]
                    .as_str()
                    .is_some_and(|s| Path::new(s).is_absolute())
            {
                return Err("Browser receipt lacks matching executable/assets/owned preview/browser identity".into());
            }
            for key in ["preparedAssets", "preparedWorker", "publication"] {
                if startup.get(key).is_some_and(|value| value != false) {
                    return Err(format!(
                        "Browser startup contradicts local candidate proof: {key}"
                    ));
                }
            }
            for key in [
                "preview_pid_absent",
                "port_closed",
                "browser_pid_absent",
                "profile_removed",
                "state_removed",
            ] {
                if cleanup[key] != true {
                    return Err(format!("Browser cleanup incomplete: {key}"));
                }
            }
            if cleanup["errors"]
                .as_array()
                .is_none_or(|errors| !errors.is_empty())
            {
                return Err("Browser cleanup errors are missing or nonempty".into());
            }
            Ok(())
        }
        _ => Err("unsupported receipt surface; qualify a project adapter first".into()),
    }
}

fn http_observation(row: &Value) -> Result<(), String> {
    if !text(&row["action"]["method"])
        || !text(&row["action"]["path"])
        || row["http_status"].as_u64().is_none()
        || row["raw_body"].as_str().is_none()
    {
        return Err("HTTP observation lacks actual action/status/raw body".into());
    }
    Ok(())
}

pub(super) fn observation(report: &Value, row: &Value, surface: &str) -> Result<(), String> {
    match surface {
        "cli" => {
            let command = row["command"]
                .as_array()
                .ok_or("CLI observation has no action")?;
            if command.first() != Some(&report["identity"]["executable"])
                || row["exit_code"].as_i64().is_none()
            {
                return Err("CLI action/executable/exit identity mismatch".into());
            }
        }
        "http" => http_observation(row)?,
        "browser" if row["entry_point"] == "HTTP" => {
            let observations = row["http_observations"]
                .as_array()
                .filter(|items| !items.is_empty())
                .ok_or("Browser HTTP group requires nonempty http_observations")?;
            for item in observations {
                http_observation(item)?;
            }
        }
        "browser" => {
            if row["entry_point"] != "Browser"
                || !text(&row["action"]["description"])
                || !text(&row["action"]["url"])
                || row["action"]["steps"]
                    .as_array()
                    .is_none_or(|steps| steps.is_empty() || !steps.iter().all(text))
                || !text(&row["dom_artifact"])
                || !text(&row["screenshot"])
            {
                return Err("Browser observation lacks actual action/DOM/screenshot".into());
            }
        }
        _ => unreachable!("identity rejects unsupported surfaces"),
    }
    Ok(())
}
