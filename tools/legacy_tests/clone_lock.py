"""Locks between runs of the tools that build in the reference clone.

qb64pe builds inside its own folder: the generated C++ goes to internal/temp, which the tools empty
before each compile and read afterwards (compilelog.txt, main0.txt), and its make writes the libqb
objects in internal/c. Two runs that compile with the same qb64pe at the same time would empty and
read each other's files, and what the old compiler answers is what gets recorded as truth. So a run
that uses qb64pe in the clone takes the clone's lock alone (lock_clone, exclusive).

A build with qb64rust keeps its C++, its qbx.o and its executable in a folder of its own, builds
against this repo's copy of libqb (libqb/) and reads from the clone only its toolchain and the files
DECLARE LIBRARY names, which no compile writes (design D5 of libqb-copy). The libqb objects its make
builds in the copy when one is missing have a lock of their own, OBJECTS_LOCK, which the driver takes
around each make (QB64RUST_OBJECTS_LOCK, crates/driver/src/build.rs): shared while the objects are up
to date, alone while they are built. So a run of qb64rust takes no lock of the clone and runs beside
a run of qb64pe, unless its programs are in the clone's own test folders, where it runs them: then it
shares the clone's lock (clone_lock_kind), and a run of qb64pe waits for it.

clear_temp empties internal/temp before a qb64pe compile; it is here because both tools do it under
lock_clone (design D2 of dry-cleanup).

lock_folder is the same lock for a folder of private copies (run_legacy_tests.py --jobs with qb64pe)
and for a folder of results, where the runner builds and runs each program; those are exclusive.

The lock is on a file under the main checkout's target/ (git-ignored), not in the clone. It belongs
to the process: the system drops it when the process ends, however it ends. On Windows it is
LockFileEx over the whole file range, which is what Rust's File::lock and File::lock_shared take, so
the CLI tests (crates/driver/tests/support) and these tools see each other's locks.

A git worktree of this repo (the runs at a merge into main, study/archive/34 section 3 item 3) takes the
locks of the main checkout (main_checkout), so its runs and the main checkout's see each other. Not
covered: two separate clones of this repo that use one reference clone; the reference clone is
read-only and gets no lock file. Use worktrees, not clones, for a second checkout.
"""

from __future__ import annotations

import os
import shutil
import time
from pathlib import Path


def main_checkout(repo: Path) -> Path:
    """The checkout whose target/ holds the locks: repo itself, or for a git worktree (its .git a
    file "gitdir: <main>/.git/worktrees/<name>", which names the main .git in its commondir file) the
    main checkout. The same rule as clone_lock_dir in crates/driver/tests/support/mod.rs."""
    dot = repo / ".git"
    if dot.is_file():
        text = dot.read_text(encoding="utf-8").strip()
        if text.startswith("gitdir:"):
            gitdir = (repo / text[len("gitdir:") :].strip()).resolve()
            common = gitdir / "commondir"
            if common.is_file():
                return (gitdir / common.read_text(encoding="utf-8").strip()).resolve().parent
    return repo


# One lock for every clone: a run with another --qb-root waits for nothing it shares, which costs
# time and never a wrong result.
CLONE_LOCK_DIR = main_checkout(Path(__file__).resolve().parents[2]) / "target" / "qb64pe-clone"
# The file the driver locks around each make (OBJECTS_LOCK_ENV in crates/driver/src/build.rs).
OBJECTS_LOCK = CLONE_LOCK_DIR / "objects.lock"
OBJECTS_LOCK_ENV = "QB64RUST_OBJECTS_LOCK"
# The exit code of a run that stops because another run holds a lock: not 1 (a test failed) and not
# 2 (a usage error), so that a script can tell "start again later" from "fix the command".
BUSY_EXIT = 3
CLONE_BUSY = (
    "another run holds the reference clone (a run of qb64pe there has it alone; runs of qb64rust "
    "share it to run programs in the clone's test folders); start again when it has ended, or pass "
    "--wait"
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


def clone_lock_kind(old_compiler: bool, jobs: int, programs_in_clone: bool) -> str | None:
    """How a run takes the clone's lock: "exclusive" for one that uses qb64pe in the clone (none at its
    start with --jobs above 1, which compiles in private copies and takes it when it comes back),
    "shared" for a run of qb64rust whose programs are in the clone's own folders (it runs them there),
    None for any other run of qb64rust (design D5 of libqb-copy)."""
    if old_compiler:
        return None if jobs > 1 else "exclusive"
    return "shared" if programs_in_clone else None


def name_objects_lock() -> None:
    """Names OBJECTS_LOCK in the environment the compiles of a qb64rust run inherit, so that the
    driver takes it around each make."""
    os.environ[OBJECTS_LOCK_ENV] = str(OBJECTS_LOCK)


def lock_clone(wait: bool = False, shared: bool = False):
    """The lock of the reference clone (lock_file's result): exclusive for a run that uses qb64pe in
    the clone, shared for a run of qb64rust that runs programs in the clone's folders."""
    return lock_file(CLONE_LOCK_DIR / "in-use.lock", wait, CLONE_WAITING, shared)


def clear_temp(qb_root: Path) -> None:
    """Empties the clone's internal/temp before a qb64pe compile, as the bash runners do (the test
    runner and the call-site tool, each under lock_clone)."""
    temp = qb_root / "internal" / "temp"
    if not temp.is_dir():
        return
    for p in temp.iterdir():
        # temp.bin is tracked in the clone (the Makefile's clean keeps it too); qb64pe.exe rewrites
        # it, another compiler would not, and deleting it would change the clone.
        if p.name == "temp.bin":
            continue
        if p.is_dir():
            shutil.rmtree(p, ignore_errors=True)
        else:
            try:
                p.unlink()
            except OSError:
                pass
