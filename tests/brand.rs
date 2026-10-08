mod common;
use common::scratch;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    process::{Command, Output},
    time::SystemTime,
};

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wstack"))
        .arg("brand")
        .args(args)
        .arg("--root")
        .arg(root)
        .output()
        .unwrap()
}

fn pass(output: Output) -> Value {
    assert!(output.status.success(), "{output:?}");
    serde_json::from_slice(&output.stdout).unwrap()
}

fn snapshot(root: &Path) -> BTreeMap<String, (Vec<u8>, SystemTime)> {
    fn read(root: &Path, dir: &Path, found: &mut BTreeMap<String, (Vec<u8>, SystemTime)>) {
        for entry in fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                read(root, &path, found);
            } else {
                found.insert(
                    path.strip_prefix(root).unwrap().to_string_lossy().into(),
                    (
                        fs::read(&path).unwrap(),
                        path.metadata().unwrap().modified().unwrap(),
                    ),
                );
            }
        }
    }
    let mut found = BTreeMap::new();
    read(root, root, &mut found);
    found
}

#[test]
fn portable_brand_lifecycle_preserves_authored_work_and_fails_before_writes() {
    let root = scratch("brand");
    let folder = root.join("brand");
    let authored = snapshot(&root);
    let result = pass(run(&root, &["refresh", "--json"]));
    let guide = fs::read_to_string(result["guide"].as_str().unwrap()).unwrap();
    assert!(guide.contains("Alder Fieldworks"));
    assert!(guide.contains("Owner field notes"));
    assert!(guide.contains("../legacy-guide.html"));
    assert!(guide.contains("Favicon master"));
    assert!(guide.contains("App-icon master"));
    for (path, before) in authored {
        let current = root.join(path);
        assert_eq!(fs::read(&current).unwrap(), before.0);
        assert_eq!(current.metadata().unwrap().modified().unwrap(), before.1);
    }
    let after = snapshot(&root);
    for _ in 0..2 {
        assert_eq!(
            pass(run(&root, &["refresh", "--json"]))["changed"],
            json!([])
        );
        assert_eq!(snapshot(&root), after);
    }
    let listed = pass(run(&root, &["list", "--query", "SUNRISE", "--json"]));
    assert_eq!(listed["assets"].as_array().unwrap().len(), 1);
    assert_eq!(listed["assets"][0]["kind"], "editable SVG");
    assert!(
        String::from_utf8(run(&root, &["list", "--query", "sunrise"]).stdout)
            .unwrap()
            .contains("symbols/sunrise.svg\tSymbols")
    );
    let cli_style = pass(run(&root, &["style"]));
    let source: Value =
        serde_json::from_slice(&fs::read(folder.join("style.json")).unwrap()).unwrap();
    assert_eq!(cli_style, source);
    let direct = Command::new("python3")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../skills/wstack-brand/scripts/brand.py"))
        .args(["list", "--query", "sunrise", "--json", "--root"])
        .arg(&root)
        .output()
        .unwrap();
    assert_eq!(pass(direct), listed);

    let added = folder.join("symbols/new mark Straße & #1.svg");
    fs::copy(folder.join("symbols/cairn.svg"), &added).unwrap();
    pass(run(&root, &["refresh", "--json"]));
    assert!(fs::read_to_string(folder.join(".wstack-brand/index.html"))
        .unwrap()
        .contains("new%20mark%20Stra%C3%9Fe%20%26%20%231.svg"));
    assert_eq!(
        pass(run(&root, &["list", "--query", "STRAẞE", "--json"]))["assets"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    fs::write(
        folder.join("guidance.html"),
        "<img src='symbols/new mark Straße &amp; #1.svg'>",
    )
    .unwrap();
    pass(run(&root, &["refresh", "--json"]));
    let revised = fs::read_to_string(&added)
        .unwrap()
        .replace("#244938", "#AD5837");
    fs::write(&added, &revised).unwrap();
    pass(run(&root, &["refresh", "--json"]));
    assert_eq!(fs::read_to_string(&added).unwrap(), revised);
    fs::remove_file(added).unwrap();
    fs::write(folder.join("guidance.html"), "<h3>Owner field notes</h3>").unwrap();
    pass(run(&root, &["refresh", "--json"]));
    assert_eq!(
        pass(run(&root, &["list", "--query", "new mark", "--json"]))["assets"],
        json!([])
    );
    assert_eq!(
        pass(run(&root, &["refresh", "--json"]))["changed"],
        json!([])
    );

    let stable_guide = fs::read(folder.join(".wstack-brand/index.html")).unwrap();
    fs::write(folder.join("style.json"), "{}").unwrap();
    for action in ["refresh", "style"] {
        let bad = run(&root, &[action]);
        assert_eq!(bad.status.code(), Some(1));
        assert!(bad.stdout.is_empty());
        assert!(String::from_utf8_lossy(&bad.stderr).contains("style.json"));
    }
    fs::write(
        folder.join("style.json"),
        serde_json::to_string(&source).unwrap(),
    )
    .unwrap();
    fs::remove_file(folder.join("symbols/sunrise.svg")).unwrap();
    let bad = run(&root, &["refresh"]);
    assert_eq!(bad.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&bad.stderr).contains("missing asset: symbols/sunrise.svg"));
    assert_eq!(
        fs::read(folder.join(".wstack-brand/index.html")).unwrap(),
        stable_guide
    );
    fs::remove_file(folder.join("symbols/sunrise.svg.meta.json")).unwrap();
    pass(run(&root, &["refresh", "--json"]));
    fs::write(folder.join(".wstack-brand/index.html"), "Owner custom page").unwrap();
    let bad = run(&root, &["refresh"]);
    assert_eq!(bad.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&bad.stderr).contains("customized file preserved"));
    assert_eq!(
        fs::read_to_string(folder.join(".wstack-brand/index.html")).unwrap(),
        "Owner custom page"
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn starter_without_suite_folders_and_invalid_assets_have_actionable_boundaries() {
    let root = scratch("brand");
    fs::rename(root.join("brand"), root.join("identity")).unwrap();
    let folder = root.join("identity");
    fs::remove_file(folder.join("legacy-guide.html")).unwrap();
    fs::remove_file(folder.join("guidance.html")).unwrap();
    let mut config: Value =
        serde_json::from_slice(&fs::read(folder.join("brand.json")).unwrap()).unwrap();
    config["name"] = json!("Harbor Tools");
    fs::write(folder.join("brand.json"), config.to_string()).unwrap();
    let result = pass(run(
        &root,
        &["refresh", "--brand-dir", "identity", "--json"],
    ));
    assert!(fs::read_to_string(result["guide"].as_str().unwrap())
        .unwrap()
        .contains("Harbor Tools"));
    assert_eq!(
        pass(run(
            &root,
            &["refresh", "--brand-dir", "identity", "--json"]
        ))["changed"],
        json!([])
    );
    let invalid = folder.join("symbols/bitmap.svg");
    for content in [
        "<svg><image href='data:image/png;base64,abc'/></svg>",
        "<svg><text>Needs font</text></svg>",
        "<svg><path d='M0 0h1'/><use href='https://example.test/a.svg'/></svg>",
        "not XML",
    ] {
        fs::write(&invalid, content).unwrap();
        assert_eq!(
            run(&root, &["list", "--brand-dir", "identity"])
                .status
                .code(),
            Some(1)
        );
    }
    fs::remove_file(invalid).unwrap();
    fs::write(folder.join("guidance.html"), "<img src='missing.png'>").unwrap();
    assert_eq!(
        run(&root, &["refresh", "--brand-dir", "identity"])
            .status
            .code(),
        Some(1)
    );
    assert_eq!(
        run(&root, &["style", "--brand-dir", "../identity"])
            .status
            .code(),
        Some(1)
    );
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root.join("owner.txt"), folder.join("outside.txt")).unwrap();
        assert_eq!(
            run(&root, &["list", "--brand-dir", "identity"])
                .status
                .code(),
            Some(1)
        );
    }
    fs::remove_dir_all(root).unwrap();
}
