#!/usr/bin/env python3
"""Lists the files a step in the suites VM copies out to the host (tools/winvm/step.ps1; OpenSpec change vm-suites,
design D8). Run in the guest, with the standard library only:

    python tools/winvm/outputs.py <tree> <output> ...     prints the files, relative to <tree>, one per line

An output is a file relative to the tree, as a step's `outputs` in tools/check/steps.py names it. A runner's results
file (`*.json`) brings the files that belong to the tests that did not pass, from the folder of each one's suite
beside it (`<results>/<suite>/`): those whose names are the test's key (`<group>-<name>`, the runner's own naming)
or start with it and a `-`, and every file in such a folder.
Those are what the failures' logs, outputs and work folders are. A missing output is left out, and so is every
executable, which is never copied out.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


def failed_keys(results: Path) -> dict[Path, set[str]]:
    """The keys of the tests that did not pass, by the folder that holds their files: the runner writes a suite's
    files in <results>/<suite>/, beside results-<suite>.json (run_legacy_tests.py, `main`)."""
    try:
        data = json.loads(results.read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return {}
    out: dict[Path, set[str]] = {}
    for r in data.get("results", []):
        if r.get("status") != "PASS" and r.get("test") and r.get("suite"):
            out.setdefault(results.parent / r["suite"], set()).add(r["test"].replace("/", "-"))
    return out


def belongs(name: str, keys: set[str]) -> bool:
    return name in keys or any(name.startswith(k + "-") for k in keys)


def listed(tree: Path, outputs: list[str]) -> list[str]:
    found: set[Path] = set()
    for rel in outputs:
        path = tree / rel
        if not path.is_file():
            continue
        found.add(path)
        for folder, keys in (failed_keys(path) if path.suffix == ".json" else {}).items():
            if not folder.is_dir():
                continue
            for item in folder.iterdir():
                if not belongs(item.name, keys):
                    continue
                if item.is_dir():
                    found.update(p for p in item.rglob("*") if p.is_file())
                elif item.is_file():
                    found.add(item)
    return sorted(p.relative_to(tree).as_posix() for p in found if p.suffix.lower() != ".exe")


def main(argv: list[str]) -> int:
    if not argv:
        print(__doc__, file=sys.stderr)
        return 2
    tree = Path(argv[0])
    for rel in listed(tree, argv[1:]):
        print(rel)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
