#!/usr/bin/env python3
"""Keeps target/ a build folder, not a dump. Run by hand.

    python tools/clean/clean.py            # do it, print what was removed and what is left
    python tools/clean/clean.py --dry-run  # print only

What it does, in order:

1. target/test-bin/<hash>/: the runner's copies of the compiler, one per build. Keeps the copy of the current
   target/release/qb64rust(.exe) (its hash) and the newest one; removes the rest.
2. The runner's result folders (target/legacy-tests*, full-*, t2-*, t-results*, fs-results*) not written to for
   KEEP_DAYS days: removed.
3. Files directly under target/ (logs, lists, probe programs, scripts: there were 1,595 on 2026-10-31) are moved to
   target/scratch/root-sweep-<date>/, never deleted on the spot. cargo's own files (.rustc_info.json, CACHEDIR.TAG,
   dotfiles) stay. The hooks refuse new ones (tools/hooks/guard_tool.py).
4. target/scratch/<entry> older than KEEP_DAYS days: removed (target/scratch/hooks stays).
5. The build caches (target/build-cache, target/build-cache-nofold) are pruned to CACHE_LIMIT_GB by the runner's
   own prune_build_cache (least recently used first). The runner's default limit, 20 GB, let one reach 154 GB.
6. target/pass-cache/, the pass cache keyed on the executable: removed (superseded by target/pass-cache-2;
   OpenSpec change testing-reset). target/pass-cache-2/ and target/check/ stay.
7. target/qb64pe-copies/: reported, not touched (a qb64pe run uses all eight at --jobs 8).

Why: study\\41 §1; the user, 2026-10-31: "You used up tons of disk space".
"""

from __future__ import annotations

import argparse
import hashlib
import os
import shutil
import sys
import time
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TARGET = ROOT / "target"
KEEP_DAYS = 7
CACHE_LIMIT_GB = 4.0
RESULT_PREFIXES = ("legacy-tests", "full-", "t2-", "t-results", "fs-results")
SUPERSEDED = ("pass-cache",)
ROOT_KEEP = {"CACHEDIR.TAG", ".rustc_info.json", ".cargo-lock"}
EXE = "qb64rust.exe" if os.name == "nt" else "qb64rust"

sys.path.insert(0, str(ROOT / "tools" / "legacy_tests"))


def human(n: int) -> str:
    for unit in ("B", "KB", "MB", "GB"):
        if n < 1024:
            return f"{n:.0f} {unit}"
        n /= 1024
    return f"{n:.1f} TB"


def tree_size(p: Path) -> int:
    total = 0
    if p.is_file():
        return p.stat().st_size
    for f in p.rglob("*"):
        try:
            if f.is_file():
                total += f.stat().st_size
        except OSError:
            pass
    return total


def newest_mtime(p: Path) -> float:
    """When the folder's files were last written (a folder's own time changes when a child is listed or made)."""
    newest = 0.0
    for f in p.rglob("*"):
        try:
            if f.is_file():
                newest = max(newest, f.stat().st_mtime)
        except OSError:
            pass
    return newest or p.stat().st_mtime


def remove(p: Path, dry: bool) -> int:
    size = tree_size(p)
    if not dry:
        if p.is_dir():
            shutil.rmtree(p, ignore_errors=True)
        else:
            try:
                p.unlink()
            except OSError:
                return 0
    return size


def current_hash(target: Path) -> str | None:
    exe = target / "release" / EXE
    if not exe.is_file():
        return None
    h = hashlib.sha256()
    with exe.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def clean_test_bin(target: Path, dry: bool, out: list[str]) -> int:
    folder = target / "test-bin"
    if not folder.is_dir():
        return 0
    copies = [d for d in folder.iterdir() if d.is_dir()]
    if len(copies) <= 1:
        return 0
    keep: set[Path] = set()
    cur = current_hash(target)
    for d in copies:
        if cur and cur.startswith(d.name):
            keep.add(d)
    keep.add(max(copies, key=newest_mtime))
    freed = 0
    for d in copies:
        if d not in keep:
            freed += remove(d, dry)
    out.append(f"test-bin: {len(copies) - len(keep)} old compiler copies removed, {len(keep)} kept")
    return freed


