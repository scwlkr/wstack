#!/usr/bin/env python3
"""Refresh a local brand guide, discover actual assets, or print prompt style JSON."""
import argparse
import json
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

from guide import refresh
from metadata import manifest, role, validate

CONTROL = {"brand.json", "style.json", "guidance.html"}
IMAGES = {".png", ".jpg", ".jpeg", ".webp", ".gif", ".avif", ".ico", ".svg"}
SHAPES = {"path", "rect", "circle", "ellipse", "polygon", "polyline", "line"}


def load(path):
    try:
        return json.loads(path.read_text(encoding="utf-8"), parse_constant=lambda value: fail(
            f"{path}: invalid JSON constant {value}"))
    except (OSError, ValueError) as error:
        raise ValueError(f"{path}: {error}") from error


def fail(message):
    raise ValueError(message)


def text(value, label):
    if not isinstance(value, str) or not value.strip():
        fail(f"{label}: expected nonempty text")
    return value


def strings(value, label):
    if not isinstance(value, list) or not value:
        fail(f"{label}: expected a nonempty list of text")
    return [text(item, label) for item in value]


def local_file(folder, relative):
    text(relative, "resource path")
    path = folder / relative
    if Path(relative).is_absolute() or ".." in Path(relative).parts:
        fail(f"{relative}: use a relative path inside the brand folder")
    if any(part.is_symlink() for part in (path, *path.parents)):
        fail(f"{relative}: symlinks are unsupported; keep resources in the brand folder")
    if not path.is_file():
        fail(f"missing asset: {relative}; restore it or update its guidance/metadata reference")
    return path


def style_data(folder):
    data = load(local_file(folder, "style.json"))
    required = {"colors", "type", "treatment", "exclude"}
    if not isinstance(data, dict) or set(data) != required:
        fail("style.json: use exactly colors, type, treatment, exclude; keep subject/medium separate")
    for key in required:
        strings(data[key], f"style.json {key}")
    compact = json.dumps(data, ensure_ascii=False, separators=(",", ":"))
    if len(compact) > 2000:
        fail("style.json: keep the reusable prompt block under 2000 characters")
    return compact


def config_data(folder):
    data = load(local_file(folder, "brand.json"))
    if not isinstance(data, dict):
        fail("brand.json: expected an object; see the skill's format reference")
    if "guide" in data:
        guide = local_file(folder, data["guide"])
        if ".wstack-brand" in guide.relative_to(folder).parts or guide.suffix.lower() != ".html":
            fail("brand.json guide: select an authored HTML file outside .wstack-brand")
        return data
    for key in ("name", "summary"):
        text(data.get(key), f"brand.json {key}")
    for key in ("palette", "typography", "applications"):
        if not isinstance(data.get(key), list) or not data[key]:
            fail(f"brand.json {key}: expected a nonempty list")
        for item in data[key]:
            if not isinstance(item, dict):
                fail(f"brand.json {key}: expected objects")
            fields = {"palette": ("name", "value", "on", "usage"),
                      "typography": ("name", "family", "usage"),
                      "applications": ("name", "usage")}[key]
            for field in fields:
                text(item.get(field), f"brand.json {key}.{field}")
            if key == "palette" and any(not re.fullmatch(r"#[0-9a-fA-F]{6}", item[c])
                                         for c in ("value", "on")):
                fail("palette value/on: use six-digit hex colors")
            if key == "typography" and not re.fullmatch(r"[\w ,'-]+", item["family"]):
                fail("typography family: use local/system font family names")
            for field in ("asset", "font", "license"):
                if field in item:
                    local_file(folder, item[field])
    strings(data.get("voice"), "brand.json voice")
    return data


def svg_kind(path, reference):
    try:
        root = ET.fromstring(path.read_bytes())
    except ET.ParseError as error:
        fail(f"{path}: invalid SVG: {error}")
    tags = [node.tag.rsplit("}", 1)[-1] for node in root.iter()]
    if tags[0] != "svg":
        fail(f"{path}: expected SVG root")
    if reference:
        return "reference SVG"
    try:
        box = [float(v) for v in root.get("viewBox", "").replace(",", " ").split()]
        if len(box) != 4 or box[2] <= 0 or box[3] <= 0 or any(not (-1e9 < v < 1e9) for v in box):
            raise ValueError()
    except ValueError:
        fail(f"{path}: reusable SVG needs a finite viewBox with positive width/height")
    forbidden = {"image", "text", "foreignObject", "script", "style", "animate", "set"}
    if forbidden.intersection(tags) or not SHAPES.intersection(tags):
        fail(f"{path}: reusable SVG needs genuine shapes/paths, no bitmap/text/active content; "
             "outline lettering or classify a supplied original as category References")
    for node in root.iter():
        for key, value in node.attrib.items():
            if key.rsplit("}", 1)[-1] in {"href", "src"} and not value.startswith("#"):
                fail(f"{path}: SVG depends on external content; inline editable paths")
            if key.lower().startswith("on") or ("url(" in value and not re.fullmatch(
                    r"url\(#[\w-]+\)", value)):
                fail(f"{path}: SVG contains active or external content")
    return "editable SVG"


