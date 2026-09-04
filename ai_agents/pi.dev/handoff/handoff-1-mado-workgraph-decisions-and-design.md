# Handoff: extension decisions and design

## Objective

Continue implementing `mado-workgraph`, a Pi extension/package for readable TypeScript workflow graphs. A workflow should be easy to inspect as code, execute through documented Pi and `pi-subagents` APIs, and render from the same normalized graph used for execution.

## Current state

A supervised implementation workflow is still active. Do not start a competing writer or edit the repository while its implementation/fix child is running.

- Top-level async workflow: `d15e946e-5984-43da-a474-fa22ebe1f9c6`
- Mission: `e6d3ed26-8173-4b60-b500-707259e225ee`
- Last observed state: architecture completed; implementation running as child `a0122168-d973-4872-8349-2ba215cb40fb`
- Planned remaining phases: correctness review, Pi API/security review, simplicity review, one resumed writer fix pass, then a fresh final blocker review

Check it first:

```text
subagent({ action: "status", id: "d15e946e-5984-43da-a474-fa22ebe1f9c6" })
```

A detached-foreground notification appeared for the completed architecture phase. The enclosing async workflow remains the authoritative run to inspect.

## Settled design decisions

### Name

The package and extension are named `mado-workgraph`.

### Canonical source

Workflows are authored in TypeScript. The source should expose workflow structure rather than normalized storage details.

The graph uses path-oriented arrays:

```ts
Workflow({
  name: "review-then-implement",
  edges: [
    [
      START,
      all({
        correctness: [correctnessReview],
        tests: [testReview],
      }),
      implement,
      END,
    ],
  ],
});
```

`START`, `END`, and `STOP` are explicit terminal tokens. Normalized nodes and edges are derived from this source and must not become a second hand-maintained definition.

### Forks and joins

`all({...})` means parallel fork plus all-branches join. Every branch runs. The object keys name the results delivered after the join:

```ts
all({
  correctness: [correctnessReview],
  tests: [testReview],
})
```

The following node receives `inputs.correctness` and `inputs.tests`. These keys are data-binding labels, not router outcomes.

A convergence after mutually exclusive router branches is a merge, not an all-join. It must wait only for the selected branch.

### Routers

A router is a node followed by a route map. Route-map keys are allowed structured router outputs and become labels on outgoing graph edges:

```ts
[
  classify,
  route,
  {
    simple_change: [quickPlan],
    cross_cutting_change: [all({ api: [apiPlan], ui: [uiPlan] }), combinePlan],
  },
  implement,
]
```

A router should select exactly one route. Unknown routes fail closed. A default route or exhaustive route contract is required. Arbitrary hidden predicates should not replace visible route cases in the first version.

### Navigable agent and model objects

Agent and model selections are imported object references, not strings in node definitions:

```ts
agentNode({
  id: "implement",
  agent: agents.worker,
  model: models.implementation,
  task: "Apply the findings.",
});
```

An agent definition owns the agent ID, description, system prompt, and tool allowlist. A model definition owns provider, model ID, and Pi thinking level. This supports editor go-to-definition and refactoring.

At the `pi-subagents` boundary, the extension extracts the agent ID and model selection. Generated runtime requests may contain strings; authored workflow code should not.

The intended separation is:

```text
workflow.ts   control flow
nodes.ts      tasks, inputs, outputs, artifacts
agents/       system prompts and tool permissions
models.ts     provider, model, and thinking allocation
graph.mmd     generated view only
```

### Model configuration

Model profiles are reusable objects:

```ts
const review = model({
  provider: "openai-codex",
  id: "gpt-5.6-sol",
  thinking: "high",
});
```

Pi's term is `thinking`, with `off`, `minimal`, `low`, `medium`, `high`, `xhigh`, and `max`. The extension must validate availability and report resolved/clamped behavior rather than pretending an unsupported setting was honored.

### Rendering

One normalized model drives validation, execution, terminal output, and Mermaid output. Renderers must not rediscover topology independently.

