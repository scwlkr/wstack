<!-- wstack-setup:start -->
## Project standards

- Project: {name}. Rust `./project` → tests/automation; `--help` → usage; `doctor` → prerequisites. Extend `tools/project-cli/src/routes.rs`; raw tools only CLI bootstrap/repair.
- Verify real app outcomes/side effects through CLI/UI drivers; scaffold/build ≠ app proof. Retain evidence; stop only processes you started.
- Smallest correct change; preserve unrelated work. Files ≈300 lines, split by responsibility; generated/vendor exempt; justify exceptions in handoff.
- Tests → behavior/credible regression/uncovered independent contract; prefer one owner-boundary check. Verify before deleting redundant/implementation-coupled tests/test-only seams; preserve regression coverage. Regression: fail before → pass after. No quotas/trivial-change tests.
- Linear only: team **{team}**, project {linear_project}. Before substantive work: read issue/discussion, reuse/create issue; ID → branches/PRs; post verification/blockers. Done → acceptance verified + landed on default branch. `SETUP-TODO.md` = setup handoff, never backlog.
- Frequent focused commits/pushes; short-lived branches; merge verified work promptly within repo rules/authorization. Isolate active work; never commit unrelated edits for a clean status.
- Local CI: applicable `./project` checks pass before push/merge on exact clean SHA; retain SHA/base/commands/results. Edits/new SHA → recheck, including landed SHA before Done. Remote → storage/review; hosted → documented requirement/owner direction; required current results pass before merge/Done. Pending/missing/failed ≠ pass.
- CI changes: non-executable docs → light; reliable scope → focused; shared code/dependencies/build/CI/uncertain scope → full; generated/executable docs → behavior checks. Cache costly dependencies/tools/builds keyed by platform/toolchain/lockfile; verify routing/invalidation. Aggregate pass/fail@SHA covers applicable checks; filtering cannot hide failures. Unverified alignment → `SETUP-TODO.md`.
- Task boundaries/after landing → remove completed inactive worktrees/obsolete branches/disposable builds. First verify useful commits on GitHub, inspect uncommitted/untracked/ignored files, preserve useful local data; never upload secrets/caches. Keep active/shared worktrees; managed → host archive tool; creation → using-git-worktrees skill.
- Investigate first; state material assumptions; finish authorized work; ask only consequential blockers. Proportional checks; report verified outcomes/gaps/landing.

<!-- wstack:begin shared/eyes-and-hands.md -->
- eyes-and-hands: throughout development → find CLI observation/control gaps across features/states.
- Gap → reuse tools; else add project CLI adapter. Parameterize actions → compose scenarios. Computer use → fallback.
- Inspect UI/layout/screenshots, state/errors, network/storage/effects; browser → relevant DevTools.
- Seed/reach/reset; drive user interactions/failure paths; assert effects; mutations → independent readback.
- Record commands/proof/gaps/next action in existing map/task notes. Required gaps block verification; unrelated → continue.
- Live proof: readiness ≠ observed acceptance. Retain exact clean candidate/base, owned instance identity, raw observations and partial case coverage before cleanup; read artifacts after teardown. Dirty/interrupted/skipped/blocked paths ≠ passed.
<!-- wstack:end shared/eyes-and-hands.md -->

<!-- wstack:begin shared/operational-home.md -->
## Operational CLI home

- Start at `./project --help`; follow its identity/capability command and linked verification skill. Preserve established commands, flags, exit status and cancellation. Command definitions own help/capabilities; discovery is not a separately maintained feature inventory.
- One canonical project feature map owns feature IDs, behavior, entry points and recipes. CLI discovery reads that map; agents discover features from current source, routes, commands, UI and docs, then reconcile the map. A valid map or passing receipt establishes neither complete discovery nor project coverage.
- Identity records project/root, tracker, resolved candidate/base and dirty state. A missing Git revision is unknown, never a clean candidate. Name supported commands and remaining adoption gaps explicitly.
- Doctor states its scope: tools, map, runtime dependencies or owned instance. Tool/scaffold readiness is not app readiness; app readiness is not observed acceptance. Give a concrete remediation for unavailable prerequisites.
- Reuse current commands and harnesses. Project-specific adapters own app readiness, real behavior, private fixture configuration and cleanup. Keep configuration local; do not replace an established tool merely to match a template.
- The recipe reaches the actual user surface and compares independent expectations/readback. Bind reports to exact clean candidate/base and actual binary/instance identity; retain actions, raw observations, required/covered cases and partial results before cleanup. Read artifacts after teardown.
- Only stop/reset state this run owns. Declare external read-only dependencies. Failed, blocked, skipped, dirty and interrupted runs do not satisfy final acceptance; record signal/cleanup limits precisely.
- Setup preserves customized CLI implementations/routes and owner files. Upgrade only unchanged generated assets; a preserved implementation missing the new contract needs a bounded adapter repair, not replacement. Scaffold checking reports this gap without claiming app proof.
- Fresh setup and upgrades need disposable CLI exercises, repeat-run idempotence and custom-command preservation. Final delivery requires local checks and real affected proof on the exact clean candidate, independent review where warranted, and fresh applicable landed verification.
<!-- wstack:end shared/operational-home.md -->

## Technical stack



{technical_stack}
<!-- wstack-setup:end -->
