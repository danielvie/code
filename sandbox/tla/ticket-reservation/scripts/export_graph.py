#!/usr/bin/env python3
"""Convert this exercise's TLC DOT output to a small Rust test fixture.

This deliberately supports TLC's pinned output format, not arbitrary DOT.
Read the raw lines: a DOT reader may merge Cancel and Expire's parallel edges
because TLC emits a 'strict digraph'. We need both action labels for testing.
"""

import argparse
import json
from pathlib import Path
import re


QUOTED = r'"(?:\\.|[^"\\])*"'
NODE = re.compile(rf'^(-?\d+) \[label=({QUOTED})')
EDGE = re.compile(rf'^(-?\d+) -> (-?\d+) \[label=({QUOTED})')


def export(dot: str) -> str:
    states = {}
    edges = set()
    initial = []
    for line in dot.splitlines():
        if match := EDGE.match(line):
            source, target, label = match.groups()
            edges.add((source, json.loads(label), target))
        elif match := NODE.match(line):
            identifier, label = match.groups()
            fields = {}
            for assignment in json.loads(label).splitlines():
                name, value = assignment.removeprefix('/\\ ').split(' = ', 1)
                fields[name] = json.loads(value) if value.startswith('"') else value
            if set(fields) != {'status', 'owner', 'buyer'}:
                raise ValueError(f'unexpected state fields: {fields}')
            states[identifier] = ','.join(fields[k] for k in ('status', 'owner', 'buyer'))
            if 'style = filled' in line:
                initial.append(identifier)
        elif re.match(r'^-?\d+ ', line):
            raise ValueError(f'unrecognized graph record: {line}')

    if len(initial) != 1 or not edges:
        raise ValueError('expected one initial state and a nonempty transition graph')
    if len(set(states.values())) != len(states):
        raise ValueError('state projection lost information')
    rows = ['# Generated from TLC. Do not edit by hand.', f'init\t{states[initial[0]]}']
    rows += [f'state\t{state}' for state in sorted(states.values())]
    rows += sorted(f'edge\t{states[src]}\t{action}\t{states[dst]}' for src, action, dst in edges)
    return '\n'.join(rows) + '\n'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('dot', type=Path)
    parser.add_argument('fixture', type=Path)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    content = export(args.dot.read_text())
    if args.check:
        if not args.fixture.exists() or args.fixture.read_text() != content:
            raise SystemExit('TLC graph differs from fixture. Review the model change, then run check_model.py --update-fixture.')
        print('TLC graph matches the Rust conformance fixture.')
    else:
        args.fixture.parent.mkdir(parents=True, exist_ok=True)
        args.fixture.write_text(content)
        print(f'Wrote {args.fixture}')


if __name__ == '__main__':
    main()
