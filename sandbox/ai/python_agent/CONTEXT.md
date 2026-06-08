# Project Context

## Project Goal

This project is a tutorial for learning how to build a Python coding agent incrementally.

The tutorial is organized as a sequence of steps. Each step represents one learning milestone and builds directly on the previous step. A learner should be able to open a step folder, read its `README.md`, implement the requested change, and run the project from that folder.

The project should stay tutorial-friendly: clear instructions, minimal code, visible progression, and no abstractions that hide the concept being taught.

## What Step Folders Represent

Each `stepN_name/` folder is a snapshot of the agent at a specific stage of the tutorial.

A step folder is not just a code directory. It represents:

- The starting point for that tutorial stage.
- The expected project state after the previous stage.
- A focused learning objective for the current stage.
- A runnable Python project with its own dependencies and task commands.

Later steps should preserve the behavior from earlier steps while adding exactly one major new concept.

`step1_hello/` is special because it also stores copied README files for multiple steps. Treat those extra README files as source/reference material for writing or updating step instructions.

## Required Structure for Each Step

Each step folder should contain, unless there is a strong reason not to:

```text
stepN_short_name/
├── README.md
├── Taskfile.yml
├── pyproject.toml
├── uv.lock
└── app/
    └── main.py
```

As the tutorial grows, a step may add focused modules such as:

```text
app/tools.py
app/mcp_client.py
mcp_servers/
mpc_config.json
```

Keep every step independently runnable from inside its own folder.

## Step Naming Convention

Use this folder naming pattern:

```text
step<number>_<short_snake_case_name>
```

Examples:

```text
step1_hello
step4_agent_loop
step8_mcp
```

Do not rename existing folders casually. Existing names are part of the tutorial sequence, even if a name contains a typo.

## README Purpose

Each step README is the main teaching document for that stage.

It should answer:

- What concept is introduced in this step?
- What changed compared with the previous step?
- What should the learner implement or inspect?
- How should the learner run the step?
- How can the learner verify the expected behavior?

Avoid describing every file in the folder unless it is necessary for the step. Focus on the concept and the learner’s task.

## README Format

Use this structure for step READMEs:

````md
# Step N — Short Title

One short paragraph explaining the concept introduced by this step.

Mention how this step builds on the previous one.

## Prerequisites

- Python `3.13+`
- `uv`
- `task`
- An OpenRouter API key

## Setup

From this step directory:

```sh
uv sync
```

Set the API key:

```sh
export OPENROUTER_API_KEY="your-api-key"
```

Optional base URL override:

```sh
export OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"
```

## Run

```sh
task run -- "'example prompt'"
```

Or directly:

```sh
uv run -m app.main -p "example prompt"
```

## Your Goal

Describe the learner’s task in concrete terms.

Use bullets or numbered steps when the implementation has multiple required parts.

## Expected Behavior

Describe what should happen when the implementation is correct.

Include exact stdout expectations when relevant.

## Test Prompt

Optional known-good prompt or `task test` command.

## Notes

Optional constraints, warnings, or debugging guidance.
````

## README Writing Style

Write READMEs as teaching material, not internal documentation.

Style rules:

- Be direct and concise.
- Prefer short paragraphs and bullet lists.
- Explain one concept per step.
- Make the learner’s task explicit.
- Include commands that can be copied and run.
- Use exact names for tools, files, arguments, and API fields.
- Keep stdout/stderr expectations explicit.
- Avoid long background sections unless they are needed to complete the task.
- Avoid describing implementation details that the learner should discover by reading code.
- Avoid vague goals like “improve the agent”; say exactly what must change.

Tone:

- Practical.
- Beginner-friendly.
- Precise.
- Challenge-oriented, but not cryptic.

## Adjusting Existing READMEs

When updating a README:

1. Preserve the step’s position in the tutorial sequence.
2. Keep only the information needed for that step.
3. Remove duplicated instructions from unrelated steps.
4. Ensure the title, goal, run command, and expected behavior match the current step.
5. Keep command examples consistent with the step’s `Taskfile.yml`.
6. If the README includes tester expectations, make stdout requirements exact.
7. If the implementation changes, update the README in the same step folder.

Do not rewrite all READMEs for style consistency unless explicitly asked. Make surgical updates.

## Taskfile Convention

Each step should include a `Taskfile.yml` with:

- A default task that lists available tasks.
- A `run` task that runs the project.
- A `test` task when the step has a reusable test prompt or validation command.
- `desc` fields for the main tasks.

The primary run command should remain:

```sh
task run -- "'prompt text'"
```

The direct Python command should remain:

```sh
uv run -m app.main -p "prompt text"
```

## Development Conventions

- Each step should add one major concept.
- Later steps should keep previous behavior working.
- Keep the code minimal and readable for learners.
- Avoid speculative flexibility or unnecessary abstractions.
- Prefer explicit code when it helps teach the concept.
- Keep stdout clean for the final expected answer.
- Send debugging or trace output to stderr.
- Keep OpenRouter-compatible API usage consistent across steps unless changing it is the point of the step.

## Environment

Required:

```sh
export OPENROUTER_API_KEY="your-api-key"
```

Optional:

```sh
export OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"
```
