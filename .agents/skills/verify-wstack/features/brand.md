# Local brand resources

Owners create/refine a cohesive identity with the portable wstack-brand skill,
retain editable artwork and expressive authored guidance, and reuse it locally.

## Sub-features

- brand-skill: inspect existing work before creating identities, variants and
  applications. Preserve identifying artwork, color roles, typography, voice and
  composition. An unrelated new identity needs its own visual system.
- brand-files: actual files own existence; optional manifest and adjacent sidecars
  add metadata. Authored HTML/CSS, source artwork, references, licenses and owner
  commands survive refresh. Repeated refreshes preserve bytes and mtimes.
- brand-cli: explicit project root and brand folder work without suite setup;
  standalone helper and installed binary share implementation.
- brand-guide: authored guides can mount optional searchable resource tools and
  exact style copying without surrendering layout. The legacy generated guide
  remains available for the documented structured format.
- brand-downloads: original SVGs, fonts and existing bundles remain downloadable
  under file and loopback URLs; per-resource payloads load only on request.
- brand-vector: reconstruct each requested raster-sheet icon as independent paths,
  visually compare outputs and retain the original. Bitmap wrappers and reusable
  font-dependent SVGs fail.
- brand-errors: invalid styles/metadata, missing references, conflicting metadata,
  symlinks and customized owned output fail clearly without rewriting sources.

## How to get to it (user POV)

Use `$wstack-brand` or `$wstack brand` with a project, idea or image. Read
`skills/wstack-brand/SKILL.md` and its self-contained format/integration references.
The helper indexes authored sources and actual assets; it does not invent a brand.

`./project brand --help` describes refresh/list/style. Supply `--root PROJECT`
and optionally `--brand-dir PATH`. Open the returned authored or generated HTML.

## Driving it with ./project

- `./project brand refresh --root PROJECT --json` refreshes local resource data.
- `./project brand list --root PROJECT --query sunrise --json` scans actual files.
- `./project brand style --root PROJECT` prints compact source style JSON.
- `./project ci` covers CLI lifecycle, standalone parity, preserved authored
  sources/commands, generated compatibility and bounded metadata diagnostics.
- `./project verify brand --base BASE --evidence-dir NEW_DIRECTORY` exercises
  owned generated, preexisting authored Alder and unrelated authored Orbit kits
  in Chromium and WebKit. Prerequisites: Node.js, Playwright with both browser
  engines, sharp and Python. Reuse packages via `NODE_PATH`.

## Proof

The browser recipe retains candidate/base/dirty identity, executable and
helper/recipe digests, commands, source hash/mtime snapshots, guides, DOM,
screenshots, actual system-clipboard paste, downloaded original bytes and request
logs. It removes only its owned preview, browser contexts and disposable state,
then rereads retained artifacts. Dirty runs remain development-only.

Both engines exercise direct file and loopback URLs at 320/390/1440, decoded
local images/fonts, keyboard navigation, previews/focus return, actual style/color
copying, denied-copy exact selection, search/categories/current versus all/empty states,
SVG and existing ZIP downloads. Initial loads and interactions request no download
payloads; first download requests its own chunk, later download reuses it, and
asset revision requests a fresh chunk with the revised original bytes.

Authored baseline/overview screenshots use matched viewport dimensions and a
normalized mean pixel-error bound of 0.005 to allow browser text rasterization
variation. A person reviews overall identity/composition. Intentional authored
HTML copy and CSS layout/style edits survive metadata/asset additions/removals
and two subsequent no-op refreshes, with unrelated source hashes/mtimes unchanged.

The independent original raster under `tests/fixtures/brand` is compared with
sprout, sunrise and cairn SVGs; normalized mean error must stay below 0.015 per
icon. Inspect retained `vector-comparison.png`. This bounds the original fixture,
not arbitrary vectorization or aesthetic quality.

## Selected existing kit

Set `WSTACK_BRAND_KIT` to an absolute project root and optional
`WSTACK_BRAND_DIR` to its relative brand folder. The recipe copies that folder
read-only into owned state, snapshots the source and verifies it remains unchanged.
Selecting a kit runs the selected-kit cases; omit this setting for the three owned
fixture cases.
`WSTACK_BRAND_GUIDE=index.html` optionally selects the authored guide in the copy.
The guide needs a closing body, local dependencies and valid helper inputs.

The recipe appends a capability section only inside the copy. Configure retained
interactions through `WSTACK_BRAND_CHECKS=/absolute/checks.json`:

```json
{
  "target": "assets/vector/logo-primary.svg",
  "font": "Montserrat",
  "search": {"query":"assets/vector/logo-primary.svg","paths":["assets/vector/logo-primary.svg"]},
  "color": {"selector":".copy-color","value":"#007CE8","status":"#feedback"},
  "preview": {"open":".asset-image","dialog":"#asset-dialog","close":"#dialog-close"},
  "download": {"selector":"a[href='assets/vector/logo-primary.svg'][download]","path":"assets/vector/logo-primary.svg"},
  "bundle": {"selector":"a[href='bubbas-vectors.zip'][download]","path":"bubbas-vectors.zip"}
}
```

Only these bounded fields are accepted. The selected font must actually load;
color checks use actual clipboard paste; preview selectors drive retained UI.
Configured download links receive the portable download hook in the copy, and
must name cataloged files. Controls/metadata remain ordinary authored links.
Select a current editable SVG target; unavailable retained behavior fails openly.
Existing integration scripts are reused, including deferred scripts; nested guide
paths resolve from the guide directory. Known legacy generated Catalog/Prompt
links are adapted only in the copy and recorded. Authored references to retired
generated files fail before writes; preserve those resources as authored inputs
before adopting a kit.
The recipe reports selected-kit coverage separately from owned fixture coverage.

## Gotchas

- No external model/service, hosting, native integration, publication or approval.
- `doctor` still describes the narrower feature-map pilot; browser prerequisites
  are checked by the separate recipe and never imply observed acceptance.
- A passing report is recipe-specific; it does not claim the separate generic
  qualified Browser receipt contract or use `evidence check`.
- Custom existing guide selectors must be configured; the recipe does not infer
  arbitrary interaction contracts or repair the owner's original project.
