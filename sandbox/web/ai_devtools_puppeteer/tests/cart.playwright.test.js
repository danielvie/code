import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { writeFile } from "node:fs/promises";
import { chromium } from "playwright";

const port = Number(process.env.PORT || 3101);
const baseUrl = `http://127.0.0.1:${port}`;
const tracePath = "trace-playwright-performance.json";
const traceCategories = [
  "-*",
  "devtools.timeline",
  "disabled-by-default-devtools.timeline",
  "disabled-by-default-devtools.timeline.frame",
  "blink.user_timing",
  "loading",
  "toplevel",
  "v8.execute",
].join(",");

function startServer() {
  return spawn("node", ["src/server.js"], {
    env: { ...process.env, PORT: String(port) },
    stdio: ["ignore", "pipe", "pipe"],
  });
}

async function waitForServer() {
  const deadline = Date.now() + 5_000;

  while (Date.now() < deadline) {
    try {
      const response = await fetch(baseUrl);
      if (response.ok) return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
  }

  throw new Error(`Server did not start at ${baseUrl}`);
}

async function savePerformanceTrace(client, events) {
  const tracingComplete = new Promise((resolve) => {
    client.once("Tracing.tracingComplete", resolve);
  });

  await client.send("Tracing.end");
  await tracingComplete;

  await writeFile(
    tracePath,
    JSON.stringify({
      traceEvents: events,
      metadata: {
        source: "Playwright CDP Tracing",
        url: baseUrl,
      },
    }),
  );
}

const server = startServer();
let browser;
let client;
let traceStarted = false;
const traceEvents = [];

try {
  await waitForServer();

  browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  client = await page.context().newCDPSession(page);
  client.on("Tracing.dataCollected", (event) => {
    traceEvents.push(...event.value);
  });

  await client.send("Tracing.start", {
    categories: traceCategories,
    options: "sampling-frequency=10000",
    transferMode: "ReportEvents",
  });
  traceStarted = true;

  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.locator("#add-coffee").click();
  await page.locator("#add-coffee").click();
  await page.waitForFunction(
    () => document.querySelector("#quantity")?.textContent === "2",
  );

  const quantity = await page.locator("#quantity").textContent();
  const total = await page.locator("#total").textContent();

  assert.equal(quantity, "2");
  assert.equal(total, "$6", "Two $3 coffees should total $6");

  await savePerformanceTrace(client, traceEvents);
  traceStarted = false;

  console.log("Playwright cart test passed");
  console.log(`Performance trace saved to ${tracePath}`);
} finally {
  if (traceStarted && client) await savePerformanceTrace(client, traceEvents);
  if (browser) await browser.close();
  server.kill();
}
