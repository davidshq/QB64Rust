"""Locks between runs of the tools that build in the reference clone.

qb64pe builds inside its own folder: the generated C++ goes to internal/temp, which the tools empty
before each compile and read afterwards (compilelog.txt, main0.txt), and its make writes the libqb
objects in internal/c. Two runs that compile with the same qb64pe at the same time would empty and
read each other's files, and what the old compiler answers is what gets recorded as truth. So a run
that uses qb64pe in the clone takes the clone's lock alone (lock_clone, exclusive).

A build with qb64rust keeps its C++, its qbx.o and its executable in a folder of its own and only
reads the clone, except for the libqb objects its make builds when one is missing. That case has a
lock of its own, OBJECTS_LOCK, which the driver takes around each make (QB64RUST_OBJECTS_LOCK,
crates/driver/src/build.rs): shared while the objects are up to date, alone while they are built.
So runs of qb64rust share the clone's lock (lock_clone(shared=True)) and run side by side, and a run
of qb64pe waits for all of them, and they for it (test-parallel-runs, design D1).

lock_folder is the same lock for a folder of private copies (run_legacy_tests.py --jobs with qb64pe)
and for a folder of results, where the runner builds and runs each program; those are exclusive.

The lock is on a file under this repo's target/ (git-ignored), not in the clone. It belongs to the
process: the system drops it when the process ends, however it ends. On Windows it is LockFileEx
over the whole file range, which is what Rust's File::lock and File::lock_shared take, so the CLI
tests (crates/driver/tests/support) and these tools see each other's locks.

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
# The file the driver locks around each make (OBJECTS_LOCK_ENV in crates/driver/src/build.rs).
OBJECTS_LOCK = CLONE_LOCK_DIR / "objects.lock"
OBJECTS_LOCK_ENV = "QB64RUST_OBJECTS_LOCK"
# The exit code of a run that stops because another run holds a lock: not 1 (a test failed) and not
# 2 (a usage error), so that a script can tell "start again later" from "fix the command".
BUSY_EXIT = 3
CLONE_BUSY = (
    "another run holds the reference clone (a run of qb64pe there has it alone; runs of qb64rust, "
    "the CLI tests among them, share it); start again when it has ended, or pass --wait"
)


CLONE_WAITING = "waiting for another run to leave the reference clone"


if os.name == "nt":
    import ctypes
    from ctypes import wintypes

    class _Overlapped(ctypes.Structure):  # pylint: disable=too-few-public-methods
        _fields_ = [
            ("Internal", ctypes.c_void_p),
            ("InternalHigh", ctypes.c_void_p),
            ("Offset", wintypes.DWORD),
            ("OffsetHigh", wintypes.DWORD),
            ("hEvent", wintypes.HANDLE),
        ]

    _KERNEL32 = ctypes.WinDLL("kernel32", use_last_error=True)
    _LOCK_FILE_EX = _KERNEL32.LockFileEx
    _LOCK_FILE_EX.argtypes = [
        wintypes.HANDLE,
        wintypes.DWORD,
        wintypes.DWORD,
        wintypes.DWORD,
        wintypes.DWORD,
        ctypes.POINTER(_Overlapped),
    ]
    _LOCK_FILE_EX.restype = wintypes.BOOL
    _LOCKFILE_FAIL_IMMEDIATELY = 0x1
    _LOCKFILE_EXCLUSIVE_LOCK = 0x2


def _try_lock(fh, shared: bool) -> bool:
    """One try at the lock of the open file fh: True when taken."""
    if os.name == "nt":
        import msvcrt  # pylint: disable=import-outside-toplevel

        flags = _LOCKFILE_FAIL_IMMEDIATELY | (0 if shared else _LOCKFILE_EXCLUSIVE_LOCK)
        handle = msvcrt.get_osfhandle(fh.fileno())
        # Offset 0, length 0xFFFFFFFF_FFFFFFFF: the range of Rust's File::lock on Windows.
        return bool(_LOCK_FILE_EX(handle, flags, 0, 0xFFFFFFFF, 0xFFFFFFFF, ctypes.byref(_Overlapped())))
    import fcntl  # pylint: disable=import-outside-toplevel,import-error

    try:
        fcntl.flock(fh, (fcntl.LOCK_SH if shared else fcntl.LOCK_EX) | fcntl.LOCK_NB)
    except OSError:
        return False
    return True


def lock_file(path: Path, wait: bool = False, waiting: str = "", shared: bool = False):
    """Takes the lock of the file at path for this process and returns the open file (keep it:
    closing it gives the lock back). shared: other shared holders may hold it too, an exclusive one
    may not. When another process holds it in a way that excludes this one: None, or with wait,
    prints the waiting line once and tries again every second until it can be taken."""
    path.parent.mkdir(parents=True, exist_ok=True)
    fh = open(path, "a+b")  # noqa: SIM115  # held for the run
    told = False
    while not _try_lock(fh, shared):
        if not wait:
            fh.close()
            return None
        if waiting and not told:
            print(waiting, flush=True)
            told = True
        time.sleep(1)
    return fh


def lock_folder(folder: Path, wait: bool = False, waiting: str = ""):
    """The exclusive lock of the folder's in-use.lock (lock_file's result)."""
    return lock_file(folder / "in-use.lock", wait, waiting)


def lock_clone(wait: bool = False, shared: bool = False):
    """The lock of the reference clone (lock_file's result): exclusive for a run that uses qb64pe in
    the clone, shared for one that builds with qb64rust only. A shared run also names OBJECTS_LOCK in
    the environment its compiles inherit, so that the driver takes it around each make."""
    fh = lock_file(CLONE_LOCK_DIR / "in-use.lock", wait, CLONE_WAITING, shared)
    if fh is not None and shared:
        os.environ[OBJECTS_LOCK_ENV] = str(OBJECTS_LOCK)
    return fh
