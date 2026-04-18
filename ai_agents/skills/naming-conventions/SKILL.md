---
name: naming-conventions
description: Enforce consistent, meaningful naming across the codebase. Use when reviewing code, writing new identifiers, or asked to audit naming quality.
---

# Naming Conventions

Validate all identifiers against these rules. Flag violations and suggest fixes.

## Core Rules

- **snake_case** for variables, functions, files
- **CamelCase** for interfaces only
- **No abbreviations** except primitive sort/matrix args (`i`, `j`, `m`, `n`)
- **Full words in scripts**: `--force` not `-f`; single-letter flags are interactive-only
- **Acronyms capitalized**: `VSRState`, `HTTPClient` not `VsrState`
- **Units/qualifiers last**: `latency_ms_max` not `max_latency_ms`
- **Same-length related names**: `source`/`target` not `src`/`dst`
- **Infuse meaning**: `arena: Allocator` not `gpa: Allocator` alone
- **Prefix helpers**: `read_sector()` + `read_sector_callback()`
- **Callbacks last** in parameter lists
- **Nouns over participles**: `replica.pipeline` not `replica.preparing`
- **No context overloading**

## Unit/Qualifier Order

```
latency_ms_max      ← most significant first, units last
latency_ms_min
count_total
count_active
offset_source      ← related names align
offset_target
```

## Quick Reference Checklist

- [ ] snake_case for all identifiers (interfaces: CamelCase)
- [ ] No abbreviations unless primitive sort/matrix args
- [ ] Acronyms capitalized: `VSRState`, `HTTPClient`
- [ ] Units/qualifiers at end: `timeout_ms`, `count_total`
- [ ] Related names same length: `source`/`target`
- [ ] Meaning infused: `arena` not just `allocator`
- [ ] Helpers prefixed: `do_work()` + `do_work_helper()`
- [ ] Callbacks last in parameters
- [ ] Nouns for names in docs
- [ ] No context-dependent overloading

## Flag Examples

| Violation | Fix | Why |
|-----------|-----|-----|
| `max_lat_ms` | `latency_ms_max` | Units last, aligns with `_min` |
| `src`, `dst` | `source`, `target` | Same length, clearer |
| `replica.preparing` | `replica.pipeline` | Noun composes better |
| `buf` | `receive_buffer` | Descriptive |
| `gpa` | `gpa: Allocator` | Infuses meaning |
| `cb` | `on_complete_callback` | Descriptive, goes last |