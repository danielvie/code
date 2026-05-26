# Step 2 — Describe a Tool

This step introduces tool definitions: metadata you send to the model so it knows what tools are available in the session.

Compared with Step 1, the app now includes a `Read` tool specification and passes it to the chat completion request.

## Prerequisites

- Python `3.13+`
- [`uv`](https://docs.astral.sh/uv/)
- [`task`](https://taskfile.dev/)
- An OpenRouter API key

## Setup

From this directory:

```sh
uv sync
```

Set your API key:

```sh
export OPENROUTER_API_KEY="your-api-key"
```

Optional: override the OpenRouter base URL if needed:

```sh
export OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"
```

## Run

Run with your own prompt:

```sh
task run -- "'how many tools are available in this session?'"
```

Or run the module directly:

```sh
uv run -m app.main -p "how many tools are available in this session?"
```

## Test prompt

This step includes a `test` task with a reusable prompt variable:

```sh
task test
```

You can inspect `Taskfile.yml` to change the prompt used by the test task.

## Your goal

Review `app/main.py` and identify how the tool is described to the model:

- the tool name
- the tool description
- the expected parameters
- where the tool list is attached to the chat request

At this stage, the tool is only described to the model. Implementing actual tool execution comes later.
