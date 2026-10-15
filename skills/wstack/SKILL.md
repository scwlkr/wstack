---
name: wstack
description: "Shane's Wstack mode: prepare early, ship quickly, concise writing and design. Use when $wstack or /wstack is invoked."
---
Wstack 0.2.0.

<!-- wstack:begin shared/principles.md -->
# Principles

- **Environment Foresight:** anticipate tools/runtime/access → acquire up front, including relevant MCP/plugins/skills (Apple apps → Xcode tooling). Reuse available tools. Shane → unavailable access, payment, consequential shared changes.
- Credentials → prefer 1Password; obtain persistent item/path references up front → record in `w.md`, reuse throughout task; secret values stay out of project files.
- **Relentless Shipping:** finish requested work → push + merge. Human review before release; merge auto-releases → review before merge.
- Owner review blocked → show local build/screenshots/preview; seek attention through configured owner channels every minute until response → follow owner timing; continue independent work.
- **Clean Workspace:** task end → merge promptly; tidy own resources + encountered stale merged leftovers, including task simulators. Preserve active/unmerged/shared/useful work.
- Keep needed preview through review → clean afterward. Retain useful screenshots/diagnostics/decisions; delete disposable evidence aggressively, cheap to regenerate.
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
- Delegate useful independent work with scoped context/file ownership; main integrates. No recursive review/prototype ritual.
- Keep useful project notes in `w.md`; tracking only when helpful.
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

Project task → launch `python3 <skill-dir>/scripts/setup.py <project-root>`.
Returns immediately; worker stops within 60s. Read `w.md`; continue independent work.
Inspect returned status when useful; discard temporary status afterward. Read-only/no project → no setup writes.
Missing setup runtime → prepare the same small footprint directly; no installation detour.

- Implement/debug/code-review/verify/restate → read `references/workflows.md`.
- Stack/architecture/dependencies → read `references/technical-stack.md`.
- Feature map/discovery/terminology → read `references/features.md`.
- Brand/identity → read `references/brand.md` + `references/unslop-design.md`.
- Prose creation/editing → read `references/unslop-writing.md`; UI creation/refinement → read `references/unslop-design.md`, alongside workflow.
- Create/edit tldraw files → automatically apply the installed `tldraw-visual` skill.
- `unslop` → writing/design by artifact; both when both requested.
- Setup → current project unless specified; `w.md`, short agent pointer, CLI help, feature/glossary pointers. Preserve owner notes/commands.
- Existing CLI → repair retired generated routes only when needed; preserve useful commands.
- Load only relevant guides; no separate member skills.
<!-- wstack:end shared/routing.md -->
