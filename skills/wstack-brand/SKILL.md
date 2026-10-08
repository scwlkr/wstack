---
name: wstack-brand
description: "Create or refine a cohesive brand, editable SVG assets, a portable local HTML guide, searchable catalog and copyable image-prompt JSON; maintain existing resources or reconstruct an icon sheet."
---
Usable directly or through `wstack brand`. Apply Wstack principles when installed;
this folder contains everything needed for the brand workflow itself.

1. Inspect the request, project instructions, existing guide/assets, product
   consumers, supplied images and licenses before designing. Locate the logical
   brand folder; preserve its layout, names and custom guidance. State the intended
   outcome and material assumptions. Ask only for missing consequential direction
   or rights/provenance that cannot be resolved. Do not impose another project's identity.
2. Develop the supplied idea/image/reference into a cohesive direction, or extend
   the existing one: palette and accessible pairings, typography, logo/symbol
   system, reverse/monochrome variants, favicon/app-icon SVG masters, voice and
   representative applications. Explain usage, spacing, minimum sizes and misuse.
   Use local licensed fonts or system fallbacks; retain license files and sources.
   Mark unresolved origin/use rights honestly. Platform masters do not imply
   product integration or native/store readiness.
3. Create actual editable artwork using available tools. For reusable SVGs use
   independent paths/shapes, outlined logo lettering, a viewBox and intrinsic
   colors; avoid external fonts, active content and hidden raster dependencies.
   Keep photographs and originals honestly labeled as references. Inspect SVG
   markup and rendered previews, including small-size, reverse and monochrome use.
   When image generation/editing is requested, use the available image tool and
   retain its original output; reconstruct vectors separately when needed.
4. For an icon-sheet request: preserve the original image under references, view
   it, inventory/name every requested icon, reconstruct each as a separate genuine
   vector SVG, then compare all previews side by side against the source. Check
   silhouettes, spacing, strokes, colors and small-size legibility. A bitmap crop
   embedded in SVG fails. Disclose approximations; do not claim perfect conversion.
5. Keep authored files in that same folder. Follow [format](references/format.md):
   `brand.json` owns guide prose/usage, `style.json` owns the compact reusable prompt
   block; optional `guidance.html` holds custom local HTML. Preserve existing
   guides; link them from guidance or catalog them. No separate asset registry.
   Add optional adjacent metadata only when filenames/folders need clarification.
   For additions/revisions refresh affected examples; remove only explicitly
   selected assets and their metadata/references, preserving unrelated work.
6. Reuse an existing project CLI through a thin forwarder to the helper or
   `wstack brand`; preserve custom commands, flags and exit status. Without project
   setup run the bundled helper from this skill folder:

   ```sh
   python3 scripts/brand.py refresh --root /absolute/project --brand-dir brand
   python3 scripts/brand.py list --root /absolute/project --query icon --json
   python3 scripts/brand.py style --root /absolute/project
   ```

   The Rust CLI exposes the same `refresh`, `list --query TEXT [--json]`, and
   `style` commands with `--root` and `--brand-dir`. Setup-generated project CLIs
   expose `./project brand refresh|list|style` using `WSTACK_BIN` when needed.
   For other existing CLIs add only a forwarding route; do not replace their setup.
7. Open the returned `.wstack-brand/index.html`. Inspect narrow/wide layouts,
   keyboard navigation, catalog search/categories/empty state, every relevant
   preview and individual download. Actually copy JSON and compare clipboard
   content with `style.json` and CLI output. The guide works from local files;
   a temporary loopback server is also useful for browser verification. No hosting
   is required. If copying is unavailable the field selects text with exact steps.
8. Refresh twice; unchanged repeats must return `changed: []` and preserve file
   contents/mtimes. Missing assets or invalid style data must fail clearly without
   replacing the last guide. Customized generated files block overwrite; keep
   authored additions in `guidance.html`, or preserve/rename the generated folder
   before rebuilding. This write guard is not a brand version/release system.
9. Follow repository delivery rules. Retain commands, exact clean candidate/base,
   file comparisons, screenshots, clipboard/download observations and icon-sheet
   comparison. Read artifacts after owned preview/fixture cleanup. Report actual
   outcome, files, approximations and remaining blockers; builds alone do not
   establish visual acceptance. No scheduled maintenance or publication is implied.
