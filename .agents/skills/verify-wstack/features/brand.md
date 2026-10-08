# Local brand resources

Owners create/refine a cohesive identity with the portable wstack-brand skill,
retain editable assets and guidance, and discover/reuse them locally.

## Sub-features

- brand-skill: direct member and Wstack routing; inspect existing work before creating identities, variants and applications. No product integration/publication is implied.
- brand-files: index actual files and optional adjacent metadata; preserve names/layout, authored guidance, references, licenses, custom commands and unrelated work. Repeated refreshes do not churn.
- brand-cli: refresh, human/JSON listing/search and compact style output accept an explicit project root without suite folders; installed binary and standalone helper share implementation.
- brand-guide: local responsive HTML/CSS covers colors/contrast, typography, voice, applications and searchable asset previews/downloads; copy uses the same stored style JSON.
- brand-vector: inspect/reconstruct each requested raster-sheet icon as independent SVG paths/shapes, visually compare outputs and retain the original. Bitmap wrappers and font-dependent reusable SVGs fail.
- brand-errors: invalid style/resource data, missing referenced assets, symlinks and customized/unowned generated output fail clearly; authored files remain untouched.

## How to get to it (user POV)

Use `$wstack-brand` or `$wstack brand` with a project, idea or image. Read
`skills/wstack-brand/SKILL.md` and its self-contained format reference. The helper
requires authored brand/style JSON and real assets; it does not invent a brand.

`./project brand --help` describes refresh/list/style. Supply `--root PROJECT`
and optionally `--brand-dir PATH`. Open the returned local `.wstack-brand/index.html`.

## Driving it with ./project

- `./project brand refresh --root PROJECT --json` creates/refreshes the guide.
- `./project brand list --root PROJECT --query sunrise --json` scans actual assets.
- `./project brand style --root PROJECT` prints compact style JSON.
- `./project ci` exercises the real CLI lifecycle, standalone helper parity,
  portable existing/starter layouts, write preservation, diagnostics and generated
  project CLI forwarding with an owner-command collision.
- `./project verify brand --base BASE --evidence-dir NEW_DIRECTORY` drives a real
  Chromium browser against owned disposable Alder/Harbor projects. It requires
  Node.js, Playwright with Chromium installed and sharp. Reuse existing packages
  via `NODE_PATH`; do not add framework/runtime dependencies to target projects.

## Proof

The browser recipe retains exact checkout/base/dirty identity, binary/skill file
digests, CLI stdout, both local guide folders, clipboard contents, downloaded SVGs,
wide/narrow screenshots/DOM, vector comparison measurements/image and teardown.
The report is specific to this recipe; it does not claim the separate generic
qualified Browser receipt contract or use `evidence check`.

The independent raster original under `tests/fixtures/brand` was inspected before
manually reconstructing sprout, sunrise and cairn SVGs. Compare the retained
`vector-comparison.png` visually; measured normalized mean pixel error must be
below 0.015 per icon. This bounds this original fixture, not arbitrary vectorization.

Browser observations cover keyboard navigation/search, categories/empty results,
image decoding, actual clipboard equality with source/CLI, download byte equality,
390/1440 layouts and direct file-URL copying. Retained files are read after owned
preview/browser/state teardown. Dirty results are development-only. Inspect visual
quality yourself; an overflow assertion alone does not establish a polished layout.

## Gotchas

- No external service/model, hosting, native/store action or brand governance.
- `doctor` continues to state the narrower feature-map pilot scope; browser recipe
  prerequisites fail explicitly at invocation and never count as passed coverage.
- Chromium is the bounded automated browser target; other browsers are not qualified.
- Real existing brands can be copied read-only into disposable projects for extra
  preservation checks; never rewrite the owner's source just to run verification.
- Reconstructing an original fixture is not a claim of automatic perfect conversion.
- Customized generated output is preserved by failing; move authored guidance to
  guidance.html or retain/rename that output folder before generating a fresh one.
