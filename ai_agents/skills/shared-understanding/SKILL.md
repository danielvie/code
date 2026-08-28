---
name: shared-understanding
description: Reach shared understanding of a problem, plan, design, or decision through collaborative analysis. Use when the user wants to combine context, evidence, critical questions, domain insights, and explicit tradeoffs into a shared model or recommendation.
---

# Goal

Reach shared understanding of the problem, relevant evidence, assumptions, constraints, tradeoffs, and recommended direction.

# Instructions

- Follow [method.md](method.md).
- Investigate available evidence before asking the user for facts.
- Contribute relevant insights, alternatives, challenges, and recommendations.
- Ask the highest-value question needed to close the next gap in understanding.
- For each question, provide a recommended answer and its rationale.
- Do not force agreement. Make any remaining disagreement or uncertainty explicit.
- Stop when further discussion is unlikely to materially change the shared model or recommendation.

Read and maintain the project's canonical engineering artifacts according to [artifacts.md](artifacts.md).

Create artifact files lazily, only when there is resolved canonical information to record. Never create empty scaffolding. Once an artifact exists, keep it current.

# IDs
Assign every question a stable identifier: `Q1`, `Q2`, `Q3`, ...
Assign every resolved decision a stable identifier: `D1`, `D2`, `D3`, ...
Assign every implementation/fix item a stable identifier when useful: `I1`, `I2`, `I3`, ...
Never reuse an identifier.
Keep identifiers stable even if the wording later changes.
When referring to an item to fix, include the identifier first.