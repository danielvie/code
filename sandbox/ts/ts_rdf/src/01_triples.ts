/**
 * --- RDF Step 1: Introduction to Triples ---
 * 
 * In this version, we've separated the PARSING from the QUERYING
 * using a modern Promise approach. This makes the code much 
 * more direct and readable.
 */

import * as N3 from 'n3';
import { readFileSync } from 'fs';
import path from 'path';

console.log("--- RDF Step 1: Introduction to Triples ---");
console.log("\nIn RDF, data is represented as a 'Subject - Predicate - Object' triple.");
console.log("Example: Alice (Subject) knows (Predicate) Bob (Object).\n");

// 1. Setup the Store and Parser
const store = new N3.Store();
const parser = new N3.Parser();

// 2. Load the data from our Turtle file
const dataPath = path.join(import.meta.dir, '../data/small.ttl');
const turtleData = readFileSync(dataPath, 'utf8');

console.log("Parsing Turtle file content...");

/**
 * HELPER: This function wraps the async parser into a Promise.
 * It populates the store and only completes when the file is fully read.
 */
async function loadTurtleData(data: string) {
    return new Promise((resolve, reject) => {
        parser.parse(data, (error, quad) => {
            if (error) {
                console.error("Parsing error:", error);
                reject(error);
            }
            if (quad) {
                // We add each triple/quad to the in-memory store
                store.add(quad);
            } else {
                // When 'quad' is null, it means we reached the end of the file
                resolve(true);
            }
        });
    });
}

// -------------------------------------------------------------
// MAIN FLOW: Load, then Query
// -------------------------------------------------------------

// Wait for the parsing to finish before doing anything else
await loadTurtleData(turtleData);

console.log("Parsing complete. Total triples in store:", store.size);

// --- QUERY 1: Everything about Alice ---
const aliceRecords = store.getQuads('http://example.org/Alice', null, null, null);
console.log("\nFacts about Alice:");
aliceRecords.forEach(q => {
    console.log(`- Predicate: ${q.predicate.id}, Object: ${q.object.id}`);
});

// --- QUERY 2: Specific restriction (Alice's name) ---
const namePredicate = 'http://xmlns.com/foaf/0.1/name';
const [nameMatch] = store.getQuads('http://example.org/Alice', namePredicate, null, null);
if (nameMatch) {
    console.log(`\nSpecific query: Alice's foaf:name is: ${nameMatch.object.id}`);
}

// --- QUERY 3: Who does Alice know? ---
const knowsPredicate = 'http://xmlns.com/foaf/0.1/knows';
const aliceConnections = store.getQuads('http://example.org/Alice', knowsPredicate, null, null);
console.log("\nWho does Alice know?");
aliceConnections.forEach(q => {
    console.log(`- Alice knows: ${q.object.id}`);
});

// --- QUERY 4: Finding Alice's Toys ---
const ownsPredicate = 'http://example.org/owns';
const typePredicate = 'http://www.w3.org/1999/02/22-rdf-syntax-ns#type';
const toyType = 'http://example.org/Toy';

const alicesStuff = store.getQuads('http://example.org/Alice', ownsPredicate, null, null);

console.log("\nToys owned by Alice (filters from Turtle file):");
alicesStuff.forEach(quad => {
    const itemUri = quad.object.id;
    // Check if THIS item has a triple that says it's a Toy
    const isToy = store.getQuads(itemUri, typePredicate, toyType, null).length > 0;
    if (isToy) {
        const itemName = itemUri.split('/').pop();
        console.log(`- 🧸 ${itemName}`);
    }
});

// --- QUERY 5: Get all itens Alice owns and print a list of the item and its category
console.log("\nAll items owned by Alice and their categories:");
alicesStuff.forEach(quad => {
    const itemUri = quad.object.id;

    // get the list of quads matching the type
    const typeQuads = store.getQuads(itemUri, typePredicate, null, null);

    // get the first one and get its ID
    const itemType = typeQuads[0]?.object.id.split('/').pop() || "Unknown";
    const itemName = itemUri.split('/').pop();
    
    console.log(`- ${itemName} is a ${itemType}`);
});


console.log("\nTIP: By using 'await', we no longer need to nest our queries inside deep callbacks!");
