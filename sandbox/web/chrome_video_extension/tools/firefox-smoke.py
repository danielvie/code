"""Native Firefox smoke. Run: python tools/firefox-smoke.py

Python standard library only. Uses a temporary extension install, profile and
loopback download fixture. Never opens an existing Firefox profile or logs in.
Marionette command reference:
https://github.com/mozilla/gecko-dev/blob/master/remote/marionette/driver.sys.mjs
"""

import json
import os
from pathlib import Path
import shutil
import socket
import subprocess
import sys
import tempfile
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from zipfile import ZipFile, ZIP_DEFLATED


FIREFOX = Path(r"C:\Program Files\Mozilla Firefox\firefox.exe")
EXTENSION = Path(__file__).resolve().parents[1] / "extension"
ADDON_ID = "lesson-video-saver@local.invalid"
FIXTURE = b"firefox-smoke-fixture\n"
TIMEOUT = 20


class Marionette:
    """The built-in length-prefixed JSON protocol, not a WebDriver HTTP server."""

    def __init__(self, connection):
        self.connection = connection
        self.sequence = 0
        self.connection.settimeout(TIMEOUT + 5)
        hello = self.receive()
        if hello.get("marionetteProtocol") != 3:
            raise RuntimeError(f"Unexpected Marionette handshake: {hello}")

    def receive(self):
        header = bytearray()
        while True:
            char = self.connection.recv(1)
            if not char:
                raise RuntimeError("Firefox closed the Marionette connection")
            if char == b":":
                break
            header.extend(char)
            if len(header) > 10 or not char.isdigit():
                raise RuntimeError("Invalid Marionette frame length")
        length = int(header)
        if length > 10_000_000:
            raise RuntimeError("Marionette frame exceeds smoke-test limit")
        body = bytearray()
        while len(body) < length:
            chunk = self.connection.recv(length - len(body))
            if not chunk:
                raise RuntimeError("Truncated Marionette frame")
            body.extend(chunk)
        return json.loads(body)

    def call(self, command, parameters=None):
        self.sequence += 1
        body = json.dumps([0, self.sequence, command, parameters or {}]).encode()
        self.connection.sendall(str(len(body)).encode() + b":" + body)
        response = self.receive()
        if response[:2] != [1, self.sequence]:
            raise RuntimeError(f"Unexpected Marionette response: {response}")
        _, _, error, result = response
        if error:
            raise RuntimeError(f"{command}: {json.dumps(error)}")
        return result

    def script(self, body, *args):
        # A null sandbox executes in the real extension page, with its real APIs.
        result = self.call("WebDriver:ExecuteAsyncScript", {
            "script": """
                const done = arguments[arguments.length - 1];
                const args = Array.from(arguments).slice(0, -1);
                (async function () { BODY }).apply(null, args).then(
                    value => done({ok: true, value: value ?? null}),
                    error => done({ok: false, error: String(error.stack || error)})
                );
            """.replace("BODY", body),
            "args": list(args), "sandbox": None, "newSandbox": True,
        })["value"]
        if not result["ok"]:
            raise RuntimeError(result["error"])
        return result["value"]

    def wait(self, body, *args):
        deadline = time.monotonic() + TIMEOUT
        while time.monotonic() < deadline:
            result = self.script(body, *args)
            if result:
                return result
            time.sleep(0.1)
        raise TimeoutError(f"Firefox condition timed out: {body}")


class FixtureHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path != "/m02s14.mp4?signature=preserved":
            self.send_error(404)
            return
        self.send_response(200)
        self.send_header("Content-Type", "video/mp4")
        self.send_header("Content-Length", str(len(FIXTURE)))
        self.end_headers()
        self.wfile.write(FIXTURE)

    def log_message(self, *_):
        pass


def check(condition, message):
    if not condition:
        raise RuntimeError(message)


