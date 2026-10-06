---
name: wstack-verify-maintain
description: "Audit a project's verification skill and feature map: read every feature from source, drive every feature live once, and ship at most one PR of proven corrections. Works with or without subagents. Use for wstack maintain or an audit of the verify skill."
---
Apply `wstack` skill shared rules; don't reroute. Format: [feature-format](references/feature-format.md).

<!-- wstack:begin shared/operational-delivery.md -->
## Operational completion and maintenance

- Implementation and verification creation start with project help, identity, scoped doctor and the canonical feature map. Identify affected behavior and entry points; maintain the necessary CLI controls, map claims and recipes together. A missing required adapter is unfinished implementation.
- Real acceptance comes from the actual CLI, HTTP or browser surface and independent expectations/readback. Record exact candidate/base, actual executable/instance, commands, raw observations, covered/uncovered cases and owned cleanup. Validate retained artifacts after teardown; configuration and readiness alone cannot pass.
- Declare machine-checkable case/entrypoint pairs in the feature's Proof section when using `wstack evidence check --root <project> --base <base> <report>`. This checks receipt consistency against the current clean revision and canonical map, not application semantics. The live harness remains responsible for acceptance assertions.
- Reviews pin the actual final candidate/base and read its applicable check and live-proof receipts. Evidence for an earlier candidate is a gap; edits, rebases and landing require fresh applicable verification. Separate Standards from Spec findings.
- Verification maintenance source-reads every mapped implemented feature and drives each at least once per complete audit. Record exact case/entrypoint coverage and any broader rotation; planned, skipped, dirty, interrupted, failed and blocked paths never become passed coverage.
- Classify from independent evidence: working approved behavior with stale text is recipe drift; working behavior that cannot be driven is a control gap; behavior contradicting approved acceptance is a product regression. An unavailable dependency is a prerequisite block. Preserve acceptance unless its owner authorized a contract change.
- Maintenance repairs only its verification directory's map, docs and owned harness. A gap in `tools/project-cli/` goes to a bounded tooling implementation task; a product regression goes to the product owner with reproduction/evidence. Keep shared-methodology changes separately reviewed. Existing implementation workflows own these tasks; add no maintainer without demonstrated recurring need.
- Re-drive every repair on the actual surface and final clean candidate before delivery. Clean audits retain logs without a branch/PR; changed audits follow repository delivery and report the remaining merge gate; blocked audits retain partial coverage, attempted routes and the exact missing action.
- Supported scheduled runners are configured only after a complete manual audit. Inspect an actual runner execution; clean runs stay quiet, while meaningful corrections, failures and required actions merit notification. Record execution host and availability limits.
<!-- wstack:end shared/operational-delivery.md -->

Unit of rigor is the feature, not every sentence. Outcome, say which:
- **clean:** every feature got source and live coverage; nothing to ship; no branch, no PR.
- **changed:** one PR of proven doc, harness or map corrections.
- **blocked:** coverage or a safe ship failed; name exactly what.

<!-- wstack:begin shared/verification-ownership.md -->
Edit scope: only the verify skill's own dir (`SKILL.md`, `features/`, harness scripts it owns). Never edit product code or a CLI outside that directory. Fix stale text only after independent evidence confirms approved behavior still works; a product regression preserves acceptance and goes to a product task. Route control gaps outside this directory to a bounded tooling implementation task. An authorized contract change may update the map; a failing app alone does not authorize redefining success.
<!-- wstack:end shared/verification-ownership.md -->

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
