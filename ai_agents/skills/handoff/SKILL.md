---
name: handoff
description: Compact the current conversation into a handoff document for another agent to pick up.
argument-hint: "What will the next session be used for?"
---

Write a handoff document summarising the current conversation so a fresh agent can continue the work. Save to the `handoff` folder in the workspace.

Name the document `handoff-{i}-{title}.md`, where:

- `{i}` is the next available positive integer across existing `handoff-*.md` files in the workspace's `handoff/` folder.
- `{title}` is a short, meaningful lower-kebab-case summary of the handoff's focus. Derive it from the User's argument when provided; otherwise use the conversation's primary work topic.
- Keep `{title}` free of sensitive or personally identifiable information. Use only lowercase letters, numbers, and single hyphens; remove leading, trailing, or repeated hyphens.

Include a "suggested skills" section in the document, which suggests skills that the agent should invoke.

Do not duplicate content already captured in other artifacts (PRDs, plans, ADRs, issues, commits, diffs). Reference them by path or URL instead.

Redact any sensitive information, such as API keys, passwords, or personally identifiable information.

If the user passed arguments, treat them as a description of what the next session will focus on and tailor the doc accordingly.
