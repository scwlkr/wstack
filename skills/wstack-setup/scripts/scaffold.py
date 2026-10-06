"""Plan every write before applying; preserve edits to previously generated files."""

import hashlib
import json
import re
from pathlib import Path

from inventory import STATE, block_parts, inspect, metadata, read
from ci import append_todos

ASSETS = Path(__file__).resolve().parent.parent / "assets"


def digest(text):
    return hashlib.sha256(text.encode()).hexdigest()


def safe(root, relative):
    path = root / relative
    for part in (path, *path.parents):
        if part == root:
            break
        if part.is_symlink():
            raise ValueError(f"Refusing to write through symlink: {relative}")
    if path.exists() and not path.is_file():
        raise ValueError(f"Expected file: {relative}")
    return path


def rust_string(text):
    return '"' + "".join(
        "\\" + char if char in '\\"' else f"\\u{{{ord(char):x}}}" if ord(char) < 32 else char
        for char in text
    ) + '"'


def routes_source(routes):
    lines = ["use crate::Route;", "", "#[rustfmt::skip]", "pub const ROUTES: &[Route] = &["]
    for route in routes:
        args = ", ".join(rust_string(arg) for arg in route["args"])
        lines.append("    Route { name: %s, program: %s, args: &[%s] }," % (
            rust_string(route["name"]), rust_string(route["program"]), args))
    return "\n".join([*lines, "];", ""])


def apply(root, args):
    info = inspect(root)
    old = metadata(root)
    config = {key: getattr(args, key, None) or info.get(key)
              for key in ("name", "team", "linear_project")}
    missing = [key for key, value in config.items() if not value]
    if missing:
        raise ValueError("Missing " + ", ".join(missing) + "; resolve from prompt/project/Linear, then pass flags")
    if any("\n" in value or "\r" in value for value in config.values()):
        raise ValueError("Setup values must be single-line strings")
    if not re.fullmatch(r"https://linear\.app/[\w-]+/project/[\w-]+(?:/overview)?", config["linear_project"]):
        raise ValueError("--linear-project must be a Linear project URL")
    writes, preserved = {}, []
    hashes = dict(old.get("hashes", {}))

    def put(relative, content):
        safe(root, relative)
        if read(root, relative) != content:
            writes[relative] = content

    def generated(relative, content):
        path = safe(root, relative)
        if path.exists():
            current = read(root, relative)
            if current != content:
                if relative not in hashes:
                    raise ValueError(f"Unowned path collision: {relative}; reconcile before applying")
                if digest(current) != hashes[relative]:
                    preserved.append(relative)
                    return
        put(relative, content)
        hashes[relative] = digest(content)

    main = "tools/project-cli/src/main.rs"
    custom_main = safe(root, main).is_file() and main in hashes and digest(read(root, main)) != hashes[main]
    for source in sorted((ASSETS / "cli").rglob("*")):
        if source.is_file():
            if custom_main:
                if main not in preserved:
                    preserved.append(main)
                continue
            content = source.read_text()
            if source.name == "metadata.rs":
                content = "".join(f"pub const {key.upper()}: &str = {rust_string(config[value])};\n"
                                  for key, value in (("name", "name"), ("team", "team"), ("tracker", "linear_project")))
            generated("tools/project-cli/" + source.relative_to(ASSETS / "cli").as_posix(), content)
    generated("project", (ASSETS / "project").read_text())
    if not custom_main:
        generated("tools/project-cli/src/routes.rs", routes_source(info["routes"]))
    agents = read(root, "AGENTS.md")
    before, block, after = block_parts(agents)
    desired = (ASSETS / "AGENTS.md").read_text().format(
        **config, technical_stack=(ASSETS / "technical-stack.md").read_text().strip()).strip()
    if block and block != desired and digest(block) != hashes.get("agent_block"):
        if any(config[key] != old.get(key) for key in config):
            raise ValueError("Custom setup block conflicts with changed metadata; reconcile AGENTS.md first")
        preserved.append("AGENTS.md setup block")
    else:
        content = before + desired + after if block else agents.rstrip() + ("\n\n" if agents else "") + desired + "\n"
        put("AGENTS.md", content)
        hashes["agent_block"] = digest(desired)
    if not (root / "README.md").exists():
        put("README.md", f"# {config['name']}\n")
    ignored = read(root, ".gitignore")
    ignore = "/tools/project-cli/target/"
    if ignore not in ignored.splitlines():
        put(".gitignore", ignored + ("\n" if ignored and not ignored.endswith("\n") else "") + ignore + "\n")
    handoff = read(root, "SETUP-TODO.md")
    if custom_main and "setup:operational-cli" not in handoff:
        handoff += "\n- [ ] [setup:operational-cli] Reconcile the preserved customized CLI with identity/capability discovery; preserve routes, flags, exit status and cancellation.\n"
    if not info["routes"] and not (root / "SETUP-TODO.md").exists():
        handoff = "# Setup handoff\n\n- [ ] Wire real app commands into `tools/project-cli/src/routes.rs`; exercise one real feature through `./project` and capture its result.\n"
    put("SETUP-TODO.md", append_todos(handoff, info["ci"]["todos"]))
    state = {"version": 1, **config, "hashes": hashes}
    put(STATE, json.dumps(state, indent=2, ensure_ascii=False) + "\n")
    # Preflight above prevents collisions or missing configuration from causing partial setup.
    for relative, content in writes.items():
        path = safe(root, relative)
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)
    launcher = safe(root, "project")
    if not launcher.stat().st_mode & 0o111:
        launcher.chmod(launcher.stat().st_mode | 0o111)
        if "project" not in writes:
            writes["project"] = ""
    return {"changed": list(writes), "preserved": preserved,
            "ci_alignment": info["ci"]["alignment"],
            "next": "Review AGENTS.md conflicts; run setup.py check", "app_routes": len(info["routes"])}
