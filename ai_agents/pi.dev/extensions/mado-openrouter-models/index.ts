import type { ExtensionAPI, ExtensionContext } from "@earendil-works/pi-coding-agent";

/** Hide OpenRouter from Pi's /model and /models pickers without touching its credentials. */
export default function madoOpenRouterModels(pi: ExtensionAPI) {
	let hidden = false;

	function setVisible(visible: boolean, ctx: ExtensionContext): void {
		if (visible === !hidden) return;

		if (visible) {
			pi.unregisterProvider("openrouter");
			hidden = false;
			return;
		}

		if (ctx.model?.provider === "openrouter") {
			throw new Error("Switch away from the active OpenRouter model before hiding it.");
		}
		if (ctx.modelRegistry.getRegisteredProviderIds().includes("openrouter")) {
			throw new Error("Another extension manages the OpenRouter provider; refusing to replace it.");
		}

		// An empty replacement list hides the models but keeps the built-in provider's
		// authentication, streaming, and any models.json settings intact for when restored.
		pi.registerProvider("openrouter", { models: [] });
		hidden = true;
	}

	pi.on("session_start", (_event, ctx) => {
		try {
			setVisible(false, ctx);
		} catch (error) {
			ctx.ui.notify(`OpenRouter models remain visible: ${String(error)}`, "warning");
		}
	});

	pi.registerCommand("mado-openrouter-models", {
		description: "Toggle OpenRouter models in /model and /models (on, off, status)",
		getArgumentCompletions: (prefix) =>
			["on", "off", "status"]
				.filter((value) => value.startsWith(prefix))
				.map((value) => ({ value, label: value })),
		handler: async (args, ctx) => {
			const action = args.trim().toLowerCase() || (hidden ? "on" : "off");
			if (action === "status") {
				ctx.ui.notify(`OpenRouter models are ${hidden ? "hidden" : "visible"}.`, "info");
				return;
			}
			if (action !== "on" && action !== "off") {
				ctx.ui.notify("Usage: /mado-openrouter-models [on|off|status]", "warning");
				return;
			}
			try {
				setVisible(action === "on", ctx);
				ctx.ui.notify(`OpenRouter models are ${hidden ? "hidden" : "visible"}. Reopen /models to see the change.`, "info");
			} catch (error) {
				ctx.ui.notify(String(error), "warning");
			}
		},
	});
}
