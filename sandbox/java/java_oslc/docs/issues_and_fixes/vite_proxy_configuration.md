# Vite Development Server Proxy Issues (ECONNREFUSED)

## Issue
While developing the SvelteKit / Vite client, attempting to fetch from the Spring Boot `/catalog` API via proxy resulted in random `ECONNREFUSED 127.0.0.1:8080` or `500 Internal Server Error` statuses in the browser console, even when the Java server was confirmed to be online natively.

## Root Cause
There were two contributing factors to this local networking issue:
1. **Hostname Mapping (IPv4 vs IPv6):** Node.js and Bun fetch mechanics often default `localhost` resolution to the IPv6 loopback (`::1`). Spring Boot's embedded Tomcat instance was specifically binding to the IPv4 loopback (`127.0.0.1`). 
2. **Persistent Proxy Socket Dropping:** Vite's internal node `http-proxy` retains background socket listeners. If the Java Spring backend is terminated array from the CLI and restarted, `vite` does not automatically rebuild those broken socket ties, resulting in permanent `ECONNREFUSED` exceptions internally until the Vite process itself is cycled.

## Resolution
1. **Explicit IPv4 / Target Config:** The `vite.config.ts` proxy block was modified from a generic string proxy (`'/catalog': 'http://localhost:8080'`) to a robust configuration object:
```typescript
proxy: {
  '/catalog': {
    target: 'http://localhost:8080',
    changeOrigin: true
  }
}
```
Using the object form with the `changeOrigin: true` flag assists `http-proxy` in managing cross-stack header routing properly.

2. **Continuous Integration Testing Framework:** To prevent relying solely on the brittle proxy for validation, a standalone integration test (`src/api.test.ts`) powered by Vitest was created. Running `bunx vitest run` queries the Spring Boot backend natively in isolation, guaranteeing we can accurately diagnose backend availability separately from Vite's proxy status.
