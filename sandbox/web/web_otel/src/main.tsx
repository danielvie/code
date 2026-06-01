import "./otel";
import React from "react";
import { createRoot } from "react-dom/client";
import { context, Span, trace } from "@opentelemetry/api";
import "./styles.css";

const tracer = trace.getTracer("react-otel-demo-ui");

type Todo = {
  id: number;
  title: string;
  completed: boolean;
};

type RequestPlan = {
  url: string;
  method: "GET";
  headers: Record<string, string>;
};

type TodoViewModel = {
  id: number;
  label: string;
  state: "done" | "open";
};

function wait(ms: number) {
  return new Promise((resolve) => window.setTimeout(resolve, ms));
}

async function spanStep<T>(parentSpan: Span, name: string, work: () => Promise<T>): Promise<T> {
  const parentContext = trace.setSpan(context.active(), parentSpan);

  return tracer.startActiveSpan(name, {}, parentContext, async (span) => {
    try {
      return await work();
    } catch (error) {
      span.recordException(error as Error);
      throw error;
    } finally {
      span.end();
    }
  });
}

async function loadTodo(commitState: (todo: TodoViewModel) => void) {
  return tracer.startActiveSpan("request.load_todo", async (span) => {
    try {
      span.setAttribute("demo.boundary.path", "ui -> request planner -> fetch -> api -> mapper -> react");
      span.addEvent("request.accepted", { source: "button.click" });

      await spanStep(span, "ui.validate_intent", async () => {
        span.addEvent("intent.validated");
        await wait(25);
      });

      await spanStep(span, "cache.lookup.todo", async () => {
        await wait(35);
      });

      const requestPlan = await spanStep<RequestPlan>(span, "http.prepare_request", async () => {
        await wait(20);
        return {
          url: "https://jsonplaceholder.typicode.com/todos/1",
          method: "GET",
          headers: {
            accept: "application/json",
          },
        };
      });

      const response = await spanStep<Response>(span, "boundary.browser_to_public_api", async () => {
        const apiResponse = await fetch(requestPlan.url, {
          method: requestPlan.method,
          headers: requestPlan.headers,
        });
        span.addEvent("boundary.crossed", {
          from: "browser",
          to: "jsonplaceholder api",
          status: apiResponse.status,
        });
        return apiResponse;
      });

      const todo = await spanStep<Todo>(span, "http.parse_json", async () => {
        const payload = (await response.json()) as Todo;
        await wait(15);
        return payload;
      });

      const viewModel = await spanStep<TodoViewModel>(span, "domain.map_todo_to_view_model", async () => {
        await wait(30);
        return {
          id: todo.id,
          label: todo.title,
          state: todo.completed ? "done" : "open",
        };
      });

      await spanStep(span, "render.prepare_todo_card", async () => {
        await wait(45);
      });

      span.addEvent("request.ready_for_state_commit", {
        todoId: viewModel.id,
        todoState: viewModel.state,
      });
      span.setAttribute("todo.completed", todo.completed);

      await spanStep(span, "react.commit_state", async () => {
        commitState(viewModel);
        await wait(10);
      });

      return viewModel;
    } finally {
      span.end();
    }
  });
}

function App() {
  const [todo, setTodo] = React.useState<TodoViewModel | null>(null);
  const [status, setStatus] = React.useState("Idle");

  async function handleLoad() {
    setStatus("Tracing request...");
    await loadTodo((nextTodo) => {
      setTodo(nextTodo);
      setStatus("Trace exported");
    });
  }

  return (
    <main className="app">
      <section className="panel">
        <div>
          <p className="eyebrow">OpenTelemetry React</p>
          <h1>Trace a user action across code and network boundaries.</h1>
          <p className="lede">
            Click the button, then inspect the trace in Jaeger to see timing, nested spans, and the
            boundary crossing from UI code to fetch and back to React state.
          </p>
        </div>

        <button onClick={handleLoad}>Capture trace</button>

        <div className="result" aria-live="polite">
          <span>{status}</span>
          {todo ? <strong>{todo.label}</strong> : <strong>No trace captured yet</strong>}
        </div>
      </section>
    </main>
  );
}

createRoot(document.getElementById("root")!).render(<App />);
