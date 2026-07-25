# Engineering artifacts

These files preserve canonical project understanding. They are not session logs, and no skill owns them.

## Artifact set

- Root `GOAL.md`: what the project needs to achieve. Follow [GOAL-FORMAT.md](GOAL-FORMAT.md).
- `CONTEXT.md`: the project's domain context and ubiquitous language. Follow [CONTEXT-FORMAT.md](CONTEXT-FORMAT.md).
- Architecture Decision Records: why consequential architectural decisions were made. Follow [ADR-FORMAT.md](ADR-FORMAT.md).

For multiple bounded contexts, follow the context-map and artifact-placement rules in those format files.

## Lifecycle

At the start of convergence:

1. Read `GOAL.md` when present.
2. Read the relevant `CONTEXT.md` or `CONTEXT-MAP.md` when present.
3. Read relevant ADRs.

Challenge contradictions as they arise. During convergence, update the affected artifact when canonical project understanding changes. Do not wait until the end and do not create a separate convergence summary.

Create artifacts lazily. If nothing canonical changed, create no artifact.
