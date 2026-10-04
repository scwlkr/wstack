# Complete a task

Done lets a user mark a task complete and see it leave the open list.

## Sub-features

- `done-mark` marks a task by ID and prints a confirmation.
- `done-hide` removes the task from the default list but keeps it under `--all`.
- `done-missing` reports an unknown ID without changing state.

## How to get to it (user POV)

- Run `tick done <id>`.

## Driving it with shell

Preconditions:

- Seed tasks exist as IDs 1 and 2 in a fresh `TICK_HOME`.

- **Mark done.** Run `tick done 1`. Exit code 0 and stdout is `done 1: Buy milk`.
- **Hidden by default.** Run `tick list`. Output has `Write report` and not `Buy milk`.
- **Kept under all.** Run `tick list --all`. Output shows `Buy milk` marked `[x]`.
- **Unknown ID.** Run `tick done 99`. Exit code 1, stderr contains `no task 99`, and `tick list --all` is unchanged.

## Proof

- Save transcripts to `artifacts/done/transcript.txt`.
- Read back with `tick list --all --json > artifacts/done/list.json`; task 1 has `"done": true` and task 2 `"done": false`.

## Gotchas

- `tick done` on an already-done task exits 0; assert `tick list --all` state, not the exit code.
- Marked tasks stay in the store, so reseed `TICK_HOME` between runs.
