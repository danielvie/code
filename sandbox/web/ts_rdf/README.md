# ◈ SemanticLabs: RDF & OWL Learning Path

A structured study material for learning the fundamentals of the Semantic Web using TypeScript and Bun.

## 🚀 Getting Started

This project uses `task` (Taskfile) for automation.

```bash
# List all study steps
task list-steps

# Run a specific step (e.g., Step 1)
task step-01

# Launch the interactive Web Dashboard
task dashboard
```

---

## 📚 Glossary: Symbols & Acronyms

| Symbol/Acronym | Full Name | Description |
| :--- | :--- | :--- |
| **RDF** | Resource Description Framework | The standard model for data interchange on the Web (Graph-based). |
| **OWL** | Web Ontology Language | A logic-based language for defining complex constraints and hierarchies. |
| **SPARQL** | SPARQL Protocol and RDF Query Language | The SQL equivalent for querying RDF graph data. |
| **URI / IRI** | Uniform/Internationalized Resource Identifier | A global unique string used to identify a resource (e.g., `http://example.org/Alice`). |
| **TTL / Turtle** | Terse RDF Triple Language | A common, human-readable text format for writing RDF data. |
| **S-P-O** | Subject-Predicate-Object | The "Triple" structure: the atomic unit of RDF data. |
| **RDFS** | RDF Schema | A basic vocabulary for defining classes and properties (e.g., `subClassOf`). |
| **Quad** | RDF Quadruple | A triple extended with a fourth element: the **Graph Name** (context). |
| **Literal** | RDF Literal | A plain data value, like a string, number, or date. |
| **NS** | Namespace | A prefix used to shorten long URIs (e.g., `foaf:` for `http://xmlns.com/foaf/0.1/`). |
| **a** | rdf:type | In Turtle, the symbol `a` is a shortcut for the predicate `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`. |
| **;** | Semicolon | In Turtle, used to repeat the same **Subject** for multiple predicates. |
| **.** | Period | In Turtle, used to end a triple statement. |

---

## 🛠️ Main Libraries
- **N3.js**: Fast, streaming RDF parser and in-memory store.
- **Comunica**: Modular SPARQL query engine.
- **Bun**: Ultra-fast JS runtime and task runner.

---

## 🌐 Common Vocabularies

In this project, we use several standard vocabularies to describe data:

| Prefix | Full Name | URI | Purpose |
| :--- | :--- | :--- | :--- |
| **foaf** | **Friend of a Friend** | `http://xmlns.com/foaf/0.1/` | Describing people, their names, and their connections (`foaf:Person`, `foaf:knows`). |
| **rdf** | RDF Syntax | `http://www.w3.org/1999/02/22-rdf-syntax-ns#` | Core RDF terms like `rdf:type` (shortcut: `a`). |
| **rdfs** | RDF Schema | `http://www.w3.org/2000/01/rdf-schema#` | Basic relationships like `rdfs:subClassOf` and `rdfs:label`. |
| **schema** | Schema.org | `http://schema.org/` | Used by search engines (Google, etc.) to describe everything from recipes to events. |

---

## 🧭 Study Steps
1. **01_triples.ts**: Core graph structure.
2. **02_sparql.ts**: Querying with SPARQL.
3. **03_crud.ts**: Managing data state.
4. **04_owl.ts**: Reasoning and Ontologies.
