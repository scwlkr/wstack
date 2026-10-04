# wstack

A small suite of agent skills for concise, verified, maintainable work, plus a Rust CLI that keeps the suite consistent.

The skills use the standard `SKILL.md` folder layout and install with the [skills.sh](https://skills.sh) CLI.

## Skills

| Skill | Use it for |
| --- | --- |
| `wstack` | Built-in principles, technical stack and routing to the members below. |
| `wstack-implement` | Implement a spec or issue; verify, review and deliver. |
| `wstack-code-review` | Review a diff against repository standards and its spec, reported as two separate axes. |
| `wstack-debug` | Reproduce a bug, trace its root cause, make the smallest fix, add a regression test that fails before and passes after. |
| `wstack-setup` | Embed the preferred stack in a project's agent instructions, scaffold a Rust `./project` CLI and record adoption gaps. |
| `wstack-restate` | Restate your goals and the underlying problem. |

`wstack` holds the shared rules. Install it together with any member you use, because members defer to it by name.

## Install

Install every skill:

```sh
npx skills add scwlkr/wstack
```

Install a subset (include `wstack`):

```sh
npx skills add scwlkr/wstack --skill wstack --skill wstack-debug
```

Install the CLI from a checkout (Rust 1.85 or newer):

```sh
cargo install --path cli
```

## How the repo is organized

```text
skills/<name>/SKILL.md   # one folder per skill, each installable on its own
shared/                  # canonical rules: principles.md, technical-stack.md, smells.md
shared/sync.json         # which skill folders carry a copy of which shared file
cli/                     # the wstack binary (Rust)
tests/                   # Rust tests and fixture repositories
```

### Why skills carry copies of shared files

Installers copy one skill folder at a time, so a link such as `../../shared/principles.md` would break after install. Each rule is written once in `shared/`. Skills that need a rule hold an identical copy (for example `skills/wstack/references/technical-stack.md`), and every skill links only to files inside its own folder.

- Edit `shared/` only, then run `wstack sync` to refresh the copies.
- `wstack check` fails if a copy drifts from its source or if a skill link leaves its folder.
- The core `wstack/SKILL.md` embeds `shared/principles.md` inline between `<!-- wstack:begin shared/principles.md -->` and `<!-- wstack:end shared/principles.md -->` markers, so the principles load with the skill itself. `wstack sync` rewrites only the lines between the markers; `shared/sync.json` marks such a target as `{ "inline": "<file>" }`.
- Members refer to the core by name (`wstack`) instead of by path.

## The `wstack` CLI

```sh
wstack check          # lint the repository; exit 1 on any problem
wstack check --json   # same result as JSON for scripts and CI
wstack list           # skill names and descriptions (--json available)
wstack sync           # refresh vendored copies and inline blocks from shared/
```

By default the CLI looks upward from the current directory for a folder containing `skills/`. Use `--root PATH` to point elsewhere.

`wstack check` verifies, and reports each problem as `file:line`:

- every skill has `SKILL.md` frontmatter with a valid `name` (matching its folder) and a `description`
- every relative markdown link resolves, and skill links stay inside their skill folder
- every skill is listed in the table above, and in `skills.sh.json`
- files under `skills/` and `shared/` are at most 300 lines
- vendored copies and inline blocks match `shared/`

No network access or model calls are involved.

### Develop

```sh
cd cli
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo run -- check
```

`wstack-setup` ships its own Python tests: `python3 -m unittest discover -s skills/wstack-setup/tests`.

## License

MIT, see [LICENSE](LICENSE). `wstack-implement` and `wstack-code-review` are adapted from [Matt Pocock's skills](https://github.com/mattpocock/skills) (MIT); see [NOTICE](NOTICE).
