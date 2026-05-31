# Debug Plan

This is the living investigation plan for the zoo catalog performance lab. Keep it updated as issues are investigated, confirmed, fixed, or deferred.

## Goal

Learn how to debug systematically by using repeatable evidence-driven loops across frontend, backend, network, and database layers.

## Investigation Loop

1. Pick one symptom.
2. Write the exact reproduction steps.
3. Capture baseline evidence.
4. Form one hypothesis.
5. Test only that hypothesis.
6. Record the result.
7. Fix only the proven cause.
8. Re-run the same reproduction.
9. Add a regression check when practical.

## Baseline Flows

Use these flows to compare behavior before and after fixes:

- Load small dataset.
- Load large dataset.
- Expand root tree nodes.
- Expand nested tree nodes.
- Select a node without expanding it.
- Ctrl/Cmd-select multiple nodes.
- Drag a node into the mirror panel.
- Expand/collapse a mirrored node.
- Create, update, and delete an animal.
- Repeat interactions for several minutes and watch for decay.

## Measurements

Frontend:
- Chrome Performance recording for slow interactions.
- Chrome Memory heap snapshots before and after repeated use.
- Chrome Network request count, response time, and duplicate calls.
- Angular DevTools component/change-detection behavior.

Backend:
- Spring Actuator health and metrics.
- Endpoint response time.
- Request logs around suspected paths.
- Thread/CPU behavior under repeated interactions.

Database:
- SQL query count per endpoint.
- Parent/child lookup cost.
- Missing indexes or repeated queries.
- Data shape differences between small and large datasets.

## Issue Log

Add entries here as investigation proceeds.

| ID | Symptom | Reproduction | Evidence | Hypothesis | Status | Fix |
| --- | --- | --- | --- | --- | --- | --- |
| FE-001 | UI becomes less responsive during repeated tree use and mouse movement | Use the app, expand/collapse/select tree nodes, then move the mouse over the catalog tree | Performance trace shows repeated `mousemove` tasks around 30-38 ms each; call tree routes through Zone.js; app template work is smaller than event/task overhead; Event Listeners shows `mousemove` on `.tree` and `Window` | Excessive or accumulating mousemove work/listeners are causing repeated Angular/Zone tasks and main-thread pressure | Investigating | TBD |

## Rules

- Do not fix by guessing.
- Do not investigate multiple symptoms at once.
- Do not delete planted issues unless that issue is the current target.
- Preserve reproduction steps after a fix.
- Update this document whenever new evidence changes the plan.
