from rdflib import Graph, Literal, RDF, URIRef
from rdflib.namespace import FOAF, XSD


def rdf_add_triples() -> Graph:
    """Populates the RDF graph with triples for Alice, Bob, and Charlie."""

    # initialize graph
    g = Graph()

    # Define a new subject
    alice = URIRef("http://example.org/person/alice")
    bob = URIRef("http://example.org/person/bob")
    charlie = URIRef("http://example.org/person/charlie")

    # Define custom property URIs for consistency
    HAS_HOBBY = URIRef("http://example.org/property/hasHobby")
    HAS_INTEREST = URIRef("http://example.org/property/hasInterest")

    # Bind prefixes for cleaner serialization and SPARQL
    g.bind("ex-person", "http://example.org/person/")
    g.bind("ex-prop", "http://example.org/property/")

    # --- Alice's Data (Age 30, knows Bob) ---
    g.add((alice, RDF.type, FOAF.Person))
    g.add((alice, FOAF.name, Literal("Alice")))
    g.add((alice, FOAF.age, Literal(30, datatype=XSD.integer)))
    g.add((alice, HAS_HOBBY, Literal("Hiking")))
    g.add((alice, HAS_INTEREST, Literal("Space")))
    g.add((alice, FOAF.knows, bob))  # Alice knows Bob

    # --- Bob's Data (Age 25, knows Charlie) ---
    g.add((bob, RDF.type, FOAF.Person))
    g.add((bob, FOAF.name, Literal("Bob")))
    g.add((bob, FOAF.age, Literal(25, datatype=XSD.integer)))
    g.add((bob, HAS_HOBBY, Literal("Reading")))
    g.add((bob, HAS_INTEREST, Literal("History")))
    g.add((bob, FOAF.knows, charlie))  # Bob knows Charlie

    # --- Charlie's Data (Person, knows Alice, no age or hobby) ---
    g.add((charlie, RDF.type, FOAF.Person))
    g.add((charlie, FOAF.name, Literal("Charlie")))
    g.add((charlie, HAS_INTEREST, Literal("Hiking")))  # Shared interest with Alice
    g.add((charlie, FOAF.knows, alice))  # Charlie knows Alice

    return g


def rdf_print_graph(g) -> None:
    print("--- My Graph (Turtle Format) ---")
    print(g.serialize(format="turtle"))
    print(f"Total Triples: {len(g)}")
    print("-----------------------------------")


def rdf_queries(g) -> None:
    # Set prefixes for SPARQL queries (best practice)
    # The prefixes match the ones we bound to the graph object
    PREFIXES = """
    PREFIX foaf: <http://xmlns.com/foaf/0.1/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    PREFIX ex-prop: <http://example.org/property/>
    """

    # Query 1: Find all names
    query1 = (
        PREFIXES
        + """
    SELECT ?name
    WHERE {
        ?person a foaf:Person .
        ?person foaf:name ?name .
    }
    """
    )

    print("\n--- SPARQL Query 1: Find all names ---")
    result1 = g.query(query1)
    for r in result1:
        print(f"Name: {r.name}")

    # Query 2: Find Alice's age
    query2 = (
        PREFIXES
        + """
    SELECT ?age
    WHERE {
        ?person a foaf:Person .
        ?person foaf:name "Alice" .
        ?person foaf:age ?age .
    }
    """
    )

    print("\n--- SPARQL Query 2: Find Alice's age ---")
    results2 = g.query(query2)
    for row in results2:
        print(f"Alice's Age: {row.age}")

    # --- NEW QUERIES ADDED ---

    # Query 3: Find who knows whom
    query3 = (
        PREFIXES
        + """
    SELECT ?name1 ?name2
    WHERE {
        ?person1 foaf:name ?name1 .
        ?person1 foaf:knows ?person2 .
        ?person2 foaf:name ?name2 .
    }
    """
    )

    print("\n--- SPARQL Query 3: Find who knows whom ---")
    results3 = g.query(query3)
    for row in results3:
        print(f"{row.name1} knows {row.name2}")

    # Query 4: Find people with Optional Age/Hobby (OPTIONAL)
    query4 = (
        PREFIXES
        + """
    SELECT ?name ?age ?hobby
    WHERE {
        ?person a foaf:Person .
        ?person foaf:name ?name .
        
        OPTIONAL { ?person foaf:age ?age . }    # Age is optional
        OPTIONAL { ?person ex-prop:hasHobby ?hobby . } # Hobby is optional
    }
    ORDER BY ?name
    """
    )

    print("\n--- SPARQL Query 4: People with Optional Age/Hobby (OPTIONAL) ---")
    results4 = g.query(query4)
    for row in results4:
        age_str = str(row.age) if row.age else "N/A"
        hobby_str = str(row.hobby) if row.hobby else "N/A"
        print(f"Name: {row.name} | Age: {age_str} | Hobby: {hobby_str}")

    # Query 5: People interested in 'Hiking' and younger than 35 (FILTER)
    # Note: Charlie is interested in Hiking, but has no age, so he is excluded by the FILTER
    query5 = (
        PREFIXES
        + """
    SELECT ?name ?age
    WHERE {
        ?person a foaf:Person .
        ?person foaf:name ?name .
        ?person ex-prop:hasInterest "Hiking" .
        ?person foaf:age ?age .
        
        FILTER (?age < 35) 
    }
    """
    )

    print("\n--- SPARQL Query 5: People interested in Hiking AND age < 35 (FILTER) ---")
    results5 = g.query(query5)
    for row in results5:
        print(f"Name: {row.name}, Age: {row.age}")

    # Query 6: Find a "friend of a friend" (Property Paths)
    # This finds people connected by two foaf:knows links.
    query6 = (
        PREFIXES
        + """
    SELECT ?name1 ?name3
    WHERE {
        ?person1 foaf:name ?name1 .
        ?person1 foaf:knows/foaf:knows ?person3 . # This is the Property Path!
        ?person3 foaf:name ?name3 .
    }
    """
    )
    print("\n--- SPARQL Query 6: Find a 'Friend of a Friend' (Property Path) ---")
    results6 = g.query(query6)
    for row in results6:
        print(f"{row.name1} knows someone who knows {row.name3}")


def main():
    # defining subject URIs
    g = rdf_add_triples()

    # print graph
    rdf_print_graph(g)

    # perform SPARQL queries
    rdf_queries(g)


if __name__ == "__main__":
    main()
