---
name: code-design-analysis
description: >
  Use when analyzing software architecture, module boundaries, and responsibility. 
  Evaluates code against "Deep Modules" (Ousterhout) and safety principles.
---

# Code Design Analysis

## Overview

This skill evaluates software based on the philosophy of **Deep Modules** (minimizing interface complexity while maximizing internal functionality) and core safety principles (bounded loops, explicit units). It prioritizes **encapsulation** and **locality of behavior** (for simple tasks) while strictly punishing **Information Leaks** and **Change Amplification**.

## When to Use
- Reviewing PRs for architectural alignment.
- Refactoring complex systems with "Classitis" or "Shallow Modules".
- Debugging code that is "hard to change" (touching one thing breaks five).
- Assessing the "depth" and "responsibility" of a new module.

## Core Patterns

### 1. Deep vs. Shallow Modules
- **Deep Module:** Simple interface (e.g., a single `config` object or a few clear methods) that hides a complex, powerful implementation. **Goal: High depth.**
- **Shallow Module:** An interface that does very little, or just wraps another library without adding value. **Avoid.**

### 2. Responsibility (Single Reason to Change)
A module or function should have exactly one reason to change. 
- `read_file` should only change if the underlying file-reading logic or requirements change. 
- If it also handles UI logging, it has leaked responsibility.

### 3. Performance & Scalability (The "Happy Path" Trap)
- **Complexity Declaration:** Every critical loop or search must have an expected maximum $N$ documented or asserted. If $N$ is unbounded (e.g., "all rows"), it is a **Scaling Time Bomb**.
- **Binary vs. Text:** High-frequency data streams (e.g., >10Hz or sensor data) must favor binary formats (Flatbuffers/Protobuf) over JSON.
- **Boundary Limits:** Data entering/leaving the system (Disk, Network, User Input) **must** have a hard size limit or a `LIMIT` clause. Internal logic is exempt.

### 4. Metrics & Observability (Checkpoint Rule)
Modules handling I/O or external communication must include:
- **Outcome Tracking:** Logging Success vs. Failure for all operations.
- **Batch Visibility:** Logging the size of data being processed (e.g., "Deleted 50 rows").
- **Correlation IDs:** Passing/Logging a unique ID to trace requests across boundaries.

### 5. Locality of Behavior (LoB): The "What vs. How" Rule
- **Keep it Local (LoB):** If a block of code describes **"What"** is happening (test setup, simple data formatting).
- **Extract to Deep Module:** If the code describes **"How"** (protocols, retry logic). If you need comments to explain "how," encapsulate it.

### 6. Safety & Precision
- **Bounded Loops:** Every loop must have a hard upper bound and an assertion or error check.
- **Explicit Units:** All numeric variables must include units in the name (e.g., `delay_ms`).

## Hierarchy of Sins (Ranking of Improvements)
1. **Scaling Time Bomb:** $O(N^2)$ or unbounded queries that will fail as data grows.
2. **Information Leak:** Internal details exposed via the interface.
3. **Serialization Bottleneck:** Using JSON/Text for high-throughput data.
4. **Change Amplification:** One conceptual change requires touching multiple files.
5. **Metric Blind Spot:** Missing telemetry for I/O operations.
6. **Missing Boundary Limits:** Unsafe, unbounded data ingestion.

## Score Format: Complexity Debt
Provide a **Complexity Debt** report:
- **Scaling/Performance Risks:** [Count/List]
- **Architectural Leaks:** [Count/List]
- **Observability Gaps:** [Count/List]
- **Safety Violations:** [Count/List]
- **Overall Rating:** [C: X Scaling Risks, Y Leaks, Z Metric Gaps]

## Common Mistakes
- **Happy Path Coding:** Assuming $N$ will always be small.
- **JSON Everywhere:** Using JSON for sensor/high-frequency streams.
- **Silence on Failure:** Returning `null` or `false` without logging the "why" or the latency.
- **Classitis:** Creating too many tiny classes that just pass data around.
