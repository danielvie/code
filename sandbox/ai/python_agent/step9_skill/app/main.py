import argparse
import asyncio
import os
import sys
from pathlib import Path
from typing import cast

from openai import OpenAI
from openai.types.chat import (
    ChatCompletionAssistantMessageParam,
    ChatCompletionMessageParam,
)

from app.mcp_client import MCPToolbox
from app.skills import build_skill_context
from app.tools import tool_executer, tool_get_tools

API_KEY = os.getenv("OPENROUTER_API_KEY")
BASE_URL = os.getenv("OPENROUTER_BASE_URL", default="https://openrouter.ai/api/v1")
MCP_CONFIG_PATH = "mpc_config.json"

async def main():
    p = argparse.ArgumentParser()
    p.add_argument("-p", required=True)
    args = p.parse_args()

    if not API_KEY:
        raise RuntimeError("OPENROUTER_API_KEY is not set")

    client = OpenAI(api_key=API_KEY, base_url=BASE_URL)

    skill_context = build_skill_context(args.p)
    print(f"Skills found: {skill_context.found_names}", file=sys.stderr, flush=True)
    print(f"Skills used: {skill_context.used_names}", file=sys.stderr, flush=True)

    messages: list[ChatCompletionMessageParam] = [
        *skill_context.messages,
        {"role": "user", "content": args.p},
    ]

    mcp_config = MCP_CONFIG_PATH if Path(MCP_CONFIG_PATH).exists() else None

    async with MCPToolbox(mcp_config) as mcp_toolbox:
        tools = tool_get_tools() + mcp_toolbox.get_tools()

        while True:
            response = client.chat.completions.create(
                # model="moonshotai/kimi-k2.6",
                model="minimax/minimax-m2.7",
                messages=messages,
                tools=tools,
            )

            if not response.choices or len(response.choices) == 0:
                raise RuntimeError("no choices in response")

            message = response.choices[0].message
            messages.append(cast(ChatCompletionAssistantMessageParam, message))

            tool_calls = message.tool_calls

            # check tools
            if tool_calls:
                for tool_call in tool_calls:
                    if tool_call.type != "function":
                        raise RuntimeError(
                            f"unsupported tool call type: {tool_call.type}"
                        )

                    tool_name = tool_call.function.name
                    if mcp_toolbox.has_tool(tool_name):
                        print(f"MCP tool called: {tool_name}", file=sys.stderr, flush=True)
                        result = await mcp_toolbox.execute(
                            tool_name=tool_name,
                            arguments=tool_call.function.arguments,
                        )
                    else:
                        result = tool_executer(tool_call=tool_call)

                    messages.append(
                        {
                            "role": "tool",
                            "tool_call_id": tool_call.id,
                            "content": result,
                        }
                    )
            else:
                print('\n')
                print(response.choices[0].message.content)
                break


if __name__ == "__main__":
    asyncio.run(main())
