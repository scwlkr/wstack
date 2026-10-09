# Setup reconciliation

Setup creates or refreshes project instructions and the standard CLI, then the agent reconciles one canonical map so current project behavior can be browsed immediately. Tooling readiness does not establish product verification.

## Sub-features

- `setup-map` requires source/command/UI/doc discovery and one usable canonical map; missing or ambiguous maps block setup completion.
- `setup-standard` executes help, identity, scoped doctor, list, show, check and view; advertised but unusable commands fail.
- `setup-preserve` upgrades unchanged generated assets while preserving custom routes, arguments, exit status, cancellation, recipes and evidence.
- `setup-provenance` records supplying Git revision and template dirty state beside generated-file hashes; older metadata remains readable and missing provenance stays unknown.
- `setup-repeat` requires a second apply with no changes, including unchanged owner file contents and mtimes.

## How to get to it (user POV)

- Invoke `wstack setup` for a fresh or existing project.
- Use `python3 skills/wstack-setup/scripts/setup.py inspect|apply|check ROOT` for the scripted portion; discovery/map reconciliation belongs to the agent workflow.
- Browse the reconciled map through the generated `./project features:view` command, or its visible `wstack:` alias on collision.

## Driving it with disposable CLI projects

Preconditions:

- Cargo, Rust, Git and Python are available; `WSTACK_BIN` points to this candidate's actual built Wstack binary.
- Use disposable projects with the existing feature-format fixture; no product services or browser opening.

- Run `./project ci`. Its setup suite invokes public setup scripts and real project CLIs for fresh setup, refresh, old metadata, unknown provenance, broken capabilities, collision routes and preserved legacy implementations.
- The map-reconciliation fixture models the agent-authored map phase. Assert checked features and generated HTML, changed-file reports, owner bytes/mtimes, custom arguments/exit status, and unchanged second apply.
- Missing maps and unusable standard commands must return nonzero readiness; repair/reconcile and repeat before accepting setup.

## Proof

- Retain local CI stdout/stderr with candidate/base and command. Disposable-project tests assert observable CLI JSON, generated HTML, file readback and repeat-run identity.
- Record source areas examined and discovery limits in the target map/index. Scripted map usability cannot establish complete discovery or passing product evidence.
- Viewer checks generate with `--no-open`, inspect the requested HTML and remove only the run-owned temporary output.

## Gotchas

- Setup's agent workflow creates/reconciles maps; the write script does not infer product acceptance from source or invent live proof.
- Preserved custom implementations missing standard adapters remain incomplete until repaired in place.
- Supplying template revision can differ from the target project's revision; installed skills without their supplying checkout report unknown provenance.
