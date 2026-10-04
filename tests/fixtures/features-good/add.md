# Add item

Add item lets a user do the thing.

## Sub-features

- `add-run` runs the thing.

## How to get to it (user POV)

- Run `demo add`.

## Driving it with shell

Preconditions:

- `demo doctor` exits 0.

- **Run.** Run `demo add`. Exit code 0.

## Proof

- stdout contains `done` and exit code is 0.

## Gotchas

- Output is plain text; no colors.
