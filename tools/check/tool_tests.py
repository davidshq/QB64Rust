#!/usr/bin/env python3
"""The tests of the Python tools: `python -m unittest discover` in each folder of tools/ that holds a test_*.py, each in
a process of its own (the `tools` step of tools/check/steps.py).

    python tools/check/tool_tests.py

One process per folder: the folders have no __init__.py (unittest does not discover into them from tools/), and two
hold a test_check.py (tools/check, tools/licenses), which one process would take for one module.
"""

from __future__ import annotations

import subprocess
import sys
import time
from pathlib import Path

TOOLS = Path(__file__).resolve().parents[1]


def folders(root: Path = TOOLS) -> list[Path]:
    return sorted(d for d in root.iterdir() if d.is_dir() and any(d.glob("test_*.py")))


def main() -> int:
    failed = []
    for d in folders():
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
    print(f"{len(folders()) - len(failed)} folders passed, {len(failed)} failed{': ' + ', '.join(failed) if failed else ''}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
