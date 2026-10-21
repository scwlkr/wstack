# Wstack SDLC current state

Audit date: October 21, 2026. Source: clean `main` at
`c3e3ac671d2df19158ba6390e356f978e1abc1a8`, matching GitHub at inspection.
This records existing policy and observed behavior. It does not introduce new
workflow requirements or resolve the SDLC design questions.

## Reading and verification scope

Read all repository documentation, canonical rules, skill metadata, installer,
setup helpers, CLI implementation, tests and fixtures. Scoped read-only audits
covered Rust enforcement, Python setup/installation, and current GitHub/Linear
state. Compared the complete shared installed skill with the repository package;
all source files were identical. The shared installation resolves through
`~/.agents/skills` to the Skillsies checkout.

Checked live GitHub state and Wstack's Linear project, including relevant current
incidents and historical decisions. This is not an audit of every consumer
project or proof that every agent session followed the skill.

## What the current instructions say

The [README](../README.md) summarizes the workflow as prepare, build, quick check,
push and merge, then clean the workspace. The fuller implementation path in
[shared/workflows.md](../shared/workflows.md) is:

1. Prepare tools, runtimes and access before dependent work.
2. For meaningful implementation choices, a proposer supplies two candidates and
   a preference. A different critic strengthens the other option; the main agent
   chooses. Candidates can be short plans or working artifacts. Mechanical work
   skips this; unavailable subagents have a main-agent fallback.
3. Implement the simplest maintainable solution within the request, preserving
   unrelated work and the project's working stack.
4. Check affected behavior and required repository gates. Inspect the actual
   change against the request for consequential defects, missed requirements and
   unnecessary complexity. Fix caused failures; resolve required blocked gates.
5. Update affected feature descriptions, then push and merge.
6. Obtain human review before release. If merging releases publicly, that review
   comes before merge. Offer a local build, screenshots or a useful preview.
7. Clean owned resources and encountered stale merged leftovers. Preserve
   active/shared/useful work, needed review previews and useful evidence.

[GLOSSARY.md](../GLOSSARY.md) defines **Ship** as push and merge, and **Release**
as publishing a product change for users. This distinction is deliberate and
recorded in [ADR 0001](adr/0001-source-delivery-before-release-review.md).

Other request types have shorter paths:

- Debug: reproduce or obtain convincing diagnostics, trace the cause, fix, check
  affected behavior. Add an automated regression when useful.
- Review: inspect the actual diff against the request. Review-only work returns
  findings; implementation work fixes consequential findings.
- Verify: observe the requested result and relevant side effects through existing
  tools; report actual coverage and gaps. A build alone does not prove behavior.
- Restate: explain the goal and underlying problem concisely.
- Read-only work: preserve that scope; do not run setup writes.

Supporting rules live in [shared/principles.md](../shared/principles.md),
[shared/routing.md](../shared/routing.md) and the feature, stack, writing, design
and brand guides. Useful tracking is optional. There is no universal requirement
for an issue, worktree, test-first development, exhaustive feature discovery,
formal evidence ledger or hosted CI. Recurring work requires a request.

The longer [rewrite record](wstack-rewrite.md) preserves prior decisions and
explicitly labels inferred implementation choices. It is context for the current
rules, not a reason to restore retired member workflows.

## What is implemented and observed

### Instruction delivery

Canonical rules live in `shared/`. The sync manifest copies or inlines them into
the self-contained `skills/wstack/` package. Current package checks pass and the
active shared installation matches that package.

Wstack is an agent skill plus helpers. The agent is responsible for following the
workflow. There is no executable lifecycle controller that tracks or enforces
requirements, options, reviews, owner approval, merging, deployment or cleanup.

### Project setup

[setup.py](../skills/wstack/scripts/setup.py) launches a background worker. On
this Mac, its tested background startup and 60-second alarm work. It writes a
version/pointer block in `w.md`, a pointer in `AGENTS.md`, missing feature/glossary
placeholders, and a starter project CLI if no existing runner is detected.

Runner detection checks file presence or package metadata, not command health.
The generated Rust CLI initially provides help only. It does not supply product
build, verification, shipping or release commands. Existing commands and useful
task controls remain project/agent responsibilities.

