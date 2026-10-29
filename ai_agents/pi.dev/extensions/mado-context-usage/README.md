# mado-context-usage

Shows the current context size in a footer status line, for example `context: 92.8k tokens`. Pi's built-in footer already shows the context limit, so this status does not repeat it. The status line puts duration first, context second, and MCP information third. Existing token totals, context percentage, and model label remain unchanged.

Load for one session:

```sh
pi --extension /path/to/ai_agents/pi.dev/extensions/mado-context-usage/index.ts
```

To load for every session, copy `index.ts` to `~/.pi/agent/extensions/mado-context-usage.ts`, then run `/reload` in an open Pi session.

The size comes from `ctx.getContextUsage()`. While the assistant streams, the status updates about every 200 ms with a `~` prefix. That live value adds Pi's character-based estimate of the unfinished response to the last known context size. After the response, Pi's provider usage replaces the estimate. Tool results appear once the turn ends, not as their output streams. This is context size, not the lifetime token total. Immediately after compaction, Pi may report `?` until the next model response.
