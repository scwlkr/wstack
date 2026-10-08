"""Validate authored references and refresh only hash-owned derived resources."""
import base64
import hashlib
import json
import re
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit

ASSETS = Path(__file__).resolve().parent.parent / "assets"
OWNED = re.compile(r"(?:index\.html|catalog\.(?:json|js)|integration\.js|guide\.(?:css|js)|downloads\.js|downloads/[0-9a-f]{64}\.js)")


def digest(content):
    return hashlib.sha256(content).hexdigest()


def validate_guide(folder, guide, generated):
    checked = set()

    def reference(source, value, hyperlink=False):
        url = urlsplit(value)
        if not value or value.startswith("#"):
            return
        if hyperlink and url.scheme in {"https", "http", "mailto", "tel"}:
            return
        if url.scheme or url.netloc or url.path.startswith("/"):
            raise ValueError(f"{source.name}: use local relative resources: {value}")
        path = source.parent / unquote(url.path)
        if any(part.is_symlink() for part in (path, *path.parents)):
            raise ValueError(f"{source.name}: unsafe symlink reference {value}")
        path = path.resolve()
        if not path.is_relative_to(folder.resolve()):
            raise ValueError(f"{source.name}: reference leaves brand folder: {value}")
        relative = path.relative_to(folder.resolve()).as_posix()
        if relative in generated:
            return
        if relative.startswith(".wstack-brand/"):
            raise ValueError(f"{source.name}: obsolete generated reference {value}; "
                             "copy/adapt legacy styles or content outside .wstack-brand")
        if not path.is_file():
            raise ValueError(f"{source.name}: missing asset reference {value}")
        if path.suffix.lower() == ".css" and path not in checked:
            checked.add(path)
            # Treat comments as tokens so their historical references are ignored;
            # quoted URLs may contain spaces and parentheses in preserved names.
            pattern = r"/\*[\s\S]*?\*/|url\(\s*(?:\"([^\"]*)\"|'([^']*)'|([^\s)]+))\s*\)"
            for match in re.finditer(pattern, path.read_text(), re.IGNORECASE):
                resource = next((part for part in match.groups() if part is not None), None)
                if resource is not None:
                    reference(path, resource)


    class Resources(HTMLParser):
        def handle_starttag(self, tag, attributes):
            for key, value in attributes:
                if key in {"src", "href", "poster"} and value is not None:
                    reference(guide, value, tag == "a" and key == "href")

    Resources().feed(guide.read_text(encoding="utf-8"))


def resources(folder, style, rows):
    contents, catalog = {}, []
    for row in rows:
        data = (folder / row["path"]).read_bytes()
        sha = digest(data)
        name = f"downloads/{sha}.js"
        contents[name] = ("window.wstackBrandPayloads=window.wstackBrandPayloads||{};\n"
                          f"window.wstackBrandPayloads[{json.dumps(sha)}]=" +
                          json.dumps(base64.b64encode(data).decode("ascii")) + ";\n").encode()
        catalog.append({**row, "sha256": sha, "download": name})
    contents["catalog.json"] = (json.dumps({"assets": catalog}, ensure_ascii=False, indent=2, sort_keys=True) + "\n").encode()
    payload = json.dumps({"assets": catalog, "style": style}, ensure_ascii=True, separators=(",", ":"), sort_keys=True)
    contents["catalog.js"] = ("window.wstackBrandData=JSON.parse(" + json.dumps(payload) + ");\n").encode()
    contents["integration.js"] = (ASSETS / "integration.js").read_bytes()
    return contents


def write_owned(output, contents):
    marker = output / "generated.json"
    if output.is_symlink() or (output.exists() and not output.is_dir()):
        raise ValueError(f"{output}: expected an owned generated directory")
    if output.exists() and not marker.is_file():
        raise ValueError(f"{output}: unowned directory; preserve/rename it before refreshing")
    if marker.is_symlink():
        raise ValueError(f"{marker}: refusing symlink")
    previous = json.loads(marker.read_text()) if marker.exists() else {}
    if not isinstance(previous, dict) or any(
            not OWNED.fullmatch(name) or not isinstance(sha, str) or not re.fullmatch(r"[0-9a-f]{64}", sha)
            for name, sha in previous.items()):
        raise ValueError(f"{marker}: expected safe generated file hashes")
    writes, removals = {}, []
    for name in sorted(set(contents) | set(previous)):
        path = output / name
        if any(part.is_symlink() for part in (path, *path.parents)) or (
                path.exists() and not path.is_file()) or any(
                parent.exists() and not parent.is_dir() for parent in path.parents):
            raise ValueError(f"{path}: expected generated file, refusing unsafe write")
        current = path.read_bytes() if path.exists() else None
        expected = contents.get(name)
        if current is not None and digest(current) != previous.get(name):
            raise ValueError(f"{path}: customized file preserved; copy/adapt it to an authored guide "
                             "or preserve/rename .wstack-brand before refreshing")
        if expected is None and current is not None:
            removals.append(name)
        elif current != expected:
            writes[name] = expected
    state = (json.dumps({name: digest(data) for name, data in sorted(contents.items())}, indent=2) + "\n").encode()
    if not marker.exists() or marker.read_bytes() != state:
        writes[marker.name] = state
    for name in writes:
        temporary = (output / name).with_name(Path(name).name + ".tmp")
        if temporary.exists() or temporary.is_symlink():
            raise ValueError(f"{temporary}: remove the stale temporary file before refreshing")
    output.mkdir(exist_ok=True)
    for name, content in writes.items():
        path = output / name
        path.parent.mkdir(exist_ok=True)
        # Replace individual derived files atomically after the complete preflight.
        temporary = path.with_name(path.name + ".tmp")
        temporary.write_bytes(content)
        temporary.replace(path)
    for name in removals:
        (output / name).unlink()
    return list(writes) + removals
