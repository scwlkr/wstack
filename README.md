# wstack

A small suite of agent skills for concise, verified, maintainable work, plus a Rust CLI that keeps the suite consistent.

The skills use the standard `SKILL.md` folder layout and install with the [skills.sh](https://skills.sh) CLI.

## Skills

| Skill | Use it for |
| --- | --- |
| `wstack` | Principles, [eyes-and-hands](shared/eyes-and-hands.md), technical stack, member routing. |
| `wstack-implement` | Implement a spec or issue; verify, review and deliver. |
| `wstack-code-review` | Review a diff against repository standards and its spec, reported as two separate axes. |
| `wstack-debug` | Reproduce a bug, trace its root cause, make the smallest fix, add a regression test that fails before and passes after. |
| `wstack-verify-create` | Create a project verification skill and feature map, from an existing project or from scratch as a spec. |
| `wstack-verify-maintain` | Audit that verification skill: every feature read from source and driven live, at most one PR of proven corrections. |
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
wstack features check # validate a feature map (--root: project or map dir; --json)
```

By default the CLI looks upward from the current directory for a folder containing `skills/`. Use `--root PATH` to point elsewhere.

`wstack check` verifies, and reports each problem as `file:line`:

- every skill has `SKILL.md` frontmatter with a valid `name` (matching its folder) and a `description`
- every relative markdown link resolves, and skill links stay inside their skill folder
- every skill is listed in the table above, and in `skills.sh.json`
- files under `skills/` and `shared/` are at most 300 lines
- vendored copies and inline blocks match `shared/`

No network access or model calls are involved.

### Operational pilot

From a checkout, `./project` runs this Rust CLI offline with the locked dependencies:

```sh
./project --help
./project info --json --base origin/main
./project doctor --json
./project features list --json
./project features show feature-map --json
./project features check
./project verify feature-map --base origin/main
./project ci
```

Help and capabilities derive from the command definitions. Feature discovery reads the [canonical pilot map](.agents/skills/verify-wstack/features/README.md), with no second catalog. The [verification skill](.agents/skills/verify-wstack/SKILL.md) explains readiness, real CLI fixture proof and cleanup. Reports and stdout/stderr survive teardown in ignored `.evidence/` directories and bind to the candidate/base revisions. Pilot readiness covers this CLI recipe; service and browser readiness remain separate.

`check` retains its existing suite lint behavior. `ci` runs the required local Rust and setup gates. Existing `wstack` commands and flags remain supported.

### Adopting the operational home

The setup skill generates a dependency-free Rust CLI with `info --json`, scoped `doctor --json`, and `features:list`, `features:show`, `features:check`. Configure `WSTACK_BIN` when the installed wstack executable is outside PATH. Command definitions own dispatch, help and capabilities; existing route-name collisions give the builtin a visible `wstack:` prefix while preserving the owner's command.

Create one project verification skill and canonical map, then reuse the project's real app and harness routes for readiness and verification. Setup reports `app_ready: null`; it does not invent a live feature or passing proof. The [shared operational contract](shared/operational-home.md) describes identity, evidence and owned cleanup.

Setup upgrades unchanged generated assets and preserves custom main implementations, routes and owner files. A preserved legacy CLI missing operational discovery remains blocked with a concrete adapter handoff. Reconcile it in a bounded implementation task and prove its custom commands still work. Repeated setup must be idempotent. The setup suite exercises fresh JSON discovery, actual wstack map checking, native app forwarding, command collisions and legacy upgrade preservation.

`wstack features check` finds `verify-*/features` under `.agents/skills`, `.cursor/skills` and `.claude/skills` (or takes a map directory) and verifies:

- `README.md` lists every feature file once, with no dead entries or orphan files
- every feature file has an H1 and the required H2s in order (`Proof` then `Gotchas` last), each with content (format: [shared/feature-format.md](shared/feature-format.md))

### Verification skills

Type `wstack verify` (or `features`) to build a project-local verification skill plus feature map; `wstack maintain` audits it. Both work in any agent that reads `SKILL.md`, with or without subagents. They are modeled on [pstack](https://github.com/cursor/plugins/tree/main/pstack) by Poteto; see [NOTICE](NOTICE).

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

MIT, see [LICENSE](LICENSE). `wstack-implement` and `wstack-code-review` are adapted from [Matt Pocock's skills](https://github.com/mattpocock/skills) (MIT). `wstack-verify-create` and `wstack-verify-maintain` are adapted from [pstack](https://github.com/cursor/plugins/tree/main/pstack) by Poteto (MIT). See [NOTICE](NOTICE).
