# OSLC Proof of Concept Architecture

This documentation details the architecture, design, and implementation of the OSLC (Open Services for Lifecycle Collaboration) Proof of Concept (PoC) project. The project is split into a Java Spring Boot backend and a SvelteKit frontend, providing a full-stack implementation of a functional OSLC provider and a consumer client.

## 1. Backend Architecture

The backend is built using **Java Spring Boot (2.7.18)** and **Jersey (JAX-RS)**. It serves as an OSLC Service Provider that exposes resources (specifically, `Defect` resources) using RESTful endpoints that adhere to OSLC specifications.

### Eclipse Lyo and OSLC Integration
The core of the backend relies on **Eclipse Lyo**, a Java SDK designed specifically to help developers implement OSLC specifications quickly.
- **Annotations**: The server leverages Lyo's JAX-RS annotations (`@OslcService`, `@OslcQueryCapability`, `@OslcCreationFactory`) to automatically define and document the API's capabilities.
- **Resource Models**: Data models like `Defect` inherit from `AbstractResource` and use Lyo annotations (`@OslcName`, `@OslcPropertyDefinition`) to map standard Java class properties to OSLC vocabulary schemas.
- **Content Negotiation**: Lyo provides message body writers and readers (`JenaProvider`, `Json4JProvider`). We configured the JAX-RS `JerseyConfig` to automatically register these Lyo providers, enabling the server to automatically consume and produce OSLC standard formats like `application/ld+json` (JSON-LD) and `application/rdf+xml`.

### Endpoints and CRUD Operations
The backend provides entirely RESTful operations mapped under `/provider/{id}`:
- `GET /catalog`: Exposes the central OSLC Service Provider Catalog.
- `GET /provider/{id}`: Returns the Service Provider details.
- `GET /provider/{id}/defects`: Queries and returns a graph of available Defects. 
- `POST /provider/{id}/defects`: Creates a new Defect using Lyo's Creation Factory specification. The server consumes JSON-LD payloads, automatically assigns a unique UUID identifier, and persists the entity in memory.
- `PUT /provider/{id}/defects/{defectId}`: Updates existing Defect metadata.
- `DELETE /provider/{id}/defects/{defectId}`: Deletes a specific defect.
- `DELETE /provider/{id}/defects`: A custom utility endpoint to flush the in-memory array list of all defects.

## 2. Frontend Architecture

The frontend is a lightweight Single Page Application (SPA) built using **SvelteKit**, **Vite**, and **Tailwind CSS**. It acts as an OSLC Consumer client.

### User Interface
The UI uses a modern, responsive "stacked" dashboard layout constructed with Tailwind CSS using a 60-30-10 color palette (dark space theme with neon accents). It's partitioned into:
- **Control Panel**: Contains dynamic summary information about the connected OSLC Catalog and offers interactive inputs and action buttons for performing CRUD operations.
- **Componentized Defect Cards**: The UI dynamically renders individual `DefectCard.svelte` components. These cards maintain their own isolated state, cleanly wrapping inline "Edit" and "Delete" interactions without bloating the main document scope.
- **Results Pipeline**: A toggleable debug component capable of reflecting the raw, parsed JSON-LD semantic structure directly to the user.

### Client-to-Server Connection & JSON-LD
The client communicates with the OSLC backend entirely over HTTP via the native `fetch` API. 
Because of CORS restrictions and the differing ports between the Svelte Dev Server (3000) and Spring Boot Server (8080), **Vite's proxy configuration** (`vite.config.ts`) transparently forwards API requests starting with `/catalog` or `/provider` directly to the backend.

Crucially, the client explicitly interfaces via **JSON-LD**:
- **Headers**: Every fetch call enforces `Accept: application/ld+json` (and `Content-Type: application/ld+json` or `application/json` for mutation verbs).
- **Payload Structuring**: Because Eclipse Lyo enforces strict Resource Definitions during parsing on the Java backend, the client must inject the appropriate semantic namespaces into mutations. 
  
An example OSLC POST payload constructed by the Svelte Client:
```json
{
  "prefixes": {
    "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    "dcterms": "http://purl.org/dc/terms/",
    "oslc": "http://open-services.net/ns/core#",
    "oslc_cm": "http://open-services.net/ns/cm#"
  },
  "dcterms:title": "Custom Bug",
  "dcterms:description": "Authentication failure on login screen."
}
```

### Response Parsing
Standard arrays are not guaranteed in OSLC JSON-LD. A server response may include strictly an `oslc:ResponseInfo` object, a single `{ ... }` object, or an array inside a `@graph` property. The Svelte logic leverages a derived reactive block to safely unpack these variations into an iterable structure, ensuring smooth UI hydration.
