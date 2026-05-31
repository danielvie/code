# Project Context

This project is a deliberately imperfect Angular 17 + Spring Boot 2.6 application for learning how to debug systematically.

The domain is a zoo catalog:
- Angular frontend renders a tree-style catalog explorer.
- Spring Boot backend owns catalog data and exposes REST APIs.
- H2/JPA stores tree nodes and animal records.
- Tree nodes are loaded lazily: root nodes load first, and expanding a node requests its details and children.

# Main Goal

Learn how to debug systematically.

Use this project to practice:
- observing symptoms,
- forming hypotheses,
- measuring behavior,
- isolating frontend/backend/database causes,
- proving root cause with evidence,
- fixing one issue at a time,
- re-measuring after each change.

# Debugging Strategy

Prefer this loop:

1. Reproduce one specific symptom.
2. Define expected vs actual behavior.
3. Capture measurements before changing code.
4. Narrow the layer: frontend, network, backend, or database.
5. Inspect only the code relevant to the symptom.
6. Make the smallest fix that addresses the proven cause.
7. Re-run the same reproduction path.

# Useful Flows

- Switch between small and large datasets.
- Expand/collapse tree nodes.
- Select tree nodes.
- Ctrl/Cmd-select multiple nodes.
- Drag nodes into the mirror panel.
- Expand/collapse mirrored nodes.
- Create, update, and delete animals.

# Commands

- `task run`: run backend and frontend.
- `task build`: build backend and frontend.
- `task back:run`: run Spring Boot backend.
- `task web:run`: run Angular frontend.
- `task back:build`: build backend.
- `task web:build`: build frontend.
- `task clean`: remove generated outputs and dependencies.

# Investigation Notes

- Do not assume the problem is in the layer where the symptom appears.
- Use browser Performance, Memory, Network, and Angular DevTools for frontend issues.
- Use Spring Actuator, logs, request timing, and SQL logging for backend/database issues.
- Keep fixes surgical. This app intentionally contains multiple issues, so avoid broad refactors that hide the learning signal.
- Keep `docs/debug-plan.md` updated as investigations proceed, including symptoms, reproduction steps, evidence, hypotheses, status, and fixes.
