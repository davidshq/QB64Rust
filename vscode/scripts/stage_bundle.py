"""Writes vscode/bundle/, what the platform .vsix carries beside the extension (design D1 and D7 of the OpenSpec
change extension-vsix):

- bin/qb64rust.exe, the release build of the new compiler (--binary, default target/release/qb64rust.exe);
- libqb/, the copy of libqb: its tracked files only (`git ls-files libqb`), so the objects a build left in the
  working tree never ship;
- libqb.id, a hash of those files' names and bytes, which names the extension's prepared copy of libqb, so that a
  package of the same version with other libqb files is not given the copy of the earlier ones;
- vscode/LICENSE (not in bundle/: vsce takes it from the extension's root), the repository's;
- THIRD-PARTY-NOTICES.txt: for each crate linked into the binary (the normal dependencies of qb64rust-driver for
  the Windows target, as `cargo tree` resolves them), its name, version, licence expression and the text of its
  licence files. A crate that ships none takes vscode/scripts/licenses/<name>.txt; a crate with neither fails
  the script, naming it, so a new dependency cannot ship without its text.

The folder is deleted and written again on each run. Exits with status 1 and a message when something is
missing.

Usage: python vscode/scripts/stage_bundle.py [--binary <qb64rust.exe>]
"""

import argparse
import hashlib
import json
import pathlib
import re
import shutil
import subprocess
import sys

SCRIPTS = pathlib.Path(__file__).resolve().parent
REPO = SCRIPTS.parents[1]
OUT = REPO / "vscode" / "bundle"
# vsce takes the licence from the extension's root folder.
LICENCE_OUT = REPO / "vscode" / "LICENSE"
OVERRIDES = SCRIPTS / "licenses"
TARGET = "x86_64-pc-windows-msvc"
DRIVER = "qb64rust-driver"
LICENCE_FILE = re.compile(r"^(licen[cs]e|copying|unlicense)", re.IGNORECASE)
# A folder as cargo tree prints a path dependency's source: absolute, Windows or POSIX.
LOCAL_PATH = re.compile(r"^([A-Za-z]:[\\/]|/|\\\\)")
# Names the copy of libqb by its content, so that the extension does not reuse a copy of different files.
LIBQB_ID = "libqb.id"


class StageError(Exception):
    pass


def run(args, cwd):
    result = subprocess.run(args, cwd=cwd, capture_output=True, check=False)
    if result.returncode != 0:
        raise StageError(
            f"{' '.join(args)} failed: {result.stderr.decode(errors='replace').strip()}"
        )
    return result.stdout


def tracked_files(repo, folder):
    """The files git tracks under `folder`, as paths relative to `repo`."""
    out = run(["git", "ls-files", "-z", "--", folder], repo)
    return [p for p in out.decode("utf-8").split("\0") if p]


def copy_tracked(repo, folder, dest):
    """Copies the tracked files of `folder` into `dest`; returns their count and a hash of their names and bytes."""
    files = tracked_files(repo, folder)
    if not files:
        raise StageError(f"no tracked files under {folder}/")
    digest = hashlib.sha256()
    for rel in sorted(files):
        target = dest / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(repo / rel, target)
        data = (repo / rel).read_bytes()
        digest.update(f"{rel}\0{len(data)}\0".encode("utf-8"))
        digest.update(data)
    return len(files), digest.hexdigest()[:16]


def linked_crates(repo):
    """(name, version, licence) of each crate `cargo tree` resolves for the driver, the workspace's left out."""
    out = run(
        [
            "cargo",
            "tree",
            "-p",
            DRIVER,
            "-e",
            "normal",
            "--target",
            TARGET,
            "--prefix",
            "none",
            "--locked",
            "--format",
            "{p}|{l}",
        ],
        repo,
    )
    return parse_tree(out.decode("utf-8"))