def main():
    check(FIREFOX.is_file(), f"Firefox not found: {FIREFOX}")
    process = connection = client = server = None
    root = Path(tempfile.mkdtemp(prefix="lesson-firefox-smoke-"))
    report = {}
    try:
        profile = root / "profile"
        downloads = root / "downloads"
        profile.mkdir()
        downloads.mkdir()
        with socket.socket() as reservation:
            reservation.bind(("127.0.0.1", 0))
            port = reservation.getsockname()[1]
        preferences = {
            "marionette.port": port,
            "browser.shell.checkDefaultBrowser": False,
            "browser.startup.page": 0,
            "browser.startup.homepage": "about:blank",
            "browser.aboutwelcome.enabled": False,
            "browser.download.folderList": 2,
            "browser.download.dir": str(downloads),
            "browser.download.useDownloadDir": True,
            "browser.download.always_ask_before_handling_new_types": False,
            "browser.download.alwaysOpenPanel": False,
            "browser.safebrowsing.downloads.enabled": False,
            "browser.helperApps.neverAsk.saveToDisk": "video/mp4",
            "datareporting.policy.dataSubmissionEnabled": False,
            "toolkit.telemetry.reportingpolicy.firstRun": False,
        }
        (profile / "user.js").write_text("\n".join(
            f"user_pref({json.dumps(key)}, {json.dumps(value)});"
            for key, value in preferences.items()
        ), encoding="utf-8")
        archive = root / "extension.xpi"
        with ZipFile(archive, "w", ZIP_DEFLATED) as package:
            for path in sorted(EXTENSION.rglob("*")):
                if path.is_file():
                    package.write(path, path.relative_to(EXTENSION).as_posix())

        server = ThreadingHTTPServer(("127.0.0.1", 0), FixtureHandler)
        threading.Thread(target=server.serve_forever, daemon=True).start()
        with (root / "firefox.log").open("wb") as log:
            process = subprocess.Popen([
                str(FIREFOX), "--no-remote", "--headless", "--marionette",
                "--remote-allow-system-access", "-profile", str(profile), "about:blank",
            ], stdin=subprocess.DEVNULL, stdout=log, stderr=log,
                # Firefox 160 requires this instead of only the command-line flag.
                env={**os.environ, "MOZ_REMOTE_ALLOW_SYSTEM_ACCESS": "1"})
        deadline = time.monotonic() + TIMEOUT
        while time.monotonic() < deadline:
            check(process.poll() is None, f"Firefox exited with {process.returncode}")
            try:
                connection = socket.create_connection(("127.0.0.1", port), timeout=1)
                break
            except OSError:
                time.sleep(0.1)
        check(connection is not None, "Firefox did not start Marionette within 20 seconds")
        client = Marionette(connection)
        session = client.call("WebDriver:NewSession", {"capabilities": {"alwaysMatch": {}}})
        report["firefoxVersion"] = session["capabilities"]["browserVersion"]
        client.call("WebDriver:SetTimeouts", {"script": TIMEOUT * 1000, "pageLoad": TIMEOUT * 1000})
        installed = client.call("Addon:Install", {"path": str(archive), "temporary": True})
        check(installed["value"] == ADDON_ID, f"Unexpected installed addon: {installed}")
        client.call("Marionette:SetContext", {"value": "chrome"})
        addon = client.script("""
            const {AddonManager} = ChromeUtils.importESModule(
                'resource://gre/modules/AddonManager.sys.mjs');
            const addon = await AddonManager.getAddonByID(arguments[0]);
            const policy = WebExtensionPolicy.getByID(arguments[0]);
            return {active: addon?.isActive, hostname: policy?.mozExtensionHostname};
        """, ADDON_ID)
        check(addon["active"] and addon.get("hostname"), f"Extension inactive: {addon}")
        report["temporaryAddonActive"] = True
        client.call("Marionette:SetContext", {"value": "content"})
        client.call("WebDriver:Navigate", {"url": f"moz-extension://{addon['hostname']}/popup.html"})
        client.wait("return document.querySelector('#scan') && document.readyState === 'complete';")
        client.script("""
            const api = globalThis.browser ?? globalThis.chrome;
            const blank = await api.tabs.create({url: 'about:blank', active: true});
            const [active] = await api.tabs.query({active: true, currentWindow: true});
            if (active?.id !== blank.id) throw new Error('The blank test tab is not active');
            document.querySelector('#scan').click();
        """)
        status = client.wait("""
            const status = document.querySelector('#status');
            return !document.querySelector('#scan').disabled &&
                status.classList.contains('error') && status.textContent;
        """)
        check(status == "Open a lesson at learn.delligattiassociates.com, then open this extension.",
              f"Unexpected popup scan error: {status}")
        report["popupUnsupportedPageError"] = status

        url = f"http://127.0.0.1:{server.server_port}/m02s14.mp4?signature=preserved"
        result = client.script("""
            return await (globalThis.browser ?? globalThis.chrome).runtime.sendMessage({
                type: 'download', video: {url: arguments[0], mime: 'video/mp4', kind: 'file'}
            });
        """, url)
        check(result and result.get("ok") is True and isinstance(result.get("id"), int),
              f"Background download failed: {result}")
        download = client.wait("""
            const [item] = await (globalThis.browser ?? globalThis.chrome).downloads.search({id: arguments[0]});
            if (item?.state === 'interrupted') throw new Error(JSON.stringify(item));
            return item?.state === 'complete' && item;
        """, result["id"])
        filename = Path(download["filename"])
        check(filename == downloads / "m02s14.mp4", f"Wrong download location or name: {filename}")
        check(filename.read_bytes() == FIXTURE, "Downloaded bytes differ from the loopback fixture")
        report["backgroundDownload"] = {"state": download["state"], "filename": filename.name, "bytes": len(FIXTURE)}
    except Exception:
        log = root / "firefox.log"
        if log.exists():
            print("Firefox log:\n" + log.read_text(encoding="utf-8", errors="replace")[-12000:], file=sys.stderr)
        raise
    finally:
        if client and process and process.poll() is None:
            try:
                client.call("Marionette:Quit", {"flags": ["eForceQuit"]})
            except (OSError, RuntimeError):
                pass
        if connection:
            connection.close()
        if process and process.poll() is None:
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                # Kill only the disposable instance and its children, never other Firefox sessions.
                if os.name == "nt":
                    subprocess.run(["taskkill", "/PID", str(process.pid), "/T", "/F"],
                                   check=True, capture_output=True, timeout=10)
                else:
                    process.kill()
                process.wait(timeout=10)
        if server:
            server.shutdown()
            server.server_close()
        for attempt in range(20):
            try:
                shutil.rmtree(root)
                break
            except OSError:
                if attempt == 19:
                    raise
                time.sleep(0.2)
    print(json.dumps(report, indent=2))
    print("Firefox smoke passed. Disposable profile and downloads removed.")


if __name__ == "__main__":
    main()
