#!/usr/bin/env python3
"""Install latest Wstack globally and retire only known Wstack installs."""
import hashlib
import io
import json
import os
import re
import shutil
import stat
import sys
import tempfile
import urllib.request
import zipfile
from datetime import datetime, timezone
from pathlib import Path, PurePosixPath

SOURCE = "scwlkr/wstack"
ARCHIVE = f"https://codeload.github.com/{SOURCE}/zip/refs/heads/main"
RETIRED = {
    "wstack-brand", "wstack-code-review", "wstack-debug", "wstack-implement",
    "wstack-restate", "wstack-setup", "wstack-verify-create", "wstack-verify-maintain",
}
SHARED = {"plain-english", "unslop-design", "unslop-writing"}
LIMIT = 20 * 1024 * 1024


def download_skill(destination):
    request = urllib.request.Request(ARCHIVE, headers={"User-Agent": "wstack-installer"})
    with urllib.request.urlopen(request, timeout=30) as response:
        data = response.read(LIMIT + 1)
    if len(data) > LIMIT:
        raise ValueError("Wstack archive exceeds 20 MiB")
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        total = 0
        for member in archive.infolist():
            parts = PurePosixPath(member.filename).parts
            if len(parts) < 4 or parts[1:3] != ("skills", "wstack"):
                continue
            relative = PurePosixPath(*parts[3:])
            mode = member.external_attr >> 16
            if ".." in parts or "\\" in member.filename or stat.S_ISLNK(mode):
                raise ValueError(f"Unsafe archive path: {member.filename}")
            total += member.file_size
            if total > LIMIT:
                raise ValueError("Expanded Wstack skill exceeds 20 MiB")
            target = destination.joinpath(*relative.parts)
            if member.is_dir():
                target.mkdir(parents=True, exist_ok=True)
            else:
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(archive.read(member))
                target.chmod(0o755 if mode & 0o111 else 0o644)
    validate_skill(destination)


def validate_skill(skill):
    text = (skill / "SKILL.md").read_text(encoding="utf-8")
    if not re.search(r"^name: wstack\s*$", text, re.MULTILINE):
        raise ValueError("Downloaded skill is not Wstack")
    for relative in ("scripts/setup.py", "references/workflows.md"):
        if not (skill / relative).is_file():
            raise ValueError(f"Downloaded skill is missing {relative}")


def git_hash(kind, data):
    return hashlib.sha1(f"{kind} {len(data)}\0".encode() + data).digest()


def folder_hash(folder):
    """Git tree SHA used by the Skills CLI's global lock format."""
    entries = []
    for path in sorted(folder.iterdir(), key=lambda p: p.name + ("/" if p.is_dir() else "")):
        if path.is_dir():
            mode, digest = "40000", folder_hash(path)
        else:
            mode = "100755" if path.stat().st_mode & 0o111 else "100644"
            digest = git_hash("blob", path.read_bytes())
        entries.append(f"{mode} {path.name}\0".encode() + digest)
    return git_hash("tree", b"".join(entries))


def global_roots(home, environment):
    roots = [home / name / "skills" for name in (".agents", ".codex", ".cursor", ".claude")]
    for key in ("CODEX_HOME", "CLAUDE_CONFIG_DIR"):
        if environment.get(key, "").strip():
            roots.append(Path(environment[key]).expanduser() / "skills")
    # Resolve directory aliases, but never follow a skill's own symlink.
    return list(dict.fromkeys(root.resolve() for root in roots))


def lock_paths(home, environment):
    default = home / ".agents/.skill-lock.json"
    configured = environment.get("XDG_STATE_HOME", "").strip()
    active = Path(configured).expanduser() / "skills/.skill-lock.json" if configured else default
    paths = [active.resolve()]
    if default.exists() and default.resolve() not in paths:
        paths.append(default.resolve())
    return paths


def read_lock(path):
    if not path.exists():
        return {"version": 3, "skills": {}}
    data = json.loads(path.read_text(encoding="utf-8"))
    if (not isinstance(data, dict) or data.get("version") not in (1, 2, 3)
            or not isinstance(data.get("skills"), dict)
            or not all(isinstance(entry, dict) for entry in data["skills"].values())):
        raise ValueError(f"Unsupported Skills lock: {path}; existing install was preserved")
    return data


def from_wstack(entry):
    url = entry.get("sourceUrl", "")
    return (entry.get("source") == SOURCE
            or isinstance(url, str) and url.rstrip("/") in (
                f"https://github.com/{SOURCE}", f"https://github.com/{SOURCE}.git"))


