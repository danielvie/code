---
name: tiger-style-coding
description: Use when writing safety-critical systems, performance-sensitive code, or when prioritizing correctness over convenience. Use when encountering undefined behavior, memory issues, or when code complexity makes reasoning difficult.
---

# Tiger Style Coding

## Overview

**Safety > Performance > Developer Experience.** In that order. All three matter, but never at the expense of the preceding goal.

Tiger Style is a zero-technical-debt approach from TigerBeetle's engineering culture. It treats code like steel: cheaper to change while hot, expensive once cooled. The style combines NASA's Power of Ten rules with systems programming discipline to produce code that is obviously correct, bounded, and fast.

**Core principle:** Simplicity is not the first attempt but the hardest revision. Do it right the first time.

## When to Use

- Building financial, safety-critical, or distributed systems
- Performance requirements demand predictable latency (no tail latency spikes)
- Debugging memory corruption, use-after-free, or undefined behavior
- Code review reveals "impossible" edge cases keep happening
- Functions exceed 70 lines or require scrolling to understand
- Abstractions leak, making debugging harder instead of easier
- Dynamic allocation causes unpredictable behavior in production

**When NOT to use:**
- Rapid prototyping where correctness is secondary to exploration
- Code that will be thrown away within days
- Situations where "good enough" truly is good enough

## Core Patterns

### Safety Patterns

#### 1. Explicit Control Flow
Use only simple, explicit control flow. No recursion. No hidden control flow in macros or exceptions.

```zig
// ✅ Good: Explicit loop with fixed bound
var i: u32 = 0;
while (i < max_iterations) : (i += 1) {
    // Bounded, predictable
}

// ❌ Bad: Recursion (unbounded stack growth)
fn process(node: *Node) void {
    if (node.next) |n| process(n); // Unbounded!
}
```

#### 2. Limits on Everything
Every loop, queue, and buffer has a fixed upper bound. Assert these bounds.

```zig
const max_pending_transfers = 10_000;

pub fn enqueue(transfer: Transfer) !void {
    assert(pending_count < max_pending_transfers); // Fail fast
    // ...
}
```

