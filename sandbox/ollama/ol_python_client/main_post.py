import json
import sys

import requests


def ollama_post_request(message: str):
    """Chat using a raw POST request to the Ollama REST API."""
    print("\n--- Raw POST Request ---")

    response = requests.post(
        "http://localhost:11434/api/chat",
        json={
            "model": "gemma3:4b",
            "messages": [{"role": "user", "content": message}],
            "stream": True,
        },
        stream=True,
    )

    print(f"prompt:\n{message}\n")

    print("reply: ")
    for line in response.iter_lines():
        if line:
            chunk = json.loads(line)
            print(chunk["message"]["content"], end="", flush=True)


def main():

    msg = "hi"
    if len(sys.argv) > 1:
        msg = sys.argv[1]

    ollama_post_request(msg)


if __name__ == "__main__":
    main()
