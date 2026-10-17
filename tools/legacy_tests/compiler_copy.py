"""The copy of the new compiler a test run uses (study/34 section 3, item 2).

A run of the new compiler takes minutes, and `cargo build --release` replaces target/release/
qb64rust.exe. A run that called that file would compile its later programs with another compiler
than its first, and its log would not say so. So a run copies the executable at its start to
target/test-bin/<first 16 hex digits of its SHA-256>/qb64rust.exe and compiles with the copy; a
build during the run changes nothing the run reads. The run's first line (header) names the hash
and the time the run started, and tools/close/close.py --check compares the hash with the current
build's.

The copy behaves as the original: the driver finds the reference clone from the folder it was built
in (a path fixed at build time). The one thing it takes from its own folder is the compiler root
for included files when no --include-root is given, and target/release holds none of them either.
Copies not used for a day are removed at the next run's start;
one still running cannot be removed on Windows and is left for a later run.
"""

from __future__ import annotations

import errno
import hashlib
import os
import shutil
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
TEST_BIN = REPO / "target" / "test-bin"
# How long a copy is kept after its last use.
KEEP_SECONDS = 24 * 3600
# The first word of the header line (header, read_header).
HEADER = "[compiler]"


def sha256_of(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def prune(folder: Path, keep: str, now: float | None = None) -> None:
    """Removes the copies in folder last used more than KEEP_SECONDS ago, except the one named keep.
    A copy that cannot be removed (in use) is left."""
    now = time.time() if now is None else now
    if not folder.is_dir():
        return
    for d in folder.iterdir():
        if d.name == keep or not d.is_dir():
            continue
        try:
            if now - d.stat().st_mtime > KEEP_SECONDS:
                shutil.rmtree(d)
        except OSError:
            pass


def run_copy(exe: Path, folder: Path = TEST_BIN) -> tuple[Path, str]:
    """The copy of exe to compile with, made if it is not there yet, and exe's SHA-256. The copy is
    written under a name of its own and renamed, so a run beside this one never runs half a file."""
    digest = sha256_of(exe)
    name = digest[:16]
    target = folder / name / exe.name
    # The folder's time is its last use (prune). It is set before the copy is looked at, so that the
    # prune of a run beside this one leaves an old copy alone once this run has taken it up; and the
    # copy is made again if that prune removed the folder first.
    for attempt in range(3):
        try:
            os.utime(target.parent)
        except FileNotFoundError:
            pass
        try:
            if not (target.is_file() and sha256_of(target) == digest):
                target.parent.mkdir(parents=True, exist_ok=True)
                part = target.with_name(f"{exe.name}.{os.getpid()}.part")
                shutil.copy2(exe, part)
                try:
                    os.replace(part, target)
                except OSError:
                    # Another run renamed its copy first and is running it: that copy is the same bytes.
                    part.unlink(missing_ok=True)
                    if not (target.is_file() and sha256_of(target) == digest):
                        raise
            os.utime(target.parent)
        except FileNotFoundError:
            if attempt == 2:
                raise
            continue
        if target.is_file():
            break
    else:
        # errno and text, so that e.strerror (which the callers print, holding no path) is the text.
        raise FileNotFoundError(errno.ENOENT, f"the copy of {exe.name} was removed while it was made")
    prune(folder, name)
    return target, digest


def header(original: str, digest: str, started: float) -> str:
    """The first line of a run's log: the compiler as named, its hash and when the run started."""
    return f"{HEADER} {original} sha256 {digest} started {started:.3f}"


def read_header(text: str) -> tuple[str, float] | None:
    """The hash and start time of a run's log (header), or None when the log has no such line."""
    for line in text.splitlines():
        if line.startswith(HEADER + " "):
            words = line.split()
            try:
                return words[words.index("sha256") + 1], float(words[words.index("started") + 1])
            except (ValueError, IndexError):
                return None
    return None
