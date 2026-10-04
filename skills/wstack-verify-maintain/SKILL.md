---
name: wstack-verify-maintain
description: "Audit a project's verification skill and feature map: read every feature from source, drive every feature live once, and ship at most one PR of proven corrections. Works with or without subagents. Use for wstack maintain or an audit of the verify skill."
---
Apply `wstack` skill shared rules; don't reroute. Format: [feature-format](references/feature-format.md).

Unit of rigor is the feature, not every sentence. Outcome, say which:
- **clean:** every feature got source and live coverage; nothing to ship; no branch, no PR.
- **changed:** one PR of proven doc, harness or map corrections.
- **blocked:** coverage or a safe ship failed; name exactly what.

Edit scope: only the verify skill's own dir (`SKILL.md`, `features/`, harness scripts it owns). Never edit product code. Map says X, app no longer does X: doc drift → fix the map; product regression → report it, don't paper over it.

1. Locate: project-local `verify-*` skill with Launch/Drive and `features/` (`.agents/skills/`, else `.cursor/skills/`, `.claude/skills/`). Several → ask; none → stop, point to `wstack-verify-create`.
2. Index: `wstack features check --root <project>`; fix missing, extra, duplicate or dead entries and missing sections.
3. Source wave: per feature file, one read-only reader answers "how does this user-facing feature work?" from source. Return: summary / source entry points / likely drift with citations, or none / one live recipe. Parallel subagents when available and authorized, else sequential in this session. Readers never drive the app or edit files.
4. Reconcile: every feature has a summary; spot-check cited drift; merge recipes into few app states. Sweep recent churn for user-facing surfaces missing from the map (needs a concrete source path).
5. Live pass, required even when source looks clean. You own all driving, following the skill's Launch model. Cover every feature once. Invariants:
   - doctor before the first drive, after any failed drive, per fresh session; reset or relaunch when doctor can't see a wedged state
   - evidence survives every cleanup; check its location
   - nothing a drive started outlives it; clean residue, not a shared instance
   - doctor failure from skill drift: fix, retry once, then `blocked`
   - unreachable feature = `verified-unreachable` only with the concrete prerequisite and route tried; omitted prerequisite is drift
   - re-drive each harness fix live; final teardown after the last drive
6. Triage: wrong or missing user-POV text → fix doc; harness can't drive working behavior → fix harness (executable, invocation documented); broken app → record for the user, outside the PR.
7. Ship or stop: changed → re-read every changed file, one PR. Clean or blocked → no PR; report coverage honestly. Keep run notes (features covered, unreachable, drift, outcome) in scratch, uncommitted.

Adapted from [pstack's maintain-verification-skill](https://github.com/cursor/plugins/tree/main/pstack/skills/maintain-verification-skill) by Poteto; [MIT license](LICENSE).
