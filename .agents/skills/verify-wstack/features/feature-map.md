# Feature map validation

Agents validate a verification map and receive usable diagnostics when its index or feature sections are invalid.

## Sub-features

- map-valid: a valid map exits zero and reports feature/planned counts.
- map-index: missing targets and orphan feature files exit one with concrete file/line diagnostics.
- map-sections: missing or misordered required sections exit one with concrete diagnostics.
- map-json: JSON stdout parses and agrees with exit status and diagnostics for each case.
- map-evidence: retained commands and observations survive cleanup of private fixture state.

## How to get to it (user POV)

Use `./project features check --root <project-or-map>` for text; add `--json` for machine output. `./project verify feature-map --base <base-ref>` executes the reusable recipe through those actual entrypoints.

## Driving it with ./project

Preconditions: `./project doctor --json` is ready; tracked fixtures exist; a Git revision/base resolves; the evidence destination is new.

- Run `./project verify feature-map --base <base-ref>`; four private fixture maps exercise text and JSON paths (eight observations).
- Inspect valid-map stdout for counts, invalid-map stderr for dead entry, orphan and missing Proof diagnostics, and JSON stdout for matching results.
- Inspect `report.json` after cleanup: every observation is pass, cleanup is true, and private `state/` is absent. The same report must retain actual exit codes and command arrays.

## Proof

`.evidence/<run-id>/report.json` records identity, resolved base, dirty state, surface, executable, commands, entrypoints, cases, status and cleanup. Paired `<fixture>-text.stdout/.stderr` and `<fixture>-json.stdout/.stderr` record each observation. Evidence is development-only when dirty; final delivery uses a clean candidate and local gate logs.

## Gotchas

- A fixture's expected failure is a passing observation only when its exact diagnostic and exit status match.
- Fixture coverage does not cover every checker diagnostic or every project feature; unit/integration checks complement it.
- Reused evidence directories and invalid bases fail. Interrupted runs may leave incomplete evidence; do not interpret it as pass.
