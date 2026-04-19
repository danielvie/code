# Playwright Multi-Window POC

This project is a Proof of Concept (POC) for cross-window drag-and-drop testing using Playwright, containerized in a headless Docker environment using Python/FastAPI backend managed with `uv`.

## Architecture
- **Backend:** A fast API application running on `uvicorn` returning the main index and a popup dropzone. 
- **Testing Engine:** Node.js Playwright.
- **Environment:** Containerized environment executing via `Taskfile` over the `mcr.microsoft.com/playwright` base image.

## Instructions

### Prerequisites
- Docker Desktop or Docker-compatible host.
- [Task](https://taskfile.dev/) installed (`npm install -g @go-task/cli` or similar) or run commands manually.
- Node.js (useful to view trace offline via Playwright CLI: `npx playwright`).

### Execution

1. Build the Docker Image:
   ```bash
   task build
   ```

2. Run Headless Playwright tests inside the container:
   ```bash
   task run-headless
   ```

3. Extract Traces & Videos:
   ```bash
   task extract-results
   ```

### Viewing the Trace Results on the Windows Host

Once you run `task extract-results`, the test traces and videos are securely brought from the Linux container onto your local host.

To play back the exact steps performed under the container run:
```bash
npx playwright show-report
```
*Note: Make sure to navigate within the trace UI to view both network logs, screenshots, trace timeline, and video attachments.*
