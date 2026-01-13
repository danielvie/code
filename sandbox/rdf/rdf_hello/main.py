from rdflib import Graph, Literal, Namespace, URIRef
from rdflib.namespace import FOAF, RDF

# Define Namespaces
# SDO (Schema.org) and EX (Example)
SDO = Namespace("https://schema.org/")
EX = Namespace("http://example.org/")


def add_triples(graph):
    """
    Populates the graph with triples representing people,
    their attributes, and their relationships.
    """
    # Define Subjects
    alice = EX.Alice
    bob = EX.Bob
    charlie = EX.Charlie
    company = EX.TechCorp

    # --- Alice's Data ---
    graph.add((alice, RDF.type, FOAF.Person))
    graph.add((alice, FOAF.name, Literal("Alice Smith")))
    graph.add((alice, SDO.jobTitle, Literal("System Engineer")))
    graph.add((alice, FOAF.knows, bob))

    # --- Bob's Data ---
    graph.add((bob, RDF.type, FOAF.Person))
    graph.add((bob, FOAF.name, Literal("Bob Jones")))
    graph.add((bob, SDO.worksFor, company))
    graph.add((bob, FOAF.knows, charlie))

    # --- Charlie's Data ---
    graph.add((charlie, RDF.type, FOAF.Person))
    graph.add((charlie, FOAF.name, Literal("Charlie Davis")))
    graph.add((charlie, SDO.colleague, alice))

    # --- Company Data ---
    graph.add((company, RDF.type, SDO.Organization))
    graph.add((company, SDO.name, Literal("TechCorp Inc.")))


def main():
    # Initialize the Graph
    g = Graph()

    # Bind prefixes for prettier output (Turtle)
    g.bind("ex", EX)
    g.bind("foaf", FOAF)
    g.bind("schema", SDO)

    # 1. CREATE
    print("Adding triples to the graph...")
    add_triples(g)
    print(f"Graph now contains {len(g)} triples.\n")

    # 2. READ (Simple Loop)
    print("--- List of People and their Job Titles (if any) ---")
    for person in g.subjects(RDF.type, FOAF.Person):
        name = g.value(person, FOAF.name)
        job = g.value(person, SDO.jobTitle) or "Unspecified"
        print(f"Name: {name} | Job: {job}")


if __name__ == "__main__":
    main()
