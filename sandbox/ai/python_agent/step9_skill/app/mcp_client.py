import json
from contextlib import AsyncExitStack
from pathlib import Path
from typing import Any

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
from openai.types.chat import ChatCompletionFunctionToolParam


class MCPToolbox:
    """Small adapter that makes MCP server tools available to the agent.

    MCP servers are separate processes. This class starts them, asks which tools
    they provide, converts those tools to OpenAI tool specs, and later routes
    tool calls back to the right MCP server.
    """

    def __init__(self, config_path: str | None):
        self.config_path = config_path

        # Keeps all server processes/sessions open while the agent is running.
        self.exit_stack = AsyncExitStack()

        # server name -> active MCP session
        self.server_sessions: dict[str, ClientSession] = {}

        # OpenAI tool name -> server name
        # Example: "mcp__random__random_number" -> "random"
        self.tool_server_names: dict[str, str] = {}

        # Tool definitions passed to the model.
        self.openai_tools: list[ChatCompletionFunctionToolParam] = []

    async def __aenter__(self):
        if self.config_path:
            await self.load_servers(self.config_path)
        return self

    async def __aexit__(self, exc_type, exc, tb):
        await self.exit_stack.aclose()

    async def load_servers(self, config_path: str) -> None:
        config = _read_json(config_path)

        for server_name, server_config in config.get("mcpServers", {}).items():
            session = await self._start_server(server_config)
            self.server_sessions[server_name] = session

            await self._add_server_tools(server_name, session)

    async def _start_server(self, server_config: dict[str, Any]) -> ClientSession:
        params = StdioServerParameters(
            command=server_config["command"],
            args=server_config.get("args", []),
            env=server_config.get("env"),
        )

        read_stream, write_stream = await self.exit_stack.enter_async_context(
            stdio_client(params)
        )
        session = await self.exit_stack.enter_async_context(
            ClientSession(read_stream, write_stream)
        )
        await session.initialize()

        return session

    async def _add_server_tools(self, server_name: str, session: ClientSession) -> None:
        tools_response = await session.list_tools()

        for mcp_tool in tools_response.tools:
            openai_tool_name = _make_openai_tool_name(server_name, mcp_tool.name)
            
            self.tool_server_names[openai_tool_name.lower()] = server_name
            self.openai_tools.append(_make_openai_tool_spec(openai_tool_name, mcp_tool))

    def get_tools(self) -> list[ChatCompletionFunctionToolParam]:
        return self.openai_tools

    def has_tool(self, tool_name: str) -> bool:
        return tool_name.lower() in self.tool_server_names

    async def execute(self, tool_name: str, arguments: str) -> str:
        server_name = self.tool_server_names[tool_name.lower()]
        session = self.server_sessions[server_name]

        mcp_tool_name = _get_original_mcp_tool_name(tool_name)
        mcp_arguments = json.loads(arguments or "{}")

        result = await session.call_tool(mcp_tool_name, mcp_arguments)
        return _content_to_text(result.content)


def _read_json(path: str) -> dict[str, Any]:
    return json.loads(Path(path).read_text(encoding="utf-8"))


def _make_openai_tool_name(server_name: str, mcp_tool_name: str) -> str:
    return f"mcp__{server_name}__{mcp_tool_name}"


def _get_original_mcp_tool_name(openai_tool_name: str) -> str:
    return openai_tool_name.split("__", 2)[2]


def _make_openai_tool_spec(
    openai_tool_name: str,
    mcp_tool: Any,
) -> ChatCompletionFunctionToolParam:
    return {
        "type": "function",
        "function": {
            "name": openai_tool_name,
            "description": mcp_tool.description or f"MCP tool {mcp_tool.name}",
            "parameters": mcp_tool.inputSchema,
        },
    }


def _content_to_text(content: list[Any]) -> str:
    parts: list[str] = []

    for item in content:
        if getattr(item, "text", None) is not None:
            parts.append(item.text)
        else:
            parts.append(json.dumps(item.model_dump(), ensure_ascii=False))

    return "\n".join(parts)
