use crate::common::scratch;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    process::{Command, Output},
    time::SystemTime,
};
pub fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wstack"))
        .current_dir(root.parent().unwrap())
        .arg("brand")
        .args(args)
        .arg("--root")
        .arg(root)
        .output()
        .unwrap()
}
pub fn pass(output: Output) -> Value {
    assert!(output.status.success(), "{output:?}");
    serde_json::from_slice(&output.stdout).unwrap()
}
pub fn digest(path: &Path) -> String {
    let output = Command::new("python3")
        .args([
            "-c",
            "import hashlib,sys; print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",
        ])
        .arg(path)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().into()
}
pub fn snapshot(folder: &Path) -> BTreeMap<String, (Vec<u8>, SystemTime)> {
    fn visit(base: &Path, folder: &Path, result: &mut BTreeMap<String, (Vec<u8>, SystemTime)>) {
        for entry in fs::read_dir(folder).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(base, &path, result);
            } else {
                let name = path.strip_prefix(base).unwrap().to_string_lossy().into();
                let content = (
                    fs::read(&path).unwrap(),
                    path.metadata().unwrap().modified().unwrap(),
                );
                result.insert(name, content);
            }
        }
    }
    let mut result = BTreeMap::new();
    visit(folder, folder, &mut result);
    result
}
pub fn write_json(path: &Path, value: &Value) {
    fs::write(path, value.to_string()).unwrap();
}
pub fn authored() -> std::path::PathBuf {
    let root = scratch("brand").canonicalize().unwrap();
    let folder = root.join("brand");
    fs::remove_file(folder.join("symbols/cairn.svg.meta.json")).unwrap();
    write_json(
        &folder.join("brand.json"),
        &json!({"guide":"index.html","metadata":"asset-manifest.json"}),
    );
    fs::write(folder.join("index.html"), "<!doctype html><title>Owner art direction</title><link rel='stylesheet' href='owner.css'><img src='symbols/cairn.svg'><script src='owner.js'></script>").unwrap();
    fs::write(
        folder.join("owner.css"),
        "body{background:#244938} /* Owner layout */",
    )
    .unwrap();
    fs::write(
        folder.join("owner.js"),
        "document.documentElement.dataset.owner='editable';",
    )
    .unwrap();
    write_json(
        &folder.join("asset-manifest.json"),
        &json!({"assets":[{
            "file":"symbols/cairn.svg", "title":"Owner cairn", "category":"Marks",
            "notes":"Hand-built paths for the primary mark.", "status":"approved", "source_id":"owner-7",
            "sha256":"0".repeat(64), "bytes":0, "vector":false,
            "tags":["primary","field"], "role":"current", "source":"references/original-icon-sheet.png",
            "license":"licenses/artwork.txt"
        }]}),
    );
    root
}
pub fn rejected_without_writes(root: &Path, generated: &BTreeMap<String, (Vec<u8>, SystemTime)>) {
    let bad = run(root, &["refresh", "--json"]);
    assert_eq!(bad.status.code(), Some(1), "{bad:?}");
    assert!(bad.stdout.is_empty());
    assert!(!bad.stderr.is_empty());
    let current = snapshot(&root.join("brand/.wstack-brand"));
    let changed: Vec<_> = current
        .keys()
        .chain(generated.keys())
        .filter(|name| current.get(*name) != generated.get(*name))
        .collect();
    assert!(changed.is_empty(), "generated files changed: {changed:?}");
}
