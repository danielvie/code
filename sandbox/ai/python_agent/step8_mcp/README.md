In this stage, you'll add support for MCP tools.

### MCP Tools

MCP (Model Context Protocol) lets the agent load tools from external MCP servers instead of only using tools defined inside `app/main.py`.

The goal is to keep the existing local tools (`Read`, `Write`, and `Bash`) working, then add MCP server tools to the same tool list sent to the model.

### MCP Config

Load MCP servers from a local `mpc.json` file when it exists. The file defines servers under `mcpServers`:

```json
{
  "mcpServers": {
    "example": {
      "command": "uv",
      "args": ["run", "python", "-m", "mcp_servers.example_mcp_server"]
    },
    "random": {
      "command": "uv",
      "args": ["run", "python", "-m", "mcp_servers.random_mcp_server"]
    }
  }
}
```

Each server entry should provide:

* `command`: the executable used to start the MCP server
* `args`: the command arguments

If `mpc.json` does not exist, the agent should still run with only the local tools.

### Loading MCP Tools

For each configured MCP server:

1. Start the server using its `command` and `args`
2. Initialize an MCP client session
3. List the tools exposed by the server
4. Convert each MCP tool into an OpenAI function tool specification
5. Add those converted tools to the tools sent with each chat completion request

MCP server tool names should be exposed to the model with this naming pattern:

```text
mcp__<server_name>__<tool_name>
```

For example, a tool named `random_number` from the `random` server should be advertised as:

```text
mcp__random__random_number
```

This avoids name collisions between local tools and tools from different MCP servers.

### Executing MCP Tool Calls

When the model requests a tool call:

1. Check whether the tool name starts with `mcp__`
2. Parse the server name and MCP tool name from `mcp__<server_name>__<tool_name>`
3. Parse the tool call arguments as JSON
4. Call the matching MCP server tool through the active MCP client session
5. Return the MCP tool result as a tool message using the original `tool_call_id`

Local tool calls should continue to be dispatched to the existing local tool handlers.

### Agent Loop Integration

MCP tools should behave like local tools inside the agent loop:

1. Send local tools and MCP tools in the chat completion request
2. Append the assistant response to `messages`
3. Execute all requested local or MCP tool calls
4. Append each result as a `role: "tool"` message
5. Continue until the model returns a final response without tool calls

### Your Goal

Review `app/main.py` and implement the MCP integration by adding:

* config loading from `mpc.json`
* MCP server startup and session initialization
* MCP tool discovery
* conversion from MCP tool schemas to OpenAI function tool definitions
* dispatch for `mcp__<server_name>__<tool_name>` tool calls
* cleanup for MCP client/server resources when the agent exits

The final agent should support both local tools and MCP server tools in the same conversation.
