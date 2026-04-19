# Variable Naming Conventions

## Why Naming Matters

Great names capture what a thing is or does, providing a crisp, intuitive mental model. They show you understand the domain. Names affect readability, maintainability, and how code composes into documentation and communication.
Use these rules for naming variables, except if the variable is an Interface, in this case use CamelCase

## Core Rules

- **Use `snake_case`** for variables, functions, and files
- **No abbreviations** unless for primitive integer sort/matrix arguments
- **Full words in scripts**: `--force` not `-f`; single-letter flags are for interactive use
- **Proper capitalization for acronyms**: `VSRState` not `VsrState`
- **Units/qualifiers last**, sorted by descending significance: `latency_ms_max`, not `max_latency_ms`
- **Infuse names with meaning**: `gpa: Allocator` tells you `deinit` is needed; `arena: Allocator` tells you it persists
- **Same-length for related names**: `source`/`target` lines up better than `src`/`dest`
- **Prefix helpers with calling function**: `read_sector()` and `read_sector_callback()`
- **Callbacks go last** in parameter lists (mirrors control flow)
- **Nouns over participles**: `replica.pipeline` works better in docs than `replica.preparing`
- **Don't overload names** with context-dependent meanings

## Unit/Qualifier Ordering

```
latency_ms_max      ← most significant first, units last
latency_ms_min
count_total
count_active
offset_source      ← related names align
offset_target
```

## Examples

| Bad | Good | Why |
|-----|------|-----|
| `max_lat_ms` | `latency_ms_max` | Units last, lines up with `_min` |
| `src`, `dest` | `source`, `target` | Same length, clearer |
| `replica.preparing` | `replica.pipeline` | Noun composes better |
| `buf` | `receive_buffer` | Descriptive |
| `gpa` | `gpa: Allocator` | Infuses meaning |
| `cb` | `on_complete_callback` | Descriptive, goes last |

## Quick Reference Checklist

- [ ] snake_case for all identifiers
- [ ] No abbreviations (unless primitive sort/matrix args)
- [ ] Acronyms capitalized: `VSRState`, `HTTPClient`
- [ ] Units/qualifiers at end: `timeout_ms`, `count_total`
- [ ] Related names same length: `source`/`target` not `src`/`dst`
- [ ] Infuse meaning: `arena` not just `allocator`
- [ ] Helper functions prefixed: `do_work()` + `do_work_helper()`
- [ ] Callbacks last in parameters
- [ ] Nouns for names that appear in docs
- [ ] No context-dependent overloading