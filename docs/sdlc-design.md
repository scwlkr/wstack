# Wstack SDLC design

Accepted and applied from Shane's October 21, 2026 decisions, including the Public
and Private Maintain refinement. Canonical guidance lives in
[workflows](../shared/workflows.md), [principles](../shared/principles.md),
[design routing](../shared/design.md), [Code Review](../shared/code-review.md) and
[maintenance](../shared/maintenance.md), synced into the installed skill package.
The [starting audit](sdlc-current-state.md) records the earlier rules and implementation.

## Approach

Wstack guides agents toward Shane's preferred way of working. It initiates useful
capabilities and favors a sound SDLC and domain-driven design (DDD), while leaving
investigation, tools, implementation details and appropriate depth to the agent.
DDD should help the agent understand the business language, rules and boundaries;
it does not require a modeling exercise or fixed artifacts for every change.

Plan → Design → Build → Test → Deploy → Maintain → Plan is a useful cycle. Stages
can overlap, and limited requests take shorter paths. Completion statements help
communicate progress; they are not a sequence of owner approval gates. The agent
continues working while Shane can steer or stop it.

Fresh independent review for every code change and human review before release
are firm requirements. Maintaining the affected public product and private
workspace throughout the work and at closeout is also a firm responsibility.
Maintenance is part of completing the work, not optional end-task housekeeping.
Routine choices follow the agent's judgment.

## Plan

Start with Restate: output the full restatement of the understood goal and
underlying problem so Shane can read it. Continue assuming that interpretation
is correct without waiting for acknowledgement. Shane can correct, steer or stop
the agent if it misunderstood.

Use relevant project context and prepare prerequisites as needed. Let the agent
choose the useful investigation and documentation; no required planning ledger
or detailed checklist is introduced.

**Done:** the agent has made its understanding visible and has a workable
interpretation of the requested outcome. Separate release-review requirements
still apply.

## Design

For meaningful choices, retain separate approaches and deliberation between them.
Show the alternatives and recommended direction, preferably visually when useful,
then continue on the chosen direction while Shane can steer. Pause for his
selection when explicitly requested. Mechanical changes can skip the exploration.

