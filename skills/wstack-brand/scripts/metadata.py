"""Bounded adapter for existing asset-manifest.json; files still own existence."""
FIELDS = {"title", "category", "description", "tags", "license", "source", "role"}
ROLES = {"current", "legacy", "reference", "support"}


def validate(meta, label, folder, local_file, text, strings):
    if not isinstance(meta, dict) or set(meta) - FIELDS:
        raise ValueError(f"{label}: expected title/category/description/tags/license/source/role metadata")
    for key, value in meta.items():
        if key == "tags":
            if value != []:
                strings(value, label)
        else:
            text(value, f"{label} {key}")
        if key in {"license", "source"}:
            local_file(folder, value)
        if key == "role" and value not in ROLES:
            raise ValueError(f"{label}: role must be current, legacy, reference or support")
    return meta


def manifest(folder, config, load, local_file, text, strings):
    name = config.get("metadata")
    if name is None:
        return {}, None
    path = local_file(folder, name)
    name = path.relative_to(folder).as_posix()
    data = load(path)
    if not isinstance(data, dict) or not isinstance(data.get("assets"), list):
        raise ValueError(f"{name}: expected an object with an assets array; see format reference")
    found = {}
    for item in data["assets"]:
        if not isinstance(item, dict):
            raise ValueError(f"{name}: expected asset objects")
        file = text(item.get("file"), f"{name} asset.file")
        file = local_file(folder, file).relative_to(folder).as_posix()
        if file in found:
            raise ValueError(f"{name}: duplicate metadata owner for {file}")
        meta = {key: item[key] for key in sorted(FIELDS) if key in item}
        if "notes" in item:
            if "description" in meta and meta["description"] != item["notes"]:
                raise ValueError(f"{name}: conflicting notes/description for {file}")
            meta["description"] = item["notes"]
        validate(meta, name, folder, local_file, text, strings)
        for key in ("status", "source_id"):
            if key in item:
                meta[key] = text(item[key], f"{name} {key}")
        found[file] = meta
    return found, name


def role(row):
    if "role" in row:
        return row["role"]
    words = (row["path"] + " " + row["category"]).lower()
    if "legacy" in words:
        return "legacy"
    if any(word in words for word in ("reference", "original", "research")):
        return "reference"
    if row["category"].lower() in {"fonts", "licenses"} or row["path"].lower().endswith(
            (".html", ".css", ".js", ".json", ".md", ".txt")):
        return "support"
    if row["category"].lower() == "resources" and not row["path"].lower().endswith(
            (".svg", ".png", ".jpg", ".jpeg", ".webp", ".gif", ".avif", ".ico", ".ai", ".eps", ".pdf")):
        return "support"
    return "current"
