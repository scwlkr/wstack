# Tick verification map

Maintained source for verifying the user-facing behavior of `tick`, a todo CLI. Read this index, then use the matching feature file as the recipe.

## Baseline

- Build once with `cargo build`; run the binary as `./target/debug/tick`.
- Set `TICK_HOME=/tmp/tick-verify-$RUN_ID` so runs never share state.
- Seed with `tick add "Buy milk"` and `tick add "Write report"`.
- Doctor: `tick --version` prints the built version and `tick list` exits 0 against the disposable home.
- Never drive an instance this run did not start.

## Conventions

- Start every recipe from the baseline state unless its preconditions say otherwise.
- Treat commands as literal; keep quoted titles and flags unchanged.
- Capture command, stdout, stderr and exit code under `artifacts/<feature>/`.
- Restore seeded data after a mutation. Cleanup never removes `artifacts/`.

## Features

- [Add a task](./add.md) covers adding by argument and from stdin, and rejecting empty titles.
- [Complete a task](./done.md) covers marking a task done and the listed result.
