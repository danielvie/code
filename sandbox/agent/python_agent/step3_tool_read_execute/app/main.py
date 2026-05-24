import json
from typing import cast
from openai.types.chat import ChatCompletionFunctionToolParam, ChatCompletionMessageToolCall
import argparse
import os

from openai import OpenAI

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
          "description": "The path to the file to read"
        }
      },
      "required": ["file_path"]
    }
  }
}


def main():
    p = argparse.ArgumentParser()
    p.add_argument("-p", required=True)
    args = p.parse_args()

    if not API_KEY:
        raise RuntimeError("OPENROUTER_API_KEY is not set")

    client = OpenAI(api_key=API_KEY, base_url=BASE_URL)

    chat = client.chat.completions.create(
        # model="moonshotai/kimi-k2.5",
        model="minimax/minimax-m2.7",
        messages=[{"role": "user", "content": args.p}],
        tools=[tool_read_spec],
    )

    if not chat.choices or len(chat.choices) == 0:
        raise RuntimeError("no choices in response")

    message = chat.choices[0].message
    tool_calls = message.tool_calls

    if tool_calls:
        tool_call = tool_calls[0]
    
        # read file
        if tool_call.type != "function":
            raise RuntimeError(f"unsupported tool call type: {tool_call.type}")
            
        tool_call_function = cast(ChatCompletionMessageToolCall, tool_call)
            
        tool_function_args = tool_call_function.function.arguments
        tool_function_filepath = json.loads(tool_function_args)["file_path"]

        with open(tool_function_filepath, "r", encoding="utf-8") as f:
            content = f.read()

        print(content)
    else:
        print(chat.choices[0].message.content)

if __name__ == "__main__":
    main()
