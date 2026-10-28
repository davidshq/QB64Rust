"""Writes and checks the compiler download, qb64rust-win-x64-<version>.zip (spec compiler/download; design D2 and D6
of the OpenSpec change m6-compiler-download).

--out <dir> writes the zip from the staged vscode/bundle/ (run `npm run package` first, which stages it) and the
repository, the version taken from vscode/package.json:

    qb64rust-<version>/qb64rust.exe             bundle/bin/qb64rust.exe
    qb64rust-<version>/libqb/...                bundle/libqb/ (libqb's tracked files)
    qb64rust-<version>/THIRD-PARTY-NOTICES.txt  bundle/THIRD-PARTY-NOTICES.txt
    qb64rust-<version>/LICENSE.txt              LICENSE
    qb64rust-<version>/MANUAL.md                MANUAL.md

--check <zip> checks a zip: one top folder named as the file is, the required files, nothing built, no toolchain,
no extension; then the licence notices against the repository (tools/licenses/check.py --zip).

Exits with status 1, naming each problem, when a check fails.

Usage: python vscode/scripts/package_zip.py --out <dir> | --check <file.zip>
"""

import argparse
import json
import pathlib
import re
import subprocess
import sys
import zipfile

SCRIPTS = pathlib.Path(__file__).resolve().parent
REPO = SCRIPTS.parents[1]
BUNDLE = REPO / "vscode" / "bundle"
PACKAGE_JSON = REPO / "vscode" / "package.json"
NAME = re.compile(r"^qb64rust-win-x64-(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)\.zip$")
REQUIRED = [
    "qb64rust.exe",
    "libqb/Makefile",
    "libqb/internal/c/qbx.cpp",
    "libqb/COPYING.txt",
    "LICENSE.txt",
    "THIRD-PARTY-NOTICES.txt",
    "MANUAL.md",
]
FORBIDDEN_SUFFIXES = (".o", ".a", ".ts")
FORBIDDEN_FOLDERS = ("c_compiler", "bundle")


class PackageError(Exception):
    pass


def zip_name(version):
    return f"qb64rust-win-x64-{version}.zip"


def top_folder(version):
    return f"qb64rust-{version}"


def entries(bundle, repo):
    """(name in the zip below the top folder, source file) of every file the download holds, sorted by name."""
    binary = bundle / "bin" / "qb64rust.exe"
    libqb = bundle / "libqb"
    notices = bundle / "THIRD-PARTY-NOTICES.txt"
    for path, what in [
        (binary, "the staged binary"),
        (libqb / "internal" / "c" / "qbx.cpp", "the staged copy of libqb"),
        (notices, "the staged notices"),
        (repo / "LICENSE", "the repository's LICENSE"),
        (repo / "MANUAL.md", "MANUAL.md"),
    ]:
        if not path.is_file():
            raise PackageError(f"{what} is missing: {path} (run `npm run package` in vscode/ first)")
    files = [
        ("qb64rust.exe", binary),
        ("THIRD-PARTY-NOTICES.txt", notices),
        ("LICENSE.txt", repo / "LICENSE"),
        ("MANUAL.md", repo / "MANUAL.md"),
    ]
    files += [
        ("libqb/" + p.relative_to(libqb).as_posix(), p) for p in libqb.rglob("*") if p.is_file()
    ]
    return sorted(files)


def write(out, version, bundle=BUNDLE, repo=REPO):
    """Writes the zip into `out`; returns its path and its file count."""
    files = entries(bundle, repo)
    out.mkdir(parents=True, exist_ok=True)
    path = out / zip_name(version)
    top = top_folder(version)
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as z:
        for name, source in files:
            z.write(source, f"{top}/{name}")
    return path, len(files)


def check_names(file_name, names):
    """Problems with a download, given its file name and the names of its entries."""
    m = NAME.match(file_name)
    if not m:
        return [f"{file_name} is not named qb64rust-win-x64-<version>.zip"]
    top = top_folder(m.group(1))
    problems = [f"not under {top}/: {n}" for n in sorted(names) if not n.startswith(top + "/")]
    problems += [f"missing: {top}/{r}" for r in REQUIRED if f"{top}/{r}" not in names]
    for n in sorted(names):
        lower = n.lower()
        folders = lower.split("/")[:-1]
        if lower.endswith(FORBIDDEN_SUFFIXES):
            problems.append(f"must not be packaged: {n}")
        elif any(f in folders for f in FORBIDDEN_FOLDERS):
            problems.append(f"must not be packaged: {n}")
    return problems


def check(path, licences=True):
    """Problems with the zip at `path`; with `licences`, also the notices against the repository."""
    with zipfile.ZipFile(path) as z:
        names = set(z.namelist())
    problems = check_names(path.name, names)
    if licences:
        result = subprocess.run(
            [sys.executable, str(REPO / "tools" / "licenses" / "check.py"), "--zip", str(path)],
            check=False,
        )
        if result.returncode:
            problems.append("repository/package license inventory check failed (see above)")
    return problems, len(names)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    which = parser.add_mutually_exclusive_group(required=True)
    which.add_argument("--out", type=pathlib.Path)
    which.add_argument("--check", type=pathlib.Path)
    args = parser.parse_args(argv)
    if args.out is not None:
        version = json.loads(PACKAGE_JSON.read_text(encoding="utf-8"))["version"]
        try:
            path, count = write(args.out, version)
        except PackageError as e:
            print(f"package_zip: {e}", file=sys.stderr)
            return 1
        print(f"package_zip: wrote {path} ({count} files)")
        return 0
    problems, count = check(args.check)
    for p in problems:
        print(f"package_zip: {p}", file=sys.stderr)
    if problems:
        return 1
    print(f"package_zip: ok ({args.check.name}, {count} files)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
