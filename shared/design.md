# Design router

Choose guidance for the task, surface and platform. Business/domain design favors
DDD; interface design can use the skills below. Respect the brief, existing
project identity, approved tokens/components and useful working behavior.

- Button/small refinement → no external skill or lightweight guidance when sufficient.
- Page/screen redesign → choose a suitable comprehensive skill; load the relevant playbook, not every available skill.
- [Impeccable](https://github.com/pbakaus/impeccable), commonly `impeccable` → broad creation, shaping, refinement, critique and audit; includes native guidance as well as web.
- [Taste Skill](https://github.com/leonxlnx/taste-skill), commonly `design-taste-frontend` → strong art direction and browser-based iteration for websites and web app screens.
- Both create and improve interfaces; avoid a fixed create-versus-polish split. Discover installed names/capabilities; add other suitable preferred skills as the list grows.
- Meaningful competing approaches may select different skills. Keep alternatives + deliberation; do not force one named skill onto each approach or combine conflicting full workflows blindly.
- Missing or unsuitable external skill → use relevant existing guidance/tools, including [design quality](unslop-design.md), without an installation detour that delays useful work.
- Show meaningful alternatives + recommendation while proceeding; visual comparisons when useful. Honor explicit requests for Shane to choose first.

Skill tooling follows project/platform/browser rules. Reuse an existing browser
for automation; disposable project tests use bundled Chromium or Chrome for
Testing, not the normal installed Chrome or its profile. Await graceful browser
closure in `finally`; allow exit before forced termination. Preserve normal
user browsers/profiles and clean disposable sessions after success or failure.
