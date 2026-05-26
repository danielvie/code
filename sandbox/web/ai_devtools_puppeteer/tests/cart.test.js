import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import puppeteer from "puppeteer";

const port = Number(process.env.PORT || 3100);
const baseUrl = `http://127.0.0.1:${port}`;

function startServer() {
  const server = spawn("node", ["src/server.js"], {
    env: { ...process.env, PORT: String(port) },
    stdio: ["ignore", "pipe", "pipe"],
  });

  return server;
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

const server = startServer();
let browser;

try {
  await waitForServer();

  // browser = await puppeteer.launch({ headless: "new" });
  browser = await puppeteer.launch({
    headless: false,
    slowMo: 100,
    args: ["--remote-debugging-port=9222"],
    executablePath:
      "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
  });

  const page = await browser.newPage();

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await page.click("#add-coffee");
  await page.click("#add-coffee");
  await page.waitForFunction(
    () => document.querySelector("#quantity")?.textContent === "2",
  );

  const quantity = await page.$eval("#quantity", (node) => node.textContent);
  const total = await page.$eval("#total", (node) => node.textContent);

  assert.equal(quantity, "2");
  assert.equal(total, "$6", "Two $3 coffees should total $6");

  console.log("Cart test passed");
} finally {
  if (browser) await browser.close();
  server.kill();
}
