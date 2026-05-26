/**
 * --- RDF Step 4: OWL & Ontologies ---
 * 
 * RDF is just about facts. Ontologies (OWL) and RDF Schema (RDFS) add
 * "Meaning" to those facts.
 * 
 * "Reasoning" (or Entailment) is the process of generating NEW facts
 * automatically from the existing facts + ontology rules.
 * 
 * E.g., Rule: subClassOf is transitive.
 * Fact 1: Dog subClassOf Mammal
 * Fact 2: Mammal subClassOf Animal
 * Reasoning: Dog subClassOf Animal
 */

import * as N3 from 'n3';
const { DataFactory } = N3;
const { namedNode } = DataFactory;

console.log("--- RDF Step 4: OWL & Ontologies ---");
console.log("Ontologies define relationships and constraints in our graph.");

const store = new N3.Store();
// Standard namespaces
const rdfs = 'http://www.w3.org/2000/01/rdf-schema#';
const rdf = 'http://www.w3.org/1999/02/22-rdf-syntax-ns#';
const ex = 'http://example.org/';

// 1. Define Class hierarchy (Schema/Ontology level)
// Dog is a specific kind of Mammal
store.addQuad(namedNode(ex + 'Dog'), namedNode(rdfs + 'subClassOf'), namedNode(ex + 'Mammal'));
// Mammal is a specific kind of Animal
store.addQuad(namedNode(ex + 'Mammal'), namedNode(rdfs + 'subClassOf'), namedNode(ex + 'Animal'));

// 2. Add Instance (Data level)
// Fido.
store.addQuad(namedNode(ex + 'Fido'), namedNode(rdf + 'type'), namedNode(ex + 'Dog'));

console.log("\nOntology Hierarchy in Store:\nDog subClassOf Mammal\nMammal subClassOf Animal\n\nInstance:\nFido type Dog");

/**
 * 3. BASIC REASONING ENGINE (Recursive Function)
 * In large systems, we use full reasoners (like Pellet or HermiT), 
 * but here we implement a simple recursive subClassOf logic.
 */
function getSuperClasses(cls: string): string[] {
    const superClasses: string[] = [];
    const directSuper = store.getQuads(namedNode(cls), namedNode(rdfs + 'subClassOf'), null, null);
    
    directSuper.forEach(q => {
        const parentClassId = q.object.id;
        superClasses.push(parentClassId);
        // Recursively find the parents of the parent (transitive check)
        superClasses.push(...getSuperClasses(parentClassId));
    });
    
    return [...new Set(superClasses)]; // Return unique classes only
}

const fidoSuper = getSuperClasses(ex + 'Dog');
console.log("\nInferred facts for Fido (using RDFS reasoning):");
console.log(`- Type: Dog (Stated)`);
fidoSuper.forEach(sc => console.log(`- Type: ${sc} (Inferred)`));

console.log("\n💡 This is the power of RDF: you don't have to state that Fido is an Animal.");
console.log("The machine can work that out from the knowledge graph.");
