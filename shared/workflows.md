# Workflows

Plan → Design → Build → Test → Deploy → Maintain guides the work. Stages overlap;
limited requests use relevant parts. Agent chooses methods and depth. Completion
means delivering the requested outcome, checking it and maintaining affected surfaces.

## Plan / Restate

- First output the full understood goal + underlying problem in your own words.
- Keep working assuming it is correct; Shane can correct, steer or stop. No acknowledgement wait.
- Use project context and prepare prerequisites as needed; choose useful investigation/notes without a required planning ledger.
- Ready to proceed → goal made visible + workable interpretation. Separate release-review requirements still apply.

## Design

- Prefer DDD for business language/rules/boundaries, at useful depth.
- Meaningful choices → proposer develops separate approaches + preference; different critic strengthens the other route; main deliberates/chooses. Scale plans/prototypes to the change; mechanical work can skip exploration.
- Show meaningful alternatives + recommendation; visual form when useful. Continue on chosen direction while Shane can steer; pause for selection when explicitly requested.
- [Design router](design.md) → fitting guidance/skills; approaches may use different skills. Missing design subagents → main supplies options + critique; this does not waive fresh code review.
- Ready to build → chosen direction with enough clarity; alternatives visible when warranted.

## Build

- One or more implementation subagents finish code; main coordinates/integrates. Preserve unrelated work; favor simple maintainable solutions within request.
- Code/run/check/refine naturally overlap. Adapt within the intended goal; explain meaningful departures; bring goal-changing choices to Shane.
- Implemented → ready to exercise, not task finished. Discard unused experiments when no longer useful.

## Test / Code Review

- Check affected behavior + required repo gates. Prefer actual app/CLI/API/workflow checks; builds/readiness alone are not behavior proof.
- Strongly prefer almost never writing unit tests; aggressively remove units scoped to code being worked on. Integration/E2E checks remain available; framework/file names do not determine category.
- Every code change, including tiny edits and review fixes → [fresh independent Code Review](code-review.md). Implementers' self-review cannot satisfy it.
- Resolve consequential findings; fix caused failures, report unrelated/flaky failures and actual coverage/gaps; resolve blocked required gates.
- Ready to deliver → relevant behavior checked + actual delivery code freshly reviewed + consequential findings resolved.

## Deploy

- Push + merge by default. Human review before release via local build/screenshots/useful preview; merge publishes → review before merge.
- Live-site changes → approved deployment + check live result; other requests retain their delivery endpoint.
- Problems → favor fixing forward toward intended outcome over automatic rollback. Recovery code also requires fresh review.
- Delivered → applicable requested target reached + checked; a successful upload/job alone is insufficient for live delivery.

## Maintain

- [Public + Private Maintenance](maintenance.md) runs throughout work and at closeout: affected public surfaces/packages, private project context and shared computer are completion responsibilities.
- Useful unrelated discoveries → existing tracker as actionable follow-ups; caused problems stay in current task. Future follow-ups do not silently create ongoing work.
- Task done → requested delivery/verification + applicable Public/Private maintenance complete. Recurring work only when requested.

## Debug

- Cheap reproduction or convincing diagnostics → trace cause with cheapest useful experiment → fix → affected check + fresh review of code changes.
- Failed attempts stop yielding information → reconsider premise.

## Verify / review-only

- Requested outcome → practical existing CLI/UI/API tools → inspect result + relevant side effects; report coverage/gaps.
- Describing every feature does not require operating/checking every feature; feature discovery is separate.
- Review-only → findings, not implementation. Preserve read-only scope and useful diagnostics; clean owned disposable state.
