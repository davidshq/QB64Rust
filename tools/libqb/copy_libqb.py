"""Writes libqb/, the plain copy of what QB64pe's Makefile reads, from the reference clone (design D1 of the
OpenSpec change libqb-copy), and checks that the copy still equals it.

The copy holds QB64pe's Makefile, internal/c (every tracked file; the toolchain internal/c/c_compiler is tracked
by no git and stays in the clone), licenses/ (read by -f:GenerateLicenseFile) and COPYING.txt, under the clone's
relative paths, so that the Makefile runs unchanged with `make -C libqb`. Each file is the commit's blob byte for
byte (read with `git cat-file`, never the clone's working tree, so line ends are the committed ones whatever the
clone's git settings). libqb/SOURCE.md names the commit.

Writing deletes the copied paths of libqb/ first (the libqb objects built there go with them and are built
again) and writes them and SOURCE.md. --check writes nothing: it lists each file that differs from the commit's
blob, is missing, or is extra, and exits with status 1 if there is any. The files builds write in the copy
(BUILT: libqb objects and archives, make's .tmp files) are not extra.

The commit is --commit, else the one SOURCE.md names, else the clone's HEAD.

The copy's own edits (design D5 of the OpenSpec change const-rulings) are the files tools/libqb/edits/*.edit, each
applied to the commit's file it names after copying, in the order of their names; --check holds the copy to the
edited files. An edit file is comment lines starting with `#`, then

    file: <path under libqb/>
    --- old
    <lines that occur exactly once in the file>
    --- new
    <the lines that replace them>
    --- end

Its lines are taken with the target file's line ends, whatever the edit file's own.

Usage: python tools/libqb/copy_libqb.py [--check] [--clone <QB64pe clone>] [--commit <commit>]
       (default clone: ../QB64pe next to this repo)
"""

import argparse
import pathlib
import re
import shutil
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parents[2]
DEST = REPO / "libqb"
# What the Makefile reads (design, Context): itself, internal/c, licenses/ with GENERATE_LICENSE=y; COPYING.txt is
# QB64pe's licence.
PATHS = ["Makefile", "internal/c", "licenses", "COPYING.txt"]
# Written by builds in the copy, never part of it (the same suffixes as `written_by_builds` in
# crates/driver/src/build.rs).
BUILT = (".o", ".a", ".tmp")
COMMIT_LINE = re.compile(r"^Commit: `([0-9a-f]+)`", re.MULTILINE)
EDITS = REPO / "tools" / "libqb" / "edits"


class EditError(Exception):
    pass


def load_edits(edits_dir):
    """[(edit file name, target path, old lines, new lines)] of edits_dir/*.edit, in name order; lines without
    their line ends."""
    edits = []
    for f in sorted(pathlib.Path(edits_dir).glob("*.edit")) if pathlib.Path(edits_dir).is_dir() else []:
        lines = f.read_bytes().replace(b"\r\n", b"\n").split(b"\n")
        target, old, new, part = None, [], [], None
        for line in lines:
            if part is None and (line.startswith(b"#") or not line.strip()):
                continue
            if part is None and line.startswith(b"file: "):
                target = line[len(b"file: ") :].decode("utf-8").strip()
            elif line == b"--- old":
                part = old
            elif line == b"--- new":
                part = new
            elif line == b"--- end":
                part = "end"
                break
            elif isinstance(part, list):
                part.append(line)
            else:
                raise EditError(f"{f.name}: unexpected line {line!r}")
        if target is None or part != "end" or not old:
            raise EditError(f"{f.name}: needs `file:`, `--- old` with lines, `--- new` and `--- end`")
        edits.append((f.name, target, old, new))
    return edits


def apply_edits(files, edits):
    """files with each edit applied (a new dict); EditError when a target is missing or its old lines do not occur
    exactly once."""
    out = dict(files)
    for name, target, old, new in edits:
        if target not in out:
            raise EditError(f"{name}: {target} is not in the copy")
        data = out[target]
        eol = b"\r\n" if b"\r\n" in data else b"\n"
        old_b = eol.join(old) + eol
        new_b = eol.join(new) + eol if new else b""
        count = data.count(old_b)
        if count != 1:
            raise EditError(f"{name}: its old lines occur {count} times in {target}, not once")
        out[target] = data.replace(old_b, new_b)
    return out


def git(clone, *args, stdin=None):
    return subprocess.run(
        ["git", "-C", str(clone), *args], check=True, capture_output=True, input=stdin
    ).stdout


def pinned_commit(dest):
    source_md = dest / "SOURCE.md"
    if not source_md.is_file():
        return None
    m = COMMIT_LINE.search(source_md.read_text(encoding="utf-8"))
    return m.group(1) if m else None


