# wstack

Shane's agent mode. Prepare early → build → quick check → push + merge → clean workspace.

| Skill | Use |
| --- | --- |
| `wstack` | Implement, debug, check, restate, feature maps, writing, design and brand. |

## Install

```sh
bash -o pipefail -c 'curl -fsSL https://raw.githubusercontent.com/scwlkr/wstack/main/install.py | python3 -'
```

On macOS/Linux, this installs the latest Wstack globally for Codex, Cursor and Claude Code.
Requires `curl` and Python 3. Run the same command to update or replace an older version.
Known retired Wstack skills are cleaned up after installation succeeds; unrelated skills survive.
Shared names such as `plain-english` are removed only when their source is Wstack.
Existing project files are preserved; project setup refreshes on the next Wstack invocation.
The installer honors `CODEX_HOME`, `CLAUDE_CONFIG_DIR` and `XDG_STATE_HOME`.

Use `$wstack` in Codex; `/wstack` in Cursor/Claude. Mode lasts until switched off.
Only Wstack installs. Supporting guides load as needed.

Project setup runs quietly in the background, stops within 60 seconds, and skips current versions.
`w.md` holds version/project pointers; `FEATURES.md` holds feature names + short descriptions.
Existing commands and handwritten notes survive updates. New project CLIs favor Rust.
Old generated Wstack instruction blocks become a short pointer; retired CLI routes adapt when needed.

Push + merge by default. Human review before release; local build/screenshots preferred.
If merging releases publicly, review before merge. Owner blockers get active reminders.

## Develop

Rules live in `shared/`; edit there, then `./project sync`.
Copies stay inside `skills/wstack/` so one-folder installs work.

```sh
./project --help
./project check
./project ci
cargo install --path cli
```

`check` validates metadata, links and shared copies; length suggestions don't block.
`ci` runs required Rust checks + focused setup/installer checks. The installed skill needs Python 3 for setup;
its setup does no network installs or compilation. Project CLI compilation happens when used.

[Feature Map](FEATURES.md) · [Terminology](GLOSSARY.md) · [Rewrite decisions](docs/wstack-rewrite.md)
