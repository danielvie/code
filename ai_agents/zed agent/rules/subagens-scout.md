# Scout Sub-Agent Strategy

## Overview

Parallelize information gathering by spawning scout agents. Each scout reads one source and returns a condensed summary. Main agent selects relevant findings without loading full documents into context.

## When to Use

- Multiple documentation pages to check
- Large codebase exploration
- Research tasks with >3 distinct sources
- Context window approaching limits

## Core Pattern

```
┌─────────────┐     ┌─────────┐     ┌─────────────┐
│   Source A  │────→│ Scout A │────→│  Summary A  │
├─────────────┤     ├─────────┤     ├─────────────┤
│   Source B  │────→│ Scout B │────→│  Summary B  │──→ Main Agent ──→ Action
├─────────────┤     ├─────────┤     ├─────────────┤
│   Source C  │────→│ Scout C │────→│  Summary C  │
└─────────────┘     └─────────┘     └─────────────┘
```

## Scout Instructions Template

```markdown
Scout: Read [URL/file] for [specific info].
Return:
1. Key findings (bullet points)
2. Relevant code snippets (max 10 lines)
3. Verdict: RELEVANT / NOT RELEVANT / NEEDS_DEEPER_READ
```

## Quick Reference

| Scenario | Scout Count | Timeout |
|----------|-------------|---------|
| 3-5 docs | 1 per doc | 30s |
| 5-10 docs | 1 per doc | 60s |
| Large files | 1 per file | 60s |

## Synthesis Rules

1. **Read summaries first** — Don't fetch sources yourself
2. **Follow RELEVANT leads only** — Ignore NOT_RELEVANT
3. **Re-scout if NEEDS_DEEPER_READ** — Spawn targeted follow-up
4. **Stop at 3 synthesis rounds** — Prevents infinite loops

## Red Flags

| Violation | Fix |
|-----------|-----|
| Main agent reads full docs before scouts return | Wait for scouts, read summaries only |
| Scout returns full copy-paste | Require bullet summary, max 200 words |
| All scouts read same source | Assign distinct sources per scout |
| No verdict in scout output | Mandate RELEVANT/NOT_RELEVANT/NEEDS_DEEPER_READ |
| >5 synthesis rounds | Hard stop at 3 rounds |