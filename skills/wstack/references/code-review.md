# Code Review

Every implemented code change requires adversarial review after implementation
finishes, including tiny changes and review-driven fixes. One or more subagents
implement; a completely new fresh-context subagent reviews. It must not be an
implementer of the change or inherit the implementation conversation.

- Start a new reviewer with `fork_turns: "none"` or the runtime's equivalent. A new agent name with inherited author context does not satisfy independence.
- Supply a neutral brief: original request, agreed requirements, applicable project instructions, review base and actual code/diff. Include intended uncommitted/new files where relevant; do not inspect only committed `HEAD` while missing the work being delivered.
- Do not supply the implementer's conversation, persuasive rationale, self-assessment or suggested verdict. Reviewer inspects surrounding code/context independently.
- Loosely adapt [Matt Pocock's code-review](https://github.com/mattpocock/skills/blob/main/skills/engineering/code-review/SKILL.md): challenge correctness, missed requirements, unnecessary scope/complexity and documented standards. Ground actionable findings in actual code and requirements; distinguish defects from judgment calls.
- Choose useful review depth without imposing tracker setup, a fixed Fowler checklist, exactly two reviewers or a reporting template. No findings need to be invented.
- Reviewer reports findings and does not edit the code. Implementers address findings; newly changed code then goes through another brand-new independent review.
- Resolve consequential findings before delivery and check fixes as relevant. Report actual review/check coverage and gaps.
- Missing fresh-context agents → report the unmet review requirement and continue independent work where possible. Main-agent/self-review is not a substitute, and the code is not ready to deliver until the requirement is met.

Done → actual code intended for delivery has the required fresh review,
consequential findings are resolved and relevant behavior has been checked.
