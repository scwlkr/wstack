# Executable eyes-and-hands audit

Use `wstack eyes-and-hands check --root <project> --json` at session start and after relevant changes. It inventories the existing verification feature map and reports missing, blocked, planned or unproven capabilities. Inspection is read-only and exits 1 until the requested capabilities have passed a live audit.

Within the task's authorization, run `wstack eyes-and-hands check --root <project> --run --json`. This executes registered commands, including their side effects. `--feature save.md` scopes the run to named feature files; repeat it for several. A scoped pass proves only that scope. `--output <directory>` chooses the parent of a fresh evidence directory; otherwise evidence is retained under the system temporary directory.

## Registration

Place `capabilities.json` beside each verification map's `features/README.md`. The feature Markdown files remain the inventory: every `Sub-features` acceptance statement starts with a unique short ID, in the form ``- `save-title` persists a title.``. Registry entries refer to those IDs; unknown or duplicate coverage is an error.

```json
{
  "version": 1,
  "probes": [
    {
      "feature": "save.md",
      "name": "save-and-reopen",
      "covers": ["save-title"],
      "setup": ["./project", "editor:seed"],
      "observe": ["./project", "editor:inspect"],
      "act": ["./project", "editor:save", "Plan trip"],
      "assert": ["./project", "editor:reopen", "Plan trip"],
      "cleanup": ["./project", "editor:cleanup"],
      "timeout_ms": 15000
    }
  ],
  "gaps": [
    {
      "feature": "save.md",
      "capability": "reject-empty",
      "reason": "The browser driver cannot reach the validation state yet",
      "owner": "editor tooling",
      "next": "Add and run an invalid-input scenario"
    }
  ]
}
```

These commands are illustrative: implement and try the project's actual recipes. Every probe requires all five commands, at least one covered ID, a unique name in its map and a timeout of 1–300000 milliseconds per command. JSON fields are strict so a misspelled assertion cannot silently disappear. A concrete gap requires a reason, owner and next action. A missing entry stays missing; `Status: planned` features are not executed or counted as passed.

## Execution and evidence

- Commands are argv arrays, executed sequentially in `--root`, without implicit shell evaluation. Route through the project's CLI when available. Use supported environment injection for credentials; registry files and command logs must contain no secret values.
- The runner supplies `WSTACK_EVIDENCE_DIR`, unique to this probe in this run. Write structured observations, screenshots and readback there. Use a run-specific namespace for fixtures/ports/profiles; setup must not overwrite shared state. Each probe must be independently runnable.
- `setup` reaches a known starting state; `observe` checks the actual surface; `act` exercises real user behavior; `assert` verifies the resulting state and independently reads mutation effects. Commands must return nonzero for failed assertions. An `echo passed` recipe cannot establish a capability.
- On setup/observation/action/assertion failure, later actions are skipped and cleanup still runs. Cleanup failure fails the probe. Timeout and interruption attempt cleanup; cleanup is also bounded by the probe timeout. Unix subprocess groups are terminated after the lifecycle, including timeout. Windows tree termination is attempted with `taskkill`; qualification on Windows is still required.
- `cleanup` removes only owned instances/data, preserves evidence and tolerates partial setup. The runner checks that command logs survived. Driver-specific proof artifacts must also survive cleanup. Forced termination of the audit process itself can prevent cleanup; fixtures must remain recoverable.
- Reports include per-capability status, exact command/stage/exit/timeout, error-log paths and a JSON receipt. Receipts record the root, Git SHA/dirty state when available, requested scope and timestamp. A clean commit identifies code; the driver must identify the actual app instance/build and assert the expected behavior.

Exit 0 means every capability in the requested scope passed the registered live commands, with no schema/map errors. It does not discover undocumented features or judge whether a weak assertion proves its claim. Agents must reconcile source with the feature inventory and inspect evidence. Re-run after relevant code/environment/driver changes; do not reuse a receipt as current proof blindly.

## Browser qualification

The repository includes a real editor fixture with CLI-driven Playwright interaction, CDP network observation, screenshots and independent persistence checks. With an existing Playwright/Chromium installation, run from the checkout:

```sh
python3 tests/browser/qualify.py --wstack cli/target/debug/wstack --playwright-module /path/to/node_modules/playwright --output /path/to/evidence
```

The working save must pass. A deliberately broken save that returns success without persisting must fail on fresh UI readback, with cleanup and evidence retained. Playwright/CDP API references: [CDPSession](https://playwright.dev/docs/api/class-cdpsession), [browser launch](https://playwright.dev/docs/api/class-browsertype).
