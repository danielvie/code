# Step 9 — Skills

This step adds local skill support on top of the MCP-enabled agent. Skills are reusable instructions stored on disk; the agent discovers them, matches them by name, and can load their full instructions only when needed.

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

Add support for local skills from:

```text
.agents/skills/<skill-name>/SKILL.md
```

The agent should:

1. Discover local skills under `.agents/skills`.
2. Print discovered skill names to `stderr`.
3. Match a skill when its name appears in the user prompt, case-insensitively.
4. Print matched skill names to `stderr`.
5. Add only matched skill names and descriptions to the initial system prompt.
6. Provide a `LoadSkill` tool that loads the full `SKILL.md` content on demand.

## Expected Behavior

If this local skill exists:

```text
.agents/skills/bad-joke/SKILL.md
```

And you run:

```sh
task run -- "'Use bad-joke to explain local skill support.'"
```

The agent should print status like this to `stderr`:

```text
Skills found: bad-joke
Skills used: bad-joke
```

The initial model request should include only the skill name and description. If the model needs the full instructions, it should call the `LoadSkill` tool with:

```json
{"name": "bad-joke"}
```

If no local skills exist, the agent should still run normally:

```text
Skills found: none
Skills used: none
```

## Test Prompt

Use the included local skill named `bad-joke`, then run:

```sh
task run -- "'Use bad-joke to explain what skills do.'"
```

## Notes

- Skills are local only. Do not load `~/.agents/skills`.
- Matching is intentionally simple for this tutorial: exact skill-name substring, case-insensitive.
- Keep stdout clean for the final model answer. Print skill discovery and matching status to `stderr`.
- Existing local tools and MCP tools should continue to work.
