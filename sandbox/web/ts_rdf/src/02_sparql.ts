/**
 * --- RDF Step 2: SPARQL Queries ---
 * 
 * SPARQL is the standard query language for RDF. If RDF is the graph data,
 * SPARQL is the SQL of that world. It allows you to search for patterns
 * (sub-graphs) within the main graph and return them as tables.
 * 
 * In this step, we use "Comunica", a powerful and modular SPARQL engine.
 */

import { QueryEngine } from '@comunica/query-sparql-rdfjs';
import * as N3 from 'n3';
import { readFileSync } from 'fs';
import path from 'path';

console.log("--- RDF Step 2: SPARQL Queries ---");
console.log("SPARQL is the SQL-like query language for RDF graphs.");

/**
 * High-level engine that parses and executes SPARQL queries.
 */
const engine = new QueryEngine();
const dataPath = path.join(import.meta.dir, '../data/small.ttl');

async function runQuery() {
    const parser = new N3.Parser();
    const store = new N3.Store();
    const turtleContent = readFileSync(dataPath, 'utf8');

    /**
     * The SPARQL Query:
     * - PREFIX: Shortcuts for long URIs.
     * - SELECT: Which variables we want in the output table.
     * - WHERE { ... }: The graph pattern we're looking for.
     * - OPTIONAL { ... }: Pattern that may or may not exist (won't exclude results if missing).
     */
    const query = `
        PREFIX : <http://example.org/>
        PREFIX foaf: <http://xmlns.com/foaf/0.1/>
        SELECT ?person ?name ?friend_name WHERE {
            ?person a foaf:Person ;
                    foaf:name ?name .
            OPTIONAL {
                ?person foaf:knows ?friend .
                ?friend foaf:name ?friend_name .
            }
        }
    `;

    // 1. Prepare an N3 store by parsing the Turtle file
    await new Promise((resolve, reject) => {
        parser.parse(turtleContent, (error: any, quad: any) => {
            if (error) reject(error);
            if (quad) store.add(quad);
            else resolve(null);
        });
    });

    console.log("Executing Query:\n", query);

    /**
     * 2. Run the Query on the N3 Store.
     * Comunica treats the 'store' as a data source.
     * It returns a 'bindingsStream'.
     */
    const bindingsStream = await engine.queryBindings(query, {
        sources: [store],
    });

    // 3. Process the results (Streaming)
    const results: any[] = [];
    bindingsStream.on('data', (binding: any) => {
        // You access variables from the SELECT clause using .get('variableName')
        results.push({
            person: binding.get('person')?.value,
            name: binding.get('name')?.value,
            friend: binding.get('friend_name')?.value || 'N/A'
        });
    });

    bindingsStream.on('end', () => {
        console.log("\nResults summarized as a table:");
        console.table(results);
    });

    bindingsStream.on('error', (error: any) => {
        console.error("Query Error:", error);
    });
}

runQuery();
