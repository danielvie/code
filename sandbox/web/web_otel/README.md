# React OpenTelemetry Trace Demo

Small learning project for browser tracing with React, OpenTelemetry, the OpenTelemetry Collector,
and Jaeger.

## Proposed stack

- React + Vite: small UI with fast local feedback.
- OpenTelemetry JS browser SDK: creates manual spans and auto-instruments document load/fetch.
- OTLP/HTTP exporter: sends browser spans to `http://localhost:4318/v1/traces`.
- OpenTelemetry Collector: receives browser OTLP data and forwards it to Jaeger.
- Jaeger all-in-one: stores traces and shows timeline, flamegraph, and trace graph views.

This stack is intentionally local and simple. For production, replace Jaeger all-in-one with Tempo,
Jaeger production storage, Honeycomb, Datadog, New Relic, or another OTLP-compatible backend.

## Run

```bash
npm install
task observability
task run
```

Open:

- App: http://localhost:5173
- Jaeger UI: http://localhost:16686

Click **Capture trace** in the app, then search Jaeger for the `react-otel-demo` service.

## Expected span tree

One click creates a trace with these manual spans, plus an automatic `fetch` span from the
OpenTelemetry fetch instrumentation:

```text
request.load_todo
  ui.validate_intent
  cache.lookup.todo
  http.prepare_request
  boundary.browser_to_public_api
    HTTP GET
  http.parse_json
  domain.map_todo_to_view_model
  render.prepare_todo_card
  react.commit_state
```

All manual substep spans use the same explicit parent span, so Jaeger should show one
`request.load_todo` trace instead of separate one-span traces.

Important attributes/events:

- `demo.boundary.path`: high-level path crossed by the request.
- `request.accepted`: user action entered the traced workflow.
- `boundary.crossed`: browser-to-public-API crossing with HTTP status.
- `request.ready_for_state_commit`: mapped data is ready for React state.

## What to inspect in Jaeger

- Flame graph: open a trace, then use the flamegraph view to compare nested span duration.
- Sequence graph / boundaries: open the trace graph view to see the UI span, fetch span, and child
  work as connected operations. The `demo.boundary` span attribute describes the crossed boundary.
- Timeline: the default trace view shows exact start time, duration, nesting, and attributes.

## Key files

- `src/otel.ts`: OpenTelemetry provider, exporter, resource, and auto-instrumentation.
- `src/main.tsx`: manual spans around user interaction and simulated local processing.
- `otel-collector-config.yml`: browser OTLP receiver, CORS, and Jaeger exporter.
- `docker-compose.yml`: local Collector + Jaeger.
