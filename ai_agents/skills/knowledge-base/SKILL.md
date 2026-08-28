---
name: knowledge-base
description: Build or maintain a navigable, source-traceable knowledge map for a dense project. Use when explicitly asked to document project structure, explain concepts, summarize inspected references, or record a dated synthesis.
disable-model-invocation: true
---

# Project knowledge

Use this skill only when the user explicitly invokes it. Build the knowledge map in the target project at `project-knowledge/`.

## Information policy

Keep four kinds of content separate:

1. **Project descriptions** — factual descriptions of the files, directories, systems, and pipelines in this project.
2. **Concept explanations** — explanations of domain terms, models, and mechanisms.
3. **Reference summaries** — neutral descriptions of what an inspected external source says: its problem, setting, approach, contribution, evidence, assumptions, limitations, useful concepts or equations, and source pointers.
4. **Provisional syntheses** — dated, question-specific analysis such as comparisons, project relevance, novelty assessments, tradeoffs, and recommendations.

The first three kinds must be descriptive and traceable to sources. Do not put “relation to this project,” novelty judgments, rewrite recommendations, or other conclusions in neutral reference summaries. Put those conclusions only in a dated file under `project-knowledge/syntheses/`, and label them as provisional.

## Workflow

1. Read applicable `AGENTS.md` files and identify the project root.
2. Read existing orientation and scope files when present, including `README.md`, `CONTEXT.md`, `GOAL.md`, and build manifests.
3. Inventory the project before writing: source files, generated files, dependencies, research material, and existing documentation.
4. Inspect whether `project-knowledge/` already exists. Preserve user-authored files; update them rather than overwriting them.
5. Create directories lazily. Create only files and directories supported by verified information; do not create empty placeholder trees.
6. Write or update a short `project-knowledge/README.md` as an index. It should explain the folder’s purpose, route readers by task, identify authoritative source files, distinguish descriptions from provisional syntheses, tell readers to open only relevant deeper files, and state the scope represented by the map.
7. Add only the descriptive project, concept, artifact, and reference files needed for the request. Prefer links and source paths over duplicating large contents.
8. For literature or external references, create one summary per source actually inspected. Never infer source content from a title, citation, or metadata alone.
9. Create a dated synthesis only when the user explicitly asks for analysis, comparison, project relevance, novelty, or a recommendation. You may suggest a synthesis, but do not create one merely because patterns seem interesting.
10. Verify the resulting inventory and report the files changed. Do not run expensive builds unless the knowledge work changes build inputs or the user asks for validation.

## Maintenance rules

- Keep canonical domain terminology in the project’s existing `CONTEXT.md` when one exists.
- Treat `project-knowledge/concepts/` as explanatory material or a mirror, not a replacement glossary.
- Keep project-specific implementation details out of `CONTEXT.md`.
- Mark uncertain or unverified information instead of filling gaps by inference.
- Preserve the established vocabulary and structure of an existing knowledge map unless the user asks for a redesign.
- Offer, but do not automatically make, a one-line `AGENTS.md` pointer such as: “For project orientation and navigation, read `project-knowledge/README.md` before inspecting project files.” Apply it only after the user confirms.

## Related skills

Use `research` when the user needs primary-source research, `domain-modeling` when canonical terminology genuinely changes, `shared-understanding` when scope or conclusions need discussion, and `handoff` when the implementation needs to be recorded for another session. This skill remains usable without any of them.
