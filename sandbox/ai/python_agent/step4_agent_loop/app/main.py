import argparse
import json
import os
from typing import cast

from openai import OpenAI
from openai.types.chat import (
    ChatCompletionAssistantMessageParam,
    ChatCompletionFunctionToolParam,
    ChatCompletionMessageParam,
    ChatCompletionMessageToolCall,
    ChatCompletionToolMessageParam,
)

API_KEY = os.getenv("OPENROUTER_API_KEY")
BASE_URL = os.getenv("OPENROUTER_BASE_URL", default="https://openrouter.ai/api/v1")

tool_read_spec: ChatCompletionFunctionToolParam = {
    "type": "function",
    "function": {
        "name": "Read",
        "description": "Read and return the contents of a file",
        "parameters": {
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The path to the file to read",
                }
            },
            "required": ["file_path"],
        },
    },
}


def main():
    p = argparse.ArgumentParser()
    p.add_argument("-p", required=True)
    args = p.parse_args()

    if not API_KEY:
        raise RuntimeError("OPENROUTER_API_KEY is not set")

    client = OpenAI(api_key=API_KEY, base_url=BASE_URL)

    messages: list[ChatCompletionMessageParam] = [{"role": "user", "content": args.p}]

    for _ in range(15):
        chat = client.chat.completions.create(
            model="minimax/minimax-m2.7",
            messages=messages,
            tools=[tool_read_spec],
        )

        if not chat.choices or len(chat.choices) == 0:
            raise RuntimeError("no choices in response")

        message = chat.choices[0].message
        messages.append(cast(ChatCompletionAssistantMessageParam, message))

        tool_calls = message.tool_calls

        if tool_calls:
            for tool_call in tool_calls:
                # read file
                if tool_call.type != "function":
                    raise RuntimeError(f"unsupported tool call type: {tool_call.type}")

                tool_call_function = cast(ChatCompletionMessageToolCall, tool_call)

                tool_function_args = tool_call_function.function.arguments
                tool_function_filepath = json.loads(tool_function_args)["file_path"]

                try:
                    print(f'reading `{tool_function_filepath}`')
                    with open(tool_function_filepath, "r", encoding="utf-8") as f:
                        content = f.read()
                except OSError as e:
                    content = f"Error reading file: {e}"

                messages.append(
                    cast(
                        ChatCompletionToolMessageParam,
                        {
                            "role": "tool",
                            "tool_call_id": tool_call.id,
                            "content": content,
                        },
                    )
                )
        else:
            print(chat.choices[0].message.content)
            break

if __name__ == "__main__":
    main()
