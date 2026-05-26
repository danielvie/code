## AI Agent Prompt: Playwright Multi-Window POC (Local Docker Headless)

**Role:** Senior QA Automation & DevOps Engineer. **Objective:** Create a standalone POC for cross-window drag-and-drop testing using Playwright. The entire execution must be containerized to simulate a headless CI environment locally on Windows 11.

### 1. Project Environment & Specs

* **Host OS:** Windows 11.

* **Containerization:** Use a `Dockerfile` based on `mcr.microsoft.com/playwright`.

* **Dependency Management:** Python backend via `uv`.

* **Automation:** Orchestrate everything via `Taskfile.yml`.

* **Reporting:** Must generate interactive **Traces** and **Videos** to prove headless execution works.

### 2. The Testable Application

Build a simple web server (FastAPI or Flask) that serves:

* **Main Page (`/`):** A draggable element and an "Undock" button that opens a popup.

* **Popup Page (`/undocked`):** A drop zone that changes state (e.g., text changes from "Empty" to "Dropped") upon a successful drop event.

### 3. The Playwright Test Logic

The script (`test.spec.ts`) must:

1. Launch the browser in **headless mode**.

2. Open the main page and click the undock button.

3. Handle the second window using `context.waitForEvent('page')`.

4. Calculate the `boundingBox()` for elements in both windows.

5. Execute a manual mouse sequence (`move` -> `down` -> `move` -> `up`) to move the item from Window A to Window B.

6. Perform a **Visual Comparison** (screenshot) of the final "Success" state.

### 4. Docker & Taskfile Integration

* **Dockerfile:** Create a multi-stage Dockerfile that installs dependencies, copies the source, and prepares the Playwright environment.

* **Taskfile.yml Requirements:**

  * `task build`: Builds the Docker image.

  * `task run-headless`: Executes the tests inside the Docker container in headless mode.

  * `task extract-results`: Copies the `playwright-report/` (including traces/videos) from the container back to the host Windows machine for inspection.

### 5. Deliverables

* Full code for the Web App and Playwright test suite.

* `playwright.config.ts` configured for `trace: 'on'` and `video: 'on'`.

* The `Dockerfile` and `Taskfile.yml`.

* Instructions on how to view the Trace result on the Windows host after the headless Docker run completes.