# Maintain

Maintain is a first-class responsibility throughout the lifecycle and at closeout.
The task includes affected Public surfaces and the Private project/environment;
the shared computer is also a workspace to maintain. Agent chooses useful methods
and records. Keep maintenance proportional to task impact without leaving cleanup
for Shane or abandoning useful work.

## Public

- Keep affected public docs, feature descriptions, website and other visible product surfaces aligned with decisions and delivered behavior.
- Verify actual release/distribution packages and other applicable artifacts, not just labels or a successful source build. Keep downloads/package descriptions coherent with what ships.
- Public updates retain applicable code-review and human release-review requirements.
- Complete → affected public surfaces are current and applicable actual artifacts checked.

## Private

- Investigate and maintain affected private docs, plans, decisions, project context and agent environment as work changes; keep useful knowledge available for later work.
- Plan resource ownership/lifecycle before creation. Prefer suitable reusable simulators, worktrees, test environments and tools when useful; use disposable isolation when needed with reliable cleanup.
- Reuse a suitable simulator rather than accumulating clones. One global simulator is not required; the agent chooses the appropriate resource.
- Clean owned disposables after success AND failure, including failed tests/experiments. Preserve useful diagnostics, then close/remove temporary resources. Remove abandoned simulators, test leftovers and experiments; do not leave half-finished worktrees littering disk.
- Finish/integrate task work. Continuing useful work → durable, recoverable state with intentional ownership; not an orphaned checkout. Tidy owned resources + encountered stale merged leftovers.
- Preserve active/shared/unrelated/useful work, needed release inputs/evidence and previews still needed for review; clean previews when review ends. Dispose of disposable leftovers when no longer needed.
- Browser sessions follow project rules: reuse existing automation browser; bundled Chromium/Chrome for Testing for disposable tests; graceful `finally` closure, preserve user browser/profiles.
- Complete → affected private context/environment maintained, owned temporary resources closed/cleaned and continuing work recoverable with ownership. Computer ready for continued use.

## Follow-up and completion

Useful unrelated discoveries → actionable follow-ups in the existing project
tracker, then finish the original task. Caused failures remain current work.
Recorded follow-ups feed a later Plan cycle; recurring work only when requested.

Task done → requested delivery and verification complete, Public + Private
maintenance complete for affected scope, and useful follow-ups captured.
