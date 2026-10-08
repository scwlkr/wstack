# Local brand format

Select the existing logical brand folder with `--brand-dir` (default `brand`),
relative to `--root`. No suite folders, setup, framework or service is required.
Python 3.9+ and ordinary files suffice. The helper supplies mechanics; the agent
and owner author the identity, guide and artwork.

## Authored guide (preferred for new work)

Keep the approved guide as the main entry point. Select it in `brand.json`:

```json
{"guide":"index.html","metadata":"asset-manifest.json"}
```

`guide` is an existing HTML file inside the brand folder, outside `.wstack-brand`.
`metadata` is optional. No second palette, typography, voice or application prose
is required. Use the existing authoritative brand sources when designing; only
structure facts that need computation or reuse. `style.json` remains the compact
prompt source. Edit HTML, CSS, JS and artwork directly. Refresh never writes them.

`refresh` returns the authored guide path and updates supporting resources only.
Add the [optional integrations](integration.md) where they help. They use classic
local scripts and work from direct file URLs and loopback. No global stylesheet
or fixed page layout is imposed. Supplemental `guidance.html` remains supported
by the legacy renderer; authored guides can link it normally.

Local `src`, `href`, `poster` and linked CSS `url()` references must exist inside
the brand folder. Relative paths within that folder and fragment anchors work;
external hyperlinks may document provenance, but dependencies must be local.
The generated catalog/integration references may be absent before first refresh.
References into `.wstack-brand` must name current resources; an adaptation cannot
retain links to legacy generated styles/pages that refresh would retire.
Keep identifying artwork, explicit color roles, concrete application, spacing,
size and misuse examples in the authored guide. The model owns semantic and
visual consistency; the helper does not infer creative direction.

## Image-prompt style

`style.json` is the sole prompt style source. Exactly four nonempty string arrays,
under 2000 compact characters; keep subject and medium in the creator's prompt:

```json
{"colors":["ink #17221F","paper #F5F5EE"],"type":["clear sans serif"],"treatment":["simple geometry","generous negative space"],"exclude":["glossy effects","visual clutter"]}
```

`style` prints compact JSON only; diagnostics go to stderr with nonzero status.
Refresh derives the exact same compact string for browser copying. Unknown keys,
NaN, empty values and oversized blocks fail. Refresh after editing the source.

## Asset discovery and metadata ownership

All non-hidden files are discovered recursively, except `brand.json`, `style.json`,
`guidance.html`, the selected metadata manifest and adjacent metadata sidecars.
Existence comes from real files. A manifest annotates them; it never registers an
absent resource. HTML/CSS/JS, fonts, licenses and research remain discoverable.
Keep source artwork, originals, licenses and stable filenames in their logical
locations. The CLI always searches the complete library.

Reuse a project's descriptive manifest when possible. The bounded adapter accepts
the motivating `asset-manifest.json` shape, selected by `metadata`:

```json
{"assets":[
  {"file":"symbols/primary.svg","title":"Primary symbol","category":"Symbols",
   "notes":"Use on quiet backgrounds.","status":"Existing","source_id":"S16",
   "tags":["logo"],"role":"current","license":"licenses/artwork.txt"}
]}
```

`file` is required and brand-relative. `notes` becomes `description`; `title`,
`category`, `tags`, `role`, local `source` and local `license` are optional.
`status` and `source_id` retain the manifest's provenance as text. Existing top-level
editorial fields and cached `sha256`, `bytes`, `vector` audit annotations are
ignored: current catalog hashes/downloads derive independently from actual bytes.
This adapter does not interpret other manifest formats or resolve source IDs.

For genuine exceptions, `symbols/primary.svg.meta.json` can supply any of:

```json
{"title":"Primary symbol","category":"Symbols","description":"Main editable mark.","tags":["logo"],"role":"current","license":"licenses/artwork.txt","source":"references/original.png"}
```

Precedence: filename/folder defaults, then adjacent metadata, then the manifest.
A manifest owns every field it supplies. Different values for the same field in
a sidecar fail with a conflict diagnostic; remove duplicated fields and edit the
owner. Identical historical duplicates remain accepted for compatibility. Duplicate
manifest entries and missing asset/source/license references fail before writes.
A manifest edit reaches the catalog on refresh without copying it into sidecars.
When removing selected assets, remove their metadata and affected examples too.

Roles are `current`, `legacy`, `reference` or `support`. Explicit metadata wins;
otherwise legacy paths, reference/original/research paths, fonts/licenses and
support documents receive corresponding roles. Other artwork defaults to current.
Browser search/category filters operate within the selected scope: Current assets
by default, All resources explicitly. Category options include the whole library;
a category outside the current scope gives an understandable empty state.

Supplied nonconforming SVG originals belong in References/Originals or role
reference. They stay downloadable without an inline SVG preview. Reusable SVGs
need a finite viewBox, actual paths/shapes and no bitmap, live text, active content
or external dependency. Structural validation does not establish visual quality
or provenance; inspect/reconstruct and compare artwork separately.

`list [--query TEXT] [--json]` scans current files. JSON contains `assets` with
`path`, `title`, `category`, `description`, `tags`, `role`, `kind`, `preview` and
supplied metadata. Search covers these descriptive values, not filenames alone.

## Generated resources and safety

`.wstack-brand/` holds `catalog.json`, browser `catalog.js`, `integration.js`,
`downloads/<sha256>.js` and `generated.json` ownership hashes. Legacy mode also
maintains its page/CSS/JS. These are derived files, excluded from discovery.
Download chunks contain original bytes; the browser loads only the selected
chunk on demand and reuses it for later clicks. A revised asset gets a new digest
URL. Refresh removes obsolete owned chunks, including the old eager downloads.js.

Inputs/references and every affected generated file are preflighted before any
write/removal. Symlinks, unowned folders, customized generated output and unsafe
hash manifests fail clearly while preserving working output. Unrelated files
remain untouched. Unchanged repeats return `changed: []` and retain contents and
mtimes. Atomic file replacement protects individual writes; refresh is not a
multi-file transaction against power loss or concurrent writers. Run it serially.

## Legacy generated guides and deliberate adaptation

Existing `brand.json` without `guide` keeps the generated guide contract. Its
required fields remain `name`, `summary`, nonempty `palette`, `typography`, `voice`
and `applications`. Palette items require `name/value/on/usage` with six-digit
hex colors; typography requires `name/family/usage` and optionally local
`font/license/sample`; applications require `name/usage` and optionally local
`asset/license`. Palette contrast is computed by the legacy renderer.

Legacy `guidance.html` is trusted local HTML with quoted brand-relative `src/href`
attributes or fragments. Its source is preserved. Do not insert untrusted HTML.
Customized generated files still block refresh. Never silently convert a project.

To deliberately adapt: preserve/copy the existing page and styles/interactions
outside `.wstack-brand`, fix its relative references, replace embedded catalog/
prompt/download mechanics with optional integrations, then set `guide` to that
file. Preserve any custom generated originals elsewhere before refreshing.
Refresh retires only unmodified obsolete generated files; customized ones fail.
Compare matched before/after screenshots and exercise retained interactions as
well as new ones. An optional subordinate utility must link to/from the main
guide and must not duplicate the authoritative brand narrative.
