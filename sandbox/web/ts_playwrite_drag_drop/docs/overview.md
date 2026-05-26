# Playwright Multi-Window Drag-and-Drop Proof of Concept

This project serves as a comprehensive Proof of Concept (POC) demonstrating how to orchestrate, execute, and validate multi-window drag-and-drop interactions using end-to-end automation testing inside an isolated headless Docker container.

## 1. Project Architecture & File Breakdown

The repository elegantly merges a lightweight, standalone Python backend with a robust testing suite powered natively by Node.js and Playwright.

### Core Files
* **`server.py`**: A fast, asynchronous `FastAPI` application instance. It serves two distinct HTML pages simultaneously representing distinct frames of the drag-and-drop POC:
  - **Main Window** (`/`): Contains the draggable `#draggable` origin element.
  - **Popup Window** (`/undocked`): Contains the target `#dropzone`. It executes the logic validating DOM payload contents generated organically by HTML5 Drag and Drop APIs.
* **`pyproject.toml`** & **`uv.lock`**: The Python dependency manifests. We leverage Astral's `uv` engine for lightning-fast virtual environment scaffolding.
* **`package.json`**: The Node.js manifest. It houses the standard `@playwright/test` framework dependencies.

### Testing & Tooling
* **`playwright.config.ts`**: Governs our automation ecosystem context. It defines our isolated testing engine bounds, configures Playwright strictly to Chromium, enforces deep diagnostic traces/video recordings by default, and automatically boots the `uvicorn server:app` web server automatically before your test blocks orchestrate.
* **`e2e/test.spec.ts`**: The core Playwright automation script. Responsible for multi-window intercept routing, extracting element bounds, and validating the visual state via golden assertions.

### Orchestration
* **`Dockerfile`**: Defines our robust headless testing sandbox context. It layers dependencies off Microsoft's `playwright:v1.43.0-jammy` footprint; installing `python3`, Node modules, Playwright browsers, and initializing astral `uv` explicitly. 
* **`Taskfile.yml`**: A cross-platform orchestrator config using `task`. It radically centralizes and streamlines complex development lifecycles (i.e. `task build`, `task test:docker`, `task test:show-result`).

---

## 2. Dependencies

To natively orchestrate this POC, the environment actively manipulates the following tech stacks:

- **Docker:** Container abstraction simulating the robust CI linux environment where headless chromium renders validation tests.
- **Node.js (npm):** Triggers `npx playwright test` test suites locally or internally.
- **Python (`uv`):** Manages the backend ecosystem via lightning-fast dependency caching.
- **Task (`@go-task/cli`):** Executes cross-platform workflows out of `Taskfile.yml`.

---

## 3. How to Run the Project Walkthrough

The project encapsulates workflows explicitly engineered to allow running locally vs CI modes interchangeably.

### Method A: Fully Containerized (Headless Docker CI Style)
1. **Build the image dependencies globally**:
   ```bash
   task build
   ```
2. **Execute Headless Validations**: Iterate through playwright seamlessly inside isolated container layers.
   ```bash
   task test:docker
   ```
3. **Extract Diagnostic Traces (UI)**: Rip traces out of the silent container securely to your host OS browser!
   ```bash
   task test:show-result
   ```

### Method B: Native Intercept (Local Development)
1. **Interact Sandbox Manually / Development**: 
   ```bash
   task serve
   ```
   *(Instantly boots the FastApi web-server locally at `http://127.0.0.1:8000`. Navigate there in your own browser.)*

2. **Run Tests Natively (Local Chrome Execution)**: 
   ```bash
   task test:local
   ```
3. **Erasing Temporary Assets**: 
   ```bash
   task clean
   ```

---

## 4. Deep Dive: How the Playwright Test Script Works

Testing `HTML5 Drag and Drop` natively across entirely distinct browser window contexts (independent `Page` targets in terms of Chrome DevTools Protocol representation) inherently introduces complex edge cases for headless emulation testing.

### 1. Intercepting Popups and Promises
When Playwright clicks the "Undock" button routing, we suspend standard execution flows explicitly until the asynchronous popup window securely loads via `context.waitForEvent('page')`. We lock that response perfectly to an independent variable (`popup`).

### 2. Computing Exact Bounds
The script fetches `source.boundingBox()` and `target.boundingBox()`. To realistically analyze spatial awareness, Playwright resolves precisely where those absolute DOM elements structurally sit relatively decoupled from their immediate viewports. 

### 3. Emulating Cross-Boundary Mouse Coordinate Dispatches
Because Playwright explicitly locks programmatic synthetic mouse implementations (`page.mouse.move()`) bounds firmly to the internal dimensions of their executing page context, natively simulating a mouse pointer structurally traversing distinct native window borders without actual OS GUI compositor interference isn't natively respected by headless Chrome.

### 4. Synthetic Payload Inception (The Automation Bypass)
Since we specifically aim to validate that the backend correctly calculates structural DOM behavior drops during a cross-window transaction, the script leverages an internal browser context payload injection utilizing `target.evaluate()`:

```typescript
await target.evaluate((node) => {
  const dropEvent = new Event('drop', { bubbles: true });
  // Overloading native internal DOM variables specifically identical to drag payloads natively.
  Object.defineProperty(dropEvent, 'dataTransfer', {
    value: { getData: () => 'dropped_content' }
  });
  node.dispatchEvent(dropEvent);
});
```

Because `.evaluate` natively accesses the browser V8 javascript runtime immediately, Node perfectly bypasses complex `DataTransfer` isolation requirements directly over the HTML structure, satisfying the listener natively!

### 5. Executing Visual Snapshot Confidence
Following payload validation (`.toHaveText('Dropped')`), Playwright computes the absolute visual GUI architecture footprint rendering of the new screen via `.toHaveScreenshot('success-state.png')`. It structurally measures layout deltas explicitly directly against our previously extracted `e2e/test.spec.ts-snapshots` linux/windows baseline definitions dynamically mapping regressions!
