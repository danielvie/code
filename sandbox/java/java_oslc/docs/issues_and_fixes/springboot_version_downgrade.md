# Spring Boot and Eclipse Lyo Compatibility Issue

## Issue
When initially setting up the OSLC Java server, we scaffolded the project using Spring Boot 3.2.x. When running the server, the `/catalog` endpoint would return a `404 Not Found` or fail to serialize the OSLC Java objects into JSON-LD/RDF properly.

## Root Cause
Eclipse Lyo 5.x relies heavily on the `JAX-RS 2.0` specification, which is packaged under the `javax.ws.rs.*` namespace. 

However, Spring Boot 3.0 and newer transitioned from Java EE to Jakarta EE, meaning Jersey (the JAX-RS implementation used by Spring Boot) was upgraded to Jersey 3, which exclusively uses the `jakarta.ws.rs.*` namespace. Because of this incompatibility, the native providers from Eclipse Lyo (like `OslcJsonLdProvider` and `OslcRdfXmlProvider`) could not be correctly injected or registered as message body readers/writers because Jersey 3 did not recognize their `javax.ws.rs.ext.Provider` annotations.

## Resolution
To resolve this mapping mismatch without resorting to complex reflection or bytecode manipulation:

1. **Downgraded Spring Boot:** The `pom.xml` was updated to use Spring Boot `2.7.18`, which is the latest stable 2.x release and inherently supports the `javax.ws.rs` API.
2. **Namespace Rollback:** All explicitly imported `jakarta.ws.rs.*` packages in the resource endpoints (`ServiceProviderCatalogService.java` and `ServiceProviderService.java`) were reverted back to `javax.ws.rs.*`.
3. **MIME Type Correction:** The OSLC output media type was updated from the standard `application/json` or `application/json-ld` back to the exact MIME type that Eclipse Lyo and OSLC core specs expect: `application/ld+json`.
