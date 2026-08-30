import { existsSync, readFileSync, statSync, unwatchFile, watchFile } from "node:fs";
import path from "node:path";
import type { ExtensionAPI, ExtensionContext } from "@earendil-works/pi-coding-agent";
import {
	createBashTool,
	createLocalBashOperations,
	isToolCallEventType,
} from "@earendil-works/pi-coding-agent";
import { Text } from "@earendil-works/pi-tui";
import {
	branchToWorktreeSlug,
	expectedWorktreePath,
	parseWorktreeCommand,
	parseWorktreePorcelain,
	routeToolPath,
	samePath,
	type WorktreeInfo,
} from "./core.ts";

const ENTRY_TYPE = "mado-worktree-router-selection";
const STATUS_KEY = "mado-worktree-router";

interface SelectionEntry {
	version: 1;
	action: "select" | "off";
	path?: string;
	requestedBranch?: string;
	timestamp: number;
}

interface ActiveWorktree {
	path: string;
	requestedBranch: string;
	currentBranch?: string;
	detachedHead?: string;
}

function entryLabel(data: SelectionEntry): string {
	if (data.action === "off") return "main worktree";
	return data.requestedBranch ?? data.path ?? "unknown worktree";
}

function findGitHeadFile(worktreePath: string): string | undefined {
	const dotGit = path.join(worktreePath, ".git");
	if (!existsSync(dotGit)) return undefined;
	try {
		if (statSync(dotGit).isDirectory()) return path.join(dotGit, "HEAD");
		const line = readFileSync(dotGit, "utf8").trim();
		const match = /^gitdir:\s*(.+)$/i.exec(line);
		if (!match) return undefined;
		return path.join(path.resolve(worktreePath, match[1]!), "HEAD");
	} catch {
		return undefined;
	}
}

