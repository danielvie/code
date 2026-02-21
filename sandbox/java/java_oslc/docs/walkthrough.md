# OSLC Proof of Concept - Usage Walkthrough

This document outlines how to run and manually test the Phase 1 standalone implementation of the OSLC Proof of Concept ecosystem.

## Prerequisites
- **Java 17+** and **Maven** installed
- **Node.js** (v18+) and **npm** installed

## Step 1: Start the Java OSLC Server

The backend server is built with Spring Boot and leverages Eclipse Lyo to expose OSLC Service Provider Catalogs and Service Providers.

1. Open a terminal and navigate to the `server` directory:
   ```bash
   cd c:\SANDBOX\code.git\sandbox\java\java_oslc\server
   ```
2. Run the Spring Boot application using Maven:
   ```bash
   mvn spring-boot:run
   ```
3. Wait for the server to start. You should see logs indicating that Tomcat started on port **8080**.
   - The OSLC Catalog is now available at: `http://localhost:8080/catalog`

## Step 2: Start the Svelte Client

The frontend client is built with SvelteKit and Vite. It is configured to seamlessly proxy API requests to the Java server to avoid CORS issues during development.

1. Open a new, separate terminal and navigate to the `client` directory:
   ```bash
   cd c:\SANDBOX\code.git\sandbox\java\java_oslc\client
   ```
2. Start the Vite development server:
   ```bash
   npm run dev
   ```
3. The Vite server will typically start on port **5173**. Open your web browser and navigate to the local URL provided in the terminal (e.g., `http://localhost:5173`).

## Step 3: Verify the Integration in the UI

Once the Svelte dashboard loads in your browser, you can interact with the OSLC ecosystem:

1. **View Catalog:** Upon loading, the client automatically fetches the `ServiceProviderCatalog` from the server. You should see a card titled "OSLC OSLC Proof of Concept Catalog" representing the successful JSON-LD response.
2. **Query Defects:** Click the **Query Defects** button. 
   - Observe the returned JSON-LD array below. The first time you click this, it should return an empty array `[]` indicating there are no Defect resources currently stored on the server.
3. **Create Defect:** Click the **Create Defect** button.
   - This sends an OSLC Creation Factory `POST` request to the server to create a mocked Defect. 
   - The UI will automatically re-run the query upon successful creation. You should now see the JSON-LD payload of the newly minted Defect in the "Queried Defects" view, including its allocated `identifier`, `title`, and `Oslc-About` URI.
