mod common;
use common::scratch;
use std::{fs, path::Path, process::Command};

fn view(root: &Path, output: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_wstack"))
        .args(["features", "view", "--no-open", "--output"])
        .arg(output)
        .arg("--root")
        .arg(root)
        .output()
        .unwrap()
}

#[test]
fn view_is_a_small_safe_snapshot_of_the_current_map() {
    let root = scratch("features-good");
    let source = root.join("add.md");
    let original = fs::read_to_string(&source).unwrap();
    let hostile = "A </script><img src=x onerror=alert(1)> & \"quoted\" feature.";
    fs::write(
        &source,
        original.replace("Add item lets a user do the thing.", hostile),
    )
    .unwrap();
    let output = root.join("view.html");
    let result = view(&root, &output);
    assert!(result.status.success(), "{result:?}");
    let html = fs::read_to_string(&output).unwrap();
    assert_eq!(html.matches("<tr data-source=").count(), 2);
    assert!(html.contains(
        "A &lt;/script&gt;&lt;img src=x onerror=alert(1)&gt; &amp; &quot;quoted&quot; feature."
    ));
    assert!(html.contains("- `add-run` runs the thing."));
    assert!(html.contains("implemented"));
    assert!(html.contains("planned"));
    assert!(!html.contains("<img"));
    assert!(!html.contains("<script src="));
    assert!(html.len() < 6000, "{} bytes", html.len());
    assert_eq!(
        fs::read_to_string(&source).unwrap(),
        original.replace("Add item lets a user do the thing.", hostile)
    );
    fs::write(
        &source,
        original.replace("Add item lets a user do the thing.", "A revised meaning."),
    )
    .unwrap();
    let fresh = root.join("fresh.html");
    assert!(view(&root, &fresh).status.success());
    assert!(fs::read_to_string(fresh)
        .unwrap()
        .contains("A revised meaning."));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn invalid_maps_and_existing_outputs_are_not_written() {
    let root = scratch("features-good");
    let output = root.join("owner.html");
    fs::write(&output, "owner content").unwrap();
    assert!(!view(&root, &output).status.success());
    assert_eq!(fs::read_to_string(&output).unwrap(), "owner content");
    fs::write(
        root.join("README.md"),
        "# Broken\n- [Missing](missing.md)\n",
    )
    .unwrap();
    let missing = root.join("absent.html");
    let result = view(&root, &missing);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("dead entry"));
    assert!(!missing.exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn default_output_is_a_new_temporary_file_outside_the_map() {
    let root = scratch("features-good");
    let mut paths = Vec::new();
    for _ in 0..2 {
        let result = Command::new(env!("CARGO_BIN_EXE_wstack"))
            .args(["features", "view", "--no-open", "--root"])
            .arg(&root)
            .output()
            .unwrap();
        assert!(result.status.success(), "{result:?}");
        let path = std::path::PathBuf::from(String::from_utf8(result.stdout).unwrap().trim());
        assert!(path.is_absolute());
        assert!(!path.starts_with(root.canonicalize().unwrap()));
        assert!(fs::read_to_string(&path)
            .unwrap()
            .contains("Add item lets a user do the thing."));
        paths.push(path);
    }
    assert_ne!(paths[0], paths[1]);
    for path in paths {
        fs::remove_file(path).unwrap();
    }
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn opening_passes_the_generated_absolute_path_and_preserves_it_on_failure() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let root = scratch("features-good");
    let bin = root.join("bin");
    fs::create_dir(&bin).unwrap();
    let opener = bin.join(if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    });
    fs::write(
        &opener,
        "#!/bin/sh\nprintf '%s' \"$1\" > \"$OPENED_PATH\"\nexit \"$OPEN_RESULT\"\n",
    )
    .unwrap();
    fs::set_permissions(&opener, fs::Permissions::from_mode(0o755)).unwrap();
    for code in ["0", "1"] {
        let output = root.join(format!("view {code}.html"));
        let captured = root.join("opened");
        let result = Command::new(env!("CARGO_BIN_EXE_wstack"))
            .args(["features", "view", "--output"])
            .arg(&output)
            .arg("--root")
            .arg(&root)
            .env("PATH", &bin)
            .env("OPENED_PATH", &captured)
            .env("OPEN_RESULT", code)
            .output()
            .unwrap();
        assert_eq!(result.status.success(), code == "0", "{result:?}");
        assert_eq!(
            fs::read_to_string(captured).unwrap(),
            output.canonicalize().unwrap().to_str().unwrap()
        );
        assert!(output.is_file());
        if code == "1" {
            assert!(String::from_utf8_lossy(&result.stderr).contains("--no-open"));
        }
    }
    let owner = root.join("owner");
    fs::write(&owner, "owner text").unwrap();
    let link = root.join("link.html");
    symlink(&owner, &link).unwrap();
    assert!(!view(&root, &link).status.success());
    assert_eq!(fs::read_to_string(owner).unwrap(), "owner text");
    fs::remove_dir_all(root).unwrap();
}
