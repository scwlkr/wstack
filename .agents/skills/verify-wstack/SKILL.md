---
name: verify-wstack
description: Discover wstack and prove feature-map validation and local brand resources through real CLI/browser surfaces.
---

1. Start at `./project --help`, `./project info --json --base <base-ref>` and `./project doctor --json`. Resolve the comparison base explicitly for delivery. Read `features/README.md`; discovery comes from `./project features list --json` and `./project features show feature-map --json`.
2. Run `./project features check` to validate this canonical map. Readiness is limited to the CLI fixture pilot; other suite commands are covered by local tests, not mapped live proof.
3. Run `./project verify feature-map --base <base-ref>`. Optional `--evidence-dir <new-directory>` selects a retained artifact directory. An existing directory is rejected. Run serially; the recipe launches the actual candidate executable on private fixture copies and tests text and JSON output.
4. Read the returned report and artifacts. A clean-candidate pass requires all eight case/entrypoint observations, unchanged candidate identity, and completed fixture cleanup. Dirty runs return `status: development`, with no candidate-stability claim; they cannot satisfy delivery. Blocked/failed reports remain nonzero and preserve completed observations; skipped paths do not pass. `status: incomplete` indicates interrupted work.
5. Evidence remains outside disposable fixture state in `.evidence/<run-id>/`. Cleanup removes only that run's fixture directory. Never delete evidence as cleanup. Reports include candidate/base, dirty state, actual executable, commands, statuses and artifacts. Dirty runs aid development; delivery requires a clean final candidate.
6. Run `./project ci` for the required Rust and setup checks. Review the actual final candidate and retain logs with its exact SHA/base. Additional changes require fresh applicable proof.
7. On a clean current revision, run `./project evidence check <retained-report.json> --base <base-ref> --json`. Required pairs come from this feature's Proof section; stale revision, skipped/missing coverage, incomplete cleanup or unreadable/outside artifacts fail. The real recipe still owns diagnostics assertions. Broader receipts require their own mapped declarations and qualified adapter.

8. For [brand resources](features/brand.md), run `./project verify brand --base <base-ref>` on the clean candidate. This separate browser recipe needs Node.js, Playwright with Chromium/WebKit and sharp from existing tooling (`NODE_PATH` supported). It retains generated and expressive authored guides, 320/390/1440 screenshots/DOM, actual clipboard paste/download bytes, lazy payload requests, preservation hash/mtime snapshots and original-raster/vector comparisons, then removes only owned preview/browser/state. Inspect the retained screenshots and `report.json` after cleanup. This recipe's report is not a generic qualified Browser receipt; use its own assertions. `./project ci` covers brand CLI lifecycle and setup forwarding.

Current bounded coverage: [feature map](features/README.md) and local brand resources. Other suite surfaces, service behavior and scheduling remain outside these recipes.
