#!/usr/bin/env python3
"""Runs tests/wasm/slice.list for the wasm target (OpenSpec change wasm-console; tests/wasm/README.md).

The list names programs of two suites, as verification/<name> or corpus/<group>/<name>. The runner takes one suite
per run, so this splits the list into one --list file per suite, in the results folder, and runs
run_legacy_tests.py --platform wasm once per suite. The Emscripten SDK comes from QB64RUST_EMSDK_ROOT, as for the
runner.

Usage: python tools/legacy_tests/wasm_slice.py [--qb64 <qb64rust>] [--jobs N] [--list <file>] [-- <runner options>]
Exit status: 0 when both runs pass, else the first failing run's.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
SUITES = ("verification", "corpus")


def split(path: Path) -> dict[str, list[str]]:
    """The names of each suite in the list, in its order. A line of another suite raises ValueError, naming it."""
    names: dict[str, list[str]] = {suite: [] for suite in SUITES}
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        suite, slash, name = line.partition("/")
        if not slash or suite not in names or not name:
            raise ValueError(f"{path}: not verification/<name> or corpus/<group>/<name>: {line}")
        names[suite].append(name)
    return names


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--qb64", type=Path, default=REPO / "target" / "release" / "qb64rust.exe")
    ap.add_argument("--jobs", type=int, default=4)
    ap.add_argument("--list", type=Path, default=REPO / "tests" / "wasm" / "slice.list")
    ap.add_argument("--results", type=Path, default=REPO / "target" / "legacy-tests")
    ap.add_argument("rest", nargs="*", help="more options for run_legacy_tests.py, after --")
    args = ap.parse_args()
    try:
        names = split(args.list)
    except ValueError as e:
        print(e, file=sys.stderr)
        return 2
    args.results.mkdir(parents=True, exist_ok=True)
    status = 0
    for suite in SUITES:
        if not names[suite]:
            continue
        part = args.results / f"wasm-slice-{suite}.list"
        part.write_text("\n".join(names[suite]) + "\n", encoding="utf-8")
        cmd = [
            sys.executable,
            str(HERE / "run_legacy_tests.py"),
            "--suite",
            suite,
            "--platform",
            "wasm",
            "--qb64",
            str(args.qb64),
            "--jobs",
            str(args.jobs),
            "--list",
            str(part),
            "--results",
            str(args.results),
            *args.rest,
        ]
        rc = subprocess.run(cmd, check=False).returncode
        status = status or rc
    return status


if __name__ == "__main__":
    sys.exit(main())
