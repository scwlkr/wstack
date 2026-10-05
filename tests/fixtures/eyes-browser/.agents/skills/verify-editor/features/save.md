# Save a draft

A user edits the title, saves it and sees the persisted title on reopening.

## Sub-features

- `save-title` saves the entered title and reopens the persisted result.
- `reject-empty` displays validation feedback and preserves the saved draft.

## How to get to it (user POV)

- Open the editor, enter a title and activate Save.

## Driving it with Playwright CLI

Preconditions: an installed Playwright module and Chromium, a disposable checkout, and a writable evidence directory supplied by the audit.

- `node driver.cjs setup` seeds the run-owned draft.
- `node driver.cjs observe` reads the real DOM, layout, console and DevTools network events.
- `node driver.cjs act TITLE` types through the browser, saves and exercises invalid input.
- `node driver.cjs assert TITLE` reopens the UI in a fresh browser and reads the persisted file independently.
- `node driver.cjs cleanup` removes only the owned state.

## Proof

Inspect the command logs, DevTools network statuses, UI screenshots and persisted-file readback. The title must match in the fresh UI and on disk. Qualification records the build and environment in the audit receipt.

## Gotchas

Do not share state between runs. Screenshots and logs live outside the state directory and must survive cleanup.
