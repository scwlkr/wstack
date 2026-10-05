# Save a draft

Save persists the title; invalid input leaves the draft unchanged.

## Sub-features

- `save-title` persists a title.
- `reject-empty` rejects an empty title without mutation.

## How to get to it (user POV)

- Run `python3 app.py save TITLE`.

## Driving it with Python

Preconditions: run `python3 driver.py setup` in a disposable copy of this project.

- Observe with `python3 driver.py observe`.
- Save and reject invalid input with `python3 driver.py act`.
- Read back with `python3 driver.py assert`.

## Proof

The command transcripts and an independent persisted-file read establish the result.

## Gotchas

Only the probe-owned `.state` directory is removed by cleanup.
