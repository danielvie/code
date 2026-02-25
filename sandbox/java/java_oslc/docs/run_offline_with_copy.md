# Running the OSLC Server Offline (Flash Drive / Air-Gapped Machine)

This guide explains how to build and run the Java Spring Boot OSLC server on a machine **without internet access**, by copying pre-resolved Maven dependencies from an online machine via a flash drive.

---

## Table of Contents

1. [Background](#background)
2. [How Maven Dependencies Work](#how-maven-dependencies-work)
3. [Strategy Overview](#strategy-overview)
4. [Strategy 1: Copy the Entire `.m2/repository` (Recommended)](#strategy-1-copy-the-entire-m2repository-recommended)
5. [Strategy 2: Create a Portable Project-Scoped Repository (Smaller)](#strategy-2-create-a-portable-project-scoped-repository-smaller)
6. [Lyo Dependency Breakdown](#lyo-dependency-breakdown)
7. [Client (Svelte) Offline Setup](#client-svelte-offline-setup)
8. [Troubleshooting](#troubleshooting)

---

## Background

The `server/pom.xml` does **not** declare any custom `<repositories>` — all dependencies (including Eclipse Lyo) are resolved from **Maven Central** (`https://repo.maven.apache.org/maven2`). This means the first build requires internet access, but subsequent builds can work entirely from the local cache.

The project has **137 transitive dependencies** totaling ~951 MB in the local Maven cache.

---

## How Maven Dependencies Work

When you run `mvn clean install`, Maven:

1. Reads `pom.xml` and resolves all `<dependency>` entries (including transitive ones).
2. Checks the **local cache** at `~/.m2/repository/` first.
3. If a JAR/POM is missing locally, downloads it from Maven Central.
4. Stores everything in `~/.m2/repository/` for future builds.

The key insight: **if `~/.m2/repository/` already has every artifact, Maven never needs the network.**

---

## Strategy Overview

| Strategy | Transfer Size | Complexity | Reliability |
|----------|--------------|------------|-------------|
| **1. Copy entire `.m2/repository`** | ~951 MB | Very Low | ✅ Highest |
| **2. Portable project-scoped repo** | ~200-400 MB | Low | ✅ High |

Both strategies work. Strategy 1 is recommended for simplicity.

---

## Strategy 1: Copy the Entire `.m2/repository` (Recommended)

This is the simplest and most reliable approach. You copy the entire local Maven cache, which already contains every JAR, POM, and metadata file needed.

### On the online machine

**Step 1:** Ensure all dependencies are fully cached (including plugins):

```bash
cd server
mvn dependency:go-offline
```

This forces Maven to download everything the project needs — dependencies, plugins, and their transitive trees — into the local cache.

**Step 2:** Copy the local Maven repository to the flash drive:

```
Source:       C:\Users\<user>\.m2\repository\
Destination:  E:\m2-repo\          (flash drive)
```

On Windows:
```cmd
xcopy "%USERPROFILE%\.m2\repository" "E:\m2-repo\" /E /H /I
```

On Linux/Mac:
```bash
cp -r ~/.m2/repository/ /media/flashdrive/m2-repo/
```

> **Size:** Approximately **951 MB**. This includes cached artifacts from all Maven projects on the machine, not just this one. If space is a concern, use [Strategy 2](#strategy-2-create-a-portable-project-scoped-repository-smaller) instead.

### On the offline machine

**Step 1:** Copy the repository from the flash drive to the local Maven cache:

```
Source:       E:\m2-repo\
Destination:  C:\Users\<user>\.m2\repository\
```

On Windows:
```cmd
xcopy "E:\m2-repo" "%USERPROFILE%\.m2\repository\" /E /H /I
```

On Linux/Mac:
```bash
cp -r /media/flashdrive/m2-repo/ ~/.m2/repository/
```

**Step 2:** Build the project in offline mode:

```bash
cd server
mvn clean install -DskipTests --offline
```

**Step 3:** Run the server:

```bash
mvn spring-boot:run --offline
```

The `--offline` (`-o`) flag tells Maven to **never** attempt any network requests. It will only resolve from the local cache.

---

## Strategy 2: Create a Portable Project-Scoped Repository (Smaller)

If the full `.m2/repository` is too large (e.g., it contains cached artifacts from many other projects), you can create a minimal, self-contained repository with only this project's dependencies.

### On the online machine

**Step 1:** Generate a portable repository inside the project:

```bash
cd server
mvn dependency:go-offline -Dmaven.repo.local=./portable-repo
```

This downloads all dependencies and plugins into `server/portable-repo/` instead of `~/.m2/repository/`.

**Step 2:** Copy to the flash drive:

```
Source:       server/portable-repo/
Destination:  E:\portable-repo\    (flash drive)
```

### On the offline machine

**Step 1:** Copy the portable repo into the project directory:

```
Source:       E:\portable-repo\
Destination:  server/portable-repo\
```

**Step 2:** Build using the portable repository:

```bash
cd server
mvn clean install -DskipTests --offline -Dmaven.repo.local=./portable-repo
```

**Step 3:** Run the server:

```bash
mvn spring-boot:run --offline -Dmaven.repo.local=./portable-repo
```

> **Important:** You must pass `-Dmaven.repo.local=./portable-repo` on **every** Maven command. Alternatively, copy the contents of `portable-repo/` into `~/.m2/repository/` on the offline machine to avoid this.

> **Note:** Add `portable-repo/` to `.gitignore` — it should not be committed to version control.

---

## Lyo Dependency Breakdown

Eclipse Lyo itself is very small (~797 KB across 6 artifacts), but it pulls in significant transitive dependencies. You **cannot** copy just the Lyo JARs — the entire dependency tree must be present.

### Direct Lyo artifacts

| Artifact | Group ID | Size |
|----------|----------|------|
| `oslc4j-core` | `org.eclipse.lyo.oslc4j.core` | ~200 KB |
| `lyo-core-model` | `org.eclipse.lyo.oslc4j.core` | ~150 KB |
| `lyo-core-settings` | `org.eclipse.lyo.oslc4j.core` | ~10 KB |
| `oslc4j-jena-provider` | `org.eclipse.lyo.oslc4j.core` | ~100 KB |
| `oslc4j-json4j-provider` | `org.eclipse.lyo.oslc4j.core` | ~80 KB |
| `oslc4j-core-wink` | `org.eclipse.lyo.oslc4j.core` | ~50 KB |

### Key transitive dependencies pulled by Lyo

| Library | Purpose | Approx. JARs |
|---------|---------|---------------|
| **Apache Jena** (4.8.0) | RDF/SPARQL processing engine | ~20 JARs |
| **Apache HttpComponents** | HTTP client for OSLC requests | 3 JARs |
| **jsonld-java** | JSON-LD parsing | 1 JAR |
| **Google Protobuf** | Serialization (Jena dependency) | 1 JAR |
| **Apache Thrift** | RPC framework (Jena dependency) | 1 JAR |
| **Apache Wink** | JSON4J provider (legacy JAX-RS) | 1 JAR |
| **Classgraph** | Runtime classpath scanning | 1 JAR |

The full dependency tree has **137 artifacts**. Both strategies above handle all of these automatically.

---

## Client (Svelte) Offline Setup

The Svelte client also has dependencies that need to be available offline. The same flash drive approach applies:

### On the online machine

```bash
cd client
npm install        # or: bun install
```

This creates the `client/node_modules/` directory with all frontend dependencies.

### Copy to flash drive

```
Source:       client/node_modules/
Destination:  E:\node_modules\     (flash drive)
```

### On the offline machine

```
Source:       E:\node_modules\
Destination:  client/node_modules\
```

Then run the dev server without needing `npm install`:

```bash
cd client
npm run dev        # or: bun run dev
```

> **Alternative:** You can also copy the `package-lock.json` (or `bun.lockb`) and use `npm ci --offline` if you configure a local npm cache, but copying `node_modules/` directly is simpler for air-gapped environments.

---

## Troubleshooting

### `Could not resolve dependencies` in offline mode

**Cause:** The local cache is missing one or more artifacts.

**Solution:** On the online machine, re-run `mvn dependency:go-offline` to ensure everything is fully cached, then re-copy `~/.m2/repository/`.

### `Plugin not found` in offline mode

**Cause:** `dependency:go-offline` sometimes misses Maven plugins (e.g., `spring-boot-maven-plugin`).

**Solution:** On the online machine, run a full build first to cache the plugins:

```bash
cd server
mvn clean install -DskipTests
mvn dependency:go-offline
```

Then re-copy the `.m2/repository/`.

### `--offline` flag causes errors, but online build works

**Cause:** Maven is trying to check for SNAPSHOT updates or plugin metadata.

**Solution:** Ensure you're using `--offline` (or `-o`) consistently. If a dependency has `-SNAPSHOT` in its version, Maven will always try to check for updates unless forced offline. This project uses `0.0.1-SNAPSHOT` for the project itself, but all external dependencies are release versions, so this should not be an issue.

### Different Java version on the offline machine

**Cause:** The project requires Java 17. If the offline machine has a different version, the build will fail.

**Solution:** Verify with `java -version` and `javac -version` on the offline machine. Both must be Java 17+.

### Different OS between online and offline machines

Maven repository caches are **OS-independent** — the same `.m2/repository/` works on Windows, Linux, and macOS. The JARs and POMs are platform-neutral.
