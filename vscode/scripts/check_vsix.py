"""Checks a packaged .vsix against "Platform package contents" of spec editor/packaging (design D8 of the
OpenSpec change extension-vsix): it targets win32-x64 or linux-x64 (design D8 of m7-linux), carries the qb64rust
binary for it (`qb64rust.exe`, `qb64rust`; a Linux package holds no `.exe`), the copy of libqb with its
licence, the crates' notice and the extension's licence, and carries no TypeScript file, no built object and
no toolchain.

With --tag, it first checks that a release tag `vscode-v<version>` names the version of vscode/package.json, that
the workspace version of Cargo.toml is the same, and that CHANGELOG.md has a section `## <version>` (written to
--notes-out with a paragraph on the four assets, the release's notes); with --tag alone
(no .vsix) it checks only that, so the release workflow fails before it packages.

Exits with status 1, naming each problem, when a check fails.

Usage: python vscode/scripts/check_vsix.py [<file.vsix>] [--tag vscode-v<version> [--notes-out <file>]]
"""

import argparse
import json
import pathlib
import re
import subprocess
import sys
import tomllib
import zipfile

PACKAGE_JSON = pathlib.Path(__file__).resolve().parents[1] / "package.json"
CHANGELOG = PACKAGE_JSON.parent / "CHANGELOG.md"
TARGET = "win32-x64"
# The binary each target's package carries.
BINARIES = {
    "win32-x64": "extension/bundle/bin/qb64rust.exe",
    "linux-x64": "extension/bundle/bin/qb64rust",
}
# What a Windows package holds; a Linux one the same with its own binary.
REQUIRED = [
    BINARIES[TARGET],
    "extension/bundle/libqb/Makefile",
    "extension/bundle/libqb/internal/c/qbx.cpp",
    "extension/bundle/libqb/COPYING.txt",
    "extension/bundle/libqb.id",
    "extension/bundle/THIRD-PARTY-NOTICES.txt",
    "extension/LICENSE.txt",
    "extension/package.json",
    "extension/out/src/extension.js",
]
FORBIDDEN_SUFFIXES = (".ts", ".o", ".a", ".map")
TAG = re.compile(r"^vscode-v(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)$")
CARGO_TOML = PACKAGE_JSON.parents[1] / "Cargo.toml"
ASSETS_PARAGRAPH = (
    "Each `.vsix` is the VS Code extension, with the compiler inside: `win32-x64` for Windows, `linux-x64` for Linux."
    " The `.zip` (Windows) and the `.tar.gz` (Linux) are the compiler alone, for a command line or another editor:"
    " unpack one into a folder you can write to and read its `MANUAL.md`.\n"
)


def changelog_section(text, version):
    """The body of `## <version>` in CHANGELOG.md, without its heading; None when there is no such section."""
    m = re.search(
        rf"^## {re.escape(version)}[ \t]*\n(.*?)(?=^## |\Z)", text, re.MULTILINE | re.DOTALL
    )
    return m.group(1).strip() + "\n" if m else None


def check_tag(tag, version, cargo_version=None):
    """Problems with a release tag against the package's version and, when given, the workspace version of
    Cargo.toml, which must equal it (design D5 of the OpenSpec change m6-compiler-download); an empty list when all
    match."""
    m = TAG.match(tag)
    if not m:
        return [f"tag {tag!r} is not of the form vscode-v<version>"]
    problems = []
    if m.group(1) != version:
        problems.append(f"tag {tag} names version {m.group(1)}, but package.json says {version}")
    if cargo_version is not None and cargo_version != version:
        problems.append(
            f"Cargo.toml's workspace version is {cargo_version}, but package.json says {version}; raise both"
        )
    return problems


def cargo_version(text):
    """The `[workspace.package] version` of Cargo.toml's text."""
    return tomllib.loads(text)["workspace"]["package"]["version"]


def release_notes(section):
    """The release's notes: the changelog section, then what the four assets are (design D8 of m6-compiler-download, D10 of m7-linux)."""
    return section + "\n" + ASSETS_PARAGRAPH


def check_vsix(names, manifest):
    """Problems with a package, given its file names and the text of its extension.vsixmanifest. Its target is the
    manifest's; one that is neither of BINARIES' is a problem, and the package is then checked as Windows'."""
    m = re.search(r'TargetPlatform="([^"]*)"', manifest)
    target = m.group(1) if m and m.group(1) in BINARIES else None
    required = [BINARIES[target or TARGET], *REQUIRED[1:]]
    problems = [f"missing: {n}" for n in required if n not in names]
    for n in names:
        lower = n.lower()
        if lower.endswith(FORBIDDEN_SUFFIXES):
            problems.append(f"must not be packaged: {n}")
        elif "/c_compiler/" in lower:
            problems.append(f"a toolchain file must not be packaged: {n}")
        elif target == "linux-x64" and lower.endswith(".exe"):
            problems.append(f"a Linux package must hold no .exe: {n}")
    if target is None:
        problems.append(f"the manifest does not target {' or '.join(BINARIES)}")
    return problems


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("vsix", nargs="?", type=pathlib.Path)
    parser.add_argument("--tag")
    parser.add_argument("--notes-out", type=pathlib.Path)
    args = parser.parse_args(argv)
    if args.vsix is None and args.tag is None:
        parser.error("give a .vsix, --tag, or both")
    problems = []
    if args.tag is not None:
        version = json.loads(PACKAGE_JSON.read_text(encoding="utf-8"))["version"]
        problems += check_tag(args.tag, version, cargo_version(CARGO_TOML.read_text(encoding="utf-8")))
        notes = changelog_section(CHANGELOG.read_text(encoding="utf-8"), version)
        if notes is None:
            problems.append(f"CHANGELOG.md has no section `## {version}`")
        elif args.notes_out is not None:
            args.notes_out.write_text(release_notes(notes), encoding="utf-8")
    if args.vsix is not None:
        with zipfile.ZipFile(args.vsix) as z:
            names = set(z.namelist())
            manifest = z.read("extension.vsixmanifest").decode("utf-8")
        problems += check_vsix(names, manifest)
        # The shared inventory checks every runtime notice and its text in the actual archive.
        result = subprocess.run([
            sys.executable,
            str(PACKAGE_JSON.parents[1] / "tools" / "licenses" / "check.py"),
            "--vsix", str(args.vsix),
        ], check=False)
        if result.returncode:
            problems.append("repository/package license inventory check failed (see above)")
    for p in problems:
        print(f"check_vsix: {p}", file=sys.stderr)
    if problems:
        return 1
    print("check_vsix: ok" + (f" ({args.vsix.name}, {len(names)} files)" if args.vsix else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())
