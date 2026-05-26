# Step 1 — Hello World Agent

This step introduces the smallest runnable Python agent: send a prompt to a chat model and print the model response.

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

Use the `run` task and pass a prompt with `-p`:

```sh
task run -- "'hello world'"
```

Or run the module directly:

```sh
uv run -m app.main -p "hello world"
```

## Your goal

Open `app/main.py`, find the first-stage TODO, and make the program print the model response instead of only writing debug output.
