---
name: sparql-querying
description: Use when writing SPARQL queries against RDF/Turtle datasets, querying knowledge graphs, or analyzing linked data with incomplete results or complex filtering requirements
---

# SPARQL Querying

## Overview

Systematic approach to writing correct SPARQL queries against RDF data. Core principle: **Complete data visibility before query construction**—never query data you haven't fully read.

## When to Use

- Querying Turtle (.ttl) or RDF files
- Filtering entities with multiple criteria (roles, dates, geographic constraints)
- Aggregating data (counts, concatenation, grouping)
- Results are incomplete or don't match expectations
- Complex joins across multiple entity types

## Workflow

### Step 1: Complete Data Acquisition

**Read the entire source file.** No exceptions.

```markdown
- Read full Turtle/RDF file without truncation
- If truncated: request continuation or use pagination
- Count distinct entities to verify completeness
- Document: entity types, predicates, relationships, data types
```

**Verification:** Entity count matches expectations before proceeding.

### Step 2: Schema Mapping

Identify the data structure:

| Element | What to Capture |
|---------|-----------------|
| Entity types | All classes (`rdfs:Class`, `rdf:type`) |
| Predicates | All properties used |
| Relationships | Entity → entity connections |
| Literals | String, date, integer formats |
| Naming conventions | Prefixes, codes, identifiers |

**Common academic patterns:**
- Roles: "Professor of", "Associate Professor", "Full Professor"
- Dates: ISO 8601, may need boundary testing
- Geography: ISO country codes (EU list below)
- Enrollment: may span departments

### Step 3: Criteria Decomposition

Break each filter into discrete, testable conditions:

```
"Full professors in EU countries"
├── Role filter: STRSTARTS(?role, "Professor of")
├── Country filter: ?country IN (EU_CODES)
└── Output: name, ALL countries (not just EU)
```

**Critical distinction:** Filter criteria ≠ Output requirements. Filter by EU, output all countries.

### Step 4: Query Construction

Build incrementally:

1. **Start with most restrictive filter** (reduces result set)
2. **Add one filter at a time**, verify intermediate results
3. **Include all SELECT variables** needed for output
4. **Add aggregation last** (GROUP BY, GROUP_CONCAT)

**Syntax checklist:**
- All prefixes declared (`PREFIX foaf: <...>`)
- FILTER expressions properly closed
- String comparisons: STRSTARTS, CONTAINS, or regex
- Numeric comparisons handle data types

**Aggregation pattern:**
```sparql
SELECT ?entity (GROUP_CONCAT(DISTINCT ?value; separator=", ") AS ?values)
WHERE { ... }
GROUP BY ?entity
```

### Step 5: Verification

Test against known expectations:

| Check | Method |
|-------|--------|
| Inclusion | Expected entities present? |
| Exclusion | Unexpected entities absent? |
| Aggregation | Manual count matches? |
| Cross-reference | Trace 2-3 entities through source data |

## Quick Reference

### EU Country Codes (Current)
```
AT, BE, BG, HR, CY, CZ, DK, EE, FI, FR, DE, GR, HU, IE, IT, LV, LT, LU, MT, NL, PL, PT, RO, SK, SI, ES, SE
```

### Common Filter Patterns

| Pattern | SPARQL |
|---------|--------|
| Prefix match | `FILTER(STRSTARTS(?role, "Professor"))` |
| Contains | `FILTER(CONTAINS(?name, "Smith"))` |
| Membership | `FILTER(?country IN ("FR", "DE", "IT"))` |
| Date range | `FILTER(?date >= "2020-01-01"^^xsd:date)` |
| Exists | `FILTER EXISTS { ?x rdf:type ?type }` |

### Aggregation Functions

| Function | Use Case |
|----------|----------|
| COUNT | Number of related entities |
| GROUP_CONCAT | Concatenate multiple values |
| SUM | Numeric totals |
| AVG | Averages |
| SAMPLE | Pick one value from group |

## Common Pitfalls

### Incomplete Data Reading
**Problem:** Truncated data → missing entities  
**Fix:** Re-read file, confirm completeness before querying

### Criterion Confusion
**Problem:** Filtering by X but only outputting X  
**Fix:** Distinguish filter criteria from output requirements

### Date Boundaries
**Problem:** Inclusive vs exclusive date comparisons  
**Fix:** Test boundary dates explicitly

### Missing GROUP BY
**Problem:** Aggregation without grouping  
**Fix:** Every non-aggregated SELECT variable must be in GROUP BY

### Incorrect EU List
**Problem:** Outdated or incomplete EU codes  
**Fix:** Use comprehensive list above (27 members)

### Cross-Entity Miscounts
**Problem:** Wrong join path (e.g., students per department)  
**Fix:** Trace full predicate path, verify join conditions

## Testing Protocol

1. **Syntax check:** Query parses without errors
2. **Subset test:** Run on known subset with expected results
3. **Full test:** Run on complete dataset
4. **Manual verification:** Trace 2-3 results through source
5. **Boundary test:** Edge cases in filters (dates, counts, strings)

## Iteration Approach

Results don't match expectations?

1. Isolate failing filter condition
2. Test each filter independently
3. Check false negatives (should appear, don't)
4. Check false positives (shouldn't appear, do)
5. Adjust logic, re-verify