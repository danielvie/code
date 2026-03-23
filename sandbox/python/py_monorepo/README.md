# Math Monorepo Examples

Welcome to the **Math Monorepo Examples** project! This repository showcases two different implementations of a math-related monorepo, each using different toolsets and programming languages (Python and Bun).

---

## 🏗️ What is a Monorepo?

A **Monorepo** (monolithic repository) is a software development strategy where code for many projects, libraries, and applications is stored in a single version control repository. 

Unlike a "polyrepo" approach (where each project lives in its own repository), a monorepo focuses on keeping related codebases together to simplify dependency management and cross-project collaboration.

### ✅ Advantages of Monorepos

1.  **Simplified Dependency Management**: All projects share the same version of a library. Updating a shared package immediately reflects across all consuming applications, avoiding "version hell."
2.  **Atomic Commits**: You can land a breaking change in a library and fix all its consumers in a single commit, ensuring the codebase is always in a consistent state.
3.  **Cross-Project Refactoring**: IDEs and tools can easily refactor code across multiple packages because they are all part of the same workspace.
4.  **Consistency**: Shared configurations (linting, formatting, CI/CD) can be enforced globally across all sub-projects.
5.  **Visibility**: Developers can easily discover and reuse code from other teams or project areas without having to clone separate repositories.

### ⚠️ Disadvantages of Monorepos

1.  **Tooling Complexity**: Standard tools often struggle with large monorepos. You need specialized tools (like **Nx**) to handle selective builds, caching, and task orchestration.
2.  **Scalability**: As the repository grows, git operations (like cloning and fetching) can become slower if not managed correctly (e.g., using sparse checkouts).
3.  **Permissions**: Most version control systems (like Git) manage permissions at the repository level. Granular access control for specific folders can be difficult to implement.
4.  **CI Build Times**: Without intelligent task orchestration (affected analysis), CI/CD pipelines might try to rebuild everything on every commit, leading to slow feedback loops.

---

## 🚀 The Projects in This Repository

This workspace contains two distinct examples:

### 🐍 [math-monorepo](./math-monorepo) (Python)
- **Toolchain**: Managed by `uv` (a fast Python package manager and workspace runner).
- **Structure**: Uses `packages/` for reusable libraries and `apps/` for executable applications.
- **Orchestration**: Integrated with **Nx** for unified task running.

### 🍞 [math-monorepo-bun](./math-monorepo-bun) (Bun/TypeScript)
- **Toolchain**: Managed by `Bun` (a fast JavaScript runtime, package manager, and test runner).
- **Structure**: Uses standard TypeScript monorepo patterns with `package.json` workspaces.
- **Orchestration**: Integrated with **Nx** to leverage advanced caching and dependency graphing.

---

## 🛠️ Usage with Nx

Both projects are powered by **Nx** to provide a consistent developer experience across languages:

- **List projects**: `nx show projects`
- **Run all tests**: `nx run-many --target=test`
- **Build everything**: `nx run-many --target=build`
