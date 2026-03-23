# Math Monorepo

Explore monorepos using `uv` with this math themed project.

## Project Structure
- `packages/add`: Addition logic
- `packages/sub`: Subtraction logic
- `packages/mult`: Multiplication logic
- `apps/calculator`: A terminal app that uses all math packages

## How to Run

### Command to run the calculator:
```bash
uv run python apps/calculator/main.py
```

### Try adding a new package:
```bash
# Example: Adding a division package
uv init --lib packages/div
uv add --package calculator packages/div
```
