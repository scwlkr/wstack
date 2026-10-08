---
name: wstack-verify-create
description: "Discover existing project features, reconcile their canonical map and complete real verification; write acceptance first for projects being built. Use for wstack verify or features."
---
Apply `wstack` skill shared rules; don't reroute. Say which mode: code exists → **existing**; not built → **scratch**. Format: [feature-format](references/feature-format.md). Worked map: [example](references/feature-map-example/README.md). Section rules: [skill-sections](references/skill-sections.md).
Operational adoption: [readiness checklist](references/adoption-readiness.md).

<!-- wstack:begin shared/operational-delivery.md -->
## Operational completion and maintenance

- Project-wide `verify` or `maintain` first discovers current user-facing behavior from source, routes, commands, UI and docs, then compares it with the canonical map. The agent finds features; do not ask the user to enumerate them. Group entry points by observable outcome. Existing pilot coverage, map exclusions and recent churn alone do not establish the requested scope; honor explicit owner scope and activation gates.
- Record examined source areas, discovered features and unresolved coverage in the existing map/index or task notes, without a second catalog. Missing, unexamined, blocked or undriven in-scope behavior prevents project-wide completion even when every old mapped case passes. A bounded audit may pass only its explicitly requested scope; name that limit.
- Implementation and verification creation start with project help, identity, scoped doctor and the canonical feature map. Identify affected behavior and entry points; maintain the necessary CLI controls, map claims and recipes together. A missing required adapter is unfinished implementation.
- Real acceptance comes from the actual CLI, HTTP or browser surface and independent expectations/readback. Record exact candidate/base, actual executable/instance, commands, raw observations, covered/uncovered cases and owned cleanup. Validate retained artifacts after teardown; configuration and readiness alone cannot pass.
- Declare machine-checkable case/entrypoint pairs in the feature's Proof section when using `wstack evidence check --root <project> --base <base> <report>`. This checks receipt consistency against the current clean revision and canonical map, not application semantics. The live harness remains responsible for acceptance assertions.
- Qualified Browser receipts retain candidate asset and executable digests, browser version/PID/profile, a distinct owned loopback preview, actual actions, DOM/screenshots and explicit teardown observations. Capture failing owned pages before closure; outer-page artifacts cannot substitute. Timestamp failure/capture separately; later readiness cannot qualify a failed deadline.
- Cleanup requires owned descendant/group absence and actual library close completion; main-process exit may leave captured stdio open. Libraries must not race the owning harness's signal cleanup. Bounded escalation requires fresh owned-group verification. Unresolved close/cleanup or retention/finalization failure cannot leave a passing receipt.
- Process ownership binds the PID to its captured start/birth identity, command/profile and live owned lineage. Recycled numeric PIDs/groups never inherit ownership; unknown or ambiguous instances cannot support cleanup or escalation claims. Retain identity precision and observation gaps honestly.
- Serialize automation sharing a foreground desktop lane. For foreground-dependent native pointer actions, finish asynchronous target lookup before the fresh focus/event baseline immediately preceding dispatch. Verify trusted target delivery and resulting state.
- Reviews pin the actual final candidate/base and read its applicable check and live-proof receipts. Evidence for an earlier candidate is a gap; edits, rebases and landing require fresh applicable verification. Separate Standards from Spec findings.
- Verification creation and maintenance source-read every discovered in-scope implemented feature and drive each at least once per complete audit. A first-feature proof qualifies the harness, not the project's coverage. Record exact case/entrypoint coverage and any broader rotation; planned, skipped, dirty, interrupted, failed and blocked paths never become passed coverage.
- Classify from independent evidence: working approved behavior with stale text is recipe drift; working behavior that cannot be driven is a control gap; behavior contradicting approved acceptance is a product regression. An unavailable dependency is a prerequisite block. Preserve acceptance unless its owner authorized a contract change.
- The maintenance member repairs its verification directory's map, docs and owned harness. Missing verification uses `wstack-verify-create`; required CLI/harness controls outside that directory use `wstack-implement` as bounded phases of the same authorized task, followed by the audit. Continue routine tooling work without asking the user to choose features or run another prompt. Honor read-only requests and explicit scope; unavailable authority/prerequisites remain concrete blocks. Product regressions preserve acceptance and go to the product owner with reproduction/evidence. Keep shared-methodology changes separately reviewed; add no maintainer without demonstrated recurring need.
- Re-drive every repair on the actual surface and final clean candidate before delivery. Clean audits retain logs without a branch/PR; changed audits follow repository delivery and report the remaining merge gate; blocked audits retain partial coverage, attempted routes and the exact missing action.
- Supported scheduled runners are configured only after a complete manual audit. Inspect an actual runner execution; clean runs stay quiet, while meaningful corrections, failures and required actions merit notification. Record execution host and availability limits.
<!-- wstack:end shared/operational-delivery.md -->

