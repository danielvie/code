import json

import requests
from ollama import chat

MODEL = "gemma3:4b"


def prompt_loop(send_message):
    """Run an interactive prompt loop. Calls send_message(user_input) for each message."""
    while True:
        user_input = input("\nYou: ").strip()

        if user_input.lower() == "q":
            print("Goodbye!")
            break

        if not user_input:
            print("Please enter a message, or type 'q' to quit.")
            continue

        print("\nAssistant: ", end="", flush=True)
        send_message(user_input)


def ollama_streaming():
    """Chat using the ollama library with streaming enabled."""
    print("\n--- Ollama Streaming ---")

    def send(message):
        stream = chat(
            model=MODEL,
            messages=[{"role": "user", "content": message}],
            stream=True,
        )
        for chunk in stream:
            print(chunk.message.content, end="", flush=True)
        print()

    prompt_loop(send)


def ollama_non_streaming():
    """Chat using the ollama library without streaming."""
    print("\n--- Ollama Non-Streaming ---")

    def send(message):
        response = chat(
            model=MODEL,
            messages=[{"role": "user", "content": message}],
        )
        print(response.message.content)

    prompt_loop(send)


def ollama_post_request():
    """Chat using a raw POST request to the Ollama REST API."""
    print("\n--- Raw POST Request ---")

    def send(message):
        response = requests.post(
            "http://localhost:11434/api/chat",
            json={
                "model": MODEL,
                "messages": [{"role": "user", "content": message}],
                "stream": True,
            },
            stream=True,
        )
        for line in response.iter_lines():
            if line:
                chunk = json.loads(line)
                print(chunk["message"]["content"], end="", flush=True)
        print()

    prompt_loop(send)


def ollama_post_request_with_session():
    """Chat using a raw POST request to the Ollama REST API."""
    print("\n--- Raw POST Request ---")

    session = requests.Session()

    def send(message):
        response = session.post(
            "http://localhost:11434/api/chat",
            json={
                "model": MODEL,
                "messages": [{"role": "user", "content": message}],
                "stream": True,
            },
            stream=True,
        )
        for line in response.iter_lines(chunk_size=1):
            if line:
                chunk = json.loads(line)
                print(chunk["message"]["content"], end="", flush=True)
        print()

    prompt_loop(send)


OPTIONS = {
    "1": ("Ollama Streaming", ollama_streaming),
    "2": ("Ollama Non-Streaming", ollama_non_streaming),
    "3": ("Raw POST Request", ollama_post_request),
    "4": ("Raw POST Request With Session", ollama_post_request_with_session),
}


def main():
    print("Select a chat method:\n")
    for key, (label, _) in OPTIONS.items():
        print(f"  {key}. {label}")

    choice = input("\nEnter your choice (1-3): ").strip()

    if choice in OPTIONS:
        _, func = OPTIONS[choice]
        func()
    else:
        print(f"Invalid choice: {choice}")


if __name__ == "__main__":
    main()
