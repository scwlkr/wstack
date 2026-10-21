# Preferences and requirements

- Plan → Design → Build → Test → Deploy → Maintain guides work. Stages overlap; choose useful methods/depth without routine approval gates. Prefer DDD: business language, rules and boundaries.
- Meaningful design choices → separate approaches, proposer + different critic, deliberation and visible recommendation. Scale exploration to the task; mechanical changes can skip it.
- Prepare needed tools/runtime/access early; reuse available tools. Prefer 1Password and durable references in `w.md`; never record secret values in project files.
- **Every code change**, including tiny edits and fixes: implementation subagents finish; a brand-new adversarial read-only reviewer checks it. Never an implementer or inherited implementation conversation (`fork_turns: "none"` or equivalent); self-review cannot substitute. Resolve consequential findings; fixes require fresh review too.
- Strongly prefer almost never adding unit tests; aggressively remove units only in code being worked on. Check actual behavior + required repo gates; integration/E2E remain. Classify tests by behavior, not framework.
- **Maintain throughout work and closeout:** Public → affected docs, website and visible surfaces current; verify actual release packages. Private → docs, plans, agent context/environment and shared computer maintained. Reuse suitable resources or clean owned disposable simulators, worktrees, tests and experiments after success/failure. Finish/integrate or preserve continuing work durably with ownership; preserve active/shared/unrelated/useful work, evidence and needed previews. Never leave abandoned resources for Shane.
- Stay scoped; read-only stays read-only. Fix caused failures; report unrelated failures/gaps. Useful unrelated discoveries → existing tracker follow-ups. Recurring work only when requested; notify meaningful changes/actions, clean runs quiet.

## Relentless Shipping

Be ambitious and autonomous toward the agreed goal: make useful decisions, try
promising approaches and fix obstacles without routinely involving Shane.
Better to fall running than fall waiting around.

Push + merge by default; human review before release, before merge if merge
publishes. Reach the requested delivery endpoint; live-site changes include
approved deployment + live check. Favor fixing forward.
