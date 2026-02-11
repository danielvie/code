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
