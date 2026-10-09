"""Exercise advertised standard commands; reuse Wstack's canonical map operations."""
import html
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

REQUIRED = {"info", "doctor", "features:list", "features:show", "features:check", "features:view"}


def discovery_ready(info, root):
    if not isinstance(info, dict) or info.get("schema") != 1:
        return False
    if any(not isinstance(info.get(key), str) or not info[key]
           for key in ("project", "root", "tracker", "team")) or Path(info["root"]).resolve() != root:
        return False
    if any(key not in info for key in ("commit", "comparison_base", "dirty", "feature_map",
                                       "verification_skill", "map_count", "routes")):
        return False
    capabilities = info.get("capabilities")
    if not isinstance(capabilities, list) or not all(
            isinstance(item, dict) and all(isinstance(item.get(key), str) and item[key]
                                          for key in ("id", "name", "arguments", "description"))
            for item in capabilities):
        return False
    ids = [item["id"] for item in capabilities]
    return len(ids) == len(set(ids)) and REQUIRED.issubset(ids)


def check_standard(root):
    results = {}
    operational = {"ready": False, "next": "Reconcile preserved CLI adapters and one canonical map from source, commands, UI and docs; rerun setup check"}

    def run(label, *args):
        process = subprocess.run([str(root / "project"), *args], cwd=root,
                                 text=True, capture_output=True, timeout=60,
                                 env={**os.environ, "RUSTUP_AUTO_INSTALL": "0"})
        results[label] = {"exit": process.returncode,
                          "output": (process.stdout + process.stderr).strip()[-4000:]}
        if process.returncode:
            raise ValueError(f"{label} failed: {results[label]['output']}")
        return process.stdout

    def read_json(label, *args):
        return json.loads(run(label, *args))

    try:
        help_text = run("help", "--help")
        commands = {}
        for line in help_text.splitlines():
            tag = re.search(r"\[id:([^\]]+)\]$", line)
            if tag:
                if tag[1] in commands:
                    raise ValueError(f"Duplicate help command: {tag[1]}")
                commands[tag[1]] = line.strip().split()[0]
        if "info" not in commands:
            raise ValueError("Missing identity/capability discovery; reconcile the preserved CLI")
        info = read_json("info", commands["info"], "--json")
        operational.update(command=commands["info"], identity=info)
        if not discovery_ready(info, root):
            raise ValueError("Incomplete identity/capability contract; reconcile the preserved CLI")
        for item in info["capabilities"]:
            if item["id"] in REQUIRED and commands.get(item["id"]) != item["name"]:
                raise ValueError(f"Help/capability mismatch: {item['id']}")
        if info["map_count"] != 1 or not info["feature_map"] or not info["verification_skill"]:
            raise ValueError("Create or reconcile one canonical verification map before setup completes")
        feature_map = Path(info["feature_map"]).resolve()
        skill = Path(info["verification_skill"]).resolve()
        if not feature_map.is_relative_to(root) or not (feature_map / "README.md").is_file() or not skill.is_file() or skill.parent != feature_map.parent:
            raise ValueError("Identity must point to the project's canonical map and verification skill")
        run("doctor", commands["doctor"])
        doctor = read_json("doctor:json", commands["doctor"], "--json")
        if not isinstance(doctor, dict) or doctor.get("ready") is not True or doctor.get("feature_map_ready") is not True or doctor.get("map_count") != 1 or not doctor.get("scope"):
            raise ValueError("Scoped doctor must report tools and the canonical map ready")
        checked = read_json("features:check", commands["features:check"], "--json")
        if not isinstance(checked, dict) or checked.get("ok") is not True or checked.get("maps") != 1:
            raise ValueError("Feature checker did not confirm one valid map")
        listed = read_json("features:list", commands["features:list"], "--json")
        features = listed.get("features") if isinstance(listed, dict) else None
        if not isinstance(features, list) or not features:
            raise ValueError("Feature list must expose at least one discovered outcome")
        ids = set()
        for feature in features:
            if not isinstance(feature, dict) or not all(isinstance(feature.get(k), str) and feature[k] for k in ("id", "title", "path")):
                raise ValueError("Incomplete feature list")
            if feature["id"] in ids or not Path(feature["path"]).resolve().is_relative_to(feature_map):
                raise ValueError("Ambiguous feature or feature outside canonical map")
            ids.add(feature["id"])
            shown = read_json(f"features:show:{feature['id']}", commands["features:show"], feature["id"], "--json")
            if not isinstance(shown, dict) or shown.get("feature") != feature:
                raise ValueError(f"Feature show disagrees with list: {feature['id']}")
        if checked.get("features") != len(features):
            raise ValueError("Feature check/list counts disagree")
        with tempfile.TemporaryDirectory(prefix="wstack-setup-view-") as temporary:
            output = Path(temporary) / "features.html"
            generated = run("features:view", commands["features:view"], "--no-open", "--output", str(output))
            if generated.strip() != str(output.resolve()) or not output.is_file():
                raise ValueError("Feature viewer did not generate the requested HTML")
            contents = html.unescape(output.read_text())
            if "<table" not in contents or any(
                    feature[key] not in contents for feature in features for key in ("id", "title", "path")):
                raise ValueError("Feature viewer does not contain the canonical features")
            results["features:view"].update(features=len(features), html_bytes=output.stat().st_size)
        operational.update(ready=True, feature_count=len(features))
    except (OSError, subprocess.TimeoutExpired, ValueError, TypeError) as error:
        operational["error"] = str(error)
    return results, operational
