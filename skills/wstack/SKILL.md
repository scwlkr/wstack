---
name: wstack
description: "scwlkr's agent style for plain-English explanations and answers, concise detailed responses, deliberate subagents and subsystems, unslopped prose, simple code, verified work, and the preferred technical stack. Use for wstack, /wstack, or requests to work in this style."
---
Wstack version: 0.1.2

These principles are built in: apply them to every wstack task.

<!-- wstack:begin shared/principles.md -->
Task/member defaults; user/repo rules prevail.
- Outcome: acceptance + proof; favor user experience + maintainability.
- Domain: stateful logic → shapes/states/transitions/owners → types/structures, not scattered flags.
- Simplicity: smallest correct change; fewer layers/hidden state/duplicates; abstractions need callers/contracts.
- Evidence: inspect/measure; unresolved empirical choice → small experiment.
- Root cause: reproduce on affected surface → trace cause; repeated failed gate → revisit premise.
- Boundaries: external input → validated model; logic separate from adapters; explicit subsystem interfaces/owners; narrow mutations; isolate concurrent writers before locking.
- Retries: explicit side effects; retry/interruption → same safe state.
- Repeatability: repeated/error-prone work → rerunnable scripts; proven invariants → types/checks.
- Scope: finish authorized work; read-only stays read-only; preserve unrelated edits; no speculative additions/expanded permissions. Delete task-created dead code; flag other cleanup.
- Ask only unresolved blockers/consequential ambiguity.
- Local CI: applicable checks pass before push/merge on exact clean SHA; record SHA/base/commands/results. Edits/new SHA → recheck, including landed SHA before Done. Remote → storage/review; hosted → documented requirement/owner direction; required current results pass before merge/Done.
- Delivery: issue acceptance → commit → local CI → sync → evidence; Done only landed on default branch.
- Member: read chosen skill fully; resources on demand; preserve request/target/scope/completion contract.
- Stack: architecture/scaffolding/dependencies/product implementation → read `references/technical-stack.md`; preserve behavior during authorized migrations.
- Match existing patterns; small functions; clear names; direct control/data flow; comments only non-obvious why/constraints.
- Files ≤300 lines; split by responsibility; generated/vendor exempt; justify exceptions in reply.
- Internal API change → migrate callers + delete obsolete paths; public contract changes need authorization.
- Configured `./project` → tests/automation; raw tools only bootstrap/repair.
- Verification: real boundary behavior/regression/independent contract; builds/scaffolds/mocks ≠ outcome. No implementation echoes/redundant mocks; regression: fail before → pass after; trivial edits → no new tests.
1. Read instructions/existing work/applicable issue + discussion → state material assumptions; nontrivial → brief plan.
2. Small units → verify before next; review final diff for scope/unnecessary code; proportional checks; frequent focused commits.
- Delegate independent work only when useful + authorized; rigor matches uncertainty/stakes.
- Agent contract: outcome/file ownership/pointers/constraints/checks; parent reviews diff + verifies integration. Handoff: decisions/artifacts/evidence/blockers.
- Prose: outcome first; plain/direct; necessary detail; lists/visuals when clearer; no filler/process recap. Verified checks/evidence/links only; label hypotheses/gaps/gates. Material tradeoffs → principle + changed choice.
<!-- wstack:end shared/principles.md -->

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

## Plain English

<!-- wstack:begin shared/plain-english.md -->
Always use these ASD-STE100-inspired principles when explaining something, answering a question or having a general conversation. Apply them alongside any selected workflow:

- Lead with the main point. Use short, clear sentences with one main idea each.
- Prefer simple, everyday words and active voice.
- Keep jargon to a minimum. Explain necessary technical terms when first used.
- Use practical examples when they help the reader understand.
- Assume the reader is a beginner unless context says otherwise. Be respectful; do not talk down to them.
- Preserve meaning, accuracy and necessary detail as you simplify.

Keep the tone natural. These are writing principles, not a claim of formal ASD-STE100 compliance.
<!-- wstack:end shared/plain-english.md -->

## Routing

<!-- wstack:begin shared/routing.md -->
Agent-agnostic: any agent that reads `SKILL.md` (Codex `$wstack`, Cursor/Claude `/wstack`) routes the same way. Members are sibling skills; install the suite to use them. Subagents are optional everywhere; without them, run the steps in sequence.

For every explanation, answer to a question and general conversation, always apply the built-in Plain English guidance alongside any selected workflow. No separate installation or explicit invocation is needed. The suite also includes `plain-english` for direct use.

Route workflows by keyword:
- `plain-english` | `plain english` → apply built-in Plain English; `plain-english` is also usable directly.
- `restate` → `wstack-restate`.
- `setup` → `wstack-setup`; "this project" = current repo unless specified.
- `brand` → `wstack-brand` (also usable directly, without project setup).
- `implement` → `wstack-implement`.
- `code-review` → `wstack-code-review`.
- `debug` → `wstack-debug`.
- `verify` | `features` → `wstack-verify-create` (feature map: existing project or from scratch).
- `maintain` → `wstack-verify-maintain`.
- Unmatched explanation/conversation → apply built-in Plain English; no implied setup. Other unmatched requests → disclose + continue authorized work. Unreadable workflow member → report path/block workflow; Plain English guidance remains built in when its standalone skill is unavailable.

## Web UI support

For implementing, debugging or refining compatible React browser UI, read and apply the separately installed `shadcn` skill alongside the selected workflow when the target uses shadcn, is new UI under the preferred stack, or the user authorizes adoption. Explicit `shadcn` requests use this support when compatible; explanations alone do not imply setup or component installation.

- Inspect the target's actual components, framework, styling and package manager first. `components.json` is a useful signal, not proof of compatibility. React DOM web surfaces and compatible Electron renderers qualify; React Native and shared Expo/React Native Web primitives keep their native workflow. Do not replace shared routes or add a web adapter merely to use shadcn.
- Preserve existing components, approved brand guidance and design tokens. Read an existing `DESIGN.md` when relevant without creating a second design authority. Keep `wstack-brand` as the identity/guide workflow; authored HTML/CSS/JS guides do not need conversion to React. Use the selected design workflow for composition and hierarchy.
- Apply shadcn within the requested scope. A design or brand document does not authorize a new app scaffold, framework migration, `add --all`, preset replacement or overwriting local component changes. Reuse installed components, fetch current component docs, use the project's package runner and inspect CLI previews/diffs before updates.
- If `shadcn` is unavailable, report the separate install command: `npx skills add https://github.com/shadcn-ui/ui --skill shadcn`. Continue the authorized workflow with current official docs and existing components; do not claim the skill was applied. Installing the Wstack suite does not install this external skill.
<!-- wstack:end shared/routing.md -->