def clean_results(target: Path, dry: bool, out: list[str], now: float) -> int:
    freed = n = 0
    for d in target.iterdir():
        if (
            d.is_dir()
            and d.name.startswith(RESULT_PREFIXES)
            and now - newest_mtime(d) > KEEP_DAYS * 86400
        ):
            freed += remove(d, dry)
            n += 1
    out.append(f"result folders older than {KEEP_DAYS} days: {n} removed")
    return freed


def sweep_root(target: Path, dry: bool, out: list[str]) -> int:
    files = [
        f
        for f in target.iterdir()
        if f.is_file() and f.name not in ROOT_KEEP and not f.name.startswith(".")
    ]
    if not files:
        return 0
    dest = target / "scratch" / f"root-sweep-{date.today().isoformat()}"
    moved = 0
    if not dry:
        dest.mkdir(parents=True, exist_ok=True)
    for f in files:
        if not dry:
            try:
                shutil.move(str(f), str(dest / f.name))
            except OSError:
                continue
        moved += 1
    out.append(
        f"files at the top of target/: {moved} moved to {dest.relative_to(target).as_posix()}/"
    )
    return 0


def clean_scratch(target: Path, dry: bool, out: list[str], now: float) -> int:
    folder = target / "scratch"
    if not folder.is_dir():
        return 0
    freed = n = 0
    for e in folder.iterdir():
        if e.name == "hooks":
            continue
        if now - newest_mtime(e) > KEEP_DAYS * 86400:
            freed += remove(e, dry)
            n += 1
    out.append(f"scratch entries older than {KEEP_DAYS} days: {n} removed")
    return freed


def prune_caches(target: Path, dry: bool, out: list[str]) -> int:
    try:
        from run_legacy_tests import prune_build_cache
    except ImportError:
        out.append("build caches: the runner could not be imported; not pruned")
        return 0
    freed = 0
    limit = int(CACHE_LIMIT_GB * 1024**3)
    for name in ("build-cache", "build-cache-nofold"):
        folder = target / name
        if not folder.is_dir():
            continue
        if dry:
            out.append(f"{name}: {human(tree_size(folder))} now, limit {CACHE_LIMIT_GB:g} GB")
            continue
        removed, removed_bytes, kept = prune_build_cache(folder, limit)
        freed += removed_bytes
        out.append(
            f"{name}: {removed} entries removed, {human(kept)} kept (limit {CACHE_LIMIT_GB:g} GB)"
        )
    return freed


def remove_superseded(target: Path, dry: bool, out: list[str]) -> int:
    """The caches no tool reads any more: target/pass-cache, keyed on the executable (its successor,
    target/pass-cache-2, keys on the build's inputs: design D1 of the OpenSpec change testing-reset). The pass
    caches in use and target/check/ (tools/check's results) stay."""
    freed = 0
    for name in SUPERSEDED:
        folder = target / name
        if folder.is_dir():
            freed += remove(folder, dry)
            out.append(f"{name}: superseded, removed")
    return freed


def report_copies(target: Path, out: list[str]) -> None:
    folder = target / "qb64pe-copies"
    if folder.is_dir():
        copies = [d for d in folder.iterdir() if d.is_dir()]
        if copies:
            out.append(
                f"qb64pe-copies: {len(copies)} copies, about {human(tree_size(copies[0]))} each; kept"
            )


def run(target: Path, dry: bool, now: float | None = None) -> tuple[int, list[str]]:
    now = time.time() if now is None else now
    out: list[str] = []
    if not target.is_dir():
        return 0, ["no target/ folder"]
    freed = 0
    freed += clean_test_bin(target, dry, out)
    freed += clean_results(target, dry, out, now)
    freed += sweep_root(target, dry, out)
    freed += clean_scratch(target, dry, out, now)
    freed += prune_caches(target, dry, out)
    freed += remove_superseded(target, dry, out)
    report_copies(target, out)
    out.append(("would free " if dry else "freed ") + human(freed))
    return freed, out


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("--dry-run", action="store_true", help="print what would be done")
    ap.add_argument("--target", type=Path, default=TARGET, help=argparse.SUPPRESS)
    a = ap.parse_args(argv)
    _, lines = run(a.target, a.dry_run)
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