export default function worktreeRouter(pi: ExtensionAPI) {
	let sessionCwd = process.cwd();
	let mainRoot: string | undefined;
	let active: ActiveWorktree | undefined;
	let watchedHead: string | undefined;
	let liveContext: ExtensionContext | undefined;

	const localBash = createLocalBashOperations();
	const bashTool = createBashTool(process.cwd(), {
		spawnHook: ({ command, cwd, env }) => ({
			command,
			cwd: active?.path ?? mainRoot ?? sessionCwd ?? cwd,
			env,
		}),
	});
	// AgentTool's public type omits the extension context, but Pi's tool runner
	// supplies it as the fifth argument and createBashTool uses it for PI_* env vars.
	const executeBashWithContext = bashTool.execute as (
		...args: [...Parameters<typeof bashTool.execute>, ExtensionContext]
	) => ReturnType<typeof bashTool.execute>;

	pi.registerTool({
		...bashTool,
		async execute(id, params, signal, onUpdate, ctx) {
			return executeBashWithContext(id, params, signal, onUpdate, ctx);
		},
	});

	function stopHeadWatcher(): void {
		if (watchedHead) unwatchFile(watchedHead);
		watchedHead = undefined;
	}

	function isUsableWorktree(worktreePath: string): boolean {
		const headFile = findGitHeadFile(worktreePath);
		return existsSync(worktreePath) && headFile !== undefined && existsSync(headFile);
	}

	function displayName(): string | undefined {
		if (!active) return undefined;
		if (active.currentBranch) return active.currentBranch;
		if (active.detachedHead) return `detached@${active.detachedHead}`;
		return active.requestedBranch;
	}

	function updateStatus(ctx: ExtensionContext): void {
		const name = displayName();
		ctx.ui.setStatus(STATUS_KEY, name ? ctx.ui.theme.fg("accent", `wt: ${name}`) : undefined);
	}

	async function refreshBranch(ctx: ExtensionContext, expectedPath = active?.path): Promise<void> {
		if (!active || !expectedPath || !samePath(active.path, expectedPath)) return;
		const symbolic = await pi.exec("git", ["-C", expectedPath, "symbolic-ref", "--quiet", "--short", "HEAD"], {
			timeout: 5000,
		});
		if (!active || !samePath(active.path, expectedPath)) return;
		if (symbolic.code === 0 && symbolic.stdout.trim()) {
			active.currentBranch = symbolic.stdout.trim();
			active.detachedHead = undefined;
		} else {
			const detached = await pi.exec("git", ["-C", expectedPath, "rev-parse", "--short", "HEAD"], {
				timeout: 5000,
			});
			if (!active || !samePath(active.path, expectedPath)) return;
			active.currentBranch = undefined;
			active.detachedHead = detached.code === 0 ? detached.stdout.trim() : "unknown";
		}
		updateStatus(ctx);
	}

	function startHeadWatcher(ctx: ExtensionContext): void {
		stopHeadWatcher();
		if (!active) return;
		const selectedPath = active.path;
		const headFile = findGitHeadFile(selectedPath);
		if (!headFile) return;
		watchedHead = headFile;
		watchFile(headFile, { interval: 500, persistent: false }, () => {
			if (liveContext && active && samePath(active.path, selectedPath)) {
				void refreshBranch(liveContext, selectedPath);
			}
		});
	}

	async function refreshWorktrees(cwd = sessionCwd): Promise<WorktreeInfo[]> {
		const result = await pi.exec("git", ["-C", cwd, "-c", "core.quotePath=false", "worktree", "list", "--porcelain"], {
			timeout: 10000,
		});
		if (result.code !== 0) {
			throw new Error(result.stderr.trim() || `${cwd} is not inside a Git repository`);
		}
		const parsed = parseWorktreePorcelain(result.stdout);
		if (parsed.length === 0) throw new Error("Git reported no worktrees");
		mainRoot = parsed[0]!.path;
		return parsed;
	}

	async function activate(
		ctx: ExtensionContext,
		worktreePath: string,
		requestedBranch: string,
		persist: boolean,
	): Promise<void> {
		active = { path: path.resolve(worktreePath), requestedBranch };
		liveContext = ctx;
		await refreshBranch(ctx);
		startHeadWatcher(ctx);
		if (persist) {
			pi.appendEntry<SelectionEntry>(ENTRY_TYPE, {
				version: 1,
				action: "select",
				path: active.path,
				requestedBranch,
				timestamp: Date.now(),
			});
		}
		updateStatus(ctx);
	}

	function deactivate(ctx: ExtensionContext, persist: boolean): void {
		stopHeadWatcher();
		active = undefined;
		liveContext = ctx;
		if (persist) {
			pi.appendEntry<SelectionEntry>(ENTRY_TYPE, {
				version: 1,
				action: "off",
				timestamp: Date.now(),
			});
		}
		updateStatus(ctx);
	}

	async function restoreSelection(ctx: ExtensionContext): Promise<void> {
		liveContext = ctx;
		let saved: SelectionEntry | undefined;
		for (const entry of [...ctx.sessionManager.getBranch()].reverse()) {
			if (entry.type === "custom" && entry.customType === ENTRY_TYPE) {
				saved = entry.data as SelectionEntry | undefined;
				break;
			}
		}
		if (!saved || saved.action === "off" || !saved.path || !saved.requestedBranch) {
			deactivate(ctx, false);
			return;
		}
		const worktrees = await refreshWorktrees(ctx.cwd);
		const registered = worktrees.find((item) => samePath(item.path, saved!.path!));
		if (!registered || registered.prunable || !isUsableWorktree(saved.path)) {
			deactivate(ctx, false);
			ctx.ui.notify(`Saved worktree no longer exists or is stale: ${saved.path}`, "warning");
			return;
		}
		await activate(ctx, registered.path, saved.requestedBranch, false);
	}

	async function validateBranch(branch: string): Promise<void> {
		const result = await pi.exec("git", ["check-ref-format", "--branch", branch], { timeout: 5000 });
		if (result.code !== 0) throw new Error(`Invalid Git branch name: ${branch}`);
	}

	async function selectExisting(ctx: ExtensionContext, branch: string): Promise<void> {
		await validateBranch(branch);
		const worktrees = await refreshWorktrees(ctx.cwd);
		const expected = expectedWorktreePath(mainRoot!, branch);
		const selected = worktrees.find((item) => item.branch === branch)
			?? worktrees.find((item) => samePath(item.path, expected));
		if (!selected) {
			throw new Error(`No worktree found for "${branch}". Create it with /wt -c ${branch}`);
		}
		if (selected.bare) throw new Error(`Cannot select bare worktree: ${selected.path}`);
		if (selected.prunable || !isUsableWorktree(selected.path)) {
			throw new Error(`Worktree is missing or stale: ${selected.path}. Repair or prune it before selecting.`);
		}
		if (samePath(selected.path, mainRoot!)) {
			deactivate(ctx, true);
			ctx.ui.notify("Worktree routing disabled; using the main worktree", "info");
			return;
		}
		await activate(ctx, selected.path, branch, true);
		ctx.ui.notify(`Selected ${selected.path}`, "info");
	}

	async function createWorktree(ctx: ExtensionContext, branch: string): Promise<void> {
		await validateBranch(branch);
		const worktrees = await refreshWorktrees(ctx.cwd);
		const target = expectedWorktreePath(mainRoot!, branch);
		const branchOwner = worktrees.find((item) => item.branch === branch);
		const pathOwner = worktrees.find((item) => samePath(item.path, target));
		if (branchOwner || pathOwner || existsSync(target)) {
			const conflict = branchOwner?.path ?? pathOwner?.path ?? target;
			throw new Error(`Worktree or target already exists at ${conflict}. Select it with /wt ${branch}`);
		}
		const branchExists = await pi.exec("git", ["-C", mainRoot!, "show-ref", "--verify", "--quiet", `refs/heads/${branch}`], {
			timeout: 5000,
		});
		if (branchExists.code === 0) {
			throw new Error(`Branch already exists: ${branch}. /wt -c only creates new branches.`);
		}
		const created = await pi.exec("git", ["-C", mainRoot!, "worktree", "add", "-b", branch, target, "HEAD"], {
			timeout: 120000,
		});
		if (created.code !== 0) throw new Error(created.stderr.trim() || `Failed to create ${target}`);
		const refreshed = await refreshWorktrees(mainRoot!);
		const selected = refreshed.find((item) => samePath(item.path, target));
		if (!selected) throw new Error(`Git created the worktree, but it was not found in git worktree list: ${target}`);
		await activate(ctx, selected.path, branch, true);
		ctx.ui.notify(`Created ${branch} at ${selected.path}`, "info");
	}

	pi.registerEntryRenderer<SelectionEntry>(ENTRY_TYPE, (entry, _options, theme) => {
		const data = entry.data;
		if (!data) return undefined;
		return new Text(theme.fg("dim", `[wt] ${entryLabel(data)}`), 0, 0);
	});

	pi.registerCommand("wt", {
		description: "Select or create a routed Git worktree",
		handler: async (args, ctx) => {
			try {
				const command = parseWorktreeCommand(args);
				await ctx.waitForIdle();
				sessionCwd = ctx.cwd;
				liveContext = ctx;
				if (command.kind === "status") {
					if (active) await refreshBranch(ctx);
					ctx.ui.notify(active ? `${displayName()}\n${active.path}` : `main\n${mainRoot ?? ctx.cwd}`, "info");
					return;
				}
				if (command.kind === "list") {
					const worktrees = await refreshWorktrees(ctx.cwd);
					const lines = worktrees.map((item) => {
						const marker = active && samePath(active.path, item.path) ? "*" : " ";
						const branch = item.branch ?? (item.detached ? `detached@${item.head?.slice(0, 7)}` : "bare");
						return `${marker} ${branch}\n  ${item.path}`;
					});
					ctx.ui.notify(lines.join("\n"), "info");
					return;
				}
				if (command.kind === "off") {
					deactivate(ctx, true);
					ctx.ui.notify("Worktree routing disabled; using the main worktree", "info");
					return;
				}
				if (command.kind === "create") await createWorktree(ctx, command.branch);
				else await selectExisting(ctx, command.branch);
			} catch (error) {
				const message = error instanceof Error ? error.message : String(error);
				if (ctx.hasUI) ctx.ui.notify(message, "error");
				else console.error(`[wt] ${message}`);
			}
		},
	});

	pi.on("session_start", async (_event, ctx) => {
		sessionCwd = ctx.cwd;
		liveContext = ctx;
		try {
			await refreshWorktrees(ctx.cwd);
			await restoreSelection(ctx);
		} catch {
			mainRoot = undefined;
			deactivate(ctx, false);
		}
	});

	pi.on("session_tree", async (_event, ctx) => {
		try {
			await restoreSelection(ctx);
		} catch (error) {
			deactivate(ctx, false);
			ctx.ui.notify(error instanceof Error ? error.message : String(error), "warning");
		}
	});

	pi.on("session_shutdown", () => {
		stopHeadWatcher();
		liveContext = undefined;
	});

	pi.on("before_agent_start", async (event, ctx) => {
		if (!active || !mainRoot) return;
		if (!isUsableWorktree(active.path)) {
			const stalePath = active.path;
			deactivate(ctx, false);
			ctx.ui.notify(`Active worktree disappeared: ${stalePath}`, "error");
			return;
		}
		await refreshBranch(ctx);
		const branch = displayName() ?? active.requestedBranch;
		return {
			systemPrompt: `${event.systemPrompt}\n\nWorktree routing is active.\nPi session root: ${mainRoot}\nActive implementation worktree: ${active.path}\nCurrent worktree branch: ${branch}\nUse the active implementation worktree for repository reads, edits, searches, tests, and Git commands. The Pi session and conversation remain anchored in the session root.`,
		};
	});

	pi.on("user_bash", () => {
		if (!active) return;
		const selectedPath = active.path;
		return {
			operations: {
				exec(command, _cwd, options) {
					return localBash.exec(command, selectedPath, options);
				},
			},
		};
	});

	pi.on("tool_call", (event, ctx) => {
		if (!active || !mainRoot || event.toolName === "bash") return;
		const pathTools = new Set(["read", "write", "edit", "grep", "find", "ls"]);
		if (!pathTools.has(event.toolName)) return;
		const mutation = isToolCallEventType("write", event) || isToolCallEventType("edit", event);
		if (mutation && !isUsableWorktree(active.path)) {
			const stalePath = active.path;
			deactivate(ctx, false);
			ctx.ui.notify(`Active worktree disappeared: ${stalePath}`, "error");
			return { block: true, reason: `Active worktree is missing or stale: ${stalePath}` };
		}
		const input = event.input as { path?: string };
		const original = input.path ?? ".";
		const routed = routeToolPath(original, mainRoot, active.path, mutation);
		if (routed.error) return { block: true, reason: routed.error };
		input.path = routed.path!;
	});
}

export { branchToWorktreeSlug };
