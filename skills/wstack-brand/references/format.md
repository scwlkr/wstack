# Local brand format

Select the existing logical brand folder with `--brand-dir` (default `brand`),
relative to `--root`. Target projects need neither suite folders nor setup.
Python 3.9+ and ordinary local files suffice. No network or model calls occur in
the helper. `refresh` requires these authored sources; it does not invent a brand.

## Authored guidance

`brand.json` (example values illustrate structure, not a prescribed identity):

```json
{
  "name": "Example Studio",
  "summary": "Useful objects, carefully made.",
  "palette": [
    {"name":"Ink","value":"#17221F","on":"#FFFFFF","usage":"Primary text and marks."},
    {"name":"Paper","value":"#F5F5EE","on":"#17221F","usage":"Quiet backgrounds."}
  ],
  "typography": [
    {"name":"Body","family":"system-ui, sans-serif","usage":"Clear everyday text."}
  ],
  "voice": ["Use concrete words.", "Be warm and brief."],
  "applications": [
    {"name":"Primary symbol","usage":"Keep one stroke width of clear space.","asset":"symbols/primary.svg"},
    {"name":"Monochrome/reverse","usage":"Use the appropriate master on light/dark backgrounds."},
    {"name":"Favicon and app icon","usage":"Review the SVG masters at small sizes before native integration."}
  ]
}
```

Palette pairs use six-digit hex colors; the guide calculates their text contrast
ratio and distinguishes normal text (4.5:1), large text (3:1) and decorative use.
Typography optionally includes `sample`, a local `font` path and a `license` path.
Applications optionally include `asset` and `license` paths. All resource paths
are relative to the brand folder and must exist. Guide presentation uses system
fonts unless an authored typography item supplies a local font file.

Optional `guidance.html` is trusted authored HTML, inserted without rewriting its
source. Use quoted `src`/`href` attributes with brand-relative local paths or
fragment anchors. Include spacing/minimum-size examples, logo variants and other
custom guidance here. Resources must exist; no remote dependencies are needed.
Do not put untrusted third-party HTML into it. Existing guides are preserved;
link one from this file or keep using it alongside the generated catalog.

## Image-prompt style

`style.json` is the sole style source. Exactly these four nonempty string arrays,
under 2000 compact characters; keep subject and medium in the creator's prompt:

```json
{"colors":["ink #17221F","paper #F5F5EE"],"type":["clear sans serif"],"treatment":["simple geometry","generous negative space"],"exclude":["glossy effects","visual clutter"]}
```

`style` prints compact JSON only; errors go to stderr with nonzero status.
Refresh embeds the same compact string in the guide's copy field. Refresh after
editing the source. Unknown fields, NaN and empty values fail clearly.

## Asset discovery

All non-hidden files are cataloged recursively, except the three source files
above and metadata sidecars. Names/folders provide default titles/categories.
Keep assets, fonts, licenses and references in their current logical locations.
Optional `symbols/primary.svg.meta.json` describes that file only:

```json
{"title":"Primary symbol","category":"Symbols","description":"Main editable mark.","tags":["logo","primary"],"license":"licenses/artwork.txt","source":"references/original.png"}
```

`title`, `category`, `description`, `tags`, `license` and `source` are optional.
License/source paths must resolve locally. Sidecars with missing target files fail
with an actionable diagnostic; remove the selected asset's sidecar too when deleting.
Put supplied nonconforming SVG originals in `References` or `Originals`; they
remain downloadable and explicitly labeled, without an inline SVG preview.
Reusable SVGs must contain shapes/paths, with no bitmap, live text, script,
stylesheet or external dependency. The helper checks structural editability;
the agent owns visual quality, provenance and vector reconstruction.

`list [--query TEXT] [--json]` scans current files, so deleted assets never linger
in CLI results. JSON contains `assets`, each with relative `path`, `title`,
`category`, `description`, `tags`, `kind`, `preview` and any supplied metadata.

## Generated presentation

`.wstack-brand/` holds `index.html`, `catalog.json`, CSS/JS and generated-file
hashes for overwrite protection. It is excluded from asset discovery. No timestamps
or generated IDs cause repeat-run churn. Refresh validates inputs and preflights
owned output before writing. It preserves all authored/unrelated files and fails
on symlinks, missing resources, unowned output folders or customized output.

Open the returned HTML directly, or serve the brand folder over loopback. Assets
and fonts remain local. Downloads use stable relative paths. Copy uses the browser
clipboard or its local-file fallback; if denied, selected text stays available.
No native icon integration, publication, approval state or release is created.
