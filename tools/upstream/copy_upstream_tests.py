"""Copies the text files of QB64pe's tests/compile_tests into tests/upstream/compile_tests (design D1 of the
OpenSpec change m2-upstream-tests).

The copy mirrors the clone's folder layout. Only text files are taken, chosen by extension (EXTENSIONS); images,
fonts, sound, DLLs and libraries stay in the clone, and .gitignore files are left out (they would change git's
behaviour inside our tree). Files are copied byte for byte. The script deletes and rewrites
tests/upstream/compile_tests and rewrites tests/upstream/SOURCE.md and LICENSE-QB64pe.txt; it never touches the
lists in tests/upstream (pass.list, deferred.list, README.md).

It refuses to run when the clone's HEAD is not the commit named in SOURCE.md, unless given --commit-ok (then
SOURCE.md names the new commit). A second run on the same commit changes no file.

Usage: python tools/upstream/copy_upstream_tests.py [--commit-ok] [<QB64pe clone>]   (default clone: ../QB64pe)
"""

import argparse
import pathlib
import re
import shutil
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parents[2]
DEST = REPO / "tests" / "upstream"
SUITE = pathlib.PurePosixPath("tests/compile_tests")
EXTENSIONS = [
    ".bas", ".bi", ".bm", ".h", ".c", ".output", ".err", ".license", ".noprompt", ".compile-from-base", ".md",
]
COMMIT_LINE = re.compile(r"^Commit: `([0-9a-f]+)`", re.MULTILINE)


def git(clone, *args):
    return subprocess.run(
        ["git", "-C", str(clone), *args], check=True, capture_output=True, text=True
    ).stdout.strip()


def wanted(path):
    return path.suffix.lower() in EXTENSIONS


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("clone", nargs="?", default=str(REPO.parent / "QB64pe"))
    ap.add_argument("--commit-ok", action="store_true", help="accept a clone HEAD other than SOURCE.md's commit")
    a = ap.parse_args()

    clone = pathlib.Path(a.clone).resolve()
    suite = clone / SUITE
    if not suite.is_dir():
        sys.exit(f"no {SUITE} in the clone given")
    head = git(clone, "rev-parse", "--short=10", "HEAD")
    date = git(clone, "log", "-1", "--format=%cs", "HEAD")

    source_md = DEST / "SOURCE.md"
    if source_md.exists():
        m = COMMIT_LINE.search(source_md.read_text(encoding="utf-8"))
        pinned = m.group(1) if m else None
        if pinned != head and not a.commit_ok:
            sys.exit(f"the clone is at {head}, SOURCE.md names {pinned}; rerun with --commit-ok to take {head}")

    out = DEST / "compile_tests"
    if out.exists():
        shutil.rmtree(out)
    counts = {}
    total_bytes = 0
    for f in sorted(p for p in suite.rglob("*") if p.is_file()):
        if not wanted(f):
            continue
        rel = f.relative_to(suite)
        target = out / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(f, target)
        ext = f.suffix.lower()
        counts[ext] = counts.get(ext, 0) + 1
        total_bytes += f.stat().st_size

    shutil.copyfile(clone / "licenses" / "license_qb64.txt", DEST / "LICENSE-QB64pe.txt")

    table = "\n".join(f"| `{ext}` | {counts.get(ext, 0)} |" for ext in EXTENSIONS)
    source_md.write_text(
        f"""# Source of `tests/upstream/`

Commit: `{head}` ({date}) of QB64pe (`github.com/QB64-Phoenix-Edition/QB64pe`), as cloned in `..\\QB64pe`.

`compile_tests/` holds the text files of the clone's `tests/compile_tests`, byte for byte, under the same
relative paths. Binary assets (images, fonts, sound, libraries) and `.gitignore` files are not copied; the
upstream tests that need them are run from the clone (`tools/legacy_tests/run_legacy_tests.py --suite compile`).

Licence: MIT, QB64pe's `licenses/license_qb64.txt`, copied as `LICENSE-QB64pe.txt` (decision of 2026-10-04 in
`CLAUDE.md`; `study/22` §4).

Made by `tools/upstream/copy_upstream_tests.py`; rerun it after updating the clone (with `--commit-ok`). Do not
edit the copied files by hand.

| Extension | Files |
|---|---|
{table}

{sum(counts.values())} files, {total_bytes:,} bytes.
""",
        encoding="utf-8",
        newline="\n",
    )
    print(f"copied {sum(counts.values())} files ({total_bytes:,} bytes) from {head}")


if __name__ == "__main__":
    main()
