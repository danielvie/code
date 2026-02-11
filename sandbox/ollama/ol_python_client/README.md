# Ollama Python Client

A simple Python client to interact with a local Ollama server using streaming chat.

## Prerequisites

1. **Install Ollama**: Download and install from [ollama.com](https://ollama.com/).
2. **Python 3.12+**: Ensure you have Python installed.

## Getting Started

### 1. Start the Ollama Server

On Windows, Ollama usually starts automatically. You can verify it is running by visiting `http://localhost:11434` in your browser or running:

```powershell
curl http://localhost:11434
```

If it is not running, start it manually:

```powershell
ollama serve
```

*Note: If you get a "port already in use" error, the server is already running in the background.*

### 2. Pull a Model

Before running the code, you must download a model. This project uses `gemma3:4b`:

```powershell
ollama pull gemma3:4b
```

### 3. Install Dependencies

Install the official Ollama Python library:

```powershell
uv add ollama
# OR
pip install ollama
```

### 4. Run the Client

Execute the main script to send a chat message and receive a streamed reply:

```powershell
uv run main.py
# OR
python main.py
```

## Example Usage

The `main.py` script uses **streaming** to print the model's response token-by-token as it is generated:

```python
from ollama import chat


def main():
    stream = chat(
        model="gemma3:4b",
        messages=[
            {
                "role": "user",
                "content": "Why is the sky blue? Answer in two sentences.",
            }
        ],
        stream=True,
    )

    for chunk in stream:
        print(chunk.message.content, end="", flush=True)
    print()


if __name__ == "__main__":
    main()
```

### Streaming vs Non-Streaming

The example above uses `stream=True`, which returns chunks incrementally as the model generates tokens. This gives a more responsive feel, especially for longer replies.

If you prefer to wait for the full response before printing, use the non-streaming approach instead:

```python
from ollama import chat

response = chat(
    model="gemma3:4b",
    messages=[
        {"role": "user", "content": "Why is the sky blue?"}
    ],
)

print(response.message.content)
```
