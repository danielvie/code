"""Package the shared extension for temporary Firefox installation or Mozilla signing."""
import json
from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED

root = Path(__file__).resolve().parent.parent
source = root / "extension"
manifest = json.loads((source / "manifest.json").read_text(encoding="utf-8"))
if not manifest.get("browser_specific_settings", {}).get("gecko", {}).get("id"):
    raise SystemExit("A stable Firefox extension ID is required.")
output = root / "artifacts" / "lesson-video-saver-firefox-unsigned.xpi"
output.parent.mkdir(exist_ok=True)
with ZipFile(output, "w", ZIP_DEFLATED) as archive:
    for path in sorted(source.rglob("*")):
        if path.is_file():
            archive.write(path, path.relative_to(source).as_posix())
with ZipFile(output) as archive:
    if archive.testzip() is not None:
        raise SystemExit("Package integrity check failed.")
    assert json.loads(archive.read("manifest.json")) == manifest
print(output)
print("Unsigned: load temporarily through about:debugging. Permanent installation requires Mozilla signing.")
