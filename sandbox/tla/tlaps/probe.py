"""Run bounded Windows-only probes with folder-local environment changes."""
from pathlib import Path
import ctypes
import json
import os
import subprocess
import sys

ROOT = Path(__file__).resolve().parent
BUNDLE = ROOT / "bundle"
if os.name != "nt":
    raise SystemExit("This experiment tests Windows executables, not WSL or Linux")
# Child processes inherit this mode: avoid missing-DLL error dialogs hanging tests.
ctypes.windll.kernel32.SetErrorMode(0x0001 | 0x0002 | 0x8000)
SCRATCH = ROOT / "scratch"
SCRATCH.mkdir(exist_ok=True)
environment = os.environ.copy()
windows = Path(environment.get("SystemRoot", r"C:\Windows"))
paths = [BUNDLE / "bin", BUNDLE / "lib" / "tlaps" / "bin"]
runtime = ROOT / "runtime" / "bin"
if runtime.exists():
    paths.append(runtime)
paths.extend([windows / "System32", windows])
environment["PATH"] = os.pathsep.join(map(str, paths))
for name in ["HOME", "USERPROFILE", "APPDATA", "LOCALAPPDATA", "TEMP", "TMP"]:
    directory = SCRATCH / name.lower()
    directory.mkdir(exist_ok=True)
    environment[name] = str(directory)
commands = [
    [BUNDLE / "bin" / "tlapm.exe", "--version"],
    [BUNDLE / "lib" / "tlaps" / "bin" / "z3.exe", "-version"],
    [BUNDLE / "lib" / "tlaps" / "bin" / "zenon", "-v"],
    [BUNDLE / "lib" / "tlaps" / "bin" / "ls4.exe", "--help"],
]
if (runtime / "sh.exe").exists():
    commands.append([runtime / "sh.exe", "--version"])
results = []
for command in commands:
    entry = {"command": list(map(str, command))}
    try:
        completed = subprocess.run(entry["command"], cwd=SCRATCH, env=environment,
                                   capture_output=True, timeout=15,
                                   creationflags=subprocess.CREATE_NO_WINDOW)
        entry.update(returncode=completed.returncode,
                     status_hex=f"0x{completed.returncode & 0xffffffff:08X}",
                     stdout=completed.stdout.decode("utf-8", errors="replace"),
                     stderr=completed.stderr.decode("utf-8", errors="replace"))
    except (OSError, subprocess.TimeoutExpired) as error:
        entry["error"] = str(error)
    results.append(entry)
    print(json.dumps(entry, ensure_ascii=True))
report = {"path": environment["PATH"], "working_directory": str(SCRATCH),
          "runtime_directory_present": runtime.exists(), "results": results}
name = "probe-with-runtime.json" if runtime.exists() else "probe-bare.json"
(ROOT / "logs" / name).write_text(json.dumps(report, indent=2), encoding="utf-8")
# Startup checks are not proof tests. Require every selected startup to succeed.
sys.exit(0 if all(entry.get("returncode") == 0 for entry in results) else 1)
