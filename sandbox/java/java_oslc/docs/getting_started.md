# Getting Started

This guide will walk you through setting up, running, and exploring the **OSLC Proof of Concept** project from scratch. By the end, you will have both the Java backend server and the Svelte frontend client running locally and communicating via OSLC standards.

---

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Project Structure](#project-structure)
4. [Installation](#installation)
   - [Clone the Repository](#clone-the-repository)
   - [Server Setup (Java/Maven)](#server-setup-javamaven)
   - [Client Setup (Svelte/Vite)](#client-setup-sveltevite)
5. [Running the Application](#running-the-application)
   - [Option A: Using Taskfile (Recommended)](#option-a-using-taskfile-recommended)
   - [Option B: Manual Startup](#option-b-manual-startup)
6. [Verifying the Setup](#verifying-the-setup)
7. [Using the Application](#using-the-application)
   - [View the Catalog](#view-the-catalog)
   - [Create a Defect](#create-a-defect)
   - [Query Defects](#query-defects)
   - [Edit a Defect](#edit-a-defect)
   - [Delete Defects](#delete-defects)
   - [Inspect Raw JSON-LD Data](#inspect-raw-json-ld-data)
8. [Running Tests](#running-tests)
9. [API Reference (Quick)](#api-reference-quick)
10. [Common Issues & Troubleshooting](#common-issues--troubleshooting)
11. [Next Steps](#next-steps)

---

## Overview

This project is a full-stack **Open Services for Lifecycle Collaboration (OSLC)** proof of concept. It consists of:

- **Server** — A Java Spring Boot application (using Eclipse Lyo) that acts as an OSLC Service Provider, exposing a `Defect` resource with full CRUD operations via RESTful endpoints in RDF/JSON-LD format.
- **Client** — A Svelte 5 single-page application (using Vite and Tailwind CSS) that acts as an OSLC Consumer, fetching catalog information and managing defects through the server's OSLC API.

The two components communicate using **JSON-LD** (`application/ld+json`) payloads that conform to the OSLC Change Management vocabulary.

---

## Prerequisites

Ensure the following tools are installed on your system before proceeding:

| Tool | Version | Purpose |
|------|---------|---------|
| **Java JDK** | 17+ | Compiling and running the Spring Boot server |
| **Apache Maven** | 3.8+ | Building the Java backend and managing dependencies |
| **Node.js** | 18+ | Running the Svelte/Vite frontend dev server |
| **npm** or **bun** | Latest | Installing client-side dependencies (the Taskfile uses `bun`, but `npm` works too) |
| **Task** *(optional)* | 3.x | [Taskfile](https://taskfile.dev/) runner for simplified commands |

### Verify installations

```bash
java -version       # Should show 17 or higher
mvn -version        # Should show 3.8+
node -version       # Should show v18+
npm -version        # or: bun -version
```

---

## Project Structure

```
java_oslc/
├── Taskfile.yml                  # Root task runner (start server, client, or both)
├── docs/                         # Project documentation
│   ├── intro.md                  # Project overview and goals
│   ├── architecture.md           # Detailed architecture documentation
│   ├── walkthrough.md            # Usage walkthrough
│   ├── getting_started.md        # This file
│   └── issues_and_fixes/        # Known issues and their solutions
│       ├── springboot_version_downgrade.md
│       └── vite_proxy_configuration.md
│
├── server/                       # Java Spring Boot OSLC Server
│   ├── pom.xml                   # Maven project configuration
│   └── src/main/java/com/example/oslc/
│       ├── Application.java              # Spring Boot entry point
│       ├── JerseyConfig.java             # JAX-RS/Jersey configuration (Lyo providers)
│       ├── resources/
│       │   ├── Defect.java               # OSLC Defect resource model
│       │   └── package-info.java         # OSLC namespace definitions
│       └── services/
│           ├── ServiceProviderCatalogService.java  # /catalog endpoint
│           ├── ServiceProviderService.java         # /provider/{id}/defects CRUD
│           └── package-info.java                   # OSLC namespace definitions
│
└── client/                       # Svelte 5 OSLC Consumer Client
    ├── package.json              # Node/Bun dependencies
    ├── vite.config.ts            # Vite config with proxy to backend
    ├── svelte.config.js          # Svelte compiler configuration
    ├── tsconfig.json             # TypeScript configuration
    ├── Taskfile.yml              # Client-specific task runner
    └── src/
        ├── main.ts               # Application entry point
        ├── App.svelte            # Main application component
        ├── DefectCard.svelte     # Individual defect card component
        ├── app.css               # Tailwind CSS theme & global styles
        ├── api.test.ts           # Integration tests (direct backend)
        └── proxy.test.ts         # Integration tests (via Vite proxy)
```

---

## Installation

### Clone the Repository

```bash
git clone <repository-url>
cd java_oslc
```

### Server Setup (Java/Maven)

Navigate to the `server/` directory and let Maven download dependencies:

```bash
cd server
mvn clean install -DskipTests
cd ..
```

This will download **Spring Boot 2.7.18**, **Eclipse Lyo 5.1.1**, **Jersey (JAX-RS)**, and all transitive dependencies.

### Client Setup (Svelte/Vite)

Navigate to the `client/` directory and install Node/Bun dependencies:

```bash
cd client

# Using npm:
npm install

# Or using bun (faster):
bun install

cd ..
```

This will install **Svelte 5**, **Vite 7**, **Tailwind CSS 4**, **Vitest**, and all other frontend dependencies.

---

## Running the Application

### Option A: Using Taskfile (Recommended)

If you have [Task](https://taskfile.dev/) installed, you can start both the server and client with a single command from the project root:

```bash
# Start both server and client concurrently
task dev

# Or start them individually:
task server    # (alias: task s) — starts the Spring Boot backend on port 8080
task client    # (alias: task c) — starts the Vite dev server on port 3000
```

To stop all processes:

```bash
task kill      # (alias: task k) — kills java, bun, and node processes
```

### Option B: Manual Startup

You will need **two separate terminals**.

**Terminal 1 — Start the Server:**

```bash
cd server
mvn spring-boot:run
```

Wait until you see log output indicating Tomcat has started on port **8080**:

```
Tomcat started on port(s): 8080 (http)
Started Application in X.XXX seconds
```

**Terminal 2 — Start the Client:**

```bash
cd client

# Using npm:
npm run dev

# Using bun:
bun run dev
```

The Vite dev server will start on port **3000** (configured in `vite.config.ts`).

---

## Verifying the Setup

### 1. Verify the Server

Open a browser or use `curl` to test the catalog endpoint directly:

```bash
curl -H "Accept: application/ld+json" http://127.0.0.1:8080/catalog
```

You should receive a JSON-LD response containing an `oslc:ServiceProviderCatalog` resource.

### 2. Verify the Client

Open your browser and navigate to:

```
http://127.0.0.1:3000
```

You should see the **OSLC Proof of Concept** dashboard with the catalog information loaded automatically (the title "OSLC OSLC Proof of Concept Catalog" should appear in the control panel).

### 3. Verify the Proxy

The Vite dev server proxies `/catalog` and `/provider` requests to the Spring Boot backend on port 8080. This proxy is configured in `client/vite.config.ts` and is what allows the frontend to call OSLC endpoints without CORS issues.

```bash
curl -H "Accept: application/ld+json" http://127.0.0.1:3000/catalog
```

This should return the same response as calling port 8080 directly.

---

## Using the Application

Once both the server and client are running, open `http://127.0.0.1:3000` in your browser.

### View the Catalog

The application automatically fetches the OSLC Service Provider Catalog on page load. You will see a **Control Panel** displaying:
- The catalog title (e.g., "OSLC OSLC Proof of Concept Catalog")
- The catalog description

### Create a Defect

1. In the Control Panel, fill in the **Defect Title** and **Description** fields (both are optional — defaults will be used if left blank).
2. Click the **Create Defect** button.
3. The app sends a `POST` request to `/provider/1/defects` with a JSON-LD payload.
4. The defect list refreshes automatically after creation.

### Query Defects

Click the **Query Defects** button to fetch all defects from the server. Defects are displayed as interactive cards below the control panel.

### Edit a Defect

1. Hover over a defect card to reveal the **Edit** and **Delete** action buttons.
2. Click **Edit** to switch the card into edit mode.
3. Modify the title and/or description.
4. Click **Save** to send a `PUT` request, or **Cancel** to discard changes.

### Delete Defects

- **Delete a single defect:** Hover over a defect card and click the **Delete** button.
- **Delete all defects:** Click the red **Delete All** button in the control panel to clear all defects from the server's in-memory store.

### Inspect Raw JSON-LD Data

After querying defects, a **Results Pipeline** section appears. Click the **Show Raw Data** toggle to view the raw JSON-LD structure returned by the OSLC server. This is useful for debugging and understanding the OSLC data format.

---

## Running Tests

The client includes integration tests powered by **Vitest** that verify connectivity to the backend.

> **Important:** The Spring Boot server must be running before executing tests, as they make real HTTP requests.

```bash
# Using the Taskfile:
task test-client    # (alias: task tc)

# Or manually:
cd client
bunx vitest run     # or: npx vitest run
```

### Test Files

| File | Description |
|------|-------------|
| `src/api.test.ts` | Tests direct connectivity to the Spring Boot server on port 8080 (fetches catalog, creates a defect) |
| `src/proxy.test.ts` | Tests connectivity through the Vite proxy on port 3000 (requires both server and client running) |

---

## API Reference (Quick)

All endpoints are served by the Spring Boot backend on port **8080** (proxied through Vite on port **3000** during development).

| Method | Endpoint | Description | Content Types |
|--------|----------|-------------|---------------|
| `GET` | `/catalog` | Fetch the OSLC Service Provider Catalog | `application/ld+json`, `application/rdf+xml`, `application/xml` |
| `GET` | `/provider/{id}` | Fetch a specific Service Provider | `application/ld+json`, `application/rdf+xml`, `application/json` |
| `GET` | `/provider/{id}/defects` | Query all Defect resources | `application/ld+json`, `application/rdf+xml`, `application/json` |
| `POST` | `/provider/{id}/defects` | Create a new Defect | Request: `application/json`; Response: `application/ld+json` |
| `PUT` | `/provider/{id}/defects/{defectId}` | Update an existing Defect | Request: `application/json`; Response: `application/ld+json` |
| `DELETE` | `/provider/{id}/defects/{defectId}` | Delete a specific Defect | `application/json` |
| `DELETE` | `/provider/{id}/defects` | Delete all Defects | `application/json` |

### Example: Create a Defect via `curl`

```bash
curl -X POST http://127.0.0.1:8080/provider/1/defects \
  -H "Content-Type: application/json" \
  -H "Accept: application/ld+json" \
  -d '{
    "prefixes": {
      "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
      "dcterms": "http://purl.org/dc/terms/",
      "oslc": "http://open-services.net/ns/core#",
      "oslc_cm": "http://open-services.net/ns/cm#"
    },
    "dcterms:title": "My First Defect",
    "dcterms:description": "Created from the command line."
  }'
```

---

## Common Issues & Troubleshooting

### Server returns 404 or fails to serialize OSLC objects

**Cause:** Spring Boot 3.x uses `jakarta.ws.rs.*` namespaces, but Eclipse Lyo 5.x requires `javax.ws.rs.*`.

**Solution:** This project uses Spring Boot **2.7.18** specifically for compatibility. Do not upgrade to Spring Boot 3.x without addressing the namespace migration. See [`docs/issues_and_fixes/springboot_version_downgrade.md`](issues_and_fixes/springboot_version_downgrade.md) for full details.

### Client shows "Error fetching catalog" or `ECONNREFUSED`

**Cause:** The Java server is not running, or there is an IPv4/IPv6 mismatch.

**Solution:**
1. Ensure the Spring Boot server is running on port 8080 before starting the client.
2. The Vite config uses `127.0.0.1` explicitly (not `localhost`) to avoid IPv6 resolution issues.
3. If the issue persists after restarting the server, restart the Vite dev server as well — Vite's proxy may hold stale socket connections. See [`docs/issues_and_fixes/vite_proxy_configuration.md`](issues_and_fixes/vite_proxy_configuration.md) for details.

### Port conflicts

- **Server (8080):** If port 8080 is in use, stop the conflicting process or configure a different port in `server/src/main/resources/application.properties` (you may need to create this file with `server.port=<port>`), then update the proxy target in `client/vite.config.ts`.
- **Client (3000):** Vite is configured with `strictPort: false`, so it will auto-increment to the next available port if 3000 is taken. Check the terminal output for the actual URL.

### Maven build fails

Ensure you are using **Java 17** (not 8, 11, or 21+). Verify with:

```bash
java -version
javac -version
```

### `bun` not found

The root `Taskfile.yml` uses `bun` to run the client. If you don't have bun installed, either:
- Install bun: `npm install -g bun`
- Or run the client manually with `npm run dev` instead of using `task client`

---

## Next Steps

After verifying the basic setup, explore these resources to deepen your understanding:

- 📖 **[Project Introduction](intro.md)** — Goals, roadmap, and high-level vision
- 🏗️ **[Architecture](architecture.md)** — Detailed backend and frontend architecture, JSON-LD payloads, and design decisions
- 🚶 **[Walkthrough](walkthrough.md)** — Step-by-step manual testing guide
- 🐛 **[Known Issues](issues_and_fixes/)** — Documented issues and their resolutions

### Key concepts to explore:

- **Eclipse Lyo** annotations (`@OslcService`, `@OslcQueryCapability`, `@OslcCreationFactory`) in the service layer
- **OSLC Resource Shapes** and namespace definitions in `package-info.java` files
- **JSON-LD content negotiation** between the Svelte client and the Java server
- **Vite proxy configuration** for seamless cross-origin development
