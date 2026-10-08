mod common;
#[path = "common/brand.rs"]
mod support;
use serde_json::json;
use std::{fs, path::Path, process::Command};
use support::{authored, digest, pass, rejected_without_writes, run, snapshot, write_json};

#[test]
fn authored_guide_metadata_and_lazy_downloads_work_away_from_the_suite() {
    let root = authored();
    let folder = root.join("brand");
    write_json(
        &folder.join("brand.json"),
        &json!({"guide":"index.html","metadata":"./asset-manifest.json"}),
    );
    fs::copy(folder.join("symbols/cairn.svg"), folder.join("logo.svg")).unwrap();
    fs::copy(
        folder.join("symbols/cairn.svg"),
        folder.join("logo (1).svg"),
    )
    .unwrap();
    fs::write(
        folder.join("owner.css"),
        "/* previous url(\"deleted-logo.svg\") */ body{background-image:url(\"logo (1).svg\")}",
    )
    .unwrap();
    let before = snapshot(&folder);
    let result = pass(run(&root, &["refresh", "--json"]));
    assert_eq!(
        result["guide"],
        folder.join("index.html").to_string_lossy().as_ref()
    );
    for (name, original) in before {
        let path = folder.join(name);
        assert_eq!(fs::read(&path).unwrap(), original.0);
        assert_eq!(path.metadata().unwrap().modified().unwrap(), original.1);
    }
    let generated = snapshot(&folder.join(".wstack-brand"));
    for name in generated.keys() {
        assert!(
            matches!(
                name.as_str(),
                "catalog.json" | "catalog.js" | "integration.js" | "generated.json"
            ) || name.starts_with("downloads/") && name.ends_with(".js"),
            "unexpected generated file: {name}"
        );
    }
    for name in "catalog.json catalog.js integration.js generated.json".split(' ') {
        assert!(generated.contains_key(name));
    }
    let listed = pass(run(&root, &["list", "--query", "owner-7", "--json"]));
    let row = &listed["assets"][0];
    assert_eq!(listed["assets"].as_array().unwrap().len(), 1);
    for (key, expected) in [
        ("path", "symbols/cairn.svg"),
        ("title", "Owner cairn"),
        ("description", "Hand-built paths for the primary mark."),
        ("status", "approved"),
        ("source_id", "owner-7"),
        ("role", "current"),
    ] {
        assert_eq!(row[key], expected);
    }
    let generated_catalog: serde_json::Value =
        serde_json::from_slice(&fs::read(folder.join(".wstack-brand/catalog.json")).unwrap())
            .unwrap();
    let row = generated_catalog["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["path"] == "symbols/cairn.svg")
        .unwrap();
    let sha = digest(&folder.join("symbols/cairn.svg"));
    assert_eq!(row["sha256"], sha);
    let chunk = format!("downloads/{sha}.js");
    assert!(row["download"].as_str().unwrap().ends_with(&chunk));
    let payload = fs::read_to_string(folder.join(".wstack-brand").join(&chunk)).unwrap();
    assert!(payload.contains(&format!("window.wstackBrandPayloads[\"{sha}\"]")));
    let catalog = pass(run(&root, &["list", "--json"]));
    for row in catalog["assets"].as_array().unwrap() {
        assert!(!"brand.json style.json asset-manifest.json"
            .split(' ')
            .any(|name| name == row["path"]));
    }
    for (path, role) in [
        ("logo.svg", "current"),
        ("legacy-guide.html", "legacy"),
        ("owner.css", "support"),
        ("references/original-icon-sheet.png", "reference"),
    ] {
        let row = catalog["assets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["path"] == path)
            .unwrap();
        assert_eq!(row["role"], role);
    }
    for _ in 0..2 {
        assert_eq!(
            pass(run(&root, &["refresh", "--json"]))["changed"],
            json!([])
        );
        assert_eq!(snapshot(&folder.join(".wstack-brand")), generated);
    }
    let helper =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../skills/wstack-brand/scripts/brand.py");
    let direct = Command::new("python3")
        .arg(helper)
        .args(["list", "--json", "--root"])
        .arg(&root)
        .output()
        .unwrap();
    assert_eq!(pass(direct), catalog);
    let installed = root.join("installed-wstack");
    fs::copy(env!("CARGO_BIN_EXE_wstack"), &installed).unwrap();
    let copied = Command::new(&installed)
        .current_dir(root.parent().unwrap())
        .args(["brand", "list", "--json", "--root"])
        .arg(&root)
        .output()
        .unwrap();
    assert_eq!(pass(copied), catalog);
    let copied = Command::new(installed)
        .current_dir(root.parent().unwrap())
        .args(["brand", "refresh", "--json", "--root"])
        .arg(&root)
        .output()
        .unwrap();
    assert_eq!(pass(copied)["changed"], json!([]));
    fs::rename(&folder, root.join("identity")).unwrap();
    let explicit = pass(run(
        &root,
        &["refresh", "--brand-dir", "identity", "--json"],
    ));
    assert_eq!(
        explicit["guide"],
        root.join("identity/index.html").to_string_lossy().as_ref()
    );
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn malformed_authored_inputs_and_conflicting_metadata_preserve_previous_generation() {
    let root = authored();
    let folder = root.join("brand");
    pass(run(&root, &["refresh", "--json"]));
    let generated = snapshot(&folder.join(".wstack-brand"));
    for (name, broken) in [
        ("brand.json", "{}"),
        ("style.json", "{}"),
        (
            "asset-manifest.json",
            "{\"assets\":[{\"file\":\"missing.svg\"}]}",
        ),
        ("index.html", "<img src='missing.png'>"),
        ("owner.css", "body{background:url('missing.png')}"),
        ("index.html", "<script src='missing.js'></script>"),
    ] {
        let path = folder.join(name);
        let original = fs::read(&path).unwrap();
        fs::write(&path, broken).unwrap();
        rejected_without_writes(&root, &generated);
        fs::write(path, original).unwrap();
    }
    let sidecar = folder.join("symbols/cairn.svg.meta.json");
    write_json(&sidecar, &json!({"title":"Conflicting owner"}));
    rejected_without_writes(&root, &generated);
    write_json(
        &sidecar,
        &json!({"title":"Owner cairn", "description":"Hand-built paths for the primary mark."}),
    );
    pass(run(&root, &["refresh", "--json"]));
    fs::remove_file(sidecar).unwrap();
    write_json(
        &folder.join("brand.json"),
        &json!({"guide":".wstack-brand/index.html","metadata":"asset-manifest.json"}),
    );
    rejected_without_writes(&root, &generated);
    #[cfg(unix)]
    {
        write_json(
            &folder.join("brand.json"),
            &json!({"guide":"index.html","metadata":"asset-manifest.json"}),
        );
        std::os::unix::fs::symlink(root.join("owner.txt"), folder.join("outside.txt")).unwrap();
        rejected_without_writes(&root, &generated);
    }
    fs::remove_dir_all(root).unwrap();
}
