---
name: wstack-verify-create
description: "Create a project verification skill with a feature map, in any agent. Existing project: interview the repo, write launch/doctor/drive/evidence/cleanup and one file per feature, prove one end to end. From scratch: interview the user and write feature files with acceptance proof first, as the spec to build against. Use for wstack verify, wstack features, or when a project has no scripted way to prove behavior."
---
Apply `wstack` skill shared rules; don't reroute. Say which mode: code exists → **existing**; not built → **scratch**. Format: [feature-format](references/feature-format.md). Worked map: [example](references/feature-map-example/README.md). Section rules: [skill-sections](references/skill-sections.md).
Operational adoption: [readiness checklist](references/adoption-readiness.md).

<!-- wstack:begin shared/operational-delivery.md -->
## Operational completion and maintenance

- Implementation and verification creation start with project help, identity, scoped doctor and the canonical feature map. Identify affected behavior and entry points; maintain the necessary CLI controls, map claims and recipes together. A missing required adapter is unfinished implementation.
- Real acceptance comes from the actual CLI, HTTP or browser surface and independent expectations/readback. Record exact candidate/base, actual executable/instance, commands, raw observations, covered/uncovered cases and owned cleanup. Validate retained artifacts after teardown; configuration and readiness alone cannot pass.
- Declare machine-checkable case/entrypoint pairs in the feature's Proof section when using `wstack evidence check --root <project> --base <base> <report>`. This checks receipt consistency against the current clean revision and canonical map, not application semantics. The live harness remains responsible for acceptance assertions.
- Qualified Browser receipts retain candidate asset and executable digests, browser version/PID/profile, a distinct owned loopback preview, actual actions, DOM/screenshots and explicit teardown observations. Libraries must not race the owning harness's signal cleanup. Retention/finalization failure cannot leave a passing receipt.
- Reviews pin the actual final candidate/base and read its applicable check and live-proof receipts. Evidence for an earlier candidate is a gap; edits, rebases and landing require fresh applicable verification. Separate Standards from Spec findings.
- Verification maintenance source-reads every mapped implemented feature and drives each at least once per complete audit. Record exact case/entrypoint coverage and any broader rotation; planned, skipped, dirty, interrupted, failed and blocked paths never become passed coverage.
- Classify from independent evidence: working approved behavior with stale text is recipe drift; working behavior that cannot be driven is a control gap; behavior contradicting approved acceptance is a product regression. An unavailable dependency is a prerequisite block. Preserve acceptance unless its owner authorized a contract change.
- Maintenance repairs only its verification directory's map, docs and owned harness. A gap in `tools/project-cli/` goes to a bounded tooling implementation task; a product regression goes to the product owner with reproduction/evidence. Keep shared-methodology changes separately reviewed. Existing implementation workflows own these tasks; add no maintainer without demonstrated recurring need.
- Re-drive every repair on the actual surface and final clean candidate before delivery. Clean audits retain logs without a branch/PR; changed audits follow repository delivery and report the remaining merge gate; blocked audits retain partial coverage, attempted routes and the exact missing action.
- Supported scheduled runners are configured only after a complete manual audit. Inspect an actual runner execution; clean runs stay quiet, while meaningful corrections, failures and required actions merit notification. Record execution host and availability limits.
<!-- wstack:end shared/operational-delivery.md -->

Write for the next agent, reading cold mid-task. Output: `.agents/skills/verify-<project>/` (Codex path; `.cursor/skills/` or `.claude/skills/` if the project uses those) holding `SKILL.md` (`name: verify-<project>`; `description` names the app, surface, when to use) and `features/`.

**Existing**
1. Interview the repo; ask the user only what code can't answer.
   - Surface: what users touch (web, CLI/TUI, API, desktop, library); primary first.
   - Run: documented dev command, ports, env, seed data, auth.
   - Drive: existing harness first (Playwright/Cypress, expect, PTY helpers, curl-able routes), else browser/CDP, tmux/PTY or plain HTTP.
   - Observe: screenshots, transcripts, bodies, logs, exit codes, stored state. Isolate: side-by-side instances?
2. Base must build and start; fix or report precisely first. Stub assets are marked scaffolding and removed in Cleanup.
3. Write `SKILL.md` with Launch, Doctor, Drive, Evidence, Cleanup per [skill-sections](references/skill-sections.md).
4. Map all discovered features from routes/commands/menus/docs: `features/README.md`. Unexamined areas → gaps.

**Scratch**
1. Interview the user: audience, core flows, done state per flow, surface, constraints. Propose, don't quiz; ask only blockers.
2. Write feature files first: `Sub-features` = acceptance statements, `Proof` = observable result, `Driving it with` = intended harness, `Gotchas` = known traps or `None known yet.`; mark `Status: planned`. In `SKILL.md`, mark Launch/Doctor/Drive `TODO until built`.
3. These files are the spec: `wstack-implement` builds against them, `wstack-code-review` checks the diff against them. After the build, rerun **existing**, fill the sections, verify each Driving section live, drop `Status: planned`.

**Both**
- `wstack features check --root <project>` passes (no CLI: apply the format by hand).
- Prove once: launch, doctor, drive ONE feature, capture evidence, clean up, confirm evidence survived. Rerun cleanup after each failed try. Unproven output is a draft; scratch mode has nothing to drive, so report it unproven.
- Offer `wstack maintain` to keep the map honest; cadence only if asked.

Adapted from [pstack's create-verification-skill](https://github.com/cursor/plugins/tree/main/pstack/skills/create-verification-skill) by Poteto; [MIT license](LICENSE).
