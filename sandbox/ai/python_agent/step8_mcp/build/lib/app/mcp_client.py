import json
from contextlib import AsyncExitStack
from pathlib import Path
from typing import Any

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
from openai.types.chat import ChatCompletionFunctionToolParam


class MCPToolbox:
    def __init__(self, config_path: str | None):
        self.config_path = config_path
        self.exit_stack = AsyncExitStack()
        self.sessions: dict[str, ClientSession] = {}
        self.tool_to_server: dict[str, str] = {}
        self.tool_specs: list[ChatCompletionFunctionToolParam] = []

    async def __aenter__(self):
        if self.config_path:
            await self.connect_from_config(self.config_path)
        return self

    async def __aexit__(self, exc_type, exc, tb):
        await self.exit_stack.aclose()

    async def connect_from_config(self, config_path: str) -> None:
        config = json.loads(Path(config_path).read_text(encoding="utf-8"))
        servers = config.get("mcpServers", {})

        for server_name, server_config in servers.items():
            command = server_config["command"]
            args = server_config.get("args", [])
            env = server_config.get("env")

            params = StdioServerParameters(command=command, args=args, env=env)
            read_stream, write_stream = await self.exit_stack.enter_async_context(
                stdio_client(params)
            )
            session = await self.exit_stack.enter_async_context(
                ClientSession(read_stream, write_stream)
            )
            await session.initialize()

            self.sessions[server_name] = session
            await self._register_server_tools(server_name, session)

    async def _register_server_tools(
        self, server_name: str, session: ClientSession
    ) -> None:
        response = await session.list_tools()

        for tool in response.tools:
            tool_name = f"mcp__{server_name}__{tool.name}"
            self.tool_to_server[tool_name.lower()] = server_name
            self.tool_specs.append(
                {
                    "type": "function",
                    "function": {
                        "name": tool_name,
                        "description": tool.description or f"MCP tool {tool.name}",
                        "parameters": tool.inputSchema,
                    },
                }
            )

    def get_tools(self) -> list[ChatCompletionFunctionToolParam]:
        return self.tool_specs

    def has_tool(self, tool_name: str) -> bool:
        return tool_name.lower() in self.tool_to_server

    async def execute(self, tool_name: str, arguments: str) -> str:
        normalized_tool_name = tool_name.lower()
        server_name = self.tool_to_server[normalized_tool_name]
        original_tool_name = tool_name.split("__", 2)[2]
        session = self.sessions[server_name]
        parsed_arguments = json.loads(arguments or "{}")

        result = await session.call_tool(original_tool_name, parsed_arguments)
        return _stringify_tool_result(result.content)


def _stringify_tool_result(content: list[Any]) -> str:
    parts: list[str] = []

    for item in content:
        text = getattr(item, "text", None)
        if text is not None:
            parts.append(text)
        else:
            parts.append(json.dumps(item.model_dump(), ensure_ascii=False))

    return "\n".join(parts)
