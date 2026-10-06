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
