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
