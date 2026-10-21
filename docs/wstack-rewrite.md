# Rewrite decisions

Historical record of the earlier lightweight-mode rewrite. The
[October 21 SDLC decisions](sdlc-design.md) and canonical `shared/` guidance
supersede this record where they differ, including Code Review and Maintain.

- Audience → Shane; Codex first, Cursor/Claude also.
- Activation → `$wstack` enables Wstack mode; principles apply; requested task routes to needed capabilities.
- Mode lifetime → active for rest of chat until switched off.
- Routing → infer capabilities from request; explicit commands override inference.
- Background → quiet execution favored; implementation explicitly uses proposer + critic subagents.
- Failure modes → ceremony, late tool preparation, perfection before shipping.
- Priority → ship features consistently; efficient checks.
- Editing → delete outcome-neutral text; shorten words; grammar optional.
- Scope → all skills, principles, tools, helpers, formats and workflows challenged.
- Compatibility → heavy structural changes allowed.
- Interview → settle decisions before rewriting skill behavior.
- Environment Foresight → anticipate tools/runtime/access; acquire or request up front.
- Tool acquisition → reuse available tools; acquire missing essentials automatically; involve Shane for unavailable access/payment/consequential shared changes.
- Credentials → prefer 1Password; obtain persistent references for task-long reuse.
- Ship → push + merge by default; no human PR review requests.
- Human review → before release; local build/screenshots/simple preview.
- Merge triggers release → review before merge.
- Relentless Shipping → make owner-blocked review unmistakable; seek attention through available channels; minute-level reminders preferred to silent waiting when unresponsive.
- Code review → tiny internal common-sense helper invoked by Wstack; no elaborate reports; separate from prototype critic.
- Implement → prepare environment → proposer subagent creates 2 prototypes + preference → different subagent argues for nonpreferred option → main chooses → implements → quick relevant check → affected feature description → push + merge.
- Prototype intensity → match change intensity; 1–3 paragraph plans through interactive working UI.
- Mechanical change without meaningful choice → skip prototype/critic loop.
- Critic → bolster nonpreferred option; main receives competing opinions.
- Missing subagents → main produces options + critique; no shipping block.
- Prototype cleanup → delete discarded experiments unless useful; keep chosen result + consequential reasoning.
- File size → ≤300 lines strong default; split by responsibility; exceptions when splitting worsens design.
- Debug → cheap reproduction or convincing diagnostics → fix → quick affected check; automated regression when useful, not a universal gate.
- Diagnostics → trace cause; cheapest useful diagnostic/experiment; avoid speculative fixes; reconsider when failed attempts stop yielding information.
- State → represent valid states directly; domain types when useful; no mandatory modeling phase/diagrams.
- Boundaries → validate external input; separate responsibilities where useful; clear mutable-state ownership; interfaces/locks when needed.
- Retry safety → make likely-to-repeat mutations safe to repeat; no universal recovery machinery for one-off actions.
- Automation → script repeated/error-prone work when it saves total effort; no cheap one-off scripting detour.
- Restate → retain useful callable built-in action; no dedicated installed skill needed.
- Explanations → concise, context-aware; explain unfamiliar terms when needed; remove beginner default.
- ASD-STE100 → retain recommendation as style guidance; interpreting user's “AST” as current ASD-STE100 reference.
- Compression scope → agent instructions may sacrifice grammar; public/brand writing preserves intended voice/readability.
- Unslop → retain concrete guidance steering AI writing/design away from slop; applies to creation as well as editing.
- UI check → responsive UI: desktop + half-desktop + phone; native-only: actual supported device sizes.
- Brand → small criteria-based skill; individual SVG assets + type/color/voice/mockup sections; remove mandatory catalog/download/prompt-JSON machinery.
- SVG assets → real editable vectors, independently usable files; no raster wrappers.
- Simplicity (Poteto inference) → simplest maintainable solution within requirement; smallest necessary change to reach it; authorized rewrite may require larger diff.
- Simplicity sources → [Laziness Protocol](https://github.com/cursor/plugins/blob/1dcc2bacde73ddd284803b7af28164d0436198fc/pstack/skills/principle-laziness-protocol/SKILL.md#L10), [Minimize Reader Load](https://github.com/cursor/plugins/blob/1dcc2bacde73ddd284803b7af28164d0436198fc/pstack/skills/principle-minimize-reader-load/SKILL.md#L9).
- Stack → retain copyable preferences; add OpenTUI for TUI.
- Backend → PostgreSQL + Rust/Axum + OpenAPI + Zod; needed layers only; generate/reuse one contract source where practical.
- Web-only → React + TypeScript + Tailwind + shadcn.
- Shared web/mobile → Expo/Expo Router + React Native + NativeWind.
- Desktop → Electron/Forge for shared web/desktop; native when OS integration makes it practical.
- Remaining tools → pnpm JS/TS, Cargo Rust, Docker/Compose services, OpenTUI TUI; needed only; preserve existing managers/lockfiles.
- Checks → affected work + required repo gates; rerun only when changes invalidate results; no automatic post-merge rerun.
- Failed checks → fix caused failures; report unrelated/flaky failures; continue where repo requirements permit; actively resolve blocked required gates.
- Verify → small requested-behavior checking capability; feature documentation separate; no framework construction by default.
- Maintain → remove separate exhaustive upkeep capability; normal work updates docs; verify handles requested checks.
- Conflicts → brief A/B paths; ask Shane to choose; continue independent work.
- Setup contents → `w.md`, short `AGENTS.md` pointer, basic CLI help, feature/terminology pointers; no inventory/install/CI overhaul/migration backlog.
- Setup refresh ownership → update small Wstack-managed portion; preserve handwritten notes, custom CLI commands and unrelated files; no hash/provenance framework.
- Git work → isolate concurrent code changes; commit useful increments; preserve unrelated work; no universal worktree/intermediate-commit ritual.
- Clean Workspace → end every task by merging promptly and tidying Git/worktrees/branches/disposable resources, including task simulators.
- Cleanup scope → own resources + obvious stale merged leftovers encountered; preserve active/unmerged/shared/useful work; no routine whole-machine hunt.
- Review previews → keep needed preview running through review; clean afterward.
- Evidence → keep useful screenshots/diagnostics/decisions; delete disposable evidence aggressively; regeneration cheap, excess retention worse; tools handle process bookkeeping.
- Scope growth → fix what materially serves requested change; defer unrelated cleanup; large changes allowed when they serve task.
- CLI → retain basic project CLI; agent adds useful controls rather than prescribed infrastructure.
- CLI baseline → `./project --help` + useful project commands; no mandatory identity/doctor/map viewer/receipts.
- CLI language → Rust highly favored; alternatives when impractical.
- CLI preference → fast/scriptable/repeatable/token-efficient actions; practical MCP/native/computer-use fallbacks; no wrapper detour as a blocker.
- CLI controls → add seed/reset/inspect/drive commands when they save useful work; use existing tools otherwise; no automatic adapter-building obligation.
- Bookkeeping → optional; serve agents and let Shane inspect progress.
- Feature maps → shared product vocabulary; Shane reviews/edits named behaviors, controls and qualities.
- Map editing → ordinary files in Cursor/VS Code; no dedicated editor required.
- Map shape → one Feature Map index listing all features; shared names + 1–2 sentence descriptions; feature-specific files when useful.
- Map location → root `FEATURES.md`, linked from `w.md`; sections for multiple apps.
- Feature identity → recognizable names; rename descriptions/references together; aliases only when useful; no permanent opaque IDs.
- Feature state → describe current features; planned additions labeled when useful; remove deleted features, history remains in Git; no verification-status bookkeeping.
- Map details → infer needed detail from task; no required proof/harness/receipt/gotcha sections.
- Map growth → add/update during feature work; full discovery separate; no startup-wide inventory.
- Full discovery → use code/docs/UI as needed to identify and describe all features; operating/checking every feature is a separate task, not a documentation prerequisite.
- Terminology → compact word source of truth; learn selectively from AI Hero dictionary.
- Vocabulary authority → compact common Wstack dictionary + project product terminology; external dictionaries inform definitions.
- Feature → intentional, distinct user-visible aspect of software; bugs belong to affected feature.
- Escalation → chat/Slack/desktop/notifications/other configured owner channels; repeat every minute until response, then follow owner's timing; continue independent work.
- Wstack Setup → automatic background startup on `wstack`; ≤60 seconds; detect previous setup with a simple persistent marker/file.
- Setup experience → quiet, short, feels like magic; no visible ceremony. Necessary task prerequisites do not expand setup scaffolding.
- Partial setup (inferred default; Q73 unclear) → retain useful partial work; current version only after success; retry later; continue where possible, surface actual task blockers.
- Setup refresh → version check; older setup reruns; retain hard ≤60-second ceiling.
- Project home → `w.md` holds setup version + project notes/pointers; `AGENTS.md` carries a short pointer; common policy maintained once.
- Project record → commit `w.md`; commands, feature/glossary pointers, decisions and reusable credential references; secret values stay in 1Password.
- Existing CLI → reuse working CLI; Rust favored for new CLI; migrate only when useful.
- Stack exceptions → prefer standard tools; choose practical alternatives without waiting; explain briefly.
- Interview adjustment → core direction aligned; simplify questions; infer technical details from settled preferences; ask only crucial unresolved choices Shane would notice.
- Confirmation → Q86: only `$wstack`; shared understanding confirmed; rewrite authorized.

## Inferred implementation choices

These follow settled preferences; not additional owner requirements.

- Readability → clear names/direct flow; group related changes; remove unused indirection. Drop named smell taxonomy and generic design lectures.
- Review → inspect actual change against request; fix consequential defects; skip cosmetic noise; rerun only when fixes invalidate prior checks.
- Checks → report actual coverage/gaps honestly; no machine-receipt requirement or blanket second-readback ritual.
- Writing → Plain English throughout Wstack conversation; preserve accuracy/voice; examples only when useful; compress concrete unslop patterns, no banned-word checklist.
- Design → content/task/identity drive hierarchy; purposeful grouping/decoration/motion; recognizable accessible controls; realistic affected content/states; no blanket aesthetic bans.
- Brand → simple local guide + independent SVG assets; HTML default, reuse existing medium when useful; requested scope only; no app-framework migration or catalog infrastructure.
- Stack → defaults for new work; preserve existing working stack; platform-compatible tooling only; consult current docs when relevant, no compatibility qualification program.
- Delegation → useful independent work; scoped context/ownership; main integrates; no recursive review/prototype ritual.
- Recurring work → only when requested; use available scheduler; requested scope; notify meaningful changes/actions, no mandatory exhaustive manual audit first.
- Helpers → remove obsolete receipts/map schema/viewer/brand catalog/download/prompt/upkeep code; keep setup and suite maintenance only where needed.
- Adoption → existing projects adapt on next Wstack invocation; old managed instructions become a pointer; preserve owner commands/notes/assets; repair obsolete generated CLI routes when needed; no portfolio-wide migration.
- Distribution → standard skill metadata/local links; retain required attribution/licenses for reused material; delete decorative examples/history/metadata that change no outcome.

## Challenge tree

Each branch includes its source rules, duplicates, examples and helper contracts.
No branch closes through an umbrella answer alone.

| Branch | State |
| --- | --- |
| Audience, editing, compatibility | Settled |
| Tool preparation, installation, fallbacks | Up-front preparation/reuse/automatic essentials/acquisition exceptions settled |
| Shipping, checks, review, delivery | Settled; remaining details inferred above |
| Architecture principles, preferred technologies | Settled; remaining details inferred above |
| Project CLI, observation and control | Help/useful commands/Rust/CLI preference/practical fallbacks/reuse/task-justified controls settled |
| Setup, generated files, provenance, upgrades | Automatic/background/≤60s/version refresh/minimal contents/preservation/tracked w.md settled; partial-setup behavior inferred |
| Tracker, plans, scope and autonomy | Settled; optional useful tracking, A/B conflict choice, task-bound scope |
| Feature maps, acceptance, discovery, schema | Definition/dictionary/root index/recognizable names/optional detail/current-vs-planned/no status bookkeeping/discovery separate from checks settled; conflicts use owner A/B choice |
| Verification creation, maintenance, schedules | Settled; schedules only requested, implementation inferred above |
| Receipts, lifecycle, cleanup, ownership | Clean Workspace scope/aggressive disposable-evidence deletion/tool-owned process bookkeeping/review-preview retention settled; tool implementation open |
| Core, routing, members, installation, sync | Settled; only `$wstack`, small internal guides |
| Implement, debug, review, restate | Settled; remaining details inferred above |
| Writing, design, brand | Settled; remaining details inferred above |
| Catalogs, metadata, examples, licenses | Settled; obsolete machinery removed, essential format/licenses retained |
