## eyes-and-hands

Mandatory subsystem in every wstack session, before member routing and throughout the task. Actively look for anything the agent cannot observe or operate in the current environment. Keep closing concrete gaps while doing the work; do not wait for a failed test or a user request for automation.

The goal is complete CLI access to the project's development surface: every feature, interaction, relevant state and observable effect can be inspected and exercised by scripts. A tool being installed, a server responding or a build passing does not establish that access. Skills guide the active agent; they do not install a background daemon or guarantee that another agent follows the instructions.

### Operating loop

1. **At session start:** inspect the actual tools, project CLI, verification skill, feature map and running environment. Reuse proven adapters; run a small probe of the capabilities this task needs. No development surface (for example, a prose-only task) → mark not applicable and continue.
2. **Find blind spots:** reconcile routes, screens, menus, commands, APIs and state transitions from source with live behavior and the feature map. Inventory the whole discovered surface; identify unexamined areas explicitly. Ask what cannot be read, reached, changed or independently checked from CLI, including failure states and hidden side effects.
3. **Close gaps:** reuse a supported driver first; then add the smallest missing command, adapter, fixture or observation hook at its owner. Wire durable automation through the existing project CLI (`./project` when configured). Run it against the real boundary before relying on it. Setup/access gaps → use the available environment workflow within authorized scope.
4. **At each feature boundary:** repeat when a feature, state, dependency, environment or harness changes, or an action produces an unexplained result. Keep working capabilities; revisit affected gaps and failed probes. New behavior needs its observe/operate/prove path in the same change.
5. **Before delivery:** run the applicable scripts on the final build and environment, inspect their evidence and update capability coverage. A required blind spot blocks the corresponding verification claim. Report unrelated gaps with their owner and next action; keep independent work moving.

Continue this loop during the session; do not create recurring jobs, idle polling or extra agents merely to keep it running. Reuse current evidence where nothing relevant changed. Read-only tasks inspect and report gaps without editing the target.

### Eyes: inspect what actually happened

- Read current application state, rendered output, errors, logs and external effects in a scriptable form. Distinguish requested, pending, committed and failed state; inspect the actual process/build/instance being driven.
- For UI work, inspect DOM or the native accessibility tree, computed styles/layout, focus and screenshots as relevant. DOM text alone cannot prove appearance, visibility, responsiveness or visual correctness; inspect the captured image when the claim is visual.
- For browser work, provide CLI access to the relevant DevTools capabilities: console and uncaught errors, requests/responses and failed network calls, storage/cookies, navigation and frames, accessibility, and performance traces when needed. Include workers, WebSockets and browser permissions when the feature uses them.
- Keep structured outputs and proof artifacts available after cleanup. A command must expose failures and a useful exit status; silently swallowed errors defeat observation.

### Hands: reach and exercise real behavior

- Start/stop owned instances, wait for readiness, seed disposable data, authenticate through supported flows, reach named states and reset fixtures without corrupting shared work. Make these operations repeatable and scriptable.
- Exercise real user actions: navigation, clicks, typing, keyboard shortcuts, focus, scroll, drag/drop, uploads/downloads, dialogs and other interactions the app exposes. Cover relevant loading, empty, validation, error, retry and permission states as well as success.
- Use the same application boundary as the user. A direct database edit can prepare a fixture or independently read an effect; it cannot prove that a save button, validation rule or UI transition works. Internal setters and test-only bypasses cannot stand in for the feature being verified.
- After an action, wait for an observable condition with a bounded timeout and assert the result. For mutations, also read back the persisted or external effect independently (reopen, query, read file). Successful dispatch alone is not proof.
- Support parameterized commands and scenario composition so the next agent can script a new sequence without another manual UI session. Prefer stable roles/names/IDs over screen coordinates, and condition waits over fixed sleeps.

### CLI adapters and capability coverage

Use the project's current automation stack. For a web app, a CLI-driven browser harness (such as Playwright, Cypress or supported CDP tooling) can provide UI interaction and DevTools observations. Use the appropriate native driver, PTY or API client for other surfaces. Respect host tool restrictions and supported access methods. Computer use is a fallback for a concrete unsupported operation; record that limitation and seek a supported CLI replacement within scope.

Each discovered feature needs a recipe that another agent can run:

| Contract | Record |
| --- | --- |
| Reach | Instance/target, prerequisites, named starting state, seed/reset and supported auth commands. |
| Observe | Exact CLI invocation to inspect UI/state/errors and relevant side effects. |
| Operate | Exact CLI invocation for each user action or composed scenario, with parameters. |
| Prove | Expected observable conditions, assertions, independent readback and evidence paths. |
| Restore | Teardown/reset for owned instances and data; evidence survives. |
| Coverage | Proven build/environment + command/result, or the precise gap, owner and next action. |

Extend the existing verification skill and feature map rather than maintaining a second inventory. Put prerequisites and action commands in `Driving it with`, results/artifacts in `Proof`, and limitations in `Gotchas`; include shared observation/reset commands in its baseline. If no map exists, record concrete gaps in the existing issue or task notes, add the needed executable recipe now, and create a map through `wstack-verify-create` when its scope is authorized. Do not make a missing sibling skill a reason to abandon useful work.

Mark each capability as **proven**, **missing**, **blocked**, **planned** or **not applicable** with a reason. Name the specific operation when a feature has mixed coverage. A map-format check establishes document structure only. A representative smoke test proves only the path it ran; it cannot establish access to every feature. Gaps stay visible until exercised successfully in the relevant environment.

For example, a web editor's save recipe should reach a disposable draft, read its initial state, type through the browser driver, activate Save, inspect validation/network/errors and the resulting UI, then reopen the draft to verify persistence. Run the whole sequence through one parameterized project command; expose its actions and observations for composing other scenarios. The exact command belongs to that project and must be implemented and tried before being documented as usable.

Close development capability gaps aggressively within the task's authorization. Prefer local instances, test accounts and disposable state. Wider account access, production writes, spending or new external services still require the applicable authorization. A legitimate authentication/device/provider limitation is a concrete blocker, never a reason to invent proof or silently skip the interaction.
