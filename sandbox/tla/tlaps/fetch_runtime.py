"""Fetch pinned x86 Cygwin DLLs and a shell into this experiment only."""
from pathlib import Path
import hashlib
import json
import re
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parent
BASE = "https://mirrors.kernel.org/sourceware/cygwin-archive/20221123/"
SETUP = (ROOT / "logs" / "cygwin-setup.ini").read_text(encoding="utf-8")
# Use archived .xz packages supported by Python's standard-library tar reader.
PACKAGES = [
    ("cygwin", "3.3.6-1", "cygwin1.dll"),
    ("libgcc1", "10.2.0-1", "cyggcc_s-1.dll"),
    ("libstdc++6", "10.2.0-1", "cygstdc++-6.dll"),
    ("cygwin", "3.3.6-1", "cygpath.exe"),
    ("cygwin", "3.3.6-1", "ps.exe"),
    ("bash", "4.4.12-3", "bash.exe"),
    ("bash", "4.4.12-3", "sh.exe"),
    ("libiconv2", "1.17-1", "cygiconv-2.dll"),
    ("libintl8", "0.21.1-2", "cygintl-8.dll"),
    ("libreadline7", "8.2-2", "cygreadline7.dll"),
    ("libncursesw10", "6.3-1.20220416", "cygncursesw-10.dll"),
    ("coreutils", "9.0-1", "mkdir.exe"),
    ("coreutils", "9.0-1", "ls.exe"),
    ("coreutils", "9.0-1", "tail.exe"),
    ("coreutils", "9.0-1", "rm.exe"),
    ("libattr1", "2.4.48-2", "cygattr-1.dll"),
    ("libgmp10", "6.2.1-1", "cyggmp-10.dll"),
]
DESTINATION = ROOT / "runtime" / "bin"
DESTINATION.mkdir(parents=True, exist_ok=True)
records = []
for name, version, dll in PACKAGES:
    stanza = re.search(r"^@ " + re.escape(name) + r"\n(.*?)(?=^@ |\Z)",
                       SETUP, re.MULTILINE | re.DOTALL).group(1)
    filename = f"{name}-{version}.tar.xz"
    candidates = [line.split() for line in stanza.splitlines()
                  if line.startswith("install: ") and line.split()[1].endswith("/" + filename)]
    if len(candidates) != 1:
        raise SystemExit(f"Expected one archived package record for {filename}")
    _, relative, size, expected = candidates[0]
    package = ROOT / "downloads" / filename
    if not package.exists():
        subprocess.run(["curl", "-4", "-fL", "--connect-timeout", "25", "--max-time", "90",
                        "--retry", "1", BASE + relative, "-o", str(package)], check=True)
    with package.open("rb") as content:
        digest = hashlib.file_digest(content, "sha512").hexdigest()
    if package.stat().st_size != int(size) or digest != expected:
        raise SystemExit(f"Archived metadata verification failed for {filename}")
    with tarfile.open(package, "r:xz") as archive:
        matches = [member for member in archive
                   if (member.isfile() or member.islnk() or member.issym())
                   and member.name in ("usr/bin/" + dll, "bin/" + dll)]
        if len(matches) != 1:
            raise SystemExit(f"Expected one runtime member for {dll}")
        with archive.extractfile(matches[0]) as content:
            (DESTINATION / dll).write_bytes(content.read())
    records.append({"package": filename, "url": BASE + relative, "sha512": digest,
                    "dll": str(DESTINATION / dll)})
    print("Verified and extracted", dll)
(ROOT / "logs" / "runtime-downloads.json").write_text(json.dumps(records, indent=2), encoding="utf-8")
