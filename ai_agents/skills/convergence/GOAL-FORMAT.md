# `GOAL.md` format

`GOAL.md` lives at the project root and states what the project needs to achieve. It is canonical current truth and must evolve with the project.

## Structure

```md
# Goal

{A concise, direct statement of what the project needs to achieve.}

## Mental model

{A conceptual description of how the solution should behave and how its important parts relate.}

## Success criteria

- {An observable condition that indicates success}

## Boundaries

### Non-goals

- {An outcome intentionally excluded from the project}

### Constraints

- {A condition every acceptable solution must satisfy}

### Accepted limitations

- {A known shortcoming deliberately accepted}
```

## Rules

- Keep the goal short, direct, and outcome-oriented.
- Keep the mental model conceptual and behavioral. Use terms from `CONTEXT.md`.
- Put consequential architecture and technology choices in ADRs. Reference them from the mental model only when needed.
- Make success criteria observable and falsifiable. Describe outcomes, not implementation tasks.
- Use non-goals to prevent scope expansion.
- Include only mandatory conditions as constraints, not current preferences.
- Record only consciously accepted shortcomings as limitations. Explain why when acceptance is not obvious, and link the relevant ADR when applicable.
- Challenge proposed work or goal changes that conflict with `GOAL.md` before updating it.
- Edit the canonical document in place after a change is resolved. Git preserves its history.
- Do not turn `GOAL.md` into a roadmap, backlog, specification, decision log, or session record.

Create `GOAL.md` lazily when the project goal first becomes clear.
