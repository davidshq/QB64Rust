"""Locks between runs of the tools that build in the reference clone.

qb64pe builds inside its own folder: the generated C++ goes to internal/temp, which the tools empty
before each compile and read afterwards (compilelog.txt, main0.txt). Two runs that compile with the
same qb64pe at the same time would empty and read each other's files, and what the old compiler
answers is what gets recorded as truth. A run with qb64rust keeps its C++ and its executable outside
the clone, but its make runs in the clone, reads internal/temp and builds the libqb objects the
clone lacks. So every run takes a lock first: lock_clone for the reference clone, whichever
compiler; lock_folder for a folder of private copies (run_legacy_tests.py --jobs with qb64pe) and
for a folder of results, where the runner builds and runs each program.

The lock is on a file under this repo's target/ (git-ignored), not in the clone. It belongs to the
process: the system drops it when the process ends, however it ends.

Not covered: runs started from two checkouts (or git worktrees) of this repo that use one clone.
Each checkout has its own target/, so they do not see each other's lock; the clone is a read-only
reference and gets no lock file. Use one checkout per clone.
"""

from __future__ import annotations

import os
import time
from pathlib import Path

# One lock for every clone: a run with another --qb-root waits for nothing it shares, which costs
# time and never a wrong result.
CLONE_LOCK_DIR = Path(__file__).resolve().parents[2] / "target" / "qb64pe-clone"
# The exit code of a run that stops because another run holds a lock: not 1 (a test failed) and not
# 2 (a usage error), so that a script can tell "start again later" from "fix the command".
BUSY_EXIT = 3
CLONE_BUSY = (
    "another run is building in the reference clone (run_legacy_tests.py with either compiler, "
    "verification/run.sh or callsite.py); start again when it has ended, or pass --wait"
)


CLONE_WAITING = "waiting for another run to leave the reference clone"


def lock_folder(folder: Path, wait: bool = False, waiting: str = ""):
    """Takes the folder's lock file for this process and returns the open file (keep it: closing it
    gives the lock back). When another process holds the lock: None, or with wait, prints the
    waiting line once and tries again every second until the lock is free."""
    folder.mkdir(parents=True, exist_ok=True)
    fh = open(folder / "in-use.lock", "a+b")  # noqa: SIM115  # held for the run
    told = False
    while True:
        try:
            if os.name == "nt":
                import msvcrt  # pylint: disable=import-outside-toplevel

                fh.seek(0)
                msvcrt.locking(fh.fileno(), msvcrt.LK_NBLCK, 1)
            else:
                import fcntl  # pylint: disable=import-outside-toplevel,import-error

                fcntl.flock(fh, fcntl.LOCK_EX | fcntl.LOCK_NB)
            return fh
        except OSError:
            if not wait:
                fh.close()
                return None
            if waiting and not told:
                print(waiting, flush=True)
                told = True
            time.sleep(1)


def lock_clone(wait: bool = False):
    """The lock of the reference clone's internal/temp (lock_folder's result)."""
    return lock_folder(CLONE_LOCK_DIR, wait, CLONE_WAITING)