def retired_paths(roots, locks):
    known_shared = {name for lock in locks for name, entry in lock["skills"].items()
                    if name in SHARED and from_wstack(entry)}
    canonical = roots[0]
    result = []
    for root in roots:
        for name in sorted(RETIRED | SHARED):
            path = root / name
            if not os.path.lexists(path):
                continue
            owned = name in RETIRED
            if name in SHARED:
                owned = name in known_shared and path.resolve() == (canonical / name).resolve()
                try:
                    owned |= f"<!-- wstack:begin shared/{name}.md -->" in (path / "SKILL.md").read_text(encoding="utf-8")
                except (OSError, UnicodeError):
                    pass
            if owned:
                result.append(path)
    return result


def validate_targets(paths):
    for path in paths:
        for other in paths:
            if path != other and (path in other.parents or path.resolve() in other.parents):
                raise ValueError(f"Overlapping install paths: {path} and {other}; existing install was preserved")


class Changes:
    """Keep same-filesystem backups until all replacements and cleanup succeed."""
    def __init__(self):
        self.stashes = []
        self.changed = []

    def prepare(self, path):
        path.parent.mkdir(parents=True, exist_ok=True)
        stash = Path(tempfile.mkdtemp(prefix=".wstack-install-", dir=path.parent))
        self.stashes.append(stash)
        return stash / "new"

    def replace(self, path, new):
        old = new.parent / "old"
        if os.path.lexists(path):
            os.replace(path, old)
        self.changed.append((path, old))
        if os.path.lexists(new):
            os.replace(new, path)

    def rollback(self):
        for path, old in reversed(self.changed):
            if path.is_symlink() or path.is_file():
                path.unlink()
            elif path.exists():
                shutil.rmtree(path)
            if os.path.lexists(old):
                os.replace(old, path)

    def discard(self):
        for stash in self.stashes:
            shutil.rmtree(stash)


def install(skill, home, environment, *, version=None):
    roots = global_roots(home, environment)
    paths = lock_paths(home, environment)
    locks = [read_lock(path) for path in paths]
    retired = retired_paths(roots, locks)
    validate_targets([root / "wstack" for root in roots] + retired + paths)
    canonical = roots[0] / "wstack"
    now = datetime.now(timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z")
    entry = {
        "source": SOURCE, "sourceType": "github", "sourceUrl": f"https://github.com/{SOURCE}.git",
        "skillPath": "skills/wstack/SKILL.md", "skillFolderHash": folder_hash(skill).hex(),
        "installedAt": locks[0]["skills"].get("wstack", {}).get("installedAt", now), "updatedAt": now,
    }
    changes = Changes()
    try:
        new = changes.prepare(canonical)
        shutil.copytree(skill, new)
        changes.replace(canonical, new)
        for root in roots[1:]:
            path = root / "wstack"
            new = changes.prepare(path)
            try:
                new.symlink_to(os.path.relpath(canonical, path.parent), target_is_directory=True)
            except (OSError, ValueError):
                shutil.copytree(skill, new)
            changes.replace(path, new)
        for path in retired:
            changes.replace(path, changes.prepare(path))
        for path, lock in zip(paths, locks):
            for name in list(lock["skills"]):
                if name in RETIRED or (name in SHARED and from_wstack(lock["skills"][name])):
                    del lock["skills"][name]
            lock["version"] = 3
            lock["skills"]["wstack"] = entry
            new = changes.prepare(path)
            new.write_text(json.dumps(lock, indent=2) + "\n", encoding="utf-8")
            changes.replace(path, new)
        for root in roots:
            validate_skill(root / "wstack")
    except BaseException:
        changes.rollback()
        changes.discard()
        raise
    changes.discard()
    release = f"Wstack {version}" if version else "latest Wstack"
    print(f"Installed {release} in {canonical}")
    print(f"Removed {len(retired)} retired Wstack installs. Ready for Codex, Cursor and Claude Code.")
    print("Start a new chat and use $wstack in Codex or /wstack in Cursor/Claude.")


def main():
    with tempfile.TemporaryDirectory(prefix="wstack-download-") as temporary:
        skill = Path(temporary) / "wstack"
        print("Downloading latest Wstack...", flush=True)
        download_skill(skill)
        install(skill, Path.home(), os.environ)


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, zipfile.BadZipFile) as error:
        print(f"Wstack install failed: {error}", file=sys.stderr)
        sys.exit(1)
