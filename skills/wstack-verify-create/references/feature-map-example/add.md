# Add a task

Add lets a user create a task from the command line and see it in the list.

## Sub-features

- `add-arg` adds a task from an argument and prints its ID.
- `add-stdin` adds a task from piped input.
- `add-empty` rejects an empty title without changing the list.

## How to get to it (user POV)

- Run `tick add <title>`.
- Pipe a title: `echo <title> | tick add -`.

## Driving it with shell

Preconditions:

- `TICK_HOME` points at a fresh disposable directory and the seed tasks exist.

- **Add by argument.** Run `tick add "Plan trip"`. Exit code 0 and stdout is `added 3: Plan trip`.
- **Add from stdin.** Run `echo "Pay rent" | tick add -`. Exit code 0 and stdout is `added 4: Pay rent`.
- **Reject empty.** Run `tick add ""`. Exit code 2, stderr contains `title required`, and `tick list` still shows 4 tasks.

## Proof

- Save the three transcripts to `artifacts/add/transcript.txt`.
- Read back with `tick list --json > artifacts/add/list.json`; it holds `Plan trip` and `Pay rent`, not an empty title.

## Gotchas

- Titles are trimmed on save. Assert the listed title, not the typed input.
- IDs continue from the highest ID; do not assume they restart at 1 after cleanup.
