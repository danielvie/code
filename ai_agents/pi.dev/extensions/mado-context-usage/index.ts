import type { ExtensionAPI, ExtensionContext } from "@earendil-works/pi-coding-agent";

// Footer statuses sort by key: duration, context, then MCP.
const STATUS_KEY = "mado-live-context";

/** Show the current context size without replacing Pi's built-in footer. */
export default function madoContextUsage(pi: ExtensionAPI) {
	let baseTokens: number | null = null;
	let contextWindow: number | undefined;
	let lastStatus: string | undefined;
	let lastStreamUpdate = 0;

	const format = (count: number) =>
		count < 1000 ? String(count) : `${(count / 1000).toFixed(1)}k`;

	// Match Pi's rough chars/4 estimate for an unfinished assistant message.
	function estimateStreamingTokens(content: Array<{ type: string; text?: string; thinking?: string; name?: string; arguments?: unknown }>): number {
		let chars = 0;
		for (const block of content) {
			if (block.type === "text") chars += block.text?.length ?? 0;
			else if (block.type === "thinking") chars += block.thinking?.length ?? 0;
			else if (block.type === "toolCall") chars += (block.name?.length ?? 0) + JSON.stringify(block.arguments ?? {}).length;
		}
		return Math.ceil(chars / 4);
	}

	function show(ctx: ExtensionContext, tokens: number | null | undefined, estimated = false): void {
		if (ctx.mode !== "tui") return;
		const status = contextWindow
			? `context: ${tokens == null ? "?" : `${estimated ? "~" : ""}${format(tokens)}`} tokens`
			: undefined;
		if (status !== lastStatus) {
			ctx.ui.setStatus(STATUS_KEY, status);
			lastStatus = status;
		}
	}

	function update(ctx: ExtensionContext): void {
		const usage = ctx.getContextUsage();
		baseTokens = usage?.tokens ?? null;
		contextWindow = usage?.contextWindow ?? ctx.model?.contextWindow;
		show(ctx, baseTokens);
	}

	pi.on("session_start", (_event, ctx) => {
		ctx.ui.setStatus("mado-context-usage", undefined);
		update(ctx);
	});

	for (const event of ["agent_settled", "model_select", "session_compact", "session_tree", "turn_start", "turn_end"] as const) {
		pi.on(event, (_event, ctx) => update(ctx));
	}

	// Pi persists the user message before the assistant starts. Keep that as the
	// baseline until the provider returns final usage for this response.
	pi.on("message_start", (event, ctx) => {
		if (event.message.role === "assistant") {
			lastStreamUpdate = 0;
			update(ctx);
		}
	});

	pi.on("message_update", (event, ctx) => {
		if (ctx.mode !== "tui" || event.message.role !== "assistant" || baseTokens == null) return;
		const now = Date.now();
		if (now - lastStreamUpdate < 200) return;
		lastStreamUpdate = now;
		show(ctx, baseTokens + estimateStreamingTokens(event.message.content), true);
	});

	pi.on("session_shutdown", (_event, ctx) => {
		baseTokens = null;
		contextWindow = undefined;
		lastStatus = undefined;
		ctx.ui.setStatus(STATUS_KEY, undefined);
		ctx.ui.setStatus("mado-context-usage", undefined);
	});
}
