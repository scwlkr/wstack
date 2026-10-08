# Wstack Brand

## Problem Statement

Brand guidance and assets become scattered across repositories, exports, and chats.
Owners need to find, reuse, and extend them without maintaining another application
or a separate brand release process.

## Solution

Deliver one portable `wstack-brand` skill, available directly and through Wstack.
It creates or refines a cohesive identity, keeps editable assets together in the
project's brand folder, and presents them in a polished local HTML/CSS guide and
searchable catalog. A small repeatable helper keeps the catalog current and makes
assets and a compact image-prompt JSON style block available through the CLI.

This is one bounded implementation ticket. Ordinary files and Git history are
enough; there is no separate brand versioning or governance layer.

## User Stories

1. As a project owner, I want the skill to inspect existing branding first, so that useful guidance and assets are preserved.
2. As a project owner, I want the skill to develop an idea, image, or reference into a cohesive identity, so that I can start without an existing guide.
3. As a project owner, I want colors, typography, logos, symbols, voice, and applications documented together, so that others can use the identity consistently.
4. As a designer, I want favicon, app-icon, reverse, and monochrome variants, so that the identity works in common contexts.
5. As a contributor, I want a complete, clearly named asset catalog, so that I can find the correct resource without searching chats.
6. As a contributor, I want previews and individual downloads, so that I can inspect and reuse assets quickly.
7. As a designer, I want independent SVGs with editable vector shapes, so that I can adjust artwork without depending on a raster image or a missing external font.
8. As a project owner, I want original references and applicable font/asset licenses kept with the brand resources, so that their origins and use remain clear.
9. As an image creator, I want a short JSON style block with a working copy control, so that I can append it to a subject and medium in ChatGPT.
10. As an agent or CLI user, I want to list/search assets and retrieve that same JSON, so that visual work is discoverable without opening the guide.
11. As a project owner, I want to add or revise an element through a normal skill request, so that the catalog and guidance stay useful over time.
12. As a project owner, I want selected elements removable without disturbing other work, so that obsolete resources can be cleaned up safely.
13. As a project owner, I want an icon-sheet image separated into named, genuine vector SVG icons, so that each icon becomes an editable resource in the existing catalog.
14. As a contributor, I want rebuilding to retain custom guidance and unchanged assets, so that repeated use does not reset the brand.
15. As a portfolio owner, I want the skill usable in different projects without copying Patri-specific branding, so that each project keeps its own identity.
16. As a project owner, I want the guide usable locally on narrow and wide screens, so that reviewing a brand needs no hosted service or account.

## Implementation Decisions

- Add one installable suite member, register it in the suite listing/grouping, and
  route Wstack's brand request to it. Keep skill resources self-contained and
  follow Wstack's shared principles and skill-folder conventions.
- Bundle only the instructions, presentation resources, and small rerunnable helper
  needed for these outcomes. Use existing Rust CLI and bundled-script patterns;
  add no application framework, database, service, or model-provider integration.
- Keep editable source assets and guidance in the project's brand folder. Preserve
  an existing logical layout, names, customized guide content, and product consumers;
  avoid mandatory migrations or duplicate authoritative brand libraries.
- Generate the catalog from the actual local assets and minimal descriptive metadata.
  Do not introduce a separately maintained asset registry. Use clear categories and
  stable relative references; missing or invalid resources get actionable diagnostics.
- Expose a minimal brand command group for catalog refresh, asset listing/search,
  and style JSON retrieval. Accept an explicit project root, including projects
  without Wstack's own suite folders. Reuse an existing project CLI through thin
  forwarding when present; the skill remains usable without project setup.
- Listing/search supports human-readable and JSON output. Style retrieval writes
  the compact JSON to stdout, with diagnostics on stderr and failure exit status
  for invalid input. Help describes the supported operations.
- The guide covers palette and usage/contrast, typography, logo/symbol variants,
  favicon/app-icon masters, voice, and brand applications. Include a searchable
  catalog, usable previews/downloads, and a copyable image-prompt style block.
  Platform icon masters do not claim native integration or store readiness.
- Keep the HTML/CSS local and portable with local assets/fonts and lightweight
  JavaScript only where useful. Catalog refresh must preserve authored guidance
  and unrelated files, and an unchanged repeat run must not create duplicates or churn.
- Maintain one compact style JSON source shared by guide copying and CLI retrieval.
  Capture colors, type, visual treatment, and exclusions; keep subject and medium
  separate so the block is reusable.
- Asset creation/revision is an agent workflow using available tools. The helper
  indexes and presents files; it does not pretend to autonomously design or vectorize.
  Raster icon sheets require vector reconstruction and visual comparison.
  Embedding bitmap crops inside SVG wrappers does not satisfy editable-vector output.
- Preserve supplied source images and licenses. Ensure reusable SVGs have genuine
  shapes/paths and no hidden raster dependency or externally required logo font.
  Catalog non-vector references honestly; photographs need not be converted to SVG.
- Additions, revisions, and explicit removals update the catalog and affected guide
  examples. Preserve unrelated assets and custom project commands.
- Implement and verify this in one ticket. Git supplies ordinary history; do not
  add brand editions, approval states, release management, or scheduled maintenance.

## Testing Decisions

- Prefer one end-to-end acceptance scenario on disposable project folders through
  the public skill/helper/CLI boundary. Reuse Wstack's actual-binary fixture and
  setup-preservation test patterns; assert outputs and behavior rather than internals.
- Exercise an existing brand and a small unrelated starter project to establish
  portability. Create/catalog assets, list and search them, retrieve JSON, add an
  asset, revise one, remove a selected asset, and refresh twice. Independently
  inspect resulting files and prove unrelated/custom content remains unchanged.
- Open the generated guide in a real browser at narrow and wide widths. Check
  navigation, keyboard use, previews/downloads, and actual clipboard copying;
  compare copied and CLI JSON with the same stored style source.
- Run one real icon-sheet skill exercise using a provided or original fixture.
  Compare the individual SVG previews visually with the sheet and inspect their
  vector content. A static catalog test alone does not prove this agent workflow.
- Cover invalid style data and missing assets with clear nonzero failures, and
  retain failures honestly. Do not substitute a successful build for visual proof.
- Run existing suite checks and the configured local CI on the exact clean
  implementation candidate, then applicable landed verification. No hosted runner
  is needed for this feature.

## Out of Scope

A standalone brand application, database, hosted portal, accounts, asset registry,
brand version/release/approval machinery, background jobs, automatic drift monitoring,
portfolio-wide migrations, product UI changes, website publication, native/store
integration, new image-generation services, or automatic perfect raster-to-vector
conversion.

## Further Notes

The motivating example is: "/wstack-brand here is an image of an icon sheet I
generated; separate these into individual SVG icons and add them to the rest of
the brand resources."

Patri's existing guide is a useful reference for presentation and reusable assets,
not a template identity to impose on other projects. Keep the implementation
proportional: one skill, a small helper, and the existing Wstack integration points.

The single Linear ticket owns implementation and delivery. Documentation alone
does not complete the feature; verify implementation and the landed candidate
before closing the ticket.

Implementation: [WLK-158](https://linear.app/wlkr-labs/issue/WLK-158/implement-wstack-brand-for-local-guides-editable-assets-and-cli).
