"""Render ordinary local files; preflight writes and retain authored content."""
import hashlib
import html
import json
import re
from pathlib import Path
from urllib.parse import quote

ASSETS = Path(__file__).resolve().parent.parent / "assets"


def esc(value):
    return html.escape(str(value), quote=True)


def href(relative):
    return "../" + quote(relative, safe="/")


def contrast(color, foreground):
    def luminance(value):
        parts = [int(value[i:i + 2], 16) / 255 for i in (1, 3, 5)]
        linear = [v / 12.92 if v <= .04045 else ((v + .055) / 1.055) ** 2.4 for v in parts]
        return sum(v * weight for v, weight in zip(linear, (.2126, .7152, .0722)))
    first, second = sorted((luminance(color), luminance(foreground)))
    return (second + .05) / (first + .05)


def card(row):
    link = esc(href(row["path"]))
    preview = (f'<img src="{link}" alt="{esc(row["title"])}" loading="lazy">' if row["preview"]
               else f'<span class="file-kind">{esc(row["kind"])}</span>')
    details = "".join(f'<a href="{esc(href(row[key]))}">{label}</a>'
                      for key, label in (("license", "License"), ("source", "Original")) if key in row)
    return (f'<article class="asset" data-search="{esc(json.dumps(row, ensure_ascii=False).lower())}" '
            f'data-category="{esc(row["category"])}"><div class="preview">{preview}</div>'
            f'<div class="asset-body"><p class="eyebrow">{esc(row["category"])} · {esc(row["kind"])}</p>'
            f'<h3>{esc(row["title"])}</h3><p>{esc(row["description"])}</p>'
            f'<code>{esc(row["path"])}</code><div class="links"><a href="{link}" download>Download</a>'
            f'{details}</div></div></article>')


def render(folder, config, style, rows):
    palette = "".join(
        f'<article class="swatch"><div style="background:{c["value"]};color:{c["on"]}">Aa</div>'
        f'<h3>{esc(c["name"])}</h3><code>{c["value"]}</code><p>{esc(c["usage"])}</p>'
        f'<p>{contrast(c["value"], c["on"]):.2f}:1 with {c["on"]} · '
        f'{"normal text ≥4.5" if contrast(c["value"], c["on"]) >= 4.5 else "large text ≥3" if contrast(c["value"], c["on"]) >= 3 else "decorative use only"}</p></article>'
        for c in config["palette"])
    fonts, typography = [], []
    for i, item in enumerate(config["typography"]):
        family = f'brand-font-{i}' if "font" in item else item["family"]
        if "font" in item:
            fonts.append(f'@font-face{{font-family:brand-font-{i};src:url("{href(item["font"])}");font-display:swap}}')
        typography.append(f'<article class="type-sample"><p class="eyebrow">{esc(item["name"])} · '
                          f'{esc(item["family"])}</p><p class="specimen" style="font-family:{esc(family)}">'
                          f'{esc(item.get("sample", "A clear voice. A lasting impression."))}</p>'
                          f'<p>{esc(item["usage"])}</p></article>')
    applications = "".join(
        f'<article><h3>{esc(item["name"])}</h3><p>{esc(item["usage"])}</p>'
        + (f'<img class="application" src="{esc(href(item["asset"]))}" alt="{esc(item["name"])}">'
           if "asset" in item else "") + '</article>' for item in config["applications"])
    guidance = folder / "guidance.html"
    if guidance.exists() and (guidance.is_symlink() or not guidance.is_file()):
        raise ValueError("guidance.html must be an authored local file")
    custom = guidance.read_text() if guidance.exists() else ""
    # Authored HTML uses brand-folder paths; the generated page lives one level below it.
    for attribute, encoded in re.findall(r'\b(src|href)=[\"\']([^\"\']+)[\"\']', custom):
        value = html.unescape(encoded)
        if value.startswith("#"):
            continue
        path = folder / value
        if Path(value).is_absolute() or ".." in Path(value).parts or ":" in value:
            raise ValueError(f"guidance.html {attribute}: use brand-relative local resources")
        if not path.is_file() or any(p.is_symlink() for p in (path, *path.parents)):
            raise ValueError(f"guidance.html: missing or unsafe asset {value}")
        custom = custom.replace(f'{attribute}="{encoded}"', f'{attribute}="{esc(href(value))}"')
        custom = custom.replace(f"{attribute}='{encoded}'", f'{attribute}="{esc(href(value))}"')
    values = {"name": esc(config["name"]), "summary": esc(config["summary"]),
              "palette": palette, "typography": "".join(typography), "fonts": "".join(fonts),
              "voice": "".join(f'<li>{esc(line)}</li>' for line in config["voice"]),
              "applications": applications, "guidance": custom,
              "categories": "".join(f'<option>{esc(c)}</option>' for c in sorted({r["category"] for r in rows})),
              "cards": "".join(card(row) for row in rows), "count": str(len(rows)),
              "style": esc(style)}
    return re.sub(r"\{\{(\w+)\}\}", lambda m: values[m[1]], (ASSETS / "guide.html").read_text())


def digest(content):
    return hashlib.sha256(content).hexdigest()


def refresh(folder, config, style, rows):
    output = folder / ".wstack-brand"
    marker = output / "generated.json"
    if output.is_symlink() or (output.exists() and not output.is_dir()):
        raise ValueError(f"{output}: expected an owned generated directory")
    if output.exists() and not marker.is_file():
        raise ValueError(f"{output}: unowned directory; preserve/rename it before refreshing")
    if marker.is_symlink():
        raise ValueError(f"{marker}: refusing symlink")
    previous = json.loads(marker.read_text()) if marker.exists() else {}
    if not isinstance(previous, dict):
        raise ValueError(f"{marker}: expected generated file hashes")
    contents = {"index.html": render(folder, config, style, rows).encode(),
                "catalog.json": (json.dumps({"assets": rows}, ensure_ascii=False, indent=2) + "\n").encode(),
                "guide.css": (ASSETS / "guide.css").read_bytes(),
                "guide.js": (ASSETS / "guide.js").read_bytes()}
    writes = {}
    for name, content in contents.items():
        path = output / name
        if path.is_symlink() or (path.exists() and not path.is_file()):
            raise ValueError(f"{path}: expected generated file, refusing unsafe write")
        current = path.read_bytes() if path.exists() else None
        if current is not None and current != content and digest(current) != previous.get(name):
            raise ValueError(f"{path}: customized file preserved; move guidance to guidance.html "
                             "or preserve/rename .wstack-brand before regenerating")
        if current != content:
            writes[name] = content
    state = (json.dumps({name: digest(data) for name, data in contents.items()}, indent=2) + "\n").encode()
    if not marker.exists() or marker.read_bytes() != state:
        writes[marker.name] = state
    output.mkdir(exist_ok=True)
    for name, content in writes.items():
        (output / name).write_bytes(content)
    return {"guide": str(output / "index.html"), "assets": len(rows), "changed": list(writes)}
