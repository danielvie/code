---
name: writing-skills
description: >
  Use when creating, editing, or verifying skills. TDD-based skill development.
  Tests with subagents, pressure scenarios, close loopholes.
---

# Writing Skills

## Core Loop: RED-GREEN-REFACTOR

1. **RED** — Write pressure scenarios (subagent tests). Watch fail.
2. **GREEN** — Write minimal skill. Watch pass.
3. **REFACTOR** — Close loopholes, update description.

**Iron Law:** If you didn't watch an agent fail without the skill, you don't know if the skill teaches the right thing.

## SKILL.md Structure

```
---
name: skill-name-with-hyphens
description: Use when [trigger]. [what it does].
---

# Skill Name

## Overview
## When to Use
## Core Pattern
## Quick Reference
## Common Mistakes
```

## CSO: Description Field

Be specific. Include triggering conditions and symptoms.

**GOOD:** `description: Use when tests use setTimeout/sleep and are flaky.`
**BAD:** `description: Use when testing.`

## Token Efficiency

- Reference other skills, don't repeat workflow
- Minimal examples (< 25 words)
- Drop redundant preamble

## Testing with Subagents

**RED Phase:** Write 3-5 pressure scenarios. Watch baseline fail.

**GREEN Phase:** Write minimal skill. Verify pass.

**REFACTOR Phase:**
- Explicit negation in rules
- Red flags list
- Update description for violation symptoms

## Close Loopholes

Agents rationalize. Close explicitly:
- "Stop if X" not "Consider stopping if X"
- "You wrote code before tests. STOP. Delete code." not "Try not to write code before tests"
- Red flags table: violation → skill fix

## Common Mistakes

- Narrative instead of reference
- Generic labels ("tips", "best practices")
- Deep nesting (> 2 levels)
- Full examples when snippets suffice

## Anti-Patterns

- ❌ Code in flowcharts
- ❌ Windows-style paths
- ❌ Offering too many options
- ❌ Time-sensitive content
- ❌ Multi-language in one skill