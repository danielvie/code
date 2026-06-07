In this stage, you'll add support for MCP tools.

### MCP Tools

So far, your program can use local tools defined in the agent project: `Read`, `Write`, and `Bash`.

This works for tools you build directly into the agent, but falls short when you want to load tools from an external process.

MCP (Model Context Protocol) lets the agent start external MCP servers, discover the tools they expose, and call those tools through an MCP client session.

For this stage, you'll keep the existing local tools working, then add MCP server tools to the same tool list sent to the model.

### MCP Config

Load MCP servers from a local `mpc_config.json` file when it exists.

If `mpc_config.json` does not exist, the agent should still run with only the local tools.

The config defines servers under `mcpServers`:

```json
{
  "mcpServers": {
    "example": {
      "command": "uv",
      "args": ["run", "python", "-m", "mcp_servers.example_mcp_server"]
    },
    "random-stuff": {
      "command": "uv",
      "args": ["run", "python", "-m", "mcp_servers.random_mcp_server"]
    }
  }
}
```

Each server entry should provide:

* `command`: The executable used to start the MCP server
* `args`: The command arguments
* `env`: Optional environment variables for the server process

### Loading MCP Tools

For each configured MCP server:

1. **Start the server**: Use the server's `command` and `args` to start the MCP server process.

2. **Create a client session**: Connect to the server with an MCP stdio client, then initialize a `ClientSession`.

3. **List the server tools**: Call `list_tools()` on the MCP session to discover the tools exposed by the server.

4. **Convert the tools**: Convert each MCP tool into an OpenAI function tool specification.

5. **Add the tools to the request**: Send the converted MCP tools in the same `tools` array as `Read`, `Write`, and `Bash`.

MCP server tool names should be exposed to the model with this naming pattern:

```text
mcp__<server_name>__<tool_name>
```

For example, a tool named `random_number` from the `random-stuff` server should be advertised as:

```text
mcp__random-stuff__random_number
```

This avoids name collisions between local tools and tools from different MCP servers.

Here is the shape of a converted MCP tool:

```python
{
    "type": "function",
    "function": {
        "name": f"mcp__{server_name}__{mcp_tool.name}",
        "description": mcp_tool.description or f"MCP tool {mcp_tool.name}",
        "parameters": mcp_tool.inputSchema,
    },
}
```

### Executing MCP Tool Calls

When the model requests a tool call:

1. **Check the tool name**: Read `tool_call.function.name` and determine whether it is an MCP tool name.

2. **Parse the MCP name**: Extract the server name and original MCP tool name from `mcp__<server_name>__<tool_name>`.

3. **Parse the arguments**: Parse `tool_call.function.arguments` as JSON.

4. **Call the MCP tool**: Use the active MCP client session for that server and call the original MCP tool name.

5. **Return the result**: Append the MCP tool result to `messages` as a `role: "tool"` message using the original `tool_call_id`.

Local tool calls should continue to be dispatched to the existing local tool handlers.

### Agent Loop Integration

MCP tools should behave like local tools inside the agent loop:

```python
messages = [{ role: "user", content: prompt }]

start MCP servers
load MCP tools
tools = local_tools + mcp_tools

loop:
    response = call_api(messages, tools)
    append response message to messages

    if response has no tool_calls:
        print response.content
        cleanup MCP resources
        exit

    for each tool_call in response.tool_calls:
        if tool_call is MCP tool:
            result = execute_mcp_tool(tool_call)
        else:
            result = execute_local_tool(tool_call)

        append {
            role: "tool",
            tool_call_id: tool_call.id,
            content: result
        } to messages
```

Let's walk through the MCP-specific parts of this loop:

1. **Initialize MCP resources**: Before entering the loop, read `mpc_config.json` if it exists, start each configured server, initialize a session for each server, and discover its tools.

2. **Build the full tool list**: Combine the existing local tool list with the converted MCP tool list. This full list is sent with every chat completion request.

3. **Record the assistant's response**: Add the assistant message to `messages` before executing any requested tool calls, just like the previous agent loop stage.

4. **Dispatch tool calls**: If the requested tool name starts with the MCP naming pattern, route it to the matching MCP session. Otherwise, route it to the local tool dispatcher.

5. **Append tool results**: Add every local or MCP tool result to `messages` with `role: "tool"`, the matching `tool_call_id`, and the result content.

6. **Clean up resources**: When the agent exits, close the MCP client sessions and server process resources.

### Tests

The tester may provide an `mpc_config.json` with MCP servers such as:

* `example`, exposing a `list_project_files` tool
* `random-stuff`, exposing random number, coin flip, and die roll tools

The tester may then execute your program like this:

```bash
$ ./your_program.sh -p "Use the example MCP list_project_files tool. Print only the tool result."
<project files>
```

Or:

```bash
$ ./your_program.sh -p "Call the random MCP tool. Print the tool result exactly as returned, including the RANDOM_MCP_USED prefix and number."
RANDOM_MCP_USED: <number>
```

The tester will verify that:

* Local tools still work
* MCP tools are advertised to the model
* MCP tool calls are routed to the correct server session
* MCP tool results are appended as tool messages
* Your program exits with exit code `0`

### Notes

* Keep `Read`, `Write`, and `Bash` working exactly as before.
* If `mpc_config.json` is missing, run with only local tools.
* MCP tool names must use `mcp__<server_name>__<tool_name>`.
* Do not print MCP tool results directly to stdout while the loop is running. Print to stdout only when the final model response is ready.
* You can print intermediate debugging information to stderr.
* Use an async context manager or equivalent cleanup mechanism so MCP sessions and server processes are closed when the agent exits.
