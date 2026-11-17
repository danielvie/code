"""Extract the verified legacy TLAPS payload without running its installer."""
from pathlib import Path, PurePosixPath
import gzip
import hashlib
import json
import shutil
import tarfile

ROOT = Path(__file__).resolve().parent
PACKAGE = ROOT / "downloads" / "tlaps-1.4.5-i686-cygwin-inst.exe"
EXPECTED_SHA256 = "d413a2cdce7f9aa1cc190ec0f5bc4734d0cffb752f1085fb449876eb837cebe8"
# Offset observed with 7-Zip 23.01. The digest binds it to this exact artifact.
GZIP_OFFSET = 689166
DESTINATION = ROOT / "bundle"

with PACKAGE.open("rb") as source:
    digest = hashlib.file_digest(source, "sha256").hexdigest()
if digest != EXPECTED_SHA256:
    raise SystemExit(f"Unexpected package digest: {digest}")
if DESTINATION.exists():
    raise SystemExit("bundle already exists; refusing to overwrite extracted files")
DESTINATION.mkdir()
extracted = 0
skipped = []
with PACKAGE.open("rb") as source:
    source.seek(GZIP_OFFSET)
    with gzip.GzipFile(fileobj=source) as compressed:
        with tarfile.open(fileobj=compressed, mode="r|") as archive:
            for member in archive:
                name = PurePosixPath(member.name)
                if name.is_absolute() or ".." in name.parts or ":" in member.name or "\\" in member.name:
                    raise SystemExit(f"Unsafe archive path: {member.name}")
                destination = DESTINATION.joinpath(*name.parts)
                if not destination.resolve().is_relative_to(DESTINATION.resolve()):
                    raise SystemExit(f"Archive path escapes destination: {member.name}")
                if member.isdir():
                    destination.mkdir(parents=True, exist_ok=True)
                elif member.isfile():
                    destination.parent.mkdir(parents=True, exist_ok=True)
                    with archive.extractfile(member) as content, destination.open("xb") as output:
                        shutil.copyfileobj(content, output)
                    extracted += 1
                else:
                    # No privilege elevation or implicit Unix symlink emulation.
                    skipped.append({"name": member.name, "target": member.linkname,
                                    "type": member.type.decode("ascii", errors="replace")})
report = {"package_sha256": digest, "gzip_offset": GZIP_OFFSET,
          "destination": str(DESTINATION), "regular_files_extracted": extracted,
          "skipped_links_or_special_files": skipped}
(ROOT / "logs" / "extraction.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
print(json.dumps({"files": extracted, "skipped_links": len(skipped),
                  "destination": str(DESTINATION)}, indent=2))
