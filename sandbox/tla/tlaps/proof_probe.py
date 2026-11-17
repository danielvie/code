"""Check valid and invalid Zenon/SMT proofs without global installation."""
from pathlib import Path
import ctypes
import json
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parent
if os.name != "nt":
    raise SystemExit("Windows-only experiment")
ctypes.windll.kernel32.SetErrorMode(0x8003)
windows = Path(os.environ.get("SystemRoot", r"C:\Windows"))
environment = os.environ.copy()
environment["PATH"] = os.pathsep.join(map(str, [
    ROOT / "bundle" / "bin", ROOT / "bundle" / "lib" / "tlaps" / "bin",
    ROOT / "runtime" / "bin", windows / "System32", windows,
]))
for name in ["HOME", "USERPROFILE", "APPDATA", "LOCALAPPDATA", "TEMP", "TMP"]:
    directory = ROOT / "scratch" / name.lower()
    directory.mkdir(parents=True, exist_ok=True)
    environment[name] = str(directory)
(ROOT / "runtime" / "tmp").mkdir(exist_ok=True)
executable = ROOT / "bundle" / "bin" / "tlapm.exe"
results = []
for filename, should_prove in [("ValidZenon.tla", True), ("ValidSMT.tla", True),
                               ("InvalidZenon.tla", False), ("InvalidSMT.tla", False)]:
    command = [str(executable), "--nofp", "--threads", "1", "--verbose",
               str(ROOT / "checks" / filename)]
    process = subprocess.Popen(command, cwd=ROOT / "scratch", env=environment,
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               creationflags=subprocess.CREATE_NO_WINDOW)
    timed_out = False
    try:
        stdout, stderr = process.communicate(timeout=45)
    except subprocess.TimeoutExpired:
        timed_out = True
        subprocess.run([str(windows / "System32" / "taskkill.exe"), "/PID",
                        str(process.pid), "/T", "/F"], capture_output=True)
        stdout, stderr = process.communicate(timeout=10)
    text = (stdout + stderr).decode("utf-8", errors="replace")
    (ROOT / "logs" / (filename + ".log")).write_text(text, encoding="utf-8")
    # This legacy CLI is silent on success. Require actual backend dispatch,
    # its successful exit status, and no rejection rather than an invented summary.
    backend = "zenon" if "Zenon" in filename else "z3"
    dispatched = any("launching process:" in line and backend in line
                     for line in text.splitlines())
    rejected = "Could not prove or check:" in text and "obligation(s) failed" in text
    passed = not timed_out and dispatched and ((process.returncode == 0 and not rejected)
                                               if should_prove else
                                               (process.returncode == 3 and rejected))
    generated_proof = ROOT / "scratch" / (Path(filename).stem + ".tlaps") / (Path(filename).stem + ".thy")
    zenon_proof_found = generated_proof.exists() and "PROOF-FOUND" in generated_proof.read_text(encoding="utf-8")
    if filename == "ValidZenon.tla":
        passed = passed and zenon_proof_found
    entry = {"file": filename, "command": command, "expected_proved": should_prove,
             "returncode": process.returncode, "timed_out": timed_out,
             "backend_dispatched": dispatched, "rejection_summary": rejected,
             "zenon_proof_found": zenon_proof_found, "passed": passed}
    results.append(entry)
    print(json.dumps(entry))
    for line in text.splitlines():
        if any(marker in line for marker in ["launching process:", "Error:", "FATAL:", "command not found"]):
            print(line[:600])
(ROOT / "logs" / "proof-smoke.json").write_text(json.dumps(results, indent=2), encoding="utf-8")
sys.exit(0 if all(entry["passed"] for entry in results) else 1)
