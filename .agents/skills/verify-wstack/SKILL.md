---
name: verify-wstack
description: Discover wstack and prove feature-map validation on disposable repositories through the real CLI.
---

1. Start at `./project --help`, `./project info --json --base <base-ref>` and `./project doctor --json`. Resolve the comparison base explicitly for delivery. Read `features/README.md`; discovery comes from `./project features list --json` and `./project features show feature-map --json`.
2. Run `./project features check` to validate this canonical map. Readiness is limited to the CLI fixture pilot; other suite commands are covered by local tests, not mapped live proof.
3. Run `./project verify feature-map --base <base-ref>`. Optional `--evidence-dir <new-directory>` selects a retained artifact directory. An existing directory is rejected. Run serially; the recipe launches the actual candidate executable on private fixture copies and tests text and JSON output.
4. Read the returned report and artifacts. A clean-candidate pass requires all eight case/entrypoint observations, unchanged candidate identity, and completed fixture cleanup. Dirty runs return `status: development`, with no candidate-stability claim; they cannot satisfy delivery. Blocked/failed reports remain nonzero and preserve completed observations; skipped paths do not pass. `status: incomplete` indicates interrupted work.
5. Evidence remains outside disposable fixture state in `.evidence/<run-id>/`. Cleanup removes only that run's fixture directory. Never delete evidence as cleanup. Reports include candidate/base, dirty state, actual executable, commands, statuses and artifacts. Dirty runs aid development; delivery requires a clean final candidate.
6. Run `./project ci` for the required Rust and setup checks. Review the actual final candidate and retain logs with its exact SHA/base. Additional changes require fresh applicable proof.

Current coverage: [feature map](features/README.md). Service, browser, scheduling and other CLI behavior are outside this pilot.
