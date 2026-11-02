"""Writes and checks the compiler download (spec compiler/download; design D2 and D6 of the OpenSpec change
m6-compiler-download, design D9 of m7-linux): qb64rust-win-x64-<version>.zip, or qb64rust-linux-x64-<version>.tar.gz.

--out <dir> [--platform win-x64|linux-x64] writes the download from the staged vscode/bundle/ (staged for that
platform: `npm run package` on Windows, `stage_bundle.py --target linux-x64` on Linux) and the repository, the
version taken from vscode/package.json:

    qb64rust-<version>/qb64rust.exe             bundle/bin/qb64rust.exe (qb64rust and bin/qb64rust on Linux)
    qb64rust-<version>/libqb/...                bundle/libqb/ (libqb's tracked files)
    qb64rust-<version>/THIRD-PARTY-NOTICES.txt  bundle/THIRD-PARTY-NOTICES.txt
    qb64rust-<version>/LICENSE.txt              LICENSE
    qb64rust-<version>/MANUAL.md                docs/MANUAL.md

The tar is the same bytes for the same bundle: its entries in order, the binary at mode 0755 and every other file at
0644, owner and group 0 with no names, each file's time its staged file's, and gzip's own time 0.

--check <file> checks a download: named as the spec says, one top folder named as the file is, the required files,
nothing built, no toolchain, no extension, in a tar the binary executable; then the licence notices against the
repository (tools/licenses/check.py --zip or --tar).

Exits with status 1, naming each problem, when a check fails.

Usage: python vscode/scripts/package_download.py --out <dir> [--platform win-x64|linux-x64] | --check <file>
"""

import argparse
import gzip
import io
import json
import pathlib
import re
import subprocess
import sys
import tarfile
import zipfile

SCRIPTS = pathlib.Path(__file__).resolve().parent
REPO = SCRIPTS.parents[1]
BUNDLE = REPO / "vscode" / "bundle"
PACKAGE_JSON = REPO / "vscode" / "package.json"
VERSION = r"(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)"
# Each platform's file name, binary, and the binary in the staged bundle.
PLATFORMS = {
    "win-x64": (re.compile(rf"^qb64rust-win-x64-{VERSION}\.zip$"), "qb64rust.exe", "{}.zip"),
    "linux-x64": (re.compile(rf"^qb64rust-linux-x64-{VERSION}\.tar\.gz$"), "qb64rust", "{}.tar.gz"),
}
COMMON = [
    "libqb/Makefile",
    "libqb/internal/c/qbx.cpp",
    "libqb/COPYING.txt",
    "LICENSE.txt",
    "THIRD-PARTY-NOTICES.txt",
    "MANUAL.md",
]
# What the Windows download holds; the Linux one the same with its own binary.
REQUIRED = ["qb64rust.exe", *COMMON]
FORBIDDEN_SUFFIXES = (".o", ".a", ".ts")
FORBIDDEN_FOLDERS = ("c_compiler", "bundle")


class PackageError(Exception):
    pass


def download_name(version, platform="win-x64"):
    return PLATFORMS[platform][2].format(f"qb64rust-{platform}-{version}")


def zip_name(version):
    return download_name(version, "win-x64")


def top_folder(version):
    return f"qb64rust-{version}"


def binary_name(platform):
    return PLATFORMS[platform][1]


def entries(bundle, repo, platform="win-x64"):
    """(name in the download below the top folder, source file) of every file it holds, sorted by name."""
    binary = bundle / "bin" / binary_name(platform)
    libqb = bundle / "libqb"
    notices = bundle / "THIRD-PARTY-NOTICES.txt"
    stage = "`npm run package` in vscode/" if platform == "win-x64" else "`stage_bundle.py --target linux-x64`"
    for path, what in [
        (binary, "the staged binary"),
        (libqb / "internal" / "c" / "qbx.cpp", "the staged copy of libqb"),
        (notices, "the staged notices"),
        (repo / "LICENSE", "the repository's LICENSE"),
        (repo / "docs" / "MANUAL.md", "MANUAL.md"),
    ]:
        if not path.is_file():
            raise PackageError(f"{what} is missing: {path} (run {stage} first)")
    files = [
        (binary_name(platform), binary),
        ("THIRD-PARTY-NOTICES.txt", notices),
        ("LICENSE.txt", repo / "LICENSE"),
        ("MANUAL.md", repo / "docs" / "MANUAL.md"),
    ]
    files += [
        ("libqb/" + p.relative_to(libqb).as_posix(), p) for p in libqb.rglob("*") if p.is_file()
    ]
    return sorted(files)


def write_tar(path, top, files, binary):
    """The tar.gz of design D9 of m7-linux: the same bytes for the same files."""
    raw = io.BytesIO()
    with tarfile.open(fileobj=raw, mode="w", format=tarfile.PAX_FORMAT) as t:
        for name, source in files:
            data = source.read_bytes()
            info = tarfile.TarInfo(f"{top}/{name}")
            info.size = len(data)
            info.mode = 0o755 if name == binary else 0o644
            info.uid = info.gid = 0
            info.uname = info.gname = ""
            info.mtime = int(source.stat().st_mtime)
            t.addfile(info, io.BytesIO(data))
    with open(path, "wb") as out, gzip.GzipFile(filename="", mode="wb", fileobj=out, mtime=0) as gz:
        gz.write(raw.getvalue())


