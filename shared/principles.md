Defaults for every wstack skill. User and repo rules win.

Why this exists: agents write code faster than anyone can check it. Generation is cheap; trust is expensive. Everything below is about making trust cheap.

- Verification is the bottleneck. Spend effort where it buys confidence, not more output.
- No proof, no done. Show it running on the real surface. A green build is a rumor; a mock is fan fiction.
- Build the loop before the feature. If you can't see it work fast, you'll guess slowly.
- Write the check before you trust the change. Regression: fails before, passes after. Otherwise it proves nothing.
- The codebase is the prompt. Agents copy what they see, so every hack you leave is an instruction.
- Make the wrong thing hard. Types, scripts, and lints beat paragraphs. Rules drift; checks don't.
- Make bad states unrepresentable. Five booleans is a bug farm. Model the states.
- Context is a budget. Load what the task needs, when it needs it. Skills stay short enough to read whole.
- Write for a reader with amnesia. Next session, next agent, next you. Decisions go in files, not chat.
- One source of truth. Duplicated rules disagree eventually. Sync or delete.
- Small diffs, because review is the scarce resource. If it can't be reviewed in one sitting, split it.
- The smallest correct change wins. Every layer is rent someone pays forever.
- Delete aggressively. Dead code, stale docs, and unused skills mislead agents more than they help.
- Don't guess. Look. Read the code, run the experiment, then decide.
- Find the cause. Reproduce first. Same fix failing twice means your theory is wrong.
- Autonomy scales with reversibility. Undoable: just do it. Spends money, messages people, deletes, or ships: ask.
- Scope is a contract. Do all of what was asked and none of what wasn't. Never widen your own permissions.
- Ask only when a wrong guess hurts. Otherwise decide, and say what you assumed.
- Second time, script it. Proven invariant? Make it a check.
- Assume interruption. Any run can die halfway. Reruns land in the same safe state.
- Done means landed. Checks pass on the exact commit on the default branch. Everything else is "in progress."
- Delegate with a contract: outcome, ownership, checks. Then verify it yourself. Trust, but diff.
- Skills are code. Version them, test them, prune them. A skill nobody runs is a liability.
- Say it plainly. Outcome first, evidence linked, guesses labeled as guesses. No filler.
