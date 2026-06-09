# Example MCP

A minimal Python MCP server runnable with `uvx`.

## Run

```bash
uvx --from . example-mcp
```

Or with Task:

```bash
task run
```

## Tools

- `flip-coin`: returns `heads` or `tails`.
- `roll-die`: rolls a die with `sides` sides and returns the result.

## Install

```json
"context_servers": {
  "example-mcp": {
    "enabled": true,
    "remote": false,
    "command": "uvx",
    "args": [
      "--from",
      "/Users/danielvieira/Sandbox/code.git/sandbox/python/py_mcp",
      "example-mcp"
    ],
    "env": {}
  }
}
```