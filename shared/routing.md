Agent-agnostic: any agent that reads `SKILL.md` (Codex `$wstack`, Cursor/Claude `/wstack`) routes the same way. Members are sibling skills; install the suite to use them. Subagents are optional everywhere; without them, run the steps in sequence.

For explanations, answers to questions and general conversation, read and apply the separately installed `plain-english` skill. Also route explicit `plain-english` requests to it. Apply its writing style alongside any selected workflow. If unavailable, use short, clear sentences, everyday words, minimal jargon and useful examples; continue the requested task.

Route workflows by keyword:
- `restate` → `wstack-restate`.
- `setup` → `wstack-setup`; "this project" = current repo unless specified.
- `brand` → `wstack-brand` (also usable directly, without project setup).
- `implement` → `wstack-implement`.
- `code-review` → `wstack-code-review`.
- `debug` → `wstack-debug`.
- `verify` | `features` → `wstack-verify-create` (feature map: existing project or from scratch).
- `maintain` → `wstack-verify-maintain`.
- Unmatched explanation/conversation → use `plain-english`; no implied setup. Other unmatched requests → disclose + continue authorized work. Unreadable workflow member → report path/block workflow.

## Web UI support

For implementing, debugging or refining compatible React browser UI, read and apply the separately installed `shadcn` skill alongside the selected workflow when the target uses shadcn, is new UI under the preferred stack, or the user authorizes adoption. Explicit `shadcn` requests use this support when compatible; explanations alone do not imply setup or component installation.

- Inspect the target's actual components, framework, styling and package manager first. `components.json` is a useful signal, not proof of compatibility. React DOM web surfaces and compatible Electron renderers qualify; React Native and shared Expo/React Native Web primitives keep their native workflow. Do not replace shared routes or add a web adapter merely to use shadcn.
- Preserve existing components, approved brand guidance and design tokens. Read an existing `DESIGN.md` when relevant without creating a second design authority. Keep `wstack-brand` as the identity/guide workflow; authored HTML/CSS/JS guides do not need conversion to React. Use the selected design workflow for composition and hierarchy.
- Apply shadcn within the requested scope. A design or brand document does not authorize a new app scaffold, framework migration, `add --all`, preset replacement or overwriting local component changes. Reuse installed components, fetch current component docs, use the project's package runner and inspect CLI previews/diffs before updates.
- If `shadcn` is unavailable, report the separate install command: `npx skills add https://github.com/shadcn-ui/ui --skill shadcn`. Continue the authorized workflow with current official docs and existing components; do not claim the skill was applied. Installing the Wstack suite does not install this external skill.
