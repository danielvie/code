# Building Behind a Proxy (Maven Central Blocked)

This guide covers the scenario where the target machine **has internet access**, but a corporate proxy or firewall **blocks access to Maven Central** (`repo.maven.apache.org`). It explores whether you can download Eclipse Lyo from the official site and install it manually, and provides practical alternatives.

---

## Table of Contents

1. [The Short Answer](#the-short-answer)
2. [Why You Can't Just Download Lyo](#why-you-cant-just-download-lyo)
3. [Full Dependency Map](#full-dependency-map)
4. [Solution 1: Configure Maven to Use the Corporate Proxy](#solution-1-configure-maven-to-use-the-corporate-proxy)
5. [Solution 2: Point Maven at a Corporate Nexus/Artifactory Mirror](#solution-2-point-maven-at-a-corporate-nexusartifactory-mirror)
6. [Solution 3: Download JARs Manually from Maven Central (Browser)](#solution-3-download-jars-manually-from-maven-central-browser)
7. [Solution 4: Copy the `.m2/repository` from Another Machine](#solution-4-copy-the-m2repository-from-another-machine)
8. [Solution 5: Build a Fat JAR on an Unrestricted Machine](#solution-5-build-a-fat-jar-on-an-unrestricted-machine)
9. [What Does Eclipse Lyo's Official Site Offer?](#what-does-eclipse-lyos-official-site-offer)
10. [Troubleshooting Proxy Issues](#troubleshooting-proxy-issues)

---

## The Short Answer

**No** — you cannot simply download a single package from the Eclipse Lyo website and build this project. Lyo does not distribute a standalone download bundle with all its dependencies. Lyo is a set of Maven artifacts that are meant to be resolved through Maven's dependency system.

However, there **are** practical solutions. Read on.

---

## Why You Can't Just Download Lyo

Eclipse Lyo itself is only **6 small JARs** (~797 KB total). But it depends on **106 other libraries** at compile time. The full tree has **112 compile-scope artifacts**. You'd need every single one of them, each in the correct version, placed in the correct directory structure inside `~/.m2/repository/`, along with their corresponding `.pom` files.

Here's the problem visualized:

```
You download from Lyo's site:
  oslc4j-core-5.1.1.Final.jar             ✅ (~200 KB)

But it immediately needs:
  ├── lyo-core-model-5.1.1.Final.jar       ← Lyo (you could download this too)
  ├── apache-jena-libs-4.8.0               ← Apache Jena (~15 JARs)
  │   ├── jena-arq → jena-core → jena-base
  │   │   ├── jena-shaded-guava            ← Google Guava (shaded)
  │   │   ├── commons-csv, commons-io      ← Apache Commons
  │   │   └── commons-compress             ← Apache Commons
  │   ├── jsonld-java                      ← JSON-LD parser
  │   ├── titanium-json-ld                 ← Another JSON-LD lib
  │   ├── protobuf-java                    ← Google Protocol Buffers
  │   ├── libthrift                        ← Apache Thrift
  │   └── ... (12+ more)
  ├── httpclient-4.5.14                    ← Apache HttpComponents
  ├── classgraph-4.8.143                   ← Classpath scanner
  └── jakarta.xml.bind-api                 ← Jakarta XML binding
```

Downloading and placing **112 JARs + 112 POMs** manually into the correct `.m2/repository` directory tree is theoretically possible but extremely tedious and error-prone.

---

## Full Dependency Map

These are all **112 compile-scope dependencies** required to build and run the server:

### Eclipse Lyo (6 artifacts)
| Artifact | Version |
|----------|---------|
| `org.eclipse.lyo.oslc4j.core:oslc4j-core` | 5.1.1.Final |
| `org.eclipse.lyo.oslc4j.core:lyo-core-model` | 5.1.1.Final |
| `org.eclipse.lyo.oslc4j.core:lyo-core-settings` | 5.1.1.Final |
| `org.eclipse.lyo.oslc4j.core:oslc4j-jena-provider` | 5.1.1.Final |
| `org.eclipse.lyo.oslc4j.core:oslc4j-json4j-provider` | 5.1.1.Final |
| `org.eclipse.lyo.oslc4j.core:oslc4j-core-wink` | 5.1.1.Final |

### Apache Jena (15 artifacts)
| Artifact | Version |
|----------|---------|
| `org.apache.jena:apache-jena-libs` (POM) | 4.8.0 |
| `org.apache.jena:jena-arq` | 4.8.0 |
| `org.apache.jena:jena-base` | 4.8.0 |
| `org.apache.jena:jena-core` | 4.8.0 |
| `org.apache.jena:jena-dboe-base` | 4.8.0 |
| `org.apache.jena:jena-dboe-index` | 4.8.0 |
| `org.apache.jena:jena-dboe-storage` | 4.8.0 |
| `org.apache.jena:jena-dboe-transaction` | 4.8.0 |
| `org.apache.jena:jena-dboe-trans-data` | 4.8.0 |
| `org.apache.jena:jena-iri` | 4.8.0 |
| `org.apache.jena:jena-rdfconnection` | 4.8.0 |
| `org.apache.jena:jena-rdfpatch` | 4.8.0 |
| `org.apache.jena:jena-shacl` | 4.8.0 |
| `org.apache.jena:jena-shaded-guava` | 4.8.0 |
| `org.apache.jena:jena-shex` | 4.8.0 |
| `org.apache.jena:jena-tdb` | 4.8.0 |
| `org.apache.jena:jena-tdb2` | 4.8.0 |

### Spring Boot (15 artifacts)
| Artifact | Version |
|----------|---------|
| `spring-boot-starter-web` | 2.7.18 |
| `spring-boot-starter` | 2.7.18 |
| `spring-boot` | 2.7.18 |
| `spring-boot-autoconfigure` | 2.7.18 |
| `spring-boot-starter-logging` | 2.7.18 |
| `spring-boot-starter-json` | 2.7.18 |
| `spring-boot-starter-tomcat` | 2.7.18 |
| `spring-boot-starter-jersey` | 2.7.18 |
| `spring-boot-starter-validation` | 2.7.18 |
| `spring-web`, `spring-webmvc`, `spring-aop` | 5.3.31 |
| `spring-beans`, `spring-context`, `spring-expression` | 5.3.31 |
| `spring-core`, `spring-jcl` | 5.3.31 |

### Jersey / JAX-RS (14 artifacts)
| Artifact | Version |
|----------|---------|
| `jersey-server`, `jersey-client`, `jersey-common` | 2.35 |
| `jersey-container-servlet`, `jersey-container-servlet-core` | 2.35 |
| `jersey-bean-validation`, `jersey-spring5` | 2.35 |
| `jersey-hk2`, `jersey-entity-filtering` | 2.35 |
| `jersey-media-json-jackson` | 2.35 |
| `jakarta.ws.rs:jakarta.ws.rs-api` | 2.1.6 |
| HK2 (`hk2`, `hk2-api`, `hk2-core`, etc.) | 2.6.1 |

### Other (60+ artifacts)
Jackson, Logback, SLF4J, ASM, Tomcat Embed, Apache Commons, HttpComponents, Protobuf, Thrift, Gson, jsonld-java, SnakeYAML, Hibernate Validator, etc.

---

## Solution 1: Configure Maven to Use the Corporate Proxy

**Best option if the proxy just requires authentication.** If Maven Central isn't blocked but just requires proxy credentials, configure Maven to route through the proxy.

Create or edit `~/.m2/settings.xml` on the restricted machine:

```xml
<settings>
  <proxies>
    <proxy>
      <id>corporate-proxy</id>
      <active>true</active>
      <protocol>https</protocol>
      <host>proxy.yourcompany.com</host>
      <port>8080</port>
      <!-- Remove username/password if no authentication required -->
      <username>your-username</username>
      <password>your-password</password>
      <nonProxyHosts>localhost|127.0.0.1</nonProxyHosts>
    </proxy>
  </proxies>
</settings>
```

Then build normally:

```bash
cd server
mvn clean install -DskipTests
```

> **Ask your network/IT team** for the proxy host, port, and credentials. This is by far the cleanest solution if the proxy allows outbound HTTPS to `repo.maven.apache.org`.

---

## Solution 2: Point Maven at a Corporate Nexus/Artifactory Mirror

Many companies run an internal Maven repository manager (Sonatype Nexus, JFrog Artifactory) that mirrors Maven Central. If your organization has one, configure Maven to use it.

Create or edit `~/.m2/settings.xml`:

```xml
<settings>
  <mirrors>
    <mirror>
      <id>company-mirror</id>
      <name>Corporate Maven Central Mirror</name>
      <url>https://nexus.yourcompany.com/repository/maven-central/</url>
      <mirrorOf>central</mirrorOf>
    </mirror>
  </mirrors>
</settings>
```

Then build normally:

```bash
cd server
mvn clean install -DskipTests
```

> **Ask your DevOps/Infrastructure team** if a Nexus or Artifactory instance is available. This is the standard corporate solution.

---

## Solution 3: Download JARs Manually from Maven Central (Browser)

If the proxy blocks Maven's HTTPS client but you can access websites through a browser (common in corporate environments), you can download each JAR and POM manually from Maven Central's web interface.

### Step 1: Download every artifact

Maven Central has a web UI at **https://search.maven.org**. For each artifact, you need **both the JAR and the POM**.

Example for `oslc4j-core`:

```
JAR: https://repo.maven.apache.org/maven2/org/eclipse/lyo/oslc4j/core/oslc4j-core/5.1.1.Final/oslc4j-core-5.1.1.Final.jar
POM: https://repo.maven.apache.org/maven2/org/eclipse/lyo/oslc4j/core/oslc4j-core/5.1.1.Final/oslc4j-core-5.1.1.Final.pom
```

The URL pattern is:
```
https://repo.maven.apache.org/maven2/{groupId-as-path}/{artifactId}/{version}/{artifactId}-{version}.jar
https://repo.maven.apache.org/maven2/{groupId-as-path}/{artifactId}/{version}/{artifactId}-{version}.pom
```

### Step 2: Install each artifact into the local Maven cache

For each downloaded JAR, run:

```bash
mvn install:install-file \
  -Dfile=oslc4j-core-5.1.1.Final.jar \
  -DgroupId=org.eclipse.lyo.oslc4j.core \
  -DartifactId=oslc4j-core \
  -Dversion=5.1.1.Final \
  -Dpackaging=jar \
  -DpomFile=oslc4j-core-5.1.1.Final.pom
```

### Reality check

You would need to do this **112 times** (once per compile-scope dependency), plus the Maven plugins. This approach is technically valid but **extremely tedious**. Only use it as a last resort for a small number of missing artifacts.

---

## Solution 4: Copy the `.m2/repository` from Another Machine

This is the same approach documented in [`run_offline_with_copy.md`](run_offline_with_copy.md), and it's the **most practical solution** when the proxy cannot be configured.

### On an unrestricted machine (home, laptop, etc.)

```bash
cd server
mvn dependency:go-offline
```

### Transfer via flash drive

```
Copy:  C:\Users\<user>\.m2\repository\  →  flash drive  →  restricted machine's C:\Users\<user>\.m2\repository\
```

### On the restricted machine

```bash
cd server
mvn clean install -DskipTests --offline
mvn spring-boot:run --offline
```

The `--offline` flag prevents Maven from attempting any network connections.

> See [`run_offline_with_copy.md`](run_offline_with_copy.md) for the full step-by-step guide.

---

## Solution 5: Build a Fat JAR on an Unrestricted Machine

If you only need to **run** the server on the restricted machine (not develop on it), you can skip Maven entirely by building a self-contained executable JAR.

### On an unrestricted machine

```bash
cd server
mvn clean package -DskipTests
```

This produces a **fat JAR** at:
```
server/target/oslc-server-0.0.1-SNAPSHOT.jar
```

This single file (~80-100 MB) contains the compiled application **and all 112 dependencies** bundled inside it.

### Transfer to the restricted machine

Copy `oslc-server-0.0.1-SNAPSHOT.jar` via flash drive.

### Run on the restricted machine

No Maven needed — just Java:

```bash
java -jar oslc-server-0.0.1-SNAPSHOT.jar
```

The server will start on port 8080 as usual.

> **Limitation:** This only lets you run the server. If you need to modify the source code and recompile on the restricted machine, you still need the full `.m2/repository` (Solution 4).

---

## What Does Eclipse Lyo's Official Site Offer?

Eclipse Lyo's distribution channels:

| Source | URL | What's Available |
|--------|-----|------------------|
| **Maven Central** | https://search.maven.org (search `org.eclipse.lyo`) | Individual JARs and POMs — the standard distribution |
| **Eclipse Lyo Repo** | https://repo.eclipse.org/content/repositories/lyo-releases/ | Mirror of the same Maven artifacts (releases + snapshots) |
| **GitHub Source** | https://github.com/eclipse/lyo | Source code only — you'd need to `mvn install` from source, which has the same dependency problem |
| **Eclipse Downloads** | https://www.eclipse.org/lyo/ | Documentation and links — no downloadable bundles |

**Key takeaway:** Lyo does **not** provide a "download all-in-one ZIP" distribution. It is exclusively distributed as Maven artifacts, which is standard practice for Java libraries. There is no installer, no SDK download, no binary distribution bundle.

---

## Troubleshooting Proxy Issues

### How to check if Maven Central is truly blocked

```bash
# Test direct access
curl -I https://repo.maven.apache.org/maven2/

# Test through proxy
curl -I --proxy http://proxy.company.com:8080 https://repo.maven.apache.org/maven2/
```

### Maven gives `Could not transfer artifact` errors

This confirms the proxy is blocking Maven. Check the error message for details:

- **`Connection refused`** → Proxy host/port is wrong
- **`407 Proxy Authentication Required`** → Need username/password in `settings.xml`
- **`403 Forbidden`** → The proxy explicitly blocks Maven Central; use Solution 4 or 5
- **`SSL handshake failed`** → The proxy does SSL inspection; you may need to import the proxy's CA certificate into Java's truststore

### Import a corporate proxy CA certificate into Java

If the proxy performs SSL interception:

```bash
keytool -importcert -alias corporate-proxy \
  -file proxy-ca-cert.crt \
  -keystore "$JAVA_HOME/lib/security/cacerts" \
  -storepass changeit
```

---

## Recommendation Summary

| Situation | Best Solution |
|-----------|--------------|
| Proxy allows outbound HTTPS with auth | **Solution 1** — Configure `settings.xml` with proxy credentials |
| Company has Nexus/Artifactory | **Solution 2** — Point Maven at the internal mirror |
| Browser works but Maven is blocked | **Solution 4** — Copy `.m2/repository` via flash drive |
| Only need to run the server, not develop | **Solution 5** — Build a fat JAR and copy it |
| Nothing else works | **Solution 3** — Download 112 JARs manually (last resort) |
