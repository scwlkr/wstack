---
name: wstack-brand
description: "Create or extend an expressive editable local brand guide, genuine SVG assets, searchable resources and compact prompt JSON; preserve approved design with repeatable CLI mechanics."
---
Usable directly or through `wstack brand`. Apply Wstack principles when installed;
this folder contains the whole brand workflow. Agent/owner author design and
artwork. The helper validates/refreshes derived resources, never creative direction.

1. Inspect the request, project instructions, approved guide, artwork, descriptive
   manifests, product consumers and licenses. Locate the existing logical brand
   folder and authoritative sources. State the outcome and material assumptions;
   ask only for consequential missing direction or unresolved rights/provenance.
   Inventory visual strengths and existing capabilities: composition, hierarchy,
   identifying art, examples, filters, preview dialogs, color copying and downloads.
   Capture matched narrow/wide visual and interaction baselines before extending
   existing work. Inspect actual source and browser behavior, not just filenames.
2. Branch by existing versus new work. Existing approved guide stays the main entry
   point: add only requested missing capabilities, preserve its visual strengths
   and stable asset references, repair affected inherited failures. A subordinate
   utility may link to/from it, but cannot become a second brand authority. New
   work begins with coherent product-specific art direction and produces bespoke
   ordinary HTML/CSS/JS. Composition, imagery, type, section order and hierarchy
   belong to the model. Recoloring a fixed template does not satisfy this workflow.
3. Develop/refine the identity within scope: primary and supporting color roles,
   accessible pairings, typography, logos/symbols and relevant reverse/monochrome,
   favicon/app-icon masters, voice and applications. Use the actual identifying
   artwork prominently where appropriate. Show concrete application, clear-space,
   minimum-size and misuse examples. Fonts/colors alone are insufficient design.
   Use local licensed fonts or system fallbacks; retain sources/license files and
   label unresolved rights honestly. Platform masters do not imply integration.
4. Create actual editable artwork with available tools. Reusable SVGs need genuine
   paths/shapes, outlined lettering, a viewBox and intrinsic colors, with no hidden
   bitmap, active content or external font. Keep photos/originals honestly labeled.
   Inspect markup and rendered previews including small/reverse/monochrome use.
   Image generation/edit requests use the available image tool; retain originals
   and reconstruct vectors separately. For a supplied icon sheet, view/preserve
   it, inventory every requested icon, reconstruct separate SVGs and compare all
   silhouettes, spacing, strokes and colors side by side. Disclose approximations.
5. Follow [format](references/format.md) and [integrations](references/integration.md).
   Select ordinary authored HTML with `brand.json`'s `guide`; edit layout/styles/
   prose/interactions directly. Refresh updates only `.wstack-brand` resources.
   Reuse the existing descriptive manifest through the bounded adapter; filenames
   and actual files remain discovery authority. Sidecars are optional exceptions,
   not a second metadata library. Avoid duplicate palette/type/voice prose for a
   catalog. Keep one compact editable `style.json`, semantically consistent with
   authoritative identity; guide and CLI derive exactly the same prompt string.
   Keep legacy generated projects working until deliberately adapted; customized
   generated output still blocks overwrite. Supplemental guidance stays supported.
6. Integrate only helpful capabilities into the main guide: catalog/search/scope,
   enlarged previews, lazy individual/bundle downloads and exact prompt copying.
   Optional hooks impose no global CSS or universal layout. Current usable artwork
   is prominent by default; legacy/reference/support resources remain discoverable
   through All resources. Preserve and exercise existing interactions too, including
   primary-guide downloads. Additions/revisions/removals update affected examples,
   references and owning metadata. Remove only explicitly selected resources.
7. Reuse an existing project CLI through a thin helper or `wstack brand` forwarder;
   preserve owner commands, flags, stdout/stderr and exit status. Without setup:

   ```sh
   python3 scripts/brand.py refresh --root /absolute/project --brand-dir brand
   python3 scripts/brand.py list --root /absolute/project --query icon --json
   python3 scripts/brand.py style --root /absolute/project
   ```

   Installed Rust CLI exposes the same commands/options. Setup-generated CLIs
   expose `./project brand refresh|list|style` (or collision-safe `wstack:brand`),
   with `WSTACK_BIN` when needed. No suite setup is required for standalone use.
8. Open the returned main guide and review it against the baseline. Check direct
   file URLs and loopback at 320/390/1440 pixels in Chromium and WebKit where
   available: navigation, keyboard/focus, local images/fonts, search/category/scope/
   empty states, enlarged previews, color copying, individual/bundle downloads,
   exact prompt copying and selected-text denied-copy fallback. Compare downloaded
   bytes with sources and real pasted/copied JSON with source/CLI. Observe that no
   download byte chunks load before a click; test first/later/revised downloads.
   Headless WebKit is not native Safari qualification. Disclose unsupported cases.
9. Make an ordinary authored design/content edit and refresh: it must survive.
   Verify asset/metadata/add/remove updates and unrelated preservation on owned
   copies. Refresh twice unchanged; require `changed: []`, identical contents and
   mtimes. Invalid style/metadata, missing references, unsafe paths and customized
   output must fail nonzero before replacing working derived resources. The guide
   remains directly editable; never solve design edits by weakening write guards.
10. Retain exact clean candidate/base, helper/binary identity, commands, source
    comparisons, matched screenshots, clipboard/download observations and owned
    teardown. Use the existing reusable verification recipe with an explicit kit
    and owned mutation copy when available. Inspect evidence after cleanup; judge
    expressive quality, meaningful art, hierarchy and practical examples yourself.
    Overflow assertions/builds do not establish beauty. Follow repository delivery
    rules and report actual outcomes, approximations and remaining gates. No
    automatic owner-kit migration, scheduling or public publication is implied.
