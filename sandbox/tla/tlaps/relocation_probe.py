"""Check the wrapper from a second, minimal bundle in a path containing spaces."""
from pathlib import Path
import ctypes
import json
import os
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parent
DESTINATION = ROOT / "relocation check"
if os.name != "nt":
    raise SystemExit("Windows-only experiment")
ctypes.windll.kernel32.SetErrorMode(0x8003)
if not DESTINATION.exists():
    DESTINATION.mkdir()
    shutil.copy2(ROOT / "tlapm.cmd", DESTINATION / "tlapm.cmd")
    for folder in ["bundle/bin", "bundle/lib/tlaps/bin", "runtime/bin"]:
        shutil.copytree(ROOT / folder, DESTINATION / folder)
    library = DESTINATION / "bundle" / "lib" / "tlaps"
    for source in (ROOT / "bundle" / "lib" / "tlaps").glob("*.tla"):
        shutil.copy2(source, library / source.name)
    (DESTINATION / "checks").mkdir()
    shutil.copy2(ROOT / "checks" / "ValidSMT.tla", DESTINATION / "checks" / "ValidSMT.tla")
wrapper = str(DESTINATION / "tlapm.cmd").replace("'", "''")
source = str(DESTINATION / "checks" / "ValidSMT.tla").replace("'", "''")
windows = Path(os.environ.get("SystemRoot", r"C:\Windows"))
powershell = windows / "System32" / "WindowsPowerShell" / "v1.0" / "powershell.exe"
command = f"& '{wrapper}' --nofp --threads 1 --verbose '{source}'; exit $LASTEXITCODE"
completed = subprocess.run([str(powershell), "-NoProfile", "-NonInteractive", "-Command", command],
                           cwd=DESTINATION, capture_output=True, timeout=45,
                           creationflags=subprocess.CREATE_NO_WINDOW)
text = (completed.stdout + completed.stderr).decode("utf-8", errors="replace")
(ROOT / "logs" / "relocation.log").write_text(text, encoding="utf-8")
passed = completed.returncode == 0 and any("launching process:" in line and "z3" in line
                                         for line in text.splitlines())
report = {"destination": str(DESTINATION), "returncode": completed.returncode,
          "backend_dispatched": "launching process:" in text, "passed": passed}
(ROOT / "logs" / "relocation.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
print(json.dumps(report))
if not passed:
    print(text[-8000:])
sys.exit(0 if passed else 1)
