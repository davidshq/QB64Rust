#!/usr/bin/env python3
"""The tests of the Python tools: `python -m unittest discover` in each folder of tools/ that holds a test_*.py, each in
a process of its own (the `tools` and `tools-check` steps of tools/check/steps.py).

    python tools/check/tool_tests.py [--only <folder> ... | --except <folder> ...]

`--only check` runs tools/check's tests alone and `--except check` every other folder's, as the two steps do. With
neither, every folder runs.

One process per folder: the folders have no __init__.py (unittest does not discover into them from tools/), and two
hold a test_check.py (tools/check, tools/licenses), which one process would take for one module.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
import time
from pathlib import Path

TOOLS = Path(__file__).resolve().parents[1]


def folders(root: Path = TOOLS) -> list[Path]:
    return sorted(d for d in root.iterdir() if d.is_dir() and any(d.glob("test_*.py")))


def chosen(all_folders: list[Path], only: list[str], skip: list[str]) -> list[Path]:
    """The folders to run: those named by `--only`, or all but those named by `--except`. A name that is no folder
    with tests is an error, so that a renamed folder cannot drop out of both steps unseen."""
    names = {d.name for d in all_folders}
    unknown = sorted(set(only + skip) - names)
    if unknown:
        raise SystemExit(f"tool_tests.py: no folder of tools/ with tests is named {', '.join(unknown)}")
    if only:
        return [d for d in all_folders if d.name in only]
    return [d for d in all_folders if d.name not in skip]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--only", action="append", default=[], metavar="FOLDER")
    group.add_argument("--except", dest="skip", action="append", default=[], metavar="FOLDER")
    args = parser.parse_args(argv)
    run = chosen(folders(), args.only, args.skip)
    failed = []
    for d in run:
        t0 = time.time()
        r = subprocess.run(
            [sys.executable, "-m", "unittest", "discover", "-s", str(d), "-p", "test_*.py"],
            capture_output=True,
            text=True,
            errors="replace",
        )
        last = [line for line in r.stderr.splitlines() if line.strip()][-1:] or ["?"]
        print(f"{'ok  ' if r.returncode == 0 else 'FAIL'} {d.name:<14} {time.time() - t0:5.1f}s  {last[0]}", flush=True)
        if r.returncode != 0:
            failed.append(d.name)
            sys.stdout.write(r.stderr)
    print(f"{len(run) - len(failed)} folders passed, {len(failed)} failed{': ' + ', '.join(failed) if failed else ''}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
