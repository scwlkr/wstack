---
name: wstack-debug
description: "Debug a failing behavior, test or regression: reproduce it, trace the root cause, make the smallest fix, and prove it with a regression test, using Wstack principles."
---
Apply `wstack` skill shared rules; don't reroute.

1. Read report/issues + discussion; repository tracker owns execution. Define failing behavior + expected outcome; capture starting SHA + existing changes.
2. Reproduce on affected surface (real command/UI/API/data, not a stand-in); record steps, inputs, environment, observed vs expected. No reproduction → report evidence + gap; no speculative fixes.
3. Trace root cause: inspect/measure, narrow by bisect/instrumentation/small experiments; separate symptom, trigger, cause. Cite file:line + evidence; label hypotheses until confirmed.
4. Smallest fix at the cause owner, not the symptom; preserve unrelated edits. Delete task-created probes/logging/dead code; flag other cleanup.
5. Regression test at the real boundary: run on unfixed code → fails for the reported reason; after fix → passes. Untestable → say why + give other proof. Then affected checks + final repository gates.
6. Repeated failed gate or fix → stop patching; revisit premise (reproduction, assumed cause, surface, test) → re-trace.
7. Commit/deliver per Wstack + repository rules; report cause, fix, before/after evidence, unresolved gates.
