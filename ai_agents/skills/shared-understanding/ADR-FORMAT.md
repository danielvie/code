# Architecture Decision Record format

ADRs record why consequential architectural decisions were made.

## Placement and naming

ADRs normally live in `docs/adr/` and use sequential numbering:

```text
docs/adr/
├── 0001-event-sourced-orders.md
└── 0002-postgres-for-write-model.md
```

Scan the target ADR directory for the highest existing number and increment it. When the project has multiple bounded contexts, place system-wide ADRs in the root `docs/adr/` and context-specific ADRs beside that context.

Create the directory lazily when the first ADR is needed.

## Structure

```md
# {Short decision title}

{In one to three sentences: the relevant context, what was decided, and why.}
```

The value is recording that a decision was made and why, not filling out sections.

## Optional sections

Include these only when they add meaningful information:

- `Status`: `proposed`, `accepted`, `deprecated`, or `superseded by ADR-NNNN`
- `Considered options`: rejected alternatives worth remembering
- `Consequences`: non-obvious downstream effects

## Qualification

Create an ADR only when the decision is all three:

1. Hard to reverse: changing it later has meaningful cost.
2. Surprising without context: a future reader would reasonably question it.
3. A real tradeoff: plausible alternatives were rejected for specific reasons.

Use ADRs for architectural shape, context boundaries, consequential integration patterns, high-lock-in technology choices, deliberate deviations, and constraints not visible in code.

Do not create ADRs for routine implementation choices, obvious decisions, or easily reversible changes.
