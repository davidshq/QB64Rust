"""Check recorded provenance and required notices; optionally verify a release VSIX or compiler download.

Run: python tools/licenses/check.py [--vsix package.vsix | --zip qb64rust-win-x64-<version>.zip]
This checks the inventory's obligations, not legal compatibility or code similarity.
"""

import argparse
import pathlib
import subprocess
import sys
import tomllib
import zipfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
INVENTORY = "tools/licenses/inventory.toml"
VENDOR_NAMES = {"vendor", "vendored", "third_party", "third-party", "external"}


def repository_files(root):
    result = subprocess.run(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
        cwd=root, capture_output=True, check=True,
    )
    return set(filter(None, result.stdout.decode("utf-8").split("\0")))


def covered(path, roots):
    return any(path == r or path.startswith(r + "/") for r in roots)


def normalized(data):
    return data.replace(b"\r\n", b"\n")


class Layout:
    """Where a package holds the notices: the repository's LICENSE, and a component's `package_prefix` path."""

    def __init__(self, license_path, place):
        self.license_path = license_path
        self.place = place


# The extension's .vsix: the notices where `package_prefix` puts them.
VSIX = Layout("extension/LICENSE.txt", lambda path: path)

BUNDLE_PREFIX = "extension/bundle/"


def zip_layout(top):
    """The compiler download (spec `compiler/download`): the bundle's files under `<top>/`, the licence beside them."""

    def place(path):
        if path.startswith(BUNDLE_PREFIX):
            return f"{top}/{path[len(BUNDLE_PREFIX):]}"
        return path

    return Layout(f"{top}/LICENSE.txt", place)


def zip_top(package):
    """The single top folder of a download, or None."""
    tops = {name.split("/", 1)[0] for name in package.namelist()}
    return tops.pop() if len(tops) == 1 else None


def check(root, files, components, package=None, layout=VSIX):
    problems = []
    provenance = set()
    roots = []
    required_package = {layout.license_path: "LICENSE"}
    for component in components:
        name = component["name"]
        for key in ("source", "license", "paths", "provenance", "notices"):
            if not component.get(key):
                problems.append(f"{name}: missing {key}")
        roots.extend(component.get("paths", []))
        provenance.update(component.get("provenance", []))
        for path in component.get("paths", []):
            if not any(covered(f, [path]) for f in files):
                problems.append(f"{name}: no repository files under {path}")
        for pattern in component.get("provenance", []) + component.get("notices", []):
            matches = sorted(p.relative_to(root).as_posix() for p in root.glob(pattern) if p.is_file())
            if not matches:
                problems.append(f"{name}: missing {pattern}")
            for path in matches:
                if path not in files:
                    problems.append(f"{name}: not in repository file scan: {path}")
                if not (root / path).read_bytes().strip():
                    problems.append(f"{name}: empty {path}")
                if component.get("package_prefix") and pattern in component.get("notices", []):
                    required_package[layout.place(component["package_prefix"] + path)] = path
    for path in sorted(files):
        if pathlib.PurePosixPath(path).name == "SOURCE.md" and path not in provenance:
            problems.append(f"unregistered provenance: {path}; review and add to {INVENTORY}")
        if VENDOR_NAMES.intersection(pathlib.PurePosixPath(path).parts[:-1]) and not covered(path, roots):
            problems.append(f"unregistered vendor content: {path}; review and add to {INVENTORY}")
    if package is not None:
        names = set(package.namelist())
        for destination, source in sorted(required_package.items()):
            if destination not in names:
                problems.append(f"package missing: {destination}")
            elif not (root / source).is_file():
                problems.append(f"repository notice missing: {source}")
            elif normalized(package.read(destination)) != normalized((root / source).read_bytes()):
                problems.append(f"package notice differs: {destination}")
    return problems


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    which = parser.add_mutually_exclusive_group()
    which.add_argument("--vsix", type=pathlib.Path)
    which.add_argument("--zip", type=pathlib.Path)
    args = parser.parse_args(argv)
    with (ROOT / INVENTORY).open("rb") as stream:
        components = tomllib.load(stream)["components"]
    files = repository_files(ROOT)
    if args.vsix:
        with zipfile.ZipFile(args.vsix) as package:
            problems = check(ROOT, files, components, package)
    elif args.zip:
        with zipfile.ZipFile(args.zip) as package:
            top = zip_top(package)
            if top is None:
                problems = [f"{args.zip}: not one top folder"]
            else:
                problems = check(ROOT, files, components, package, zip_layout(top))
    else:
        problems = check(ROOT, files, components)
    for problem in problems:
        print(f"licenses: {problem}", file=sys.stderr)
    if not problems:
        print(f"licenses: ok ({len(components)} inventory entries)")
    return int(bool(problems))


if __name__ == "__main__":
    sys.exit(main())