#### 3. Assertions as Safety Net
Average minimum two assertions per function. Assert preconditions, postconditions, invariants. Assert positive space (what you expect) AND negative space (what you don't expect).

```zig
pub fn transfer_amount(from: u64, to: u64, amount: u64) !void {
    assert(from != to); // Negative space: same account transfer makes no sense
    assert(amount > 0); // Positive space: must transfer something
    assert(amount <= max_transfer_amount); // Bounded
    
    // ... implementation ...
    
    assert(balance_after <= max_balance); // Postcondition
}
```

**Split compound assertions:**
```zig
// ✅ Good: Precise failure information
assert(a != null);
assert(b > 0);

// ❌ Bad: Which condition failed?
assert(a != null and b > 0);
```

#### 4. Static Allocation Only
All memory allocated at startup. No dynamic allocation after initialization.

```zig
pub const StateMachine = struct {
    // Pre-allocated at init
    transfers: [max_transfers]Transfer,
    accounts: [max_accounts]Account,
    
    pub fn init() !StateMachine {
        return .{
            .transfers = undefined, // Will be populated
            .accounts = undefined,
        };
    }
};
```

#### 5. Smallest Possible Scope
Declare variables at the smallest scope. Minimize variables in scope to reduce misuse probability.

```zig
// ✅ Good: Tight scope
{
    const temp_buffer = try allocator.alloc(u8, size);
    defer allocator.free(temp_buffer);
    // Use temp_buffer...
} // temp_buffer no longer accessible

// Continue with other logic...
```

### Performance Patterns

#### 1. Back-of-the-Envelope Sketches
Before coding, sketch the four resources: network, disk, memory, CPU. Consider bandwidth and latency for each.

| Resource | Bandwidth | Latency | Optimize Order |
|----------|-----------|---------|----------------|
| Network | ~1 Gbps | ~1-100ms | 1st |
| Disk | ~500 MB/s | ~1-10ms | 2nd |
| Memory | ~50 GB/s | ~100ns | 3rd |
| CPU | ~100 GFLOPS | ~1ns | 4th |

**Rule:** Optimize for the slowest resources first, after compensating for frequency of usage.

#### 2. Batching and Amortization
Let the CPU be a sprinter doing the 100m. Give it large chunks of work. Don't zigzag.

```zig
// ✅ Good: Batch operations
pub fn commit_batch(transfers: []const Transfer) !void {
    // Process many transfers in one go
    // Amortize fsync cost across batch
}

// ❌ Bad: Individual commits
for (transfers) |t| {
    try commit_single(t); // Overhead multiplied by N
}
```

#### 3. Explicit Over Clever
Minimize dependence on the compiler doing the right thing. Extract hot loops into standalone functions with primitive arguments.

```zig
// ✅ Good: Compiler can optimize, human can verify
fn calculate_checksum(data: []const u8) u64 {
    var sum: u64 = 0;
    for (data) |byte| {
        sum = sum *% 31 +% byte;
    }
    return sum;
}

// ❌ Bad: Self parameter prevents register caching
fn calculate_checksum(self: *const Self) u64 { ... }
```

### Developer Experience Patterns

#### 1. Naming with Units and Qualifiers
Add units to names. Put qualifiers last, sorted by descending significance.

```zig
// ✅ Good: Groups by concept, lines up
latency_ms_max: u64,
latency_ms_min: u64,
latency_ms_avg: u64,

// ❌ Bad: Scattered, hard to scan
max_latency_ms: u64,
min_latency_ms: u64,
avg_latency_ms: u64,
```

#### 2. Same-Length Related Names
Find names with the same character count so related variables line up.

```zig
// ✅ Good: Lines up in slices and calculations
source: []const u8,
target: []u8,
source_offset: u32,
target_offset: u32,

// ❌ Bad: Misaligned
src: []const u8,
dest: []u8,
src_off: u32,
dest_off: u32,
```

#### 3. Infuse Names with Meaning
```zig
// Boring but acceptable
allocator: Allocator,

// Excellent: Tells you about cleanup
gpa: Allocator,      // General purpose - needs explicit deinit
arena: Allocator,    // Arena - bulk free only
```

#### 4. Function Prefix for Callbacks
When a function calls a helper or callback, prefix the helper with the caller's name.

```zig
pub fn read_sector(disk: *Disk, sector: u64) !void {
    disk.io.submit(.{ .callback = read_sector_callback });
}

fn read_sector_callback(completion: *Completion) void {
    // Clear relationship visible in stack traces
}
```

#### 5. Order Matters
Files read top-down. Put important things first.

```zig
// Struct order: fields, types, methods
pub const Tracer = struct {
    // 1. Fields first
    time: Time,
    process_id: ProcessID,
    
    // 2. Types next
    const ProcessID = struct { ... };
    
    // 3. Methods last
    pub fn init(...) !Tracer { ... }
};
```

## Quick Reference

| Principle | Rule |
|-----------|------|
| **Control Flow** | Simple and explicit. No recursion. |
| **Bounds** | Every loop/queue has fixed upper bound. |
| **Assertions** | Minimum 2 per function. Split compounds. |
| **Allocation** | Static only. All at startup. |
| **Function Size** | Hard limit 70 lines. |
| **Scope** | Smallest possible. Declare close to use. |
| **Naming** | snake_case, units last, same-length pairs. |
| **Performance** | Sketch resources first. Batch everything. |
| **Dependencies** | Zero dependencies policy. |
| **Line Length** | Max 100 columns. |
| **Indentation** | 4 spaces. |

## Implementation

### Starting a New Module

1. **Define limits first:** What are the maximums? (transfers, accounts, connections)
2. **Sketch resources:** Back-of-the-envelope for network/disk/memory/CPU
3. **Design assertions:** What must always be true? What must never happen?
4. **Static allocation:** Pre-allocate all needed memory
5. **Write code:** Simple control flow, small functions, explicit everything

### Refactoring Existing Code

1. **Identify unbounded operations:** Loops without limits, recursion, dynamic allocation
2. **Add assertions:** Start with pre/postconditions on public functions
3. **Reduce scope:** Move variable declarations to innermost blocks
4. **Split functions:** Cut at 70 lines, centralize control flow in parent
5. **Rename:** Add units, align lengths, infuse meaning

### Code Review Checklist

- [ ] No recursion
- [ ] All loops bounded
- [ ] 2+ assertions per function
- [ ] No dynamic allocation after init
- [ ] Functions under 70 lines
- [ ] Variables at smallest scope
- [ ] Names have units/qualifiers
- [ ] Error handling for all error cases
- [ ] "Why" comments for non-obvious code

## Common Mistakes

| Mistake | Why It Hurts | Fix |
|---------|--------------|-----|
| "I'll add assertions later" | Later never comes. Assertions find bugs during development. | Add assertions as you write. |
| "This loop is obviously bounded" | Obvious to you now, not to reviewer or future you. | Explicit bound + assertion. |
| "Dynamic allocation is fine here" | Unpredictable latency, fragmentation, use-after-free risk. | Pre-allocate, use pools. |
| "Function is 100 lines but readable" | Physical constraint: scrolling breaks context. | Split at 70 lines. |
| "Abstraction makes it cleaner" | Abstractions leak and hide costs. | Prefer explicit code. |
| "The compiler will optimize this" | Optimizer has constraints you don't understand. | Write explicitly optimal code. |
| "max_latency_ms reads better" | Doesn't group with related variables. | latency_ms_max for alignment. |

## Red Flags - STOP and Fix

- Recursion in any form
- `while (true)` without proven exit condition
- `malloc`/`free` after initialization
- Functions requiring scroll to see end
- Compound assertions (`assert(a and b)`)
- Missing `else` branch on `if` (assert negative space)
- Variable used far from where declared
- Dynamic-length arrays
- "Trust me, this is safe" comments instead of assertions

## The Zero Technical Debt Policy

> "You shall not pass!" — Gandalf

TigerBeetle does not allow potential latency spikes, exponential algorithms, or unbounded operations to slip through. Do it right the first time because:

1. The second time may not transpire
2. Doing good work builds momentum
3. Steady incremental progress requires known-solid foundations

**The question to ask:** Does this code make for more or less safety, performance, or developer experience? That is why we need style.