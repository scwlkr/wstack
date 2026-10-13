# Workflows

## Implement
- Prepare environment → proposer subagent: 2 prototypes + preference → different critic: strengthen nonpreferred option → main chooses → implement.
- Scale prototypes to change: 1–3 paragraph plans → working UI. Mechanical/no meaningful choice → skip; unavailable subagents → main supplies options + critique.
- Quick affected check + repo gates → tiny review → update affected `FEATURES.md` description → push + merge. Apply release-review rule.
- Delete discarded experiments unless useful; retain chosen result + consequential reasoning.

## Debug / review
- Debug: cheap reproduction or convincing diagnostics → trace cause with cheapest useful experiment → fix → quick affected check. Regression only when useful.
- Failed attempts stop yielding information → reconsider premise.
- Review: actual diff vs request → consequential defects, missed requirements, unnecessary complexity. Implementation → fix; review-only → findings.

## Verify
- Requested outcome → practical existing CLI/UI/API tools → inspect actual result + relevant side effects; report coverage/gaps.
- Builds/readiness ≠ observed behavior; preserve useful diagnostics, clean owned state.
- Feature-map creation/discovery is separate; describing every feature does not require operating/checking every feature.

## Restate
- Goals + underlying problem → concise restatement in own words.
