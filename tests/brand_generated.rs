mod common;
#[path = "common/brand.rs"]
mod support;
use common::scratch;
use serde_json::json;
use std::fs;
use support::{authored, digest, pass, rejected_without_writes, run, snapshot, write_json};

#[test]
fn changed_and_removed_assets_retire_chunks_only_when_owned() {
    let root = authored();
    let folder = root.join("brand");
    pass(run(&root, &["refresh", "--json"]));
    let asset = folder.join("symbols/cairn.svg");
    let old = folder.join(format!(".wstack-brand/downloads/{}.js", digest(&asset)));
    let changed = fs::read_to_string(&asset)
        .unwrap()
        .replace("#244938", "#AD5837");
    fs::write(&asset, changed).unwrap();
    pass(run(&root, &["refresh", "--json"]));
    assert!(!old.exists());
    let current = folder.join(format!(".wstack-brand/downloads/{}.js", digest(&asset)));
    assert!(current.exists());
    fs::write(&current, "Owner customized payload").unwrap();
    fs::remove_file(asset).unwrap();
    write_json(&folder.join("asset-manifest.json"), &json!({"assets":[]}));
    fs::write(
        folder.join("index.html"),
        "<!doctype html><title>Owner revised guide</title>",
    )
    .unwrap();
    let customized = snapshot(&folder.join(".wstack-brand"));
    rejected_without_writes(&root, &customized);
    fs::remove_file(current).unwrap();
    pass(run(&root, &["refresh", "--json"]));
    assert_eq!(
        pass(run(&root, &["list", "--query", "cairn", "--json"]))["assets"],
        json!([])
    );
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn legacy_migration_requires_explicit_ownership_and_preserves_customized_generation() {
    let root = scratch("brand");
    let folder = root.join("brand");
    let legacy = pass(run(&root, &["refresh", "--json"]));
    assert!(legacy["guide"]
        .as_str()
        .unwrap()
        .ends_with(".wstack-brand/index.html"));
    let generated_page = folder.join(".wstack-brand/index.html");
    fs::copy(&generated_page, folder.join("index.html")).unwrap();
    // A copied page still needs explicit authored ownership; refresh keeps legacy routing.
    assert!(pass(run(&root, &["refresh", "--json"]))["guide"]
        .as_str()
        .unwrap()
        .ends_with(".wstack-brand/index.html"));
    fs::write(
        folder.join("index.html"),
        "<!doctype html><title>Owner migration</title>",
    )
    .unwrap();
    fs::write(&generated_page, "Owner customized legacy page").unwrap();
    write_json(&folder.join("brand.json"), &json!({"guide":"index.html"}));
    let prior = snapshot(&folder.join(".wstack-brand"));
    rejected_without_writes(&root, &prior);
    assert_eq!(
        fs::read_to_string(&generated_page).unwrap(),
        "Owner customized legacy page"
    );
    let migrated = folder.join("custom-guide.html");
    fs::rename(&generated_page, &migrated).unwrap();
    let content = fs::read(&migrated).unwrap();
    let mtime = migrated.metadata().unwrap().modified().unwrap();
    write_json(
        &folder.join("brand.json"),
        &json!({"guide":"custom-guide.html"}),
    );
    let result = pass(run(&root, &["refresh", "--json"]));
    assert_eq!(
        result["guide"],
        migrated.canonicalize().unwrap().to_string_lossy().as_ref()
    );
    assert_eq!(fs::read(&migrated).unwrap(), content);
    assert_eq!(migrated.metadata().unwrap().modified().unwrap(), mtime);
    assert!(!generated_page.exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn matching_future_bytes_still_require_recorded_generated_ownership() {
    let root = authored();
    let future = authored();
    for project in [&root, &future] {
        pass(run(project, &["refresh", "--json"]));
    }
    let stylesheet = "body{color:#AD5837} /* New owner layout */";
    fs::write(future.join("brand/owner.css"), stylesheet).unwrap();
    pass(run(&future, &["refresh", "--json"]));
    fs::copy(
        future.join("brand/.wstack-brand/catalog.js"),
        root.join("brand/.wstack-brand/catalog.js"),
    )
    .unwrap();
    fs::write(root.join("brand/owner.css"), stylesheet).unwrap();
    let before = snapshot(&root.join("brand/.wstack-brand"));
    rejected_without_writes(&root, &before);
    fs::remove_dir_all(root).unwrap();
    fs::remove_dir_all(future).unwrap();
}

#[test]
fn unsafe_generated_download_parent_is_rejected_before_catalog_writes() {
    let root = authored();
    pass(run(&root, &["refresh", "--json"]));
    let downloads = root.join("brand/.wstack-brand/downloads");
    fs::rename(&downloads, root.join("kept-downloads")).unwrap();
    fs::write(
        downloads,
        "Owner file occupies the generated download directory",
    )
    .unwrap();
    let before = snapshot(&root.join("brand/.wstack-brand"));
    rejected_without_writes(&root, &before);
    fs::remove_dir_all(root).unwrap();
}
