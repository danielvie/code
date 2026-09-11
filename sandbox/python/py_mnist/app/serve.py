"""Serve the browser demo's static files. This server never runs predictions.

Run: uv run -m app.serve
Open: http://127.0.0.1:8000
"""

import argparse
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


WEB_DIR = Path(__file__).resolve().parents[1] / "web"


class WebHandler(SimpleHTTPRequestHandler):
    # Windows registry MIME mappings can label .mjs as text/plain. Browsers
    # reject modules with that type, so do not rely on OS-specific defaults.
    extensions_map = {
        **SimpleHTTPRequestHandler.extensions_map,
        ".html": "text/html",
        ".css": "text/css",
        ".js": "text/javascript",
        ".mjs": "text/javascript",
        ".wasm": "application/wasm",
        ".onnx": "application/octet-stream",
    }

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(WEB_DIR), **kwargs)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--port", type=int, default=8000)
    args = parser.parse_args()
    if not (WEB_DIR / "mnist.onnx").exists():
        parser.error("Model missing. Run uv run -m app.main --export first.")
    with ThreadingHTTPServer(("127.0.0.1", args.port), WebHandler) as server:
        print(f"Open http://127.0.0.1:{server.server_port}. Press Ctrl+C to stop.", flush=True)
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped.")


if __name__ == "__main__":
    main()
