---
name: claude-code
description: Use only when the user explicitly mentions `claude` or `opus`. Teaches the agent to invoke Claude Code with the Opus 5 model for visual design, UI layout, and standalone SVG generation, while keeping business and application logic in the primary agent.
---

# Claude Code

Claude Code is a secondary visual-design collaborator. Do not invoke this skill, run the Claude CLI, or mention Claude as a solution unless the current user request explicitly contains `claude` or `opus` (case-insensitive).

## Scope

Use Claude Code only for:

- visual design direction and critique;
- UI composition, spacing, typography, color, hierarchy, and responsive layout;
- presentation-only HTML, JSX/TSX, CSS, or Tailwind changes;
- generating or refining standalone SVG icons, illustrations, and decorative graphics.

Do not use it for business rules, domain modeling, financial calculations, data models, persistence, API work, backend code, migrations, tests, security, dependency choices, or general refactoring. Keep those tasks with the primary agent.

## Model

Use Opus 5 as the default and only model for this workflow. Invoke its Claude Code alias as:

```bash
claude -p "..." --model opus --effort high
```

Run it from the repository root through Bash when Bash execution is needed:

```bash
bash -lc 'claude -p "..." --model opus --effort high'
```

Treat `opus` as the configured Opus 5 alias. Do not silently substitute Haiku, Sonnet, or another model. If the installed CLI rejects the alias, verify the available model syntax with `claude --help` and report the mismatch instead of guessing.

Before the first real request, a minimal availability check is acceptable:

```bash
claude --version
claude -p "Reply with exactly: claude-ok" --model opus --effort high
```

If authentication fails, stop and report that the Claude session needs re-authentication. Do not repeatedly retry or expose credentials.

## Workflow

1. Confirm the explicit `claude`/`opus` trigger and that the task is visual-only.
2. Inspect the relevant project files, existing design tokens, component conventions, and asset paths before prompting Claude.
3. Start with a read-only design pass. Ask Claude for a concise proposal, affected files, layout constraints, and acceptance criteria; explicitly say not to edit files.
4. If implementation is requested, constrain Claude to presentation files and the named UI surface. Tell it not to change business logic, state semantics, calculations, persistence, or unrelated styling.
5. Review Claude’s response or diff in the primary agent. Do not apply generated code blindly.
6. Run the project’s narrowest relevant check, then inspect the final diff for scope creep and accidental logic changes.
7. Report the exact command, model alias, result, and files changed.

## Prompt patterns

### Design review

```text
Inspect [screen/component] and propose a visual redesign.
Focus only on hierarchy, spacing, typography, color, responsive behavior, and interaction states.
Do not edit files and do not discuss or change business logic.
Return: (1) problems, (2) proposed layout, (3) exact files that would change, and (4) acceptance criteria.

Project constraints:
- [design system or existing tokens]
- [target viewport/device]
- [accessibility or branding constraints]
```

### Layout implementation

```text
Implement the approved visual layout for [screen/component].
You may change only [explicit presentation files].
Preserve all existing state, calculations, data flow, persistence, labels with domain meaning, and event behavior.
Do not add dependencies or refactor unrelated code.
After editing, summarize each changed file and any visual trade-offs.
```

### SVG generation

```text
Generate a standalone SVG for [icon/illustration].
Requirements:
- valid SVG with an explicit viewBox;
- no external fonts, images, scripts, or network references;
- use [fill/stroke/color constraints];
- include a <title> and <desc> when the image conveys meaning;
- keep geometry simple and editable;
- provide the SVG only, followed by a one-sentence usage note.
```

## SVG guardrails

Before accepting generated SVG, check that it:

- has the intended dimensions and viewBox;
- contains no embedded secrets, remote URLs, scripts, or unnecessary metadata;
- uses project-compatible colors and stroke widths;
- remains legible at the requested size;
- has accessible labeling when it is not purely decorative;
- does not introduce an unrequested asset format or dependency.

Keep Claude’s contribution visual. The primary agent owns correctness, domain meaning, behavior, and final integration.
