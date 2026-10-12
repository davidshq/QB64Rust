"""Checks tracked files for two project rules (CLAUDE.md):

* Rule 2: nothing private goes into the repo. Flags full local paths (any drive path such as C:\\Users\\... or
  D:\\work\\..., and /c/Users/..., /c/code/...), home folders (/home/<name>/, /Users/<name>/), temp folders
  (AppData\\Local\\Temp) and network shares (\\\\host\\share). Host names cannot be recognised; check by eye.
* Rule 7: text passed through a shell can collapse backslash escapes into control characters. Flags bytes
  0x00-0x1F other than tab, LF and the CR of CR LF.

Files marked -text in .gitattributes (recorded program output, byte-exact test programs), other recorded
output (RECORDED below) and binary files are not checked for control bytes; every file is checked for paths. A line that must show such a path as an example
(as CLAUDE.md does) is allowed by ALLOWED below, by file and text.

Usage: python tools/repo_check/check_repo.py [--untracked]   (from anywhere in the repo; exit status 1 on
findings; --untracked also checks new files that are not staged yet, unless git ignores them)
"""

import fnmatch
import re
import subprocess
import sys

PATH_PATTERNS = [
    (re.compile(rb"(?<![\w%])[A-Za-z]:[\\/]+[\w$ .-]+[\\/]"), "local path"),
    (re.compile(rb"(?<![\w.])/[a-z]/(Users|code)/", re.IGNORECASE), "local path (POSIX form)"),
    (re.compile(rb"(?<![\w.])/(home|Users)/[\w.-]+/"), "home folder"),
    (re.compile(rb"AppData[\\/]+Local[\\/]+Temp", re.IGNORECASE), "temp folder"),
    (re.compile(rb"(?<![\\\w])\\\\[A-Za-z][\w.-]*\\[\w$]"), "network share"),
]

# (file glob, bytes the line contains): paths that are not private. The examples in the rules themselves; a path
# made up to not exist; GitHub's runner folder (public); and the compiler-discovery unit tests, which are about
# made-up paths (C:\qb, /home/u/proj).
ALLOWED = [
    ("CLAUDE.md", rb"no full local paths"),
    ("tools/repo_check/check_repo.py", rb""),
    ("*", rb"does\\not\\exist"),
    ("*", rb"does/not/exist"),
    ("*", rb"/home/runner/"),
    ("vscode/test/unit/discovery.test.ts", rb""),
    # Upstream's test of its relative-path function, copied unedited (design D1 of m2-upstream-tests): made-up
    # paths (/home/user/proj, C:\proj).
    ("tests/upstream/compile_tests/qb64pe/file.bas", rb""),
    ("tests/upstream/compile_tests/qb64pe/file.output", rb""),
]

BINARY_SUFFIXES = (".bin", ".exe", ".png", ".ico", ".7z", ".zip", ".vsix")

# Output recorded from the old compiler or its programs: control bytes there are measured behaviour (NULs after
# `INF` in v02, a 0x01 in a qb64pe include message, the `-x` progress bar's CRs).
RECORDED = [
    "verification/*.out.txt",
    "verification/*.compile.txt",
    "vscode/test-fixtures/compiler/*.out.txt",
]


def git(*args: str, stdin: bytes = b"", cwd: str | None = None) -> bytes:
    return subprocess.run(
        ["git", *args], input=stdin, cwd=cwd, check=True, capture_output=True
    ).stdout


def unchecked_text(root: str, paths: list[str]) -> set[str]:
    """Paths whose `text` attribute is unset (-text)."""
    out = git(
        "check-attr", "--stdin", "-z", "text", stdin="\0".join(paths).encode(), cwd=root
    ).split(b"\0")
    triples = zip(out[0::3], out[1::3], out[2::3])
    return {p.decode() for p, _, v in triples if v == b"unset"}


def allowed(path: str, line: bytes) -> bool:
    return any(fnmatch.fnmatch(path, f) and text in line for f, text in ALLOWED)


def control_bytes(data: bytes) -> list[tuple[int, int]]:
    """(line number, byte) for each control byte other than tab, LF and the CR of CR LF."""
    found = []
    line = 1
    for i, b in enumerate(data):
        if b == 0x0A:
            line += 1
        elif b < 0x20 and b != 0x09 and not (b == 0x0D and data[i + 1 : i + 2] == b"\n"):
            found.append((line, b))
    return found


def main() -> int:
    root = git("rev-parse", "--show-toplevel").decode().strip()
    listing = ["ls-files", "-z"]
    if "--untracked" in sys.argv[1:]:
        # Also new files not yet staged (and not ignored), to check them before staging (CLAUDE.md rule 2).
        listing += ["--cached", "--others", "--exclude-standard"]
    paths = [p for p in git(*listing, cwd=root).decode().split("\0") if p]
    skip_bytes = unchecked_text(root, paths) if paths else set()
    findings = []
    for path in paths:
        try:
            with open(f"{root}/{path}", "rb") as f:
                data = f.read()
        except FileNotFoundError:
            continue  # deleted in the working tree
        for n, line in enumerate(data.split(b"\n"), 1):
            for pattern, what in PATH_PATTERNS:
                if pattern.search(line) and not allowed(path, line):
                    findings.append(f"{path}:{n}: {what}")
        recorded = any(fnmatch.fnmatch(path, g) for g in RECORDED)
        if path in skip_bytes or recorded or path.endswith(BINARY_SUFFIXES):
            continue
        for n, b in control_bytes(data):
            findings.append(f"{path}:{n}: control byte 0x{b:02X}")
    for f in findings:
        print(f)
    print(f"{len(paths)} files checked, {len(findings)} findings")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
