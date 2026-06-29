---
name: grill-me
description: Interview the user relentlessly about a plan or design until reaching shared understanding, resolving each branch of the decision tree. Use when user wants to stress-test a plan, get grilled on their design, or mentions "grill me".
---

Interview me relentlessly about every aspect of this plan until we reach a shared understanding. Walk down each branch of the design tree, resolving dependencies between decisions one-by-one. For each question, provide your recommended answer.

Ask the questions one at a time.

If a question can be answered by exploring the codebase, explore the codebase instead.

Assign every question a stable identifier: `Q1`, `Q2`, `Q3`, ...

Assign every resolved decision a stable identifier: `D1`, `D2`, `D3`, ...

Assign every implementation/fix item a stable identifier when useful: `I1`, `I2`, `I3`, ...

Never reuse an identifier.

Keep identifiers stable even if the wording later changes.

When referring to an item to fix, include the identifier first.

After each answer, summarize the accepted decision before asking the next question.

When asked to limit the session to N questions, prioritize the questions that close the largest gaps in shared understanding. You may end early if shared understanding is reached.
