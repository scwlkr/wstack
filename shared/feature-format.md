# Feature map format

A feature map is a `features/` folder inside a verification skill (`verify-<project>`): a `README.md` index plus one `<feature>.md` per user-facing feature. `wstack features check` validates it.

## README.md

- `Baseline`: launch URL or command, disposable data dir, seed/reset and supported auth commands, doctor and shared CLI observation commands. Never drive an instance this run did not start.
- `Conventions`: stable handles first (roles, names, prompts, routes), commands literal, restore state after mutations, never delete proof artifacts.
- `Features`: one link per feature file, `- [Name](./file.md) one-line scope`.

## Feature file

H1 title, one paragraph of user-visible behavior, then these H2s in order, `Gotchas` last. Keep implementation details out: name user paths, handles, state, commands and observable results.

1. `Sub-features`: short IDs, one line each. Each line is one acceptance statement (`add-save` persists a title and body).
2. `How to get to it (user POV)`: every entry point a user has (button, key, command, route).
3. `Driving it with <harness>`: `Preconditions:`, then bullets pairing each user action with an exact, parameterized CLI command and observable result. Include state reach/reset and inspection recipes; drive the real boundary. Label unimplemented commands as planned.
4. `Proof`: artifacts that show it works, with paths and the proven build/environment, command and result. Capture the action and the resulting state, plus a second read-only view for mutations (reopen the item, query the store, read the file). State which operations are proven; a smoke test cannot prove the whole feature.
5. `Gotchas`: traps that waste or invalidate a run, including missing or blocked observation/interaction capabilities with the operation, reason, owner and next action. Last section, after `Proof`. If none are known yet, say so in one line (`None known yet.`).

Every section must have content. A planned feature (from-scratch mode, not built yet) carries the line `Status: planned` until its Driving section is verified against the running app, then drops it.

`eyes-and-hands` keeps this coverage current during every wstack session. Inventory every discovered feature; record unexamined areas and partial coverage explicitly. `wstack features check` validates map structure, not live CLI access or proof.
