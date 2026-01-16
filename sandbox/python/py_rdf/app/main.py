from rdflib import RDF, Graph, Literal, Namespace
from rdflib.namespace import FOAF

# 1. Setup Namespaces
SCHEMA = Namespace("https://schema.org/")
MYDATA = Namespace("http://example.org/data/")
TECH = Namespace("http://example.org/tech/")

# 2. Define Entities
bob = MYDATA.bob
alice = MYDATA.alice
esp32_project = MYDATA.esp32_firmware
rust_lang = TECH.rust
cpp_lang = TECH.cpp


def add_triples(g):
    # 3. Adding Triples: People & Roles
    g.add((bob, RDF.type, FOAF.Person))
    g.add((bob, FOAF.name, Literal("Bob Builder")))
    g.add((bob, SCHEMA.jobTitle, Literal("System Engineer")))

    g.add((alice, RDF.type, FOAF.Person))
    g.add((alice, FOAF.name, Literal("Alice Ada")))
    g.add((alice, SCHEMA.jobTitle, Literal("Software Architect")))

    # 4. Adding Triples: Relationships & Skills
    g.add((alice, FOAF.knows, bob))
    g.add((bob, SCHEMA.knowsLanguage, cpp_lang))
    g.add((alice, SCHEMA.knowsLanguage, rust_lang))

    # 5. Adding Triples: Project Details
    g.add((esp32_project, RDF.type, SCHEMA.SoftwareSourceCode))
    g.add((esp32_project, SCHEMA.name, Literal("ESP32 Secure Gateway")))
    g.add((esp32_project, SCHEMA.programmingLanguage, cpp_lang))

    # Link People to Projects
    g.add((bob, SCHEMA.contributor, esp32_project))
    g.add((alice, SCHEMA.editor, esp32_project))

    # Binding prefixes
    g.bind("foaf", FOAF)
    g.bind("schema", SCHEMA)
    g.bind("mydata", MYDATA)
    g.bind("tech", TECH)


def main():
    g = Graph()

    add_triples(g)

    update_alice_language(g)

    print(f"Graph initialized with {len(g)} triples.")

    print(g.serialize(format="turtle"))
    update_with_sparql(g)
    print(g.serialize(format="turtle"))

    print("================")
    # all_that_knows_cpp(g)

    query_all_alice(g)

    # query_projects(g)


def update_alice_language(g: Graph):
    # triple to remove
    triple = (alice, SCHEMA.knowsLanguage, rust_lang)

    # remove it from graph
    g.remove(triple)

    # add a new relationship
    g.add((alice, SCHEMA.knowsLanguage, cpp_lang))


def query_projects(g: Graph):
    # define the SPARQL query string

    query_str = """
    SELECT ?personName ?projectName
    WHERE {
        ?person foaf:name ?personName .
        ?person schema:contributor ?project .
        ?project schema:name ?projectName .
    }
    """

    # execute the query
    results = g.query(query_str)

    print("\n--- Project Contributions ---")

    for row in results:
        print(f"{row.personName} worked on '{row.projectName}'")


def query_all_alice(g: Graph):
    for s, p, o in g.triples((alice, None, None)):
        print("s, p, o: ", s, p, o)


def update_with_sparql(g: Graph):
    update_str = """
    DELETE { ?alice schema:jobTitle "Software Architect" }
    INSERT { ?alice schema:jobTitle "Chief Technology Officer" }
    WHERE {
        ?alice foaf:name "Alice Ada"
    }
    """

    g.update(update_str)


def all_that_knows_cpp(
    g: Graph,
):
    for s, p, o in g.triples((alice, None, None)):
        print(f"s, p, o: {s}, {p}, {o}")


if __name__ == "__main__":
    main()
