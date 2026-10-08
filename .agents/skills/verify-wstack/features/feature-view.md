# Feature map view

Owners scan the current feature map as a compact table and copy a feature's name, ID, description, acceptance goals, mapped status and source path into a Codex follow-up. Domain terms come from the existing description and goals; there is no second catalog.

## Sub-features

- view-rows: one row per indexed feature shows its identity, description, full goals and implemented/planned status; long cells scroll to keep rows compact.
- view-copy: an accessible icon button copies the complete plain-text feature context, including content outside the visible cell.
- view-search: case-insensitive search filters the current rows without changing the map.
- view-local: one self-contained generated HTML file opens directly in a browser without a server, external assets or new runtime dependencies; its shell stays below 4 KiB.
- view-safe: source Markdown is inert text; invalid maps and existing output files fail without overwriting owner content.
- view-snapshot: each invocation reads the current map and creates a new snapshot; no tracked project files are changed.

## How to get to it (user POV)

Run `./project features view` in Wstack, or `wstack features view --root <project>` for another project. Newly generated project CLIs expose `./project features:view`; command collisions preserve the owner's route and prefix the builtin with `wstack:`.

## Driving it with ./project

Preconditions: one valid canonical map; a default browser for opening. Browser verification additionally needs Node.js and existing Playwright Chromium/WebKit installations (`NODE_PATH` can locate the package).

- Run `./project features view`; the browser opens the generated file and stdout retains its path.
- Run `./project features view --no-open --output <new-file.html>` for automation. Existing output paths, including symlinks, are rejected.
- Run `./project verify feature-view --base <base-ref>`; the real CLI generates fixture and project snapshots, then Chromium/WebKit drive file rendering, search, real clipboard paste, denied-API fallback and manual selection when copying is unavailable.
- Inspect desktop/mobile screenshots, retained HTML, clipboard text and `report.json` after owned browser/fixture cleanup.

## Proof

`.evidence/feature-view-<run>/report.json` binds candidate/base, binary digest, browser versions, commands, file requests, covered cases, artifact readbacks and cleanup. Retained `*-clipboard.txt` and `*-fallback.txt` must match independent fixture expectations. `chromium-1440.png`, `webkit-1440.png` and 390px counterparts show the real generated table. The script checks compact desktop rows, literal hostile Markdown, no external requests and the UI shell size.

Rust integration checks cover fresh source changes, required heading suffixes, planned status, invalid maps, output preservation and opener behavior; setup checks drive forwarding and preserve route collisions. Run `./project ci` for these checks. This browser recipe has its own assertions/report, like brand proof; it is not a generic qualified Browser receipt.

## Gotchas

- This is a snapshot. Rerun the command after editing the map.
- Implemented/planned status comes from the map and does not establish passing verification.
- Clipboard restrictions fall back to native copy; if both methods fail, a focused selected text field exposes the full context for manual copy.
- Markdown formatting remains literal to keep rendering small and safe.
- Linux needs `xdg-open`; Windows uses `explorer`; macOS uses `open`. Browser behavior is qualified here on macOS Chromium/WebKit; other OS openers require their own qualification.
