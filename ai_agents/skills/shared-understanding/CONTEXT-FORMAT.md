# `CONTEXT.md` format

`CONTEXT.md` defines the project's domain context and ubiquitous language. It contains no implementation details, goals, requirements, or architecture decisions.

## Structure

```md
# {Context name}

{One or two sentences describing the context and why it exists.}

## Language

**{Canonical term}**:
{A one or two sentence definition}
_Avoid_: {ambiguous or rejected synonyms}
```

Group terms under subheadings when natural clusters emerge. Keep a flat `Language` section when they do not.

## Rules

- Pick one canonical term when several words describe the same concept.
- List ambiguous or rejected synonyms under `_Avoid_`.
- Keep definitions to one or two sentences.
- Define what a concept is, not how it is implemented.
- Include only concepts specific to the project's domain.
- Challenge language that conflicts with an existing definition before updating it.
- Update the document when terminology converges.

## Placement

Most projects have one context and one root `CONTEXT.md`.

For multiple bounded contexts, add a root `CONTEXT-MAP.md` and place a `CONTEXT.md` in each context:

```text
/
├── CONTEXT-MAP.md
└── src/
    ├── ordering/
    │   └── CONTEXT.md
    └── billing/
        └── CONTEXT.md
```

Use this map format:

```md
# Context map

## Contexts

- [{Context name}]({path to CONTEXT.md}) — {responsibility}

## Relationships

- **{Context} → {Context}**: {conceptual relationship}
```

Create `CONTEXT.md` lazily when the first domain term is resolved. Create `CONTEXT-MAP.md` only when the project has multiple bounded contexts.
