import argparse
import os
from typing import cast

from openai import OpenAI
from openai.types.chat import (
    ChatCompletionAssistantMessageParam,
    ChatCompletionMessageParam,
    ChatCompletionToolMessageParam,
)
from app.tools import tool_read_spec, tool_write_spec, tool_bash_spec, tool_executer

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
        response = client.chat.completions.create(
            model="anthropic/claude-haiku-4.5",
            messages=messages,
            tools=[tool_read_spec, tool_write_spec, tool_bash_spec],
        )
        message = response.choices[0].message

        # append response to messages
        messages.append(cast(ChatCompletionAssistantMessageParam, message))
        
        tool_calls = message.tool_calls
    
        # check tools
        if tool_calls:
            for tool_call in tool_calls:
                result = tool_executer(tool_call)
                messages.append(cast(ChatCompletionToolMessageParam, {
                                "role": "tool",
                                "tool_call_id": tool_call.id,
                                "content": result,
                            }))
        else:
            print(response.choices[0].message.content)
            break
    
        # TODO: Uncomment the following line to pass the first stage


if __name__ == "__main__":
    main()
