# TODO: OpenTelemetry React demo

- [x] Create a minimal React project
- [x] Add OpenTelemetry browser instrumentation
- [x] Add local collector and Jaeger trace viewer configuration
- [x] Add flame graph and sequence graph usage notes
- [x] Add Taskfile.yml with run/test/clear tasks
- [x] Fix TypeScript issues found during verification
- [x] Verify project structure and available scripts

# TODO: Request substep spans

- [x] Add nested spans for each request action
- [x] Add span attributes/events that show data boundaries
- [x] Update README with the expected span tree
- [x] Verify TypeScript and build

# TODO: Fix scattered button trace

- [x] Preserve one parent context for all button spans
- [x] Move React state commit into the root request workflow
- [x] Update README expected span tree
- [x] Verify TypeScript and build

# TODO: HTML concept explanation

- [x] Create standalone HTML explainer for the span parenting fix
- [x] Include visuals for scattered vs fixed trace topology
- [x] Include annotated code references

# TODO: Improve before/after explainer

- [x] Add before/after Jaeger result diagrams
- [x] Add before/after code snippets
- [x] Explain the exact failure in plain language
- [x] Verify the HTML has no broken template leftovers
