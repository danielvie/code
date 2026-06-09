# rs_mcp

Minimal Rust MCP example server over stdio.

It exposes two read-only online tools:

- `weather_now`: fetches current weather for a city using Open-Meteo.
- `public_ip`: fetches the public IP address seen by an external service.

## Run

```sh
task run
```

Or directly:

```sh
cargo run
```

## Example MCP requests

Send one JSON-RPC request per line.

Current weather:

```json
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"weather_now","arguments":{"city":"Amsterdam"}}}
```

Public IP:

```json
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"public_ip","arguments":{}}}
```

## Install

```json
"rs-mcp-example": {
  "command": "/Users/danielvieira/Sandbox/code.git/sandbox/rust/rs_mcp/target/release/rs_mcp",
  "args": [],
}
```