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


def main():
    g = Graph()

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

    print(f"Graph initialized with {len(g)} triples.")

    print("----------------")
    print("----------------")
    print(g.serialize(format="turtle"))
    print("----------------")
    print("----------------")

    print("================")
    print(all_that_knows_cpp(g))


def all_that_knows_cpp(
    g: Graph,
):
    res: list[str] = []
    lang = cpp_lang

    for person in g.subjects(SCHEMA.knowsLanguage, lang):
        name = g.value(person, FOAF.name)

        res.append(f"- {name} knows {lang}")

    return res


if __name__ == "__main__":
    main()
