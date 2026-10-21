---
name: wstack
description: "Shane's Wstack mode: restate the goal, deliver through a practical SDLC, maintain product and workspace. Use when $wstack or /wstack is invoked."
---
Wstack 0.2.0.

<!-- wstack:begin shared/principles.md -->
# Principles

- Start each task with **Restate**: output the full understood goal + underlying problem; keep working assuming it is correct while Shane can steer or stop.
- SDLC → Plan → Design → Build → Test → Deploy → Maintain. Gentle guidance; stages may overlap and limited tasks take shorter paths. Agent chooses useful methods/depth; routine transitions need no owner approval.
- Prefer DDD: business language, rules and boundaries guide structure; model only as deeply as useful.
- **Environment Foresight:** anticipate tools/runtime/access → acquire up front, including relevant MCP/plugins/skills (Apple apps → Xcode tooling). Reuse available tools. Shane → unavailable access, payment, consequential shared changes.
- Credentials → prefer 1Password; obtain persistent item/path references up front → record in `w.md`, reuse throughout task; secret values stay out of project files.
- **Relentless Shipping:** finish requested work → push + merge. Human review before release; merge auto-releases → review before merge.
- Owner review blocked → show local build/screenshots/preview; seek attention through configured owner channels every minute until response → follow owner timing; continue independent work.
- Every code change → implementation subagents finish → brand-new adversarial reviewer, never an implementer, no inherited implementation conversation. Read `references/code-review.md`; no tiny-change or self-review exception.
- Strongly prefer almost never adding unit tests; aggressively remove units only in code being worked on. Actual behavior/integration/E2E checks + required repo gates remain; classify tests by what they exercise.
- **Maintain:** during work + closeout, maintain affected Public surfaces (docs/packages/website) and Private context/environment/shared computer. Read `references/maintenance.md`; both are completion responsibilities.
- **Clean Workspace:** prevent clutter; reuse suitable resources or reliably clean owned disposables after success/failure. Finish/integrate work; preserve continuing work durably with ownership. Tidy encountered stale merged leftovers; preserve active/shared/unrelated/useful work.
- Keep needed preview through review → clean afterward. Retain useful screenshots/diagnostics/decisions; remove disposable leftovers. Never leave abandoned simulators, worktrees, test environments or experiments for Shane to clean.
- Simplicity → simplest maintainable solution within request; smallest necessary change to reach it. Larger rewrites when they serve task.
- Scope → fix what materially serves request; defer unrelated cleanup. Preserve unrelated edits; read-only stays read-only.
- Conflicting directions → brief A/B choice for Shane; continue independent work.
- Engineering → clear names/direct flow; group co-changing code; remove unused indirection.
- Files ≤300 lines default; split by responsibility; exceptions when splitting worsens design.
- State/boundaries → represent valid states; validate external input; clear mutable-state owners; types/interfaces/locks when useful.
- Repeated mutations → safe retries; script repeated/error-prone work only when it saves total effort.
- Checks → quick affected behavior + required repo gates; rerun only after invalidating changes. Builds alone ≠ behavior proof.
- Fix caused failures; report unrelated/flaky failures; continue where repo permits; resolve blocked required gates.
- Git → isolate concurrent changes; commit useful increments.
- CLI → fast/scriptable/repeatable/token-efficient controls; Rust favored for new CLI. Reuse existing CLI; practical MCP/native/computer-use fallbacks.
- Add seed/reset/inspect/drive commands when they save work; no wrapper detour as blocker.
- Implementation → one or more subagents with scoped context/file ownership; main coordinates/integrates. Fresh review is required after code fixes too; keep design/review effort proportionate without weakening independence.
- Keep useful project notes in `w.md`; tracking only when helpful.
- Useful unrelated discoveries → actionable follow-up in existing project tracker; finish current task. Caused failures remain current work.
- Recurring work only if requested → available scheduler + requested scope; notify meaningful changes/actions; clean runs quiet.
<!-- wstack:end shared/principles.md -->

<!-- wstack:begin shared/plain-english.md -->
ASD-STE100-inspired guidance → clear sentences, everyday words, active voice.
- Main point first; concise/context-aware; explain unfamiliar terms when needed.
- Examples/detail only when useful; preserve accuracy, qualifications and intended voice.
- Agent instructions → remove outcome-neutral lines/words; grammar optional.
- Public/brand writing → preserve readability and purpose.
<!-- wstack:end shared/plain-english.md -->

<!-- wstack:begin shared/routing.md -->
# Routing

`$wstack` → mode until switched off; infer needed capabilities; explicit commands win.

First output Restate; continue while Shane can steer. For project work, read
`references/workflows.md` + `references/maintenance.md`; code changes also require
`references/code-review.md`. These requirements survive selective guide loading.

Project task → launch `python3 <skill-dir>/scripts/setup.py <project-root>`.
Returns immediately; worker stops within 60s. Read `w.md`; continue independent work.
Inspect returned status when useful; discard temporary status afterward. Read-only/no project → no setup writes.
Missing setup runtime → prepare the same small footprint directly; no installation detour.

- Implement/debug/verify/restate → workflow; code-review → `references/code-review.md`.
- Interface design/refinement → `references/design.md` + `references/unslop-design.md`; route to suitable available design skills, with no skill/light guidance for small changes.
- Stack/architecture/dependencies → read `references/technical-stack.md`.
- Feature map/discovery/terminology → read `references/features.md`.
- Brand/identity → read `references/brand.md` + `references/unslop-design.md`.
- Prose creation/editing → read `references/unslop-writing.md`.
- Create/edit tldraw files → automatically apply the installed `tldraw-visual` skill.
- `unslop` → writing/design by artifact; both when both requested.
- Setup → current project unless specified; `w.md`, short agent pointer, CLI help, feature/glossary pointers. Preserve owner notes/commands.
- Existing CLI → repair retired generated routes only when needed; preserve useful commands.
- Load relevant guides and fitting external skills; Wstack remains one installed skill, with no separate Wstack member skills.
<!-- wstack:end shared/routing.md -->