def write(out, version, bundle=BUNDLE, repo=REPO, platform="win-x64"):
    """Writes the download into `out`; returns its path and its file count."""
    files = entries(bundle, repo, platform)
    out.mkdir(parents=True, exist_ok=True)
    path = out / download_name(version, platform)
    top = top_folder(version)
    if platform == "win-x64":
        with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as z:
            for name, source in files:
                z.write(source, f"{top}/{name}")
    else:
        write_tar(path, top, files, binary_name(platform))
    return path, len(files)


def platform_of(file_name):
    """The platform a download's file name names, and its version; (None, None) for no download's name."""
    for platform, (pattern, _, _) in PLATFORMS.items():
        m = pattern.match(file_name)
        if m:
            return platform, m.group(1)
    return None, None


def check_names(file_name, names, modes=None):
    """Problems with a download, given its file name, the names of its entries and, for a tar, their modes."""
    platform, version = platform_of(file_name)
    if platform is None:
        return [
            f"{file_name} is not named qb64rust-win-x64-<version>.zip or qb64rust-linux-x64-<version>.tar.gz"
        ]
    top = top_folder(version)
    required = [binary_name(platform), *COMMON]
    problems = [f"not under {top}/: {n}" for n in sorted(names) if not n.startswith(top + "/")]
    problems += [f"missing: {top}/{r}" for r in required if f"{top}/{r}" not in names]
    for n in sorted(names):
        lower = n.lower()
        folders = lower.split("/")[:-1]
        if lower.endswith(FORBIDDEN_SUFFIXES) or (platform == "linux-x64" and lower.endswith(".exe")):
            problems.append(f"must not be packaged: {n}")
        elif any(f in folders for f in FORBIDDEN_FOLDERS):
            problems.append(f"must not be packaged: {n}")
    binary = f"{top}/{binary_name(platform)}"
    if modes is not None and binary in modes and modes[binary] & 0o111 != 0o111:
        problems.append(f"not executable: {binary} (mode {modes[binary]:o})")
    return problems


def toolchain_packages(rust):
    """The install lines of the PACKAGES table of crates/driver/src/toolchain.rs (its text): `<command> <packages>`,
    one per distribution, in the table's order."""
    table = re.search(r"pub const PACKAGES: .*?= \[(.*?)\n\];", rust, re.DOTALL)
    if table is None:
        return []
    lines = []
    for row in re.finditer(r'\(\s*&\[[^\]]*\],\s*"([^"]+)",\s*&\[([^\]]*)\],?\s*\)', table.group(1)):
        packages = re.findall(r'"([^"]+)"', row.group(2))
        lines.append(f"{row.group(1)} {' '.join(packages)}")
    return lines


def manual_problems(manual, rust):
    """Problems with MANUAL.md's install lines (design D9 of m7-linux, spec compiler/download, "The Linux packages
    in the manual"): each line of the table must stand in the manual as a line of its own."""
    wanted = toolchain_packages(rust)
    if not wanted:
        return ["no PACKAGES table in crates/driver/src/toolchain.rs"]
    have = {line.strip() for line in manual.splitlines()}
    return [f"MANUAL.md lacks the install line of toolchain.rs: {line}" for line in wanted if line not in have]


def check(path, licences=True):
    """Problems with the download at `path`; with `licences`, also the notices against the repository."""
    modes = None
    if path.name.endswith(".tar.gz"):
        with tarfile.open(path) as t:
            members = [m for m in t.getmembers() if m.isfile()]
        names = {m.name for m in members}
        modes = {m.name: m.mode for m in members}
        option = "--tar"
    else:
        with zipfile.ZipFile(path) as z:
            names = set(z.namelist())
        option = "--zip"
    problems = check_names(path.name, names, modes)
    problems += manual_problems(
        (REPO / "docs" / "MANUAL.md").read_text(encoding="utf-8"),
        (REPO / "crates" / "driver" / "src" / "toolchain.rs").read_text(encoding="utf-8"),
    )
    if licences:
        result = subprocess.run(
            [sys.executable, str(REPO / "tools" / "licenses" / "check.py"), option, str(path)],
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
    parser.add_argument("--platform", choices=sorted(PLATFORMS), default="win-x64")
    args = parser.parse_args(argv)
    if args.out is not None:
        version = json.loads(PACKAGE_JSON.read_text(encoding="utf-8"))["version"]
        try:
            path, count = write(args.out, version, platform=args.platform)
        except PackageError as e:
            print(f"package_download: {e}", file=sys.stderr)
            return 1
        print(f"package_download: wrote {path} ({count} files)")
        return 0
    problems, count = check(args.check)
    for p in problems:
        print(f"package_download: {p}", file=sys.stderr)
    if problems:
        return 1
    print(f"package_download: ok ({args.check.name}, {count} files)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
