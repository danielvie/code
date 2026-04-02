const steps = {
    "1": {
        title: "Triples & Graphs",
        description: "RDF stores data as triples: <strong>Subject → Predicate → Object</strong>. Every piece of data is a node in a massive global graph.",
        code: `/**\\n * RDF is a graph format.\\n * Everything is a "triple":\\n * 1. Subject (Alice)\\n * 2. Predicate (knows)\\n * 3. Object (Bob)\\n */\\nimport * as N3 from 'n3';\\nconst store = new N3.Store();`,
        terminal: "> bun src/01_triples.ts\\n> Parsing Turtle file...\\n> Quad: Alice type Person\\n> Quad: Alice knows Bob\\n> Parsing complete. Total triples: 5",
        table: [
            { s: "ex:Alice", p: "rdf:type", o: "foaf:Person" },
            { s: "ex:Alice", p: "foaf:name", o: "\"Alice\"" },
            { s: "ex:Alice", p: "foaf:knows", o: "ex:Bob" }
        ]
    },
    "2": {
        title: "SPARQL Querying",
        description: "<strong>SPARQL</strong> is the standard query language for RDF. It allows you to search for patterns and retrieve structured data tables.",
        code: `/**\\n * SPARQL Query Structure:\\n * SELECT: variables to return\\n * WHERE: graph patterns to match\\n */\\nconst query = \`\\n  SELECT ?name ?friend_name WHERE {\\n    ?person foaf:name ?name .\\n    ?person foaf:knows ?friend .\\n  }\`;`,
        terminal: "> bun src/02_sparql.ts\\n> Executing SPARQL query...\\n> Results found: 2\\n+--------+-------------+\\n| name   | friend_name |\\n+--------+-------------+\\n| Alice  | Bob         |\\n+--------+-------------+",
        table: [
            { s: "Alice", p: "knows", o: "Bob" }
        ]
    },
    "3": {
        title: "CRUD Operations",
        description: "Modifying RDF is simple: you remove existing triples and add new ones. The <strong>N3.Store</strong> makes this efficient.",
        code: `/**\\n * To "update" in RDF:\\n * 1. Remove the old triple\\n * 2. Add the new triple\\n */\\nstore.removeQuads(oldTriples);\\nstore.addQuad(subject, predicate, newValue);`,
        terminal: "> bun src/03_crud.ts\\n> ADDED: Alice age 30\\n> MODIFIED: Name updated to 'Alice Wonderland'\\n> DELETED: Removed age triple\\n> Store synchronized.",
        table: [
            { s: "ex:Alice", p: "foaf:name", o: "\"Alice Wonderland\"" }
        ]
    },
    "4": {
        title: "OWL & Reasoning",
        description: "<strong>OWL/RDFS</strong> allow us to define hierarchies. Reasoning engines can infer new facts like 'If X is a Dog, X is also an Animal'.",
        code: `/**\\n * Reasoning/Entailment:\\n * Automatically generating NEW facts\\n * based on ontology rules.\\n */\\nconst inferred = reasoner.infer(graph);`,
        terminal: "> bun src/04_owl.ts\\n> Running RDFS Reasoner...\\n> Explicit: Fido is a Dog\\n> Inferred: Fido is a Mammal\\n> Inferred: Fido is an Animal\\n> Total facts: 3",
        table: [
            { s: "ex:Fido", p: "rdf:type", o: "ex:Dog" },
            { s: "ex:Fido", p: "rdf:type (inferred)", o: "ex:Animal" }
        ]
    }
};

const navItems = document.querySelectorAll('.nav-item');
const titleEl = document.getElementById('step-title');
const descEl = document.getElementById('step-description');
const codeEl = document.getElementById('code-display');
const termEl = document.getElementById('terminal-output');
const tableBody = document.getElementById('table-body');
const pathEl = document.querySelector('.path');

function updateStep(stepId) {
    const step = steps[stepId];
    titleEl.innerText = step.title;
    descEl.innerHTML = step.description;
    codeEl.innerText = step.code;
    termEl.innerText = step.terminal;
    pathEl.innerText = `~/ts_rdf > bun src/0${stepId}_${step.title.split(' ')[0].toLowerCase()}.ts`;

    // Update Table
    tableBody.innerHTML = '';
    step.table.forEach(row => {
        const tr = document.createElement('tr');
        tr.innerHTML = `<td>${row.s}</td><td>${row.p}</td><td>${row.o}</td>`;
        tableBody.appendChild(tr);
    });

    // Handle Nav Classes
    navItems.forEach(item => item.classList.remove('active'));
    document.querySelector(`[data-step="${stepId}"]`).classList.add('active');
}

navItems.forEach(item => {
    item.addEventListener('click', () => {
        updateStep(item.getAttribute('data-step'));
    });
});

// Initial Step
updateStep("1");