The design router selects guidance that fits the job. A button change may need
no skill or lightweight guidance; a page redesign may benefit from a comprehensive
skill. Preferred routes start with [Impeccable](https://github.com/pbakaus/impeccable)
and [Taste Skill](https://github.com/leonxlnx/taste-skill), with room for more later.
Competing approaches may use different suitable skills; there is no compulsory
assignment or fixed creation-versus-refinement split.

Keep DDD as the business/domain-design preference. The named skills provide
interface-design guidance. Select for the surface and platform, respect the
project's identity, and follow its browser rules when using skill tooling.

**Done:** a direction has been chosen with enough clarity to build. When meaningful
choices warrant alternatives, those alternatives and the recommendation have been
made visible, in visual form only when useful. No routine approval wait is added.

## Build

One or more implementation subagents finish the code. The main agent coordinates
and integrates their work. Favor simple, maintainable solutions within the
request, with DDD informing the work where useful.

Implementation, running the app, checking behavior and refinement can overlap
naturally. When the selected approach needs adjustment, let the agent adapt it,
explain meaningful departures and continue. Bring choices that change the intended
outcome to Shane.

**Done:** the requested change is implemented and ready to exercise. Code written
is a progress milestone; the task continues through verification and delivery.

## Test and code review

Strongly prefer almost never writing unit tests, and aggressively remove unit
tests associated with the code being worked on. Keep removal scoped to the task;
broader purges are separate work.

Distinguish isolated-unit checks from integration or end-to-end checks of real
workflows by what they exercise, not their framework or filename. Use practical
behavior checks and required repository gates. A successful build alone does not
prove the requested behavior. Report actual coverage and gaps.

### Fresh independent review

Every implemented code change must go through Code Review, including tiny changes
and review-driven fixes. After implementers finish, start a completely new
fresh-context subagent to review adversarially. The reviewer must not be an
implementer of that change and must not inherit the implementation conversation.

In the current agent tools, start the reviewer with `fork_turns: "none"`; use the
equivalent elsewhere. Supply a neutral brief containing the original request,
agreed requirements, applicable project instructions and the actual code/diff to
inspect. Do not pass the implementer's conversation or persuasive self-assessment.
The reviewer can inspect the repository independently.

Loosely adapt [Matt Pocock's code-review skill](https://github.com/mattpocock/skills/blob/main/skills/engineering/code-review/SKILL.md):
check the requested outcome and repository standards, ground findings in code,
and challenge correctness, missed requirements and unnecessary complexity. This
does not import tracker setup, a mandatory pair of reviewers or fixed reporting
machinery. Wstack's fresh-context requirement controls the adaptation.

Reviewers return findings and do not edit the implementation they review.
Implementers address findings; subsequent code fixes also receive fresh independent
review before delivery. Self-review does not satisfy this requirement.

**Done:** appropriate behavior checks have been performed, the actual code intended
for delivery has received the required fresh review, and consequential findings
have been resolved.

## Deploy

Push and merge requested work by default. Human review happens before release,
using a local build, screenshots or a suitable preview. If merging publishes the
change, that review happens before merge.

For a change to a live website, finish at the approved deployment and a check of
the live result, not just merged source. Other release targets follow the request.

If deployment exposes a problem, favor fixing forward to reach the intended
outcome rather than automatically rolling back. This is a preference, not a claim
that rollback is never appropriate. Recovery code changes retain the same fresh
review requirement.

**Done:** the applicable delivery target has been reached and checked. An approved
live-website change is live and its relevant result has been checked; source or
review tasks retain their requested endpoint. A successful upload or deployment
job alone is insufficient for live delivery.

## Maintain

Maintenance is a first-class responsibility throughout the lifecycle, with an
explicit closeout. It has two parts: the public product and the private project
and working environment. Keep both aligned with decisions and delivered work,
within the task's impact. The shared computer is also a workspace to maintain.

### Public maintenance

Maintain the publicly visible aspects affected by the work: documentation,
feature descriptions, release and distribution packages, the website, and other
public places affected by product decisions. Update them to match the delivered
behavior and decisions. Verify the actual release packages and other applicable
artifacts, rather than relying on their labels or a successful build. Public
updates retain the applicable code-review and release-review requirements.

**Public done:** affected documentation, distribution packages, website and other
public surfaces match the delivered decisions, with the applicable actual
artifacts checked.

### Private maintenance

Investigate and maintain the affected private documentation, planning, project
context and agent environment as the work changes. Keep useful knowledge and
decisions available for later work. Let the agent choose the useful records and
methods.

Prevent resource clutter before creating it. Plan the lifecycle of a simulator,
worktree, test environment or experiment before starting it. Prefer reusing a
suitable resource when useful; otherwise use a disposable resource with reliable
cleanup. This does not prescribe one global simulator or a fixed resource layout.

Clean up owned disposable resources after both successful and failed runs,
including failed tests and experiments. Preserve useful diagnostics, then close
or remove the temporary resources. Do not leave unused simulators, abandoned
experiments, test leftovers or half-finished worktrees littering the disk.

Finish and integrate task work. If work must continue later, preserve it in a
durable, recoverable form with intentional ownership instead of an orphaned
checkout. Tidy owned resources and encountered stale merged leftovers while
preserving active, shared, unrelated and useful work, useful evidence, and
previews still needed for review. Keep maintenance proportional to task-related
impact; it is not a whole-machine deletion pass.

**Private done:** affected private project context and environment are maintained,
owned temporary resources are closed or cleaned, and any continuing work is
recoverable with intentional ownership. The computer is ready for continued use.

### Follow-up and task completion

Capture useful, actionable unrelated discoveries in the project's existing
tracker as follow-up work, then finish the original task. Problems caused by the
current change remain part of finishing that task. Recorded follow-ups can feed
a later Plan cycle. Recurring work happens only when requested.

**Task done:** the applicable delivery and verification are complete, both public
and private maintenance are complete for the affected scope, and useful follow-up
work is captured. The task ends; follow-up records do not silently create ongoing
work.
