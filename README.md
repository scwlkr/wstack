# wstack

Shane's agent mode. Restate → Plan → Design → Build → Test → Deploy → Maintain.
Guidance scales to the task; stages can overlap while the agent keeps working and Shane can steer.

| Skill | Use |
| --- | --- |
| `wstack` | SDLC guidance, implementation, fresh Code Review, Public and Private Maintenance, feature maps, writing, design and brand. |

## Workflow

| Stage | Guidance |
| --- | --- |
| Plan | Output the full understood goal and underlying problem through Restate, then continue without an acknowledgement wait. |
| Design | Prefer DDD for business concepts. Explore meaningful alternatives, deliberate and show a recommendation. Choose design guidance that fits the job. |
| Build | Implementation subagents finish the code; the main agent coordinates. Coding, running and checking can overlap. |
| Test | Check actual behavior and required repository gates. Every code change gets a new adversarial reviewer with no inherited implementation conversation. |
| Deploy | Push and merge by default. Human review before release, or before merge when merge publishes. Approved live-site changes include deployment and a live check. Favor fixing forward. |
| Maintain | Keep affected public surfaces and private project context current; verify actual release packages and maintain the shared computer workspace. |

Strongly prefer almost never adding unit tests and aggressively remove units within the code being worked on.
Integration, end-to-end and actual workflow checks remain available; the framework name does not determine the category.

Code Review is required for tiny changes and review-driven fixes too. The reviewer is never an implementer
and starts with fresh context, using `fork_turns: "none"` or its equivalent. It receives the request,
requirements, project rules and actual code, without the implementer's conversation or self-assessment.

Design routing starts with [Impeccable](https://github.com/pbakaus/impeccable) and
[Taste Skill](https://github.com/leonxlnx/taste-skill), with room for more. A button change may need no
skill or light guidance; a redesign may need a comprehensive skill. Competing approaches may use different skills.

Maintain runs throughout work and at closeout. Public documentation, websites and packages affected by decisions
stay aligned with what ships. Private docs, planning, agent context and the computer are maintained too.
Prefer suitable reusable resources; clean owned disposable simulators, worktrees, tests and experiments after
success or failure. Preserve active/shared/useful work, useful evidence and previews still needed for review.
Useful unrelated discoveries become follow-ups in the existing tracker; recurring work happens only when requested.

The [SDLC reference](docs/sdlc-design.md) records the agreed responsibilities and completion conditions.
These are agent instructions; the maintenance CLI checks package consistency rather than enforcing an agent's actions.

## Install

Windows PowerShell or Command Prompt (Python 3 with the `py` launcher):

```powershell
py -3 -X utf8 -c "import urllib.request; exec(urllib.request.urlopen('https://raw.githubusercontent.com/scwlkr/wstack/main/install.py', timeout=30).read().decode('utf-8'))"
```

macOS/Linux (`curl` and Python 3):

```sh
bash -o pipefail -c 'curl -fsSL https://raw.githubusercontent.com/scwlkr/wstack/main/install.py | python3 -'
```

Both commands install the latest Wstack globally for Codex, Cursor and Claude Code.
Run the same command to update or replace an older version.
Windows copies the skill when symbolic links are unavailable; administrator access is not required.
Known retired Wstack skills are cleaned up after installation succeeds; unrelated skills survive.
Shared names such as `plain-english` are removed only when their source is Wstack.
The installer updates skill files; project setup refreshes on the next Wstack invocation.
The installer honors `CODEX_HOME`, `CLAUDE_CONFIG_DIR` and `XDG_STATE_HOME`.

Use `$wstack` in Codex; `/wstack` in Cursor/Claude. Mode lasts until switched off.
Only Wstack installs. The short core carries preferences and firm requirements.
Workflow, review, maintenance and design details load only for a concrete need;
ordinary project/code/UI work does not automatically load a reference bundle.

Project setup runs quietly in the background, stops within 60 seconds, and skips current versions.
`w.md` holds version/project pointers; `FEATURES.md` holds feature names + short descriptions.
Setup reuses existing commands and keeps notes outside managed blocks. New project CLIs favor Rust.
Old generated Wstack instruction blocks become a short pointer; retired CLI routes adapt when needed.
The [starting audit](docs/sdlc-current-state.md#project-setup) records current setup limitations,
including customized content inside managed blocks and native Windows project setup.

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

[Feature Map](FEATURES.md) · [Terminology](GLOSSARY.md) · [SDLC](docs/sdlc-design.md) · [Earlier rewrite decisions](docs/wstack-rewrite.md)
