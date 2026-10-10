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
| `wstack-setup` | Embed the preferred stack in a project's agent instructions, reconcile a Rust `./project` CLI and canonical feature map, and record adoption gaps. |
| `wstack-restate` | Restate your goals and the underlying problem. |
| `wstack-brand` | Create or refine a local brand guide, editable assets, searchable catalog and prompt style JSON. Usable directly. |

`wstack` holds the shared rules. Install it with members that defer to it by name.
`wstack-brand` can also be installed and used on its own.

For explanations, questions and general conversation, Wstack also uses the
standalone `plain-english` skill from [scwlkr/skillsies](https://github.com/scwlkr/skillsies).
Call it directly as `$plain-english` or through `$wstack plain-english`.
Install it separately with `npx skills add scwlkr/skillsies --skill plain-english`.

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
./project features view
./project features view --no-open --output /tmp/features.html
./project verify feature-map --base origin/main
./project verify feature-view --base origin/main
./project evidence check <report.json> --base origin/main --json
./project ci
```

Help and capabilities derive from the command definitions. Feature discovery reads the [canonical pilot map](.agents/skills/verify-wstack/features/README.md), with no second catalog. The [verification skill](.agents/skills/verify-wstack/SKILL.md) explains readiness, real CLI fixture proof and cleanup. Reports and stdout/stderr survive teardown in ignored `.evidence/` directories and bind to the candidate/base revisions. Pilot readiness covers this CLI recipe; service and browser readiness remain separate.

`check` retains its existing suite lint behavior. `ci` runs the required local Rust and setup gates. Existing `wstack` commands and flags remain supported.

`features view` opens a standalone HTML snapshot of the current map: one compact row per feature, description, acceptance goals, mapped status and an icon button that copies the full context and source path. Search filters rows; long cells scroll. No framework, server, network assets or new dependencies. The UI shell is under 4 KiB; map text adds to the file size. Markdown stays literal text. Status means implemented/planned, not passing evidence. Each run generates a new temporary file; `--no-open` prints its path, and `--output PATH` selects a new file (existing files are never overwritten). Regenerate after changing the map.

Receipt checking reads required case/entrypoint pairs from the canonical feature's Proof section. It rejects stale or dirty revision claims, nonpassing/missing observations, incomplete cleanup and unreadable/outside artifact paths. CLI, HTTP and Browser receipt shapes follow the proven pilots; Browser includes owned preview/browser identities, asset/executable digests, actions, retained DOM/screenshots and explicit teardown. Project harnesses still own behavior assertions and runtime qualification. Other surfaces need a qualified adapter. A structurally valid older map without required declarations needs adoption before receipt checking can pass.

A Browser receipt may include required `HTTP` coverage groups alongside strict `Browser` observations. Each HTTP group retains nonempty metadata artifacts and a nonempty `http_observations` array of `{action: {method, path}, http_status, raw_body}` readbacks. Every body path must be an owned readable retained file; empty bodies are allowed for responses such as HEAD and redirects. Browser identity, cleanup, DOM and screenshot requirements still apply. Standalone HTTP and CLI receipt shapes are unchanged.

Implementation, verification creation, review and maintenance share [operational completion rules](shared/operational-delivery.md). Maintenance discovers current project features before trusting the map. Missing verification or CLI controls use the existing creation/implementation workflows as bounded phases of the same task; product regressions preserve acceptance for their owner. Scheduled upkeep follows a complete manual audit and an observed runner execution.

### Adopting the operational home

The setup skill generates a dependency-free Rust CLI with `info --json`, scoped `doctor --json`, and `features:list`, `features:show`, `features:check`, `features:view`. Configure `WSTACK_BIN` when the installed wstack executable is outside PATH. Command definitions own dispatch, help and capabilities; existing route-name collisions give the builtin a visible `wstack:` prefix while preserving the owner's command.

Setup discovers source, commands, UI and docs to create or refresh one project verification skill and canonical map, then reuses the project's real app and harness routes for readiness and verification. Setup reports `app_ready: null`; it does not invent a live feature or passing proof. The [shared operational contract](shared/operational-home.md) describes identity, evidence and owned cleanup.
The [adoption readiness checklist](shared/adoption-readiness.md) applies the qualified CLI, service and browser lessons to each additional project. Other portfolio repositories still need individual adoption and real proof.

Setup upgrades unchanged generated assets and preserves custom main implementations, routes and owner files. A preserved legacy CLI missing operational discovery remains blocked with a concrete adapter handoff. Repair preserved adapters during the same setup pass and prove its custom commands still work. Setup checks actual help, identity, scoped doctor, feature list/show/check and a generated viewer without opening a browser. Missing or falsely advertised commands and absent/ambiguous maps block completion. Repeat apply must report no changes. Metadata records the supplying Wstack Git revision and template dirty state alongside generated-file hashes; installed templates without their supplying checkout report unknown provenance. The disposable setup suite exercises fresh and refreshed maps, native app forwarding, command collisions, legacy preservation and repeat-run idempotence.

`wstack features check` finds `verify-*/features` under `.agents/skills`, `.cursor/skills` and `.claude/skills` (or takes a map directory) and verifies:

- `README.md` lists every feature file once, with no dead entries or orphan files
- every feature file has an H1 and the required H2s in order (`Proof` then `Gotchas` last), each with content (format: [shared/feature-format.md](shared/feature-format.md))

### Verification skills

Type `wstack verify` (or `features`) to build a project-local verification skill plus feature map; `wstack maintain` audits it. Both work in any agent that reads `SKILL.md`, with or without subagents. They are modeled on [pstack](https://github.com/cursor/plugins/tree/main/pstack) by Poteto; see [NOTICE](NOTICE).

The agent discovers features from current source, routes, commands, UI and docs, then reconciles the existing map. CLI feature discovery reads that map; structural checks and a one-feature pilot do not establish project-wide coverage. A project-wide request stays incomplete while known features are unmapped, unexamined, blocked or unverified. Explicit owner scope and activation limits still apply.

### Develop

```sh
cd cli
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo run -- check
```

`wstack-setup` ships its own Python tests: `python3 -m unittest discover -s skills/wstack-setup/tests`.

### Local brand resources

Use `$wstack brand` or `$wstack-brand` to inspect and refine a project's identity.
The [brand skill](skills/wstack-brand/SKILL.md) keeps authored guidance, editable
assets, references and licenses in the existing brand folder. Its small Python
helper refreshes derived resources while the model authors an expressive, editable
HTML/CSS/JS guide. Existing approved guides remain the main experience. Asset
discovery reuses files and optional existing manifests; the helper does not design
or automatically vectorize artwork.

```sh
wstack brand refresh --root /path/to/project
wstack brand list --root /path/to/project --query favicon --json
wstack brand style --root /path/to/project
```

Use `--brand-dir docs/identity` for an existing layout. No suite setup is required.
Select ordinary authored HTML with `brand.json`'s `guide`; open the returned path
locally. Optional [integrations](skills/wstack-brand/references/integration.md)
provide search, enlarged previews, lazy individual/bundle downloads and exact
prompt copying without global CSS or a fixed layout. Current assets lead;
legacy/reference/support files remain discoverable through All resources.

Refresh preserves authored files, rejects customized generated resources and
leaves unchanged contents/mtimes alone. Existing generated guides stay supported;
adaptation is deliberate. See the [format reference](skills/wstack-brand/references/format.md)
for ownership, the existing manifest adapter and the compact prompt source.
The installed Rust binary embeds the same helper/resources and needs Python 3.
`./project verify brand --base origin/main` runs the reusable local browser recipe
with existing Node.js, Playwright Chromium/WebKit and sharp (`NODE_PATH` supported).
Retained matched screenshots, clipboard/download checks, edit/refresh observations
and original-icon comparisons complement lifecycle/setup tests in `./project ci`.

## License

MIT, see [LICENSE](LICENSE). `wstack-implement` and `wstack-code-review` are adapted from [Matt Pocock's skills](https://github.com/mattpocock/skills) (MIT). `wstack-verify-create` and `wstack-verify-maintain` are adapted from [pstack](https://github.com/cursor/plugins/tree/main/pstack) by Poteto (MIT). See [NOTICE](NOTICE).
