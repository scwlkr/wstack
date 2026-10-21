# Code Review

Use for reviewer isolation or briefing details; the core fresh-review requirement
applies whether this guide is read or not.

- Supply a neutral brief: original request, agreed requirements, applicable project instructions, review base and actual code/diff. Include intended uncommitted/new files where relevant; do not inspect only committed `HEAD` while missing the work being delivered.
- Do not supply the implementer's conversation, persuasive rationale, self-assessment or suggested verdict. Reviewer inspects surrounding code/context independently.
- Loosely adapt [Matt Pocock's code-review](https://github.com/mattpocock/skills/blob/main/skills/engineering/code-review/SKILL.md): challenge correctness, missed requirements, unnecessary scope/complexity and documented standards. Ground actionable findings in actual code and requirements; distinguish defects from judgment calls.
- Choose useful review depth without imposing tracker setup, a fixed Fowler checklist, exactly two reviewers or a reporting template. No findings need to be invented.
- Report actual review/check coverage and gaps. Missing fresh-context agents → report the unmet requirement and continue independent work; code cannot be delivered until it is met.
