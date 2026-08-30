import path from "node:path";

export interface WorktreeInfo {
	path: string;
	head?: string;
	branch?: string;
	detached: boolean;
	bare: boolean;
	locked?: string;
	prunable?: string;
}

export type WorktreeCommand =
	| { kind: "status" }
	| { kind: "list" }
	| { kind: "off" }
	| { kind: "select"; branch: string }
	| { kind: "create"; branch: string };

export interface RoutedPath {
	path?: string;
	error?: string;
}

export function parseWorktreePorcelain(output: string): WorktreeInfo[] {
	const records: WorktreeInfo[] = [];
	let current: WorktreeInfo | undefined;

	for (const line of output.split(/\r?\n/)) {
		if (line.startsWith("worktree ")) {
			if (current) records.push(current);
			current = {
				path: path.resolve(line.slice("worktree ".length)),
				detached: false,
				bare: false,
			};
			continue;
		}
		if (!current || line === "") continue;
		if (line.startsWith("HEAD ")) current.head = line.slice(5);
		else if (line.startsWith("branch ")) current.branch = line.slice(7).replace(/^refs\/heads\//, "");
		else if (line === "detached") current.detached = true;
		else if (line === "bare") current.bare = true;
		else if (line === "locked") current.locked = "locked";
		else if (line.startsWith("locked ")) current.locked = line.slice(7);
		else if (line === "prunable") current.prunable = "prunable";
		else if (line.startsWith("prunable ")) current.prunable = line.slice(9);
	}
	if (current) records.push(current);
	return records;
}

export function parseWorktreeCommand(raw: string): WorktreeCommand {
	const args = raw.trim().split(/\s+/).filter(Boolean);
	if (args.length === 0) return { kind: "status" };
	if (args.length === 1 && args[0] === "list") return { kind: "list" };
	if (args.length === 1 && args[0] === "off") return { kind: "off" };
	if ((args[0] === "-c" || args[0] === "--create") && args.length === 2) {
		return { kind: "create", branch: args[1]! };
	}
	if (args.length === 1 && !args[0]!.startsWith("-")) {
		return { kind: "select", branch: args[0]! };
	}
	throw new Error("Usage: /wt [<branch> | -c <branch> | --create <branch> | list | off]");
}

/** Worktrunk-style directory slug: preserve the branch, but use '-' for '/' separators. */
export function branchToWorktreeSlug(branch: string): string {
	return branch.replaceAll("/", "-");
}

export function expectedWorktreePath(mainRoot: string, branch: string): string {
	return path.join(path.dirname(mainRoot), `${path.basename(mainRoot)}.${branchToWorktreeSlug(branch)}`);
}

export function samePath(left: string, right: string): boolean {
	const normalize = (value: string) => {
		const resolved = path.resolve(value).replace(/[\\/]+$/, "");
		return process.platform === "win32" ? resolved.toLowerCase() : resolved;
	};
	return normalize(left) === normalize(right);
}

export function isWithin(parent: string, candidate: string): boolean {
	const relative = path.relative(path.resolve(parent), path.resolve(candidate));
	const escapes = relative === ".." || relative.startsWith(`..${path.sep}`);
	return relative === "" || (!escapes && !path.isAbsolute(relative));
}

/**
 * Route repository paths to the selected worktree.
 * Relative paths are always worktree-relative. Absolute paths under the main
 * checkout are mapped to the same relative location in the selected worktree.
 * External absolute reads remain available for Pi docs and global resources;
 * external writes are blocked.
 */
export function routeToolPath(
	input: string,
	mainRoot: string,
	selectedRoot: string,
	mutation: boolean,
): RoutedPath {
	const raw = input.startsWith("@") ? input.slice(1) : input;
	let result: string;

	if (!path.isAbsolute(raw)) {
		result = path.resolve(selectedRoot, raw || ".");
		if (!isWithin(selectedRoot, result)) {
			return { error: `Relative path escapes the active worktree: ${input}` };
		}
		return { path: result };
	}

	const absolute = path.resolve(raw);
	if (isWithin(selectedRoot, absolute)) return { path: absolute };
	if (isWithin(mainRoot, absolute)) {
		return { path: path.join(selectedRoot, path.relative(mainRoot, absolute)) };
	}
	if (mutation) return { error: `Write path is outside the active worktree: ${input}` };
	return { path: absolute };
}
