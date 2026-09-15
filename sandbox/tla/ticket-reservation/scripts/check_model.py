#!/usr/bin/env python3
"""Download local TLC, require the expected bug, then validate the fixed model."""

import argparse
import hashlib
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
JAR = ROOT / 'tools/tla2tools.jar'
URL = 'https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar'
# Pin the bytes used for this exercise, not just the release URL.
SHA256 = '2c903dcd6f50f12b0c0a2e14c1406be782126c7cee64f3e0b8550c2020870293'


def check_model(name, config, expected_code, expected_message, extra=()):
    command = [
        'java', '-Xmx512m', '-XX:+UseParallelGC',
        f'-Djava.io.tmpdir={ROOT / "artifacts"}',
        '-cp', str(JAR), 'tlc2.TLC', '-workers', '1', '-seed', '1', '-fp', '0',
        '-noGenerateSpecTE', '-metadir', f'artifacts/{name}-states',
        '-config', f'model/{config}', *extra, 'model/Ticket.tla',
    ]
    result = subprocess.run(command, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    (ROOT / f'artifacts/{name}.log').write_text(result.stdout)
    print(result.stdout, end='', flush=True)
    if result.returncode != expected_code or expected_message not in result.stdout:
        raise SystemExit(f'{name}: unexpected TLC result, exit {result.returncode}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--update-fixture', action='store_true', help='replace the committed graph fixture after a reviewed model change')
    args = parser.parse_args()
    JAR.parent.mkdir(exist_ok=True)
    (ROOT / 'artifacts').mkdir(exist_ok=True)
    if not JAR.exists():
        download = JAR.with_suffix('.download')
        subprocess.run(['curl', '-fL', '--retry', '2', URL, '-o', str(download)], check=True)
        if hashlib.sha256(download.read_bytes()).hexdigest() != SHA256:
            download.unlink()
            raise SystemExit('Downloaded TLC checksum mismatch; refusing to execute it.')
        download.replace(JAR)
    if hashlib.sha256(JAR.read_bytes()).hexdigest() != SHA256:
        raise SystemExit('Local TLC checksum mismatch; refusing to execute it.')

    check_model('buggy', 'Buggy.cfg', 12, 'Invariant OnlyOwnerCanBuy is violated.')
    check_model('fixed', 'Ticket.cfg', 0, 'Model checking completed. No error has been found.',
                ['-dump', 'dot,actionlabels', 'artifacts/states.dot'])
    subprocess.run([
        sys.executable, str(ROOT / 'scripts/export_graph.py'),
        str(ROOT / 'artifacts/states.dot'), str(ROOT / 'tests/fixtures/tlc-graph.tsv'),
        *([] if args.update_fixture else ['--check']),
    ], check=True, cwd=ROOT)


if __name__ == '__main__':
    main()