The portable baseline is a terminal representation plus explicit edge/adjacency information. Mermaid is the first generated file format. Do not auto-open generated HTML or SVG.

### Execution

Use only documented Pi and `pi-subagents` integration APIs. Do not import private scheduler internals. Pi project trust is a loading decision, not an OS sandbox. TypeScript workflow files execute trusted code.

Large child output stays in artifacts; Pi receives bounded summaries and paths. Run records should retain enough identity information to diagnose the effective workflow, agent, and model configuration.

## Accepted examples

### Review then implement

The existing design example is under [`examples/review-then-implement/`](../examples/review-then-implement/). It has two parallel review nodes, an all-join, and one implementation node. The active writer is replacing its local placeholder DSL with the real package API.

### Cake workflow acceptance example

A real example must support this flow:

```text
START
  -> write cake guide
  -> fork French review and Dutch review
  -> join after both return
  -> compare reviews
  -> END
```

Expected Markdown artifacts:

```text
artifacts/
  cake-guide.md
  french-review.md
  dutch-review.md
  cake-comparison.md
```

The French reviewer must produce French output. The Dutch reviewer must produce Dutch output. The final comparison node reads both named branch results/artifact paths and writes the comparison. Normal tests should validate graph, binding, artifact, and language-policy wiring without making live model calls. A real end-to-end Pi run remains necessary after the package is stable.

This requirement was steered into the active architecture/implementation workflow before implementation began.

## Existing references

Do not reproduce the package survey. Read [`docs/research/graph-workflow-extension-options.md`](../docs/research/graph-workflow-extension-options.md). It records existing Pi workflow packages, current join semantics, Pi feasibility, renderer choices, and security constraints.

The implementation currently has package metadata and source under `src/`. Treat it as in-progress until the supervised workflow finishes. At handoff time, files already existed for public constructors/types, normalization, validation, fingerprinting, Mermaid/terminal rendering, runtime-agent registration, and delegation.

## Open verification work

After the active workflow finishes:

1. Read its final implementation report and final review artifact from the workflow result/receipt.
2. Inspect the current files rather than trusting child prose.
3. Run `npm run check` from the repository root.
4. Confirm package imports and peer dependency names match the installed Pi distribution. Do not silently mix incompatible upstream package namespaces.
5. Verify workflow discovery respects project trust and does not load untrusted project TypeScript.
6. Verify cancellation and lifecycle cleanup against the actual public delegation API.
7. Verify router selection, selected-branch merge, `all` join result binding, duplicate IDs, cycles, unreachable nodes, and artifact collisions.
8. Run the package as a Pi extension with `pi -e .` or the package's documented equivalent.
9. Run the cake workflow end to end only after static checks and mocked tests pass. Inspect all four Markdown files and confirm the language constraints and comparison behavior.

If the supervised run failed or stopped, recover its reports and continue with one writer. Do not relaunch the whole orchestration without checking which changes and reviews already landed.

## Known risks

- The implementation is still changing, so no completion or test claim is final.
- `pi-subagents` provides public extension RPC and structured delegation seams, but they have different foreground/background and lifecycle behavior. Verify the selected adapter against current installed docs.
- TypeScript task functions can accidentally close over values that cannot cross an execution boundary. The supported task contract must be explicit and tested.
- Router cycles make the workflow a general directed graph rather than a strict DAG. Cycle policy and bounds must be intentional.
- Markdown artifact paths need traversal protection and collision checks.
- Language detection can support the cake example, but it is heuristic. Prompts and validation should report failures clearly rather than claim certainty.
- Pi extensions run with user permissions. Agent tool allowlists are not an OS sandbox.

## Suggested skills

- `pi-subagents`: recover and supervise the active workflow, inspect receipts, and continue the single-writer review loop.
- `how`: explain the finished architecture and runtime flow before making structural changes.
- `domain-modeling`: settle terms such as path, branch, fork, join, merge, router, node, artifact, and run before expanding the DSL.
- `research`: use only if current external Pi or `pi-subagents` API facts need to be rechecked; prefer official source/docs.
- `unslop`: apply to README, API names, diagnostics, and final delivery text.