def commit_files(clone, commit):
    """{relative path: blob bytes} of the commit's files under PATHS."""
    listing = git(clone, "ls-tree", "-r", "-z", commit, "--", *PATHS)
    entries = []
    for item in listing.split(b"\0"):
        if not item:
            continue
        meta, path = item.split(b"\t", 1)
        _mode, kind, sha = meta.split(b" ")
        if kind != b"blob":
            continue  # a submodule: none under PATHS at the reference commit
        entries.append((path.decode("utf-8"), sha.decode()))
    out = git(
        clone, "cat-file", "--batch", stdin=b"".join(f"{sha}\n".encode() for _, sha in entries)
    )
    files = {}
    pos = 0
    for path, sha in entries:
        end = out.index(b"\n", pos)
        header = out[pos:end].split(b" ")
        size = int(header[2])
        files[path] = out[end + 1 : end + 1 + size]
        pos = end + 1 + size + 1
    return files


def copied_files(dest):
    """The relative paths of the files of dest under PATHS, without those builds write."""
    found = []
    for top in PATHS:
        p = dest / top
        if p.is_file():
            found.append(top)
        elif p.is_dir():
            for f in p.rglob("*"):
                if f.is_file() and not f.name.endswith(BUILT):
                    found.append(f.relative_to(dest).as_posix())
    return sorted(found)


def check(dest, files):
    """The problems of the copy against the commit's files, one line each."""
    problems = []
    for path, data in sorted(files.items()):
        f = dest / path
        if not f.is_file():
            problems.append(f"missing: {path}")
        elif f.read_bytes() != data:
            problems.append(f"differs: {path}")
    for path in copied_files(dest):
        if path not in files:
            problems.append(f"extra: {path}")
    return problems


def write(dest, files, commit, date, edits=()):
    for top in PATHS:
        p = dest / top
        if p.is_dir():
            shutil.rmtree(p)
        elif p.is_file():
            p.unlink()
    for path, data in files.items():
        f = dest / path
        f.parent.mkdir(parents=True, exist_ok=True)
        f.write_bytes(data)
    total = sum(len(d) for d in files.values())
    if edits:
        listed = "\n".join(f"- `tools/libqb/edits/{name}`: `{target}`" for name, target, _, _ in edits)
        edited = f"""
Edits of this project, applied by the tool after copying (each file's header says why; `QB64PE-DIVERGENCES.md` has the
row, `UPSTREAM.md` the fix as offered to QB64pe):

{listed}
"""
    else:
        edited = ""
    (dest / "SOURCE.md").write_text(
        f"""# Source of `libqb/`

Commit: `{commit}` ({date}) of QB64pe (`github.com/QB64-Phoenix-Edition/QB64pe`), the reference clone's commit
(`DECISIONS.md` 2026-10-09).

`libqb/` holds QB64pe's `Makefile`, `internal/c/` (without the toolchain `internal/c/c_compiler`, which no git
tracks; the build takes it from the clone), `licenses/` and `COPYING.txt`: {len(files)} files, {total} bytes,
each the commit's file byte for byte under its relative path, but for the edits below. The new compiler builds
against it with `make -C libqb` (design of the OpenSpec change `libqb-copy`); the libqb objects it builds here are
git-ignored.
{edited}
Licence: MIT for QB64pe (`COPYING.txt`, `licenses/license_qb64.txt`); the parts under `internal/c/parts` keep
their own licences, listed in `licenses/`.

Made by `tools/libqb/copy_libqb.py`; `--check` says whether the copy still equals the commit with its edits. Do not
edit the copied files by hand: add an edit file and run the tool.
""",
        encoding="utf-8",
        newline="\n",
    )


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument(
        "--check", action="store_true", help="compare the copy with the commit; write nothing"
    )
    ap.add_argument("--clone", default=str(REPO.parent / "QB64pe"))
    ap.add_argument(
        "--commit", help="the commit to copy (default: SOURCE.md's, else the clone's HEAD)"
    )
    ap.add_argument("--dest", default=str(DEST), help=argparse.SUPPRESS)  # for the tests
    ap.add_argument("--edits", default=str(EDITS), help=argparse.SUPPRESS)  # for the tests
    a = ap.parse_args(argv)

    clone = pathlib.Path(a.clone).resolve()
    dest = pathlib.Path(a.dest)
    commit = a.commit or pinned_commit(dest) or "HEAD"
    commit = git(clone, "rev-parse", "--short=10", commit).decode().strip()
    files = commit_files(clone, commit)
    if not files:
        sys.exit(f"no files under {', '.join(PATHS)} at {commit}")
    try:
        edits = load_edits(a.edits)
        files = apply_edits(files, edits)
    except EditError as e:
        print(f"edit: {e}")
        return 1

    if a.check:
        problems = check(dest, files)
        for line in problems:
            print(line)
        print(f"libqb/ against {commit} with {len(edits)} edits: {len(files)} files, {len(problems)} problems")
        return 1 if problems else 0

    date = git(clone, "log", "-1", "--format=%cs", commit).decode().strip()
    dest.mkdir(parents=True, exist_ok=True)
    write(dest, files, commit, date, edits)
    print(f"libqb/ written from {commit} with {len(edits)} edits: {len(files)} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
