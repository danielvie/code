"""Run the unchanged upstream Euclid example using the folder-local Windows tools."""
from pathlib import Path
import ctypes
import hashlib
import json
import os
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent
SOURCE = ROOT / "checks" / "upstream" / "Euclid.tla"
WORK = ROOT / "scratch" / "upstream-euclid"
if os.name != "nt":
    raise SystemExit("Windows-only experiment")
ctypes.windll.kernel32.SetErrorMode(0x8003)
WORK.mkdir(parents=True, exist_ok=True)
windows = Path(os.environ.get("SystemRoot", r"C:\Windows"))
environment = os.environ.copy()
environment["PATH"] = os.pathsep.join(map(str, [
    ROOT / "bundle" / "bin", ROOT / "bundle" / "lib" / "tlaps" / "bin",
    ROOT / "runtime" / "bin", windows / "System32", windows,
]))
for name in ["HOME", "USERPROFILE", "APPDATA", "LOCALAPPDATA", "TEMP", "TMP"]:
    directory = ROOT / "scratch" / name.lower()
    directory.mkdir(exist_ok=True)
    environment[name] = str(directory)
(ROOT / "runtime" / "tmp").mkdir(exist_ok=True)
command = [str(ROOT / "bundle" / "bin" / "tlapm.exe"), "--nofp", "--threads", "1",
           "--verbose", "--timing", str(SOURCE)]
started = time.monotonic()
process = subprocess.Popen(command, cwd=WORK, env=environment,
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                           creationflags=subprocess.CREATE_NO_WINDOW)
timed_out = False
try:
    stdout, stderr = process.communicate(timeout=180)
except subprocess.TimeoutExpired:
    timed_out = True
    subprocess.run([str(windows / "System32" / "taskkill.exe"), "/PID",
                    str(process.pid), "/T", "/F"], capture_output=True)
    stdout, stderr = process.communicate(timeout=10)
text = stdout.decode("utf-8", errors="replace") + stderr.decode("utf-8", errors="replace")
log = ROOT / "logs" / "Euclid.tla.log"
log.write_text(text, encoding="utf-8")
obligations = re.search(r'module "Euclid": (\d+) unique obligations', text)
launches = [line for line in text.splitlines() if line.startswith("launching process:")]
report = {
    "source": str(SOURCE), "source_sha256": hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
    "command": command, "working_directory": str(WORK),
    "returncode": process.returncode, "timed_out": timed_out,
    "elapsed_seconds": round(time.monotonic() - started, 3),
    "unique_obligations": int(obligations.group(1)) if obligations else None,
    "backend_launch_counts": {name: sum(name in line for line in launches)
                              for name in ["zenon", "z3", "ptl_to_trp"]},
    "errors": [line for line in text.splitlines() if line.startswith(("Error:", "FATAL:"))],
    "log": str(log),
}
(ROOT / "logs" / "Euclid.result.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
print(json.dumps(report, indent=2))
if process.returncode or timed_out:
    print(text[-16000:])
sys.exit(124 if timed_out else process.returncode)
