

# What is SPARQL?
SPARQL (SPARQL Protocol and RDF Qeury Language) is the standard query language for RDF (Resource Description Framework) data.

- **Goal:** to retrieve, manupulate, and define data stored in RDF format across various data sources (knowledge graphs)
- **Data Model:** it queries triples (Subject-Predicate-Object) within a graph structure, now rows and columns in a relational table

# SPARQL vs. SQL: A Quick Comparison

While SPARQL and SQL share the goal of data retrieval, their underlying data models lead to different syntax and capabilities

| Feature    | SQL (Structured Query Language)                              | SPARQL (SPARQL Protocol and RDF Query Language)                        |
| ---------- | ------------------------------------------------------------ | ---------------------------------------------------------------------- |
| Data Model | "Relational tables (rows, columns)."                         | "Graph (triples: Subject, Predicate, Object)."                         |
| Basic Unit | Row (a record).                                              | Triple (a statement).                                                  |
| Joins      | Explicit JOIN clauses based on foreign keys.                 | Implicit joining where a variable is used in multiple triple patterns. |
| Keyword    | SELECT ... FROM ... WHERE ...                                | SELECT ... WHERE { ... }                                               |
| Open World | Closed World Assumption (data not present is assumed false). | Open World Assumption (data not present is simply unknown).            |

# The Core Analogy
- **SQL** `SELECT` is like **SPARQL** `SELECT` (retrieving variables)
- **SQL** `FROM` is implicitly handled by the SPARQL endpoint **SPARQL**
- **SQL** `WHERE` is like **SPARQL** `WHERE`, but instead of specifying conditions on columns, you specify *triple patterns*

# Examples of How to Use SPARQL

| SPARQL Element          | Description                                                                                                  |
| ----------------------- | ------------------------------------------------------------------------------------------------------------ |
| PREFIX                  | Shorthand for common URIs. foaf: is for the Friend of a Friend vocabulary.                                   |
| SELECT ?name ?age       | List the variables you want the query engine to return.                                                      |
| ?person a foaf:Person   | Triple Pattern 1: Find a Subject (?person) that has the type (a is shorthand for rdf:type) of a foaf:Person. |
| ?person foaf:name ?name | Triple Pattern 2: Find the value of foaf:name for that ?person and bind it to ?name.                         |
| ?person foaf:age ?age   | Triple Pattern 3: Find the value of foaf:age for that ?person and bind it to ?age.                           |

**SPARQL query**

```sparql
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?name ?age
WHERE {
    ?person a foaf:Person .
    ?person foaf:name ?name .
    ?person foaf:age ?age .
}
```

**filtering results**
Similar to SQL, you can restrict results using conditional expressions.

Goal: Find people who are interested in 'Hiking' and are younger than 35.

```sparql
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX ex-prop: <http://example.org/property/>

SELECT ?name ?age
WHERE {
    ?person a foaf:Person .
    ?person foaf:name ?name .
    ?person ex-prop:hasInterest "Hiking" .
    ?person foaf:age ?age .
    
    FILTER (?age < 35) # The filter expression
}
```

**handling missing data**
A critical difference from SQL. If a triple pattern is required and fails, the whole query fails for that subject. OPTIONAL allows the pattern to be missing without discarding the subject.

Goal: Find all people and their name, and include their age if it is available.
```sparql

PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?name ?age
WHERE {
    ?person a foaf:Person .
    ?person foaf:name ?name .
    
    OPTIONAL { ?person foaf:age ?age . } # If age is missing, ?age will be unbound (NULL)
}
```

**property paths**
Allows you to query for relationships without knowing the intermediate nodes, saving you from complex joins.

Goal: Find all "friends of friends" (people who are two degrees of separation away).

```sparql
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?name1 ?name3
WHERE {
    ?person1 foaf:name ?name1 .
    # The Property Path: Find a chain of two 'foaf:knows' relationships
    ?person1 foaf:knows/foaf:knows ?person3 . 
    ?person3 foaf:name ?name3 .
}
```