A recorded setup version at least `0.2.0` returns `current` before checking for
missing pointers, documents or a runner. Current version is not a health check.
Individual writes are atomic; the overall setup is not transactional. Partial
useful files can remain after failure, and there is no stale-lock recovery.

Two material limitations affect the written preservation/platform promises:

- **Reproduced:** custom instructions inside recognized current setup markers or
  legacy `wstack-setup:start/end` markers are replaced with the pointer and are
  not migrated into `w.md` or backed up. Owner prose outside those blocks remains.
  An isolated `wstack:begin core` block is not recognized and remains intact.
  Three disposable-project reproductions established these boundaries. This
  matches the open preservation concern in
  [WLK-381](https://linear.app/wlkr-labs/issue/WLK-381/preserve-customized-agents-content-during-wstack-setup-migration).
- **Reported, supported by source:** native Windows project setup fails at
  `signal.SIGALRM`; the starter launcher also uses POSIX shell.
  [WLK-240](https://linear.app/wlkr-labs/issue/WLK-240/refresh-revitthyme-for-wstack-02-project-setup)
  remains In Progress. Windows installer simulations do not qualify Windows
  project setup. This audit did not run on Windows or Linux.

### Repository checks

The repository's Rust CLI supplies `check`, `list`, `sync` and `ci`.
`check` validates skill metadata, relative file links, README/catalog membership
and canonical-copy consistency. The 300-line guidance is advisory and scans only
the skill/shared directories. It does not validate the truth of workflow prose,
agent compliance, product behavior, approval or release status.

The repository launcher delegates to Cargo. Its `ci` command runs the four
required Rust gates and Python tests, stopping on failures. Executed separately
for this audit, all components passed:

- `cargo build`
- `cargo test`: 22 passed
- `cargo clippy --all-targets -- -D warnings`
- `cargo run -- check`: one skill, no warnings
- Python unittest discovery: 17 passed, including 13 installer and four setup tests

The additional preservation reproductions were independent of the passing suite.
Passing these checks establishes package/tool behavior within their coverage,
not that every project using Wstack works correctly.

### GitHub and distribution

At inspection, GitHub had no open PRs, tags, GitHub releases, configured Actions
workflows or Actions runs. `main` had no branch protection, required status checks
or rulesets. Local check and review rules therefore depend on the agent/project,
without GitHub enforcement. This does not establish whether owner review happened
outside GitHub. Repository auto-merge being disabled does not prevent an agent
from merging through available tools.

[install.py](../install.py) downloads current GitHub `main` when explicitly run.
It installs/replaces the skill and removes identified retired Wstack skills with
backup/rollback handling. It is not an unattended updater or a signed release
channel. A merge changes what the next install fetches; existing installations
do not automatically update. A missing GitHub Release does not mean a change is
unavailable for installation.

Consumer migration is also separate from updating the skill:
[WLK-241](https://linear.app/wlkr-labs/issue/WLK-241/restore-patribible-verification-compatibility-with-wstack-02)
records a repaired consumer failure after retirement of `wstack features`.

## Questions the SDLC discussion needs to resolve

The current rules are clearest from implementation through source integration.
They do not give equally explicit treatment to intake and acceptance, release
qualification, post-release checks, recovery, or feedback into later work.

Decisions remain open about the intended lifecycle boundary, what counts as
complete for different requests, and how much coordination belongs in prose
versus targeted automation. These are design questions, not findings that every
project needs more process.

Historical context must not silently become new requirements:

- [WLK-86](https://linear.app/wlkr-labs/issue/WLK-86/make-local-first-ci-the-wstack-default)
  chose local-first CI. No hosted Actions is not itself a defect.
- [WLK-200](https://linear.app/wlkr-labs/issue/WLK-200/bound-implement-spec-delivery-stages-and-preflight-expensive)
  was canceled after the owner corrected an agent's interpretation. Its proposed
  delivery-stage customization was reverted and is not current policy.

New terms and decisions should be captured as they are settled. Preserve the
existing Ship/Release distinction unless the owner explicitly changes it. Create
an ADR only for a consequential trade-off that needs lasting explanation.
