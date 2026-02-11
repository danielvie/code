import json

import requests

OLLAMA_URL = "http://10.191.234.51:11434"
MODEL = "gemma3:4b"


def chat():
    print("Connected to Ollama. Type 'q' to quit.\n")

    session = requests.Session()
    session.trust_env = False  # ✅ Ignore system proxy settings

    while True:
        user_input = input("You: ").strip()

        if user_input.lower() == "q":
            print("Goodbye!")
            break

        if not user_input:
            continue

        print("Assistant: ", end="", flush=True)

        response = session.post(
            f"{OLLAMA_URL}/api/chat",
            json={
                "model": MODEL,
                "messages": [{"role": "user", "content": user_input}],
                "stream": True,
            },
            stream=True,
            timeout=60,
        )

        response.raise_for_status()

        for line in response.iter_lines():
            if not line:
                continue

            chunk = json.loads(line)

            # Ollama sends multiple message chunks
            if "message" in chunk:
                print(chunk["message"]["content"], end="", flush=True)

            # End of stream
            if chunk.get("done"):
                break

        print("\n")


if __name__ == "__main__":
    chat()
