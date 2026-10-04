# Verify skill sections

Each section is grounded in what the repo interview found. No placeholders in an existing-project skill.

- **Launch:** exact start command and the ready signal (log line, port answering, prompt), plus teardown. Short-lived CLI/TUI: build or install once, then start each drive in its own isolated PTY or tmux session.
- **Doctor:** one read-only check that answers "is this instance worth driving?": process up, right build, port owned by us, auth valid. Run it first whenever anything looks off.
- **Drive:** the harness recipe with real selectors and commands from this repo. Prefer stable handles (ARIA names, data attributes, prompt strings, routes) over coordinates and tab order.
- **Evidence:** what to capture and where it goes. Proof standards:
  - exercise the real user path, not internal setters or test-only endpoints
  - capture the action and the resulting state, not only the final screen
  - verify side effects (files, rows, messages) next to what is visible
  - mock only where a production boundary already isolates the external system
  - observe what a dry-run or test mode really skips (files, network, git refs, browser) instead of trusting its name
- **Cleanup:** tear down only what the run started; never kill by process name. Remove instances and scratch state, never the proof artifacts; name where they live.
- **Helpers:** scripts are executable and their invocation is shown in `SKILL.md`.

Isolation: if two instances cannot run side by side (ports, data dirs, profiles), say so and refuse to double-drive a shared instance.
