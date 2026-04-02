/**
 * --- RDF Step 3: CRUD Operations ---
 * 
 * Unlike a relational database where you might 'UPDATE' a record, RDF graphs
 * are essentially a collection of immutable facts (triples).
 * 
 * To "update" something, we strictly follow this pattern:
 * 1. find and remove the old triple.
 * 2. add the new triple.
 * 
 * CRUD translates to:
 * - Create: store.addQuad(...)
 * - Read:   store.getQuads(...)
 * - Update: store.removeQuads(...) + store.addQuad(...)
 * - Delete: store.removeQuads(...)
 */

import * as N3 from 'n3';
const { DataFactory } = N3;
const { namedNode, literal } = DataFactory;

console.log("--- RDF Step 3: CRUD Operations ---");
console.log("Modify, Add, and Delete data in the RDF store.");

/**
 * NAMED NODES (<URI>): Represents an global entity (e.g., Alice).
 * LITERALS ("string"): Represents a flat value (e.g., "Alice").
 */
const store = new N3.Store();
const PREFIXES = {
    ex: 'http://example.org/',
    schema: 'http://schema.org/',
};

// CREATE: Initial fact about Alice
store.addQuad(
    namedNode(PREFIXES.ex + 'Alice'), // Subject
    namedNode(PREFIXES.schema + 'name'), // Predicate
    literal('Alice') // Object
);

console.log("\nInitial Triple in Store:");
console.log(`- Subject: ${PREFIXES.ex + 'Alice'}, Predicate: ${PREFIXES.schema + 'name'}, Object: Alice`);

// CREATE: Add Alice's age
store.addQuad(
    namedNode(PREFIXES.ex + 'Alice'),
    namedNode(PREFIXES.schema + 'age'),
    literal(30)
);
console.log("\nADD: Added Alice's age (30). Store size:", store.size);

/**
 * UPDATE: In RDF, we search for the specific triple and replace it.
 * Here we update the name to "Alice Wonderland".
 */
const oldQuads = store.getQuads(namedNode(PREFIXES.ex + 'Alice'), namedNode(PREFIXES.schema + 'name'), null, null);
if (oldQuads.length > 0) {
    store.removeQuads(oldQuads); // Wipe old triples
    store.addQuad(               // Add new triple
        namedNode(PREFIXES.ex + 'Alice'),
        namedNode(PREFIXES.schema + 'name'),
        literal('Alice Wonderland')
    );
}
console.log("\nMODIFY: Alice's name updated to 'Alice Wonderland'.");

// DELETE: Remove a specific property triple completely
const ageQuads = store.getQuads(namedNode(PREFIXES.ex + 'Alice'), namedNode(PREFIXES.schema + 'age'), null, null);
store.removeQuads(ageQuads);
console.log("\nDELETE: Removed Alice's age triple.");

console.log("\nFinal Triples in Store (Read All):");
// null, null, null is like SELECT * in SQL
store.getQuads(null, null, null, null).forEach(q => {
    console.log(`- ${q.subject.id} ${q.predicate.id} ${q.object.id}`);
});