def assets(folder):
    config = load(local_file(folder, "brand.json")) if (folder / "brand.json").exists() else {}
    if not isinstance(config, dict):
        fail("brand.json: expected an object")
    annotations, metadata_name = manifest(folder, config, load, local_file, text, strings)
    rows = []
    for path in sorted(folder.rglob("*")):
        relative = path.relative_to(folder)
        if any(part.startswith(".") for part in relative.parts):
            continue
        if path.is_symlink():
            fail(f"{relative}: symlinks are unsupported")
        if not path.is_file() or str(relative) in CONTROL or str(relative) == metadata_name:
            continue
        if path.name.endswith(".meta.json"):
            local_file(folder, str(relative)[:-10])
            continue
        suffix = path.suffix.lower()
        category = ("Licenses" if "license" in path.name.lower() or "ofl" in path.name.lower()
                    else "Fonts" if suffix in {".ttf", ".otf", ".woff", ".woff2"}
                    else relative.parts[0].replace("-", " ").title() if len(relative.parts) > 1
                    else "Resources")
        row = {"path": relative.as_posix(), "title": path.stem.replace("-", " ").replace("_", " "),
               "category": category, "description": "", "tags": []}
        sidecar = path.with_name(path.name + ".meta.json")
        if sidecar.exists():
            meta = load(local_file(folder, sidecar.relative_to(folder).as_posix()))
            validate(meta, str(sidecar), folder, local_file, text, strings)
            owned = annotations.get(relative.as_posix(), {})
            conflicts = [key for key in meta if key in owned and owned[key] != meta[key]]
            if conflicts:
                fail(f"{relative}: conflicting metadata ownership ({', '.join(conflicts)}); "
                     f"edit {metadata_name} and remove the duplicate sidecar fields")
            row.update(meta)
        row.update(annotations.get(relative.as_posix(), {}))
        row["role"] = role(row)
        reference = row["role"] == "reference" or row["category"].lower() in {"references", "originals"}
        row["kind"] = svg_kind(path, reference) if suffix == ".svg" else (
            "raster image" if suffix in IMAGES else "file")
        row["preview"] = suffix in IMAGES and not (suffix == ".svg" and reference)
        rows.append(row)
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("refresh", "list", "style"))
    parser.add_argument("--root", type=Path, default=Path.cwd(), help="project root (suite setup not required)")
    parser.add_argument("--brand-dir", default="brand", help="existing brand folder relative to root")
    parser.add_argument("--query", default="", help="case-insensitive asset search")
    parser.add_argument("--json", action="store_true", help="machine-readable catalog/refresh output")
    args = parser.parse_args()
    try:
        root = args.root.expanduser().resolve(strict=True)
        folder = root / args.brand_dir
        if Path(args.brand_dir).is_absolute() or ".." in Path(args.brand_dir).parts:
            fail("--brand-dir must be relative to the project root")
        if not folder.is_dir() or any(part.is_symlink() for part in (folder, *folder.parents)):
            fail(f"{folder}: create/select a local brand folder; see the skill's format reference")
        if args.action == "style":
            print(style_data(folder))
            return 0
        rows = assets(folder)
        if args.action == "list":
            query = args.query.lower()
            rows = [row for row in rows if query in json.dumps(row, ensure_ascii=False).lower()]
            if args.json:
                print(json.dumps({"assets": rows}, ensure_ascii=False, separators=(",", ":")))
            else:
                for row in rows:
                    print(f'{row["path"]}\t{row["category"]}\t{row["title"]}\t{row["kind"]}')
        else:
            result = refresh(folder, config_data(folder), style_data(folder), rows)
            print(json.dumps(result) if args.json else f'Guide: {result["guide"]} ({len(rows)} assets)')
        return 0
    except (OSError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
