"""Checks a packaged .vsix against "Platform package contents" of spec editor/packaging (design D8 of the
OpenSpec change extension-vsix): it targets win32-x64, carries the qb64rust binary, the copy of libqb with its
licence, the crates' notice and the extension's licence, and carries no TypeScript file, no built object and
no toolchain.

With --tag, it first checks that a release tag `vscode-v<version>` names the version of vscode/package.json and
that CHANGELOG.md has a section `## <version>` (written to --notes-out, the release's notes); with --tag alone
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
import zipfile

PACKAGE_JSON = pathlib.Path(__file__).resolve().parents[1] / "package.json"
CHANGELOG = PACKAGE_JSON.parent / "CHANGELOG.md"
TARGET = "win32-x64"
REQUIRED = [
    "extension/bundle/bin/qb64rust.exe",
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


def changelog_section(text, version):
    """The body of `## <version>` in CHANGELOG.md, without its heading; None when there is no such section."""
    m = re.search(
        rf"^## {re.escape(version)}[ \t]*\n(.*?)(?=^## |\Z)", text, re.MULTILINE | re.DOTALL
    )
    return m.group(1).strip() + "\n" if m else None


def check_tag(tag, version):
    """Problems with a release tag against the package's version (an empty list when it matches)."""
    m = TAG.match(tag)
    if not m:
        return [f"tag {tag!r} is not of the form vscode-v<version>"]
    if m.group(1) != version:
        return [f"tag {tag} names version {m.group(1)}, but package.json says {version}"]
    return []


def check_vsix(names, manifest):
    """Problems with a package, given its file names and the text of its extension.vsixmanifest."""
    problems = [f"missing: {n}" for n in REQUIRED if n not in names]
    for n in names:
        lower = n.lower()
        if lower.endswith(FORBIDDEN_SUFFIXES):
            problems.append(f"must not be packaged: {n}")
        elif "/c_compiler/" in lower:
            problems.append(f"a toolchain file must not be packaged: {n}")
    if f'TargetPlatform="{TARGET}"' not in manifest:
        problems.append(f"the manifest does not target {TARGET}")
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
        problems += check_tag(args.tag, version)
        notes = changelog_section(CHANGELOG.read_text(encoding="utf-8"), version)
        if notes is None:
            problems.append(f"CHANGELOG.md has no section `## {version}`")
        elif args.notes_out is not None:
            args.notes_out.write_text(notes, encoding="utf-8")
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
