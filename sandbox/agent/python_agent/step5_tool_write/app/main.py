import argparse
import json
import os
from typing import cast

from openai import OpenAI
from openai.types.chat import (
    ChatCompletionAssistantMessageParam,
    ChatCompletionMessageParam,
    ChatCompletionMessageToolCall,
    ChatCompletionToolMessageParam,
)
from app.tools import tool_read, tool_read_spec, \
                      tool_write_spec, tool_write

API_KEY = os.getenv("OPENROUTER_API_KEY")
BASE_URL = os.getenv("OPENROUTER_BASE_URL", default="https://openrouter.ai/api/v1")

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
            # model="moonshotai/kimi-k2.5",
            model="minimax/minimax-m2.7",
            messages=messages,
            tools=[tool_read_spec, tool_write_spec],
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
                tool_function_name = tool_call_function.function.name.lower()

                result = ''
                match(tool_function_name):
                    case "read":
                        result = tool_read(tool_call_function)
                    case "write":
                        result = tool_write(tool_call_function)

                messages.append(cast(ChatCompletionToolMessageParam, {
                    "role": "tool",
                    "tool_call_id": tool_call.id,
                    "content": result, 
                }))
        else:
            print(chat.choices[0].message.content)
            break

if __name__ == "__main__":
    main()
