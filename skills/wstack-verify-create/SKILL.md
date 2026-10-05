---
name: wstack-verify-create
description: "Create a project verification skill with a feature map, in any agent. Existing project: interview the repo, write launch/doctor/drive/evidence/cleanup and one file per feature, prove one end to end. From scratch: interview the user and write feature files with acceptance proof first, as the spec to build against. Use for wstack verify, wstack features, or when a project has no scripted way to prove behavior."
---
Apply `wstack` skill shared rules; don't reroute. Say which mode: code exists → **existing**; not built → **scratch**. Format: [feature-format](references/feature-format.md). Worked map: [example](references/feature-map-example/README.md). Section rules: [skill-sections](references/skill-sections.md).

Write for the next agent, reading cold mid-task. Output: `.agents/skills/verify-<project>/` (Codex path; `.cursor/skills/` or `.claude/skills/` if the project uses those) holding `SKILL.md` (`name: verify-<project>`; `description` names the app, surface, when to use) and `features/`.

**Existing**
1. Interview the repo; ask the user only what code can't answer.
   - Surface: what users touch (web, CLI/TUI, API, desktop, library); primary first.
   - Run: documented dev command, ports, env, seed data, auth.
   - Drive: existing harness first (Playwright/Cypress, expect, PTY helpers, curl-able routes), else browser/CDP, tmux/PTY or plain HTTP.
   - Observe: screenshots, transcripts, bodies, logs, exit codes, stored state. Isolate: side-by-side instances?
2. Base must build and start; fix or report precisely first. Stub assets are marked scaffolding and removed in Cleanup.
3. Write `SKILL.md` with Launch, Doctor, Drive, Evidence, Cleanup per [skill-sections](references/skill-sections.md).
4. Inventory every discovered feature from routes, commands, menus and docs: `features/README.md` + one file each. Work in bounded batches when needed; name unexamined areas and partial CLI coverage. Apply the built-in `eyes-and-hands` contract to prerequisites, observation, actions, proof and reset.

**Scratch**
1. Interview the user: audience, core flows, done state per flow, surface, constraints. Propose, don't quiz; ask only blockers.
2. Write feature files first: `Sub-features` = acceptance statements, `Proof` = observable result, `Driving it with` = intended harness, `Gotchas` = known traps or `None known yet.`; mark `Status: planned`. In `SKILL.md`, mark Launch/Doctor/Drive `TODO until built`.
3. These files are the spec: `wstack-implement` builds against them, `wstack-code-review` checks the diff against them. After the build, rerun **existing**, fill the sections, verify each Driving section live, drop `Status: planned`.

**Both**
- `wstack features check --root <project>` passes (no CLI: apply the format by hand).
- Prove once: launch, doctor, drive ONE feature, capture evidence, clean up, confirm evidence survived. Rerun cleanup after each failed try. Unproven output is a draft; scratch mode has nothing to drive, so report it unproven.
- Offer `wstack maintain` to keep the map honest; cadence only if asked.

Adapted from [pstack's create-verification-skill](https://github.com/cursor/plugins/tree/main/pstack/skills/create-verification-skill) by Poteto; [MIT license](LICENSE).