<!-- wstack:begin shared/verification-creation.md -->
Write for the next agent, reading cold mid-task. Reuse or create `.agents/skills/verify-<project>/` (Codex path; `.cursor/skills/` or `.claude/skills/` if the project uses those) with `SKILL.md` and one canonical `features/` map. Its frontmatter uses `name: verify-<project>` and a description naming the app, surface and when to use it. Preserve existing project-specific instructions and harnesses.

**Existing**
1. Discover the requested project's features yourself; ask only what source and current instructions cannot answer. Inspect application routes, commands, UI/menus, docs and their implementation, including surfaces absent from the map. Group entry points into user-facing outcomes; recent changes supplement this inventory. Explicit owner scope wins; inherited pilot limits do not silently narrow a project-wide request.
   - Run: documented dev command, ports, env, seed data, auth and activation gates.
   - Drive: existing harness first (Playwright/Cypress, expect, PTY helpers, curl-able routes), else browser/CDP, tmux/PTY or plain HTTP.
   - Observe: screenshots, transcripts, bodies, logs, exit codes, stored state. Isolate side-by-side instances and read-only shared dependencies.
2. Record examined source areas and discovered/blocked/unexamined behavior in the existing index or task notes. Map every discovered in-scope implemented outcome; missing proof does not make implemented behavior `planned`. Keep out-of-scope areas explicit, with the owner instruction or activation constraint; a local map's omission alone is not an exemption.
3. Base must build and start; fix authorized tooling or report the precise prerequisite first. Stub assets are scaffolding and removed in Cleanup. Missing CLI or harness controls use a bounded `wstack-implement` phase of this task; preserve product behavior and return here for real proof.
4. Write or update `SKILL.md` with Launch, Doctor, Drive, Evidence, Cleanup per the skill-sections reference. Add acceptance and required observation pairs to feature files using the existing canonical index. Preserve owner acceptance when an actual product regression blocks proof.

**Scratch**
1. Interview the user: audience, core flows, done state per flow, surface, constraints. Propose, don't quiz; ask only blockers.
2. Write feature files first: `Sub-features` = acceptance, `Proof` = observable result, `Driving it with` = intended harness, `Gotchas` = known traps or `None known yet.`; mark `Status: planned`. In `SKILL.md`, mark Launch/Doctor/Drive `TODO until built`.
3. `wstack-implement` builds against these files; `wstack-code-review` checks the diff against them. After the build, rerun **existing**, fill the sections, prove each feature live and drop `Status: planned`.

**Completion**
- `wstack features check --root <project>` passes (no CLI: apply the format by hand); this is structural validation only.
- Launch, doctor, drive every discovered in-scope implemented feature, retain independent acceptance evidence, clean up and read the artifacts afterward. A first-feature pilot is useful progress, not project-wide completion. Known unmapped, unexamined, blocked or unverified behavior stays incomplete with exact next actions; scratch mode remains unproven until implemented and driven.
- Re-drive repaired tooling on the final clean candidate, run applicable checks and follow repository delivery. Report discovery coverage separately from live coverage. Offer `wstack maintain` for ongoing reconciliation; configure cadence only if asked.
<!-- wstack:end shared/verification-creation.md -->

Adapted from [pstack's create-verification-skill](https://github.com/cursor/plugins/tree/main/pstack/skills/create-verification-skill) by Poteto; [MIT license](LICENSE).