def parse_tree(text):
    """The crates of `cargo tree --prefix none --format {p}|{l}` output, each once, path dependencies left out."""
    crates = set()
    for line in text.splitlines():
        # `(*)` (shown before) ends the line, after the format; `(proc-macro)` follows `{p}`.
        line = line.replace(" (*)", "").replace(" (proc-macro)", "")
        if "|" not in line:
            continue
        package, licence = line.split("|", 1)
        source = re.search(r"\((.*)\)", package)
        if source and LOCAL_PATH.match(source.group(1)):
            continue  # a path dependency, `name vX (folder)`: one of the workspace's crates
        # Kept: a git or other registry's dependency, `name vX (https://…#rev)`, whose licence ships too.
        name, version = package.split()[:2]
        crates.add((name, version.lstrip("v"), licence.strip()))
    return sorted(crates)


def crate_dirs(repo):
    """The source folder of each package `cargo metadata` knows, by (name, version)."""
    out = run(["cargo", "metadata", "--format-version", "1", "--locked"], repo)
    meta = json.loads(out)
    return {
        (p["name"], p["version"]): pathlib.Path(p["manifest_path"]).parent
        for p in meta["packages"]
        if p.get("source")
    }


def licence_texts(name, folder, overrides):
    """(file name, text) of a crate's licence files, else of its override; empty if it has neither."""
    found = []
    if folder is not None and folder.is_dir():
        for f in sorted(folder.iterdir()):
            if f.is_file() and LICENCE_FILE.match(f.name):
                found.append((f.name, f.read_text(encoding="utf-8", errors="replace")))
    if not found:
        override = overrides / f"{name}.txt"
        if override.is_file():
            found.append((override.name, override.read_text(encoding="utf-8")))
    return found


def notice(crates, dirs, overrides):
    parts = [
        "Third-party notices of the qb64rust binary\n",
        (
            "The qb64rust binary in bundle/bin/ is MIT-licensed (LICENSE in the extension's folder). It\n"
            "links the crates below, each under the licence named. The copy of libqb in bundle/libqb/ is\n"
            "QB64 Phoenix Edition's, under the licences in libqb/COPYING.txt and libqb/licenses/.\n"
        ),
    ]
    missing = []
    for name, version, licence in crates:
        texts = licence_texts(name, dirs.get((name, version)), overrides)
        if not texts:
            missing.append(f"{name} {version}")
            continue
        parts.append(f"\n{'=' * 78}\n{name} {version} ({licence})\n{'=' * 78}\n")
        for file_name, text in texts:
            parts.append(f"\n--- {file_name} ---\n\n{text.rstrip()}\n")
    if missing:
        raise StageError(
            "no licence file for "
            + ", ".join(missing)
            + f"; add its licence text as <name>.txt in {overrides.name}/ beside this script"
        )
    return "".join(parts)


def stage(repo, out, binary, licence_out, overrides=OVERRIDES, crates=None, dirs=None):
    if not binary.is_file():
        raise StageError(
            f"the qb64rust binary is missing: {binary} (build it with cargo build --release)"
        )
    if not (repo / "libqb" / "internal" / "c" / "qbx.cpp").is_file():
        raise StageError("libqb/internal/c/qbx.cpp is missing: the copy of libqb is not there")
    text = notice(
        linked_crates(repo) if crates is None else crates,
        crate_dirs(repo) if dirs is None else dirs,
        overrides,
    )
    if out.exists():
        shutil.rmtree(out)
    (out / "bin").mkdir(parents=True)
    shutil.copy2(binary, out / "bin" / "qb64rust.exe")
    count, libqb_id = copy_tracked(repo, "libqb", out)
    (out / LIBQB_ID).write_text(libqb_id + "\n", encoding="utf-8", newline="\n")
    shutil.copy2(repo / "LICENSE", licence_out)
    (out / "THIRD-PARTY-NOTICES.txt").write_text(text, encoding="utf-8", newline="\n")
    return count


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--binary", type=pathlib.Path, default=REPO / "target" / "release" / "qb64rust.exe"
    )
    args = parser.parse_args(argv)
    try:
        count = stage(REPO, OUT, args.binary.resolve(), LICENCE_OUT)
    except StageError as e:
        print(f"stage_bundle: {e}", file=sys.stderr)
        return 1
    print(f"stage_bundle: wrote {OUT.relative_to(REPO).as_posix()}/ (libqb: {count} files)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
