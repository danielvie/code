#!/usr/bin/env python3
"""Rebuild on source changes; run a copy so Windows can link while the app is open."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
BINARY = "gpui_diagram_poc"


def fingerprint(root=ROOT):
    """Only build inputs; saving .local/diagram.json never triggers a rebuild."""
    paths = [root / name for name in ("Cargo.toml", "Cargo.lock", "build.rs", "rust-toolchain.toml", "rust-toolchain")]
    for folder in ("src", ".cargo"):
        base = root / folder
        if base.exists():
            paths.extend(p for p in base.rglob("*") if p.is_file())
    result = []
    for path in sorted(set(paths)):
        try:
            stat = path.stat()
            result.append((str(path.relative_to(root)), stat.st_mtime_ns, stat.st_size))
        except FileNotFoundError:
            pass
    return tuple(result)


def stable_fingerprint():
    previous = fingerprint()
    while True:
        time.sleep(0.4)
        current = fingerprint()
        if current == previous:
            return current
        previous = current


def stop(process):
    if process is None:
        return
    if process.poll() is None:
        if os.name == "nt":
            # Only our child process tree, never other Cargo or diagram processes.
            subprocess.run(["taskkill", "/PID", str(process.pid), "/T", "/F"],
                           stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
        else:
            import signal
            os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            if os.name != "nt":
                os.killpg(process.pid, signal.SIGKILL)
            else:
                process.kill()
            process.wait()


def spawn(command, **kwargs):
    options = {"creationflags": subprocess.CREATE_NEW_PROCESS_GROUP} if os.name == "nt" else {"start_new_session": True}
    return subprocess.Popen(command, cwd=ROOT, **options, **kwargs)


def build():
    print("\n[dev] Building...", flush=True)
    process = spawn(["cargo", "build", "--bin", BINARY, "--message-format=json-render-diagnostics"],
                    stdout=subprocess.PIPE, text=True, encoding="utf-8", errors="replace")
    executable = None
    try:
        for line in process.stdout:
            try:
                message = json.loads(line)
            except json.JSONDecodeError:
                print(line, end="", flush=True)
                continue
            if message.get("reason") == "compiler-message":
                print(message["message"].get("rendered", ""), end="", flush=True)
            elif message.get("reason") == "compiler-artifact" and message.get("executable") and message["target"]["name"] == BINARY:
                executable = Path(message["executable"])
        code = process.wait()
    finally:
        stop(process)
        process.stdout.close()
    return executable if code == 0 else None


def run():
    # Keep staging on the same filesystem; clean up only this watcher's directory.
    staging_root = ROOT / "target" / "dev-runs"
    staging_root.mkdir(parents=True, exist_ok=True)
    app = None
    print("[dev] Watching src/, Cargo.toml, Cargo.lock, .cargo/, and Rust toolchain files.", flush=True)
    print("[dev] Autosave: " + os.environ.get("GPUI_DIAGRAM_STATE", str(ROOT / ".local" / "diagram.json")), flush=True)
    print("[dev] Close any separately launched diagram first. Ctrl+C stops this watcher and its app.", flush=True)
    with tempfile.TemporaryDirectory(prefix="session-", dir=staging_root) as directory:
        try:
            seen = None
            pending = True
            while True:
                current = fingerprint()
                if seen is not None and current != seen:
                    pending = True
                if pending:
                    before = stable_fingerprint()
                    executable = build()
                    after = fingerprint()
                    # Do not launch a potentially stale build if sources changed during compilation.
                    # Cargo may update Cargo.lock itself; one extra build then settles it.
                    if after != before:
                        print("[dev] Build inputs changed during compilation; rebuilding.", flush=True)
                        continue
                    seen = after
                    pending = False
                    if executable is None:
                        print("[dev] Build failed. Existing app stays open; fix the code and save again.", flush=True)
                    else:
                        # Copy first, before stopping the old app. Failed staging must not close it.
                        candidate = Path(directory) / (str(time.time_ns()) + executable.suffix)
                        try:
                            shutil.copy2(executable, candidate)
                        except OSError as error:
                            print(f"[dev] Cannot stage build; existing app stays open: {error}", flush=True)
                            continue
                        if app is not None:
                            stop(app)
                        app = None
                        # Save files are independent of the staged executable's location.
                        app = spawn([str(candidate)])
                        print("[dev] Started updated app; loading diagram and viewport from autosave.", flush=True)
                        # Remove obsolete copies only after the previous process released them.
                        for old in Path(directory).iterdir():
                            if old != candidate:
                                try:
                                    old.unlink()
                                except OSError:
                                    pass
                if app is not None and app.poll() is not None:
                    print(f"[dev] App exited with code {app.returncode}. Watching for the next code change.", flush=True)
                    app = None
                time.sleep(0.25)
        except KeyboardInterrupt:
            print("\n[dev] Stopping.", flush=True)
        finally:
            stop(app)


if __name__ == "__main__":
    try:
        run()
    except (OSError, ValueError) as error:
        print(f"[dev] {error}", file=sys.stderr)
        sys.exit(1)
