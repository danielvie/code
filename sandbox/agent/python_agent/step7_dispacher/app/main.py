import argparse
import os
from typing import cast

from openai import OpenAI
from openai.types.chat import (
    ChatCompletionAssistantMessageParam,
    ChatCompletionMessageParam,
    ChatCompletionToolMessageParam,
)

from app.tools import tool_get_tools, tool_executer

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

    while True:
        response = client.chat.completions.create(
            # model="moonshotai/kimi-k2.5",
            model="minimax/minimax-m2.7",
            messages=messages,
            tools=tool_get_tools(),
        )

        if not response.choices or len(response.choices) == 0:
            raise RuntimeError("no choices in response")

        message = response.choices[0].message
        messages.append(cast(ChatCompletionAssistantMessageParam, message))

        tool_calls = message.tool_calls

        # check tools
        if tool_calls:
            for tool_call in tool_calls:
                result = tool_executer(tool_call=tool_call)
                messages.append(cast(ChatCompletionToolMessageParam, {
                    "role": "tool",
                    "tool_call_id": tool_call.id,
                    "content": result, 
                }))
        else:
            print(response.choices[0].message.content)
            break

if __name__ == "__main__":
    main()
