"""Optional end-to-end test. Requires Chrome and network access to jsDelivr.

First export the model: uv run -m app.main --export
Then: uv run --with playwright python tests/browser_smoke.py
The test starts and stops its own local static server and isolated browser.
"""

from http.server import ThreadingHTTPServer
from pathlib import Path
import sys
from threading import Thread

from playwright.sync_api import sync_playwright


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from app.serve import WebHandler

assert (ROOT / "web" / "mnist.onnx").exists(), "Run uv run -m app.main --export first"
server = ThreadingHTTPServer(("127.0.0.1", 0), WebHandler)
Thread(target=server.serve_forever, daemon=True).start()
url = f"http://127.0.0.1:{server.server_port}"

try:
    with sync_playwright() as playwright:
        browser = playwright.chromium.launch(channel="chrome")
        page = browser.new_page(viewport={"width": 1100, "height": 1100}, reduced_motion="reduce")
        errors = []
        page.on("pageerror", lambda error: errors.append(str(error)))
        page.on("console", lambda message: print("Browser:", message.text) if message.type == "error" else None)
        page.goto(url)
        page.wait_for_function(
            "document.querySelector('#runtime-status').textContent.startsWith('Ready')",
            timeout=90000,
        )
        assert page.locator("#predict").is_disabled()
        assert page.locator("#prediction").inner_text() == "?"

        # Exercise actual pointer handlers by drawing a closed, handwritten zero.
        bounds = page.locator("#drawing").bounding_box()
        points = [(140, 45), (95, 60), (72, 110), (70, 165), (95, 220),
                  (145, 235), (190, 208), (208, 150), (198, 90), (175, 52), (140, 45)]
        for index, (x, y) in enumerate(points):
            page.mouse.move(bounds["x"] + x / 280 * bounds["width"],
                            bounds["y"] + y / 280 * bounds["height"], steps=4)
            if index == 0:
                page.mouse.down()
        page.mouse.up()
        page.wait_for_function("document.querySelector('#prediction').textContent !== '?'")
        assert page.locator("#prediction").inner_text() == "0"
        assert page.locator(".score-row.winner").count() == 1
        assert page.locator("#scores .score-value").count() == 10
        assert page.locator("#predict").is_enabled()

        # Check the full browser-only canvas preprocessing, not just pure helpers.
        prepared = page.evaluate("""async () => {
            const { prepareInput } = await import('./preprocessing.mjs');
            const input = prepareInput(document.querySelector('#drawing'));
            return { length: input.length, min: Math.min(...input), max: Math.max(...input),
                     mass: input.reduce((sum, value) => sum + value, 0) };
        }""")
        assert prepared["length"] == 784
        assert 0 <= prepared["min"] < prepared["max"] <= 1
        assert prepared["mass"] > 0
        (ROOT / "outputs").mkdir(exist_ok=True)
        page.screenshot(path=str(ROOT / "outputs" / "web-preview.png"), full_page=True)
        print("Desktop drawing, WASM inference, input tensor, and score display: passed")

        # Hold a real inference result until after Clear. It must not overwrite
        # the now-blank pad with a stale prediction when the promise resolves.
        page.evaluate("""() => {
            const originalRun = ort.InferenceSession.prototype.run;
            ort.InferenceSession.prototype.run = async function(...args) {
                const result = await originalRun.apply(this, args);
                await new Promise(resolve => { window.releasePrediction = resolve; });
                ort.InferenceSession.prototype.run = originalRun;
                return result;
            };
        }""")
        page.locator("#predict").click()
        page.wait_for_function("typeof window.releasePrediction === 'function'")
        page.locator("#clear").click()
        page.evaluate("window.releasePrediction()")
        assert page.locator("#prediction").inner_text() == "?"
        assert page.locator("#predict").is_disabled()
        assert page.evaluate("""async () => {
            const { prepareInput } = await import('./preprocessing.mjs');
            return prepareInput(document.querySelector('#drawing')) === null;
        }""")
        assert not errors, errors
        print("Clear, blank input, and stale inference protection: passed")

        # A narrow touch viewport should fit without horizontal scrolling.
        mobile = browser.new_page(viewport={"width": 390, "height": 844}, has_touch=True)
        mobile.goto(url)
        mobile.wait_for_function("document.querySelector('#runtime-status').textContent.startsWith('Ready')", timeout=90000)
        assert mobile.evaluate("document.documentElement.scrollWidth <= window.innerWidth")
        mobile.locator("#drawing").tap()
        mobile.wait_for_function("document.querySelector('#prediction').textContent !== '?'")
        mobile.locator("#clear").tap()
        assert mobile.locator("#predict").is_disabled()
        print("Mobile layout and touch input: passed")

        # Fail clearly instead of presenting guesses from an unloaded model.
        missing = browser.new_page()
        missing.route("**/mnist.onnx", lambda route: route.fulfill(status=404, body="Missing"))
        missing.goto(url)
        missing.wait_for_function("document.querySelector('#runtime-status').classList.contains('error')")
        assert "--export" in missing.locator("#runtime-status").inner_text()
        assert missing.locator("#predict").is_disabled()
        print("Missing-model error: passed")
        browser.close()
finally:
    server.shutdown()
    server.server_close()
