#!/usr/bin/env python3
"""Minimal Wstack setup. Default: background; --apply: foreground."""
import argparse
import json
import os
import re
import signal
import subprocess
import sys
import tempfile
from pathlib import Path

VERSION = "0.2.0"
BEGIN, END = "<!-- wstack:begin setup -->", "<!-- wstack:end setup -->"
ASSETS = Path(__file__).resolve().parents[1] / "assets"


def guard(path, root):
    for part in (path, *path.parents):
        if part == root:
            break
        if part.is_symlink():
            raise ValueError(f"Preserve symlink: {part}")
        if part != path and part.exists() and not part.is_dir():
            raise ValueError(f"Expected directory: {part}")
    if path.exists() and not path.is_file():
        raise ValueError(f"Expected file: {path}")


def managed(text, body, legacy=False):
    begin, end = ("<!-- wstack-setup:start -->", "<!-- wstack-setup:end -->") if legacy else (BEGIN, END)
    lines = text.splitlines(keepends=True)
    opens = [i for i, line in enumerate(lines) if line.strip() == begin]
    closes = [i for i, line in enumerate(lines) if line.strip() == end]
    block = f"{begin}\n{body}\n{end}\n"
    if not opens and not closes and begin not in text and end not in text:
        return text + ("\n" if text and not text.endswith("\n") else "") + ("\n" if text else "") + block, ""
    if len(opens) != 1 or len(closes) != 1 or opens[0] >= closes[0] or text.count(begin) != 1 or text.count(end) != 1:
        raise ValueError("Malformed Wstack setup markers; preserve document and repair markers")
    start, end = opens[0], closes[0]
    return "".join(lines[:start]) + block + "".join(lines[end + 1:]), "".join(lines[start + 1:end])


def runner(root):
    choices = [("project", "./project --help"), ("task", "./task --help"),
               ("justfile", "just --help"), (".justfile", "just --help"),
               ("Taskfile.yml", "task --list"), ("Taskfile.yaml", "task --list"),
               ("Makefile", "make --help"), ("makefile", "make --help"), ("GNUmakefile", "make --help")]
    for name, command in choices:
        guard(root / name, root)
        if (root / name).is_file():
            return command
    package = root / "package.json"
    guard(package, root)
    if package.exists():
        data = json.loads(package.read_text())
        if not isinstance(data, dict):
            raise ValueError("package.json must contain an object")
        if data.get("scripts"):
            manager = str(data.get("packageManager", "")).split("@")[0]
            if manager not in ("pnpm", "npm", "yarn", "bun"):
                manager = next((tool for lock, tool in [("pnpm-lock.yaml", "pnpm"), ("yarn.lock", "yarn"), ("bun.lock", "bun")]
                                if (root / lock).exists()), "npm")
            return f"{manager} run"
    return None


def save(path, text, root, fresh=False, mode=0o644):
    guard(path, root)
    path.parent.mkdir(parents=True, exist_ok=True)
    if fresh:
        with path.open("x") as stream:
            stream.write(text)
        path.chmod(mode)
        return
    descriptor, name = tempfile.mkstemp(prefix=f".{path.name}.wstack-", dir=path.parent)
    try:
        with os.fdopen(descriptor, "w") as stream:
            stream.write(text)
        os.chmod(name, path.stat().st_mode & 0o777 if path.exists() else mode)
        guard(path, root)
        os.replace(name, path)
    finally:
        Path(name).unlink(missing_ok=True)


def apply(root):
    lock = root / ".wstack-setup.lock"
    if lock.is_symlink():
        raise ValueError(f"Preserve symlink: {lock}")
    try:
        lock.mkdir()
    except FileExistsError:
        return {"status": "busy"}
    try:
        paths = [root / name for name in ("w.md", "AGENTS.md", "FEATURES.md", "GLOSSARY.md")]
        for path in paths:
            guard(path, root)
        home, agents = [path.read_text() if path.exists() else "" for path in paths[:2]]
        _, old = managed(home, "")
        legacy = "wstack-setup:start" in agents or "wstack-setup:end" in agents
        if legacy and (BEGIN in agents or END in agents):
            raise ValueError("Conflicting Wstack setup blocks; choose which to keep")
        pointer, _ = managed(agents, "Wstack project notes → [w.md](w.md).", legacy=legacy)
        version = re.search(r"^Wstack: (\d+)\.(\d+)\.(\d+)\s*$", old, re.M)
        if version and tuple(map(int, version.groups())) >= tuple(map(int, VERSION.split("."))):
            return {"status": "current"}
        command = runner(root)
        writes = []
        if command is None:
            source, launcher = root / "tools/project-cli/main.rs", root / "project"
            for path in (source, launcher):
                guard(path, root)
                if path.exists():
                    raise ValueError(f"Preserve existing starter: {path}")
            writes = [(source, (ASSETS / "project.rs").read_text(), 0o644),
                      (launcher, (ASSETS / "project").read_text(), 0o755)]
            command = "./project --help"
        writes += [(path, f"# {title}\n", 0o644) for path, title in zip(paths[2:], ("Features", "Glossary")) if not path.exists()]
        created = []
        for path, text, mode in writes:
            save(path, text, root, fresh=True, mode=mode)
            created.append(str(path.relative_to(root)))
        if pointer != agents:
            if not paths[1].exists():
                created.append("AGENTS.md")
            save(paths[1], pointer, root)
        body = f"Wstack: {VERSION}\nFeatures: [FEATURES.md](FEATURES.md)\nTerms: [GLOSSARY.md](GLOSSARY.md)\nCLI: `{command}`"
        updated, _ = managed(home, body)
        if not paths[0].exists():
            created.append("w.md")
        save(paths[0], updated, root)
        return {"status": "ready", "version": VERSION, "cli": command, "created": created}
    finally:
        lock.rmdir()


def expired(_signal, _frame):
    raise TimeoutError("Setup exceeded 60 seconds; useful partial files preserved")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path.cwd())
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--report", type=Path, help=argparse.SUPPRESS)
    args = parser.parse_args()
    try:
        if args.root.is_symlink():
            raise ValueError(f"Preserve symlink: {args.root}")
        root = args.root.expanduser().resolve(strict=True)
        if not root.is_dir():
            raise ValueError(f"Expected project directory: {root}")
        if not args.apply:
            directory = Path(tempfile.mkdtemp(prefix="wstack-setup-"))
            report = directory / "status.json"
            report.write_text(json.dumps({"status": "running", "root": str(root)}))
            with (directory / "output.log").open("w") as output:
                subprocess.Popen([sys.executable, str(Path(__file__).resolve()), str(root), "--apply", "--report", str(report)],
                                 stdin=subprocess.DEVNULL, stdout=output, stderr=output, start_new_session=True)
            print(report)
            return 0
        signal.signal(signal.SIGALRM, expired)
        signal.alarm(60)
        result = apply(root)
    except (OSError, ValueError, TypeError, TimeoutError) as error:
        result = {"status": "failed", "error": str(error)}
    finally:
        if args.apply:
            signal.alarm(0)
    if args.report:
        temporary = args.report.with_suffix(".tmp")
        temporary.write_text(json.dumps(result))
        temporary.replace(args.report)
    else:
        print(json.dumps(result))
    return int(result["status"] == "failed")


if __name__ == "__main__":
    sys.exit(main())
