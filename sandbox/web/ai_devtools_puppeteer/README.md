# DevTools + Puppeteer learning lab

A tiny website for learning browser automation, Chrome DevTools MCP, Puppeteer, and Playwright.

## Setup

```sh
task install
```

This installs Node dependencies and Playwright's Chromium browser.

## Run the site

```sh
task run
```

Open <http://localhost:3000>.

## Run tests

Run the Puppeteer testcase:

```sh
task test:puppeteer
```

Run the Playwright testcase and generate a Chrome performance trace:

```sh
task test:playwright
```

The Playwright trace is saved to:

```txt
trace-playwright-performance.json
```

Open it with:

```txt
Chrome DevTools → Performance → Load profile
```

or:

```txt
chrome://tracing
```

## DevTools MCP setup for Zed

Add this to your Zed `settings.json`:

```json
{
  "context_servers": {
    "chrome-devtools": {
      "enabled": true,
      "remote": false,
      "command": "npx",
      "args": ["-y", "chrome-devtools-mcp@latest"]
    }
  }
}
```

If you already have `context_servers`, add only the `chrome-devtools` entry inside it.

After editing settings:

1. Save `settings.json`.
2. Reload Zed: `Command Palette → Developer: Reload Window`.
3. Start the demo site with `task run`.
4. Ask Zed Agent to use `chrome-devtools`.

## Check that Chrome DevTools MCP works

Test the package from a terminal:

```sh
npx -y chrome-devtools-mcp@latest --help
```

Start the MCP server manually:

```sh
npx -y chrome-devtools-mcp@latest
```

Expected behavior: it may look like it is hanging. That is normal because MCP servers wait for stdio messages from the MCP client. Press `Ctrl+C` to stop it.

If you see a profile lock error like this:

```txt
The browser is already running for ... chrome-devtools-mcp ... chrome-profile
```

Close the Chrome window started by MCP, or on Windows run:

```powershell
taskkill /IM chrome.exe /F
```

Then reload Zed.

## Example DevTools MCP prompts

With the site running:

```sh
task run
```

Ask your MCP-capable agent:

```txt
Use chrome-devtools MCP to open http://localhost:3000 and inspect the page title.
```

```txt
Use chrome-devtools MCP to click Add coffee twice, then report #quantity and #total.
```

```txt
Use chrome-devtools MCP to start a performance trace, click Add coffee, stop the trace, and summarize INP.
```

```txt
Use chrome-devtools MCP to inspect console messages and network requests for http://localhost:3000.
```

```txt
Use chrome-devtools MCP to take a DOM snapshot and identify the button, quantity, total, and status elements.
```

## Puppeteer vs Playwright commands

Puppeteer:

```sh
task test:puppeteer
```

Playwright with Chrome performance trace output:

```sh
task test:playwright
```

Default test task currently runs Puppeteer:

```sh
task test
```

## Using Brave with Puppeteer

In `tests/cart.test.js`, pass Brave's executable path:

```js
browser = await puppeteer.launch({
  headless: false,
  slowMo: 250,
  executablePath:
    "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
});
```

Verify the path on Windows:

```powershell
Test-Path "C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe"
```

## Intentional bug history

The original app bug was:

```js
const total = price * (quantity + 1);
```

The correct version is:

```js
const total = price * quantity;
```

The tests assert that two `$3` coffees produce quantity `2` and total `$6`.

## DevTools MCP investigation idea

Use [`chrome-devtools-mcp`](https://github.com/ChromeDevTools/chrome-devtools-mcp) from your MCP-capable agent/editor to:

1. Open `http://localhost:3000`.
2. Click **Add coffee** twice.
3. Inspect `#quantity` and `#total` in the DOM.
4. Set a breakpoint in `renderCart()` in `public/app.js`.
5. Step through the calculation and compare `quantity` with `total`.

The Puppeteer testcase in `tests/cart.test.js` and the Playwright testcase in `tests/cart.playwright.test.js` automate the same reproduction path.
