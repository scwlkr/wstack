---
name: wstack-implement
description: "Implement a requested spec or issue through verified delivery, using Wstack principles and code review."
---
Apply `wstack` skill shared rules; don't reroute.

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

1. Read requested spec/issues + discussion; repository tracker owns execution. Verify-skill feature map (`features/`) present → its `Sub-features`/`Proof` are acceptance; update the map with the change. Acceptance + existing code/work → bounded plan; capture starting SHA + existing changes.
2. TDD where useful, at agreed behavioral seams; typecheck + focused behavior checks as units change; final repository gates.
3. Review task diff with `wstack-code-review`; pass spec + baseline/artifact + check evidence. Fix substantiated in-scope findings → repeat affected checks/review.
4. Commit/deliver per Wstack + repository rules; report acceptance evidence + unresolved gates.

Adapted from [Matt Pocock's implement](https://github.com/mattpocock/skills/blob/d81f3a183412e71a5b1e84ca21bc1a35eea03a60/skills/engineering/implement/SKILL.md); [MIT license](LICENSE).
