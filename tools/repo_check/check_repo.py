"""Checks tracked files for two project rules (CLAUDE.md):

* Rule 2: nothing private goes into the repo. Flags full local paths (C:\\Users\\..., C:\\code\\..., /c/Users/...,
  /c/code/...), temp folders (AppData\\Local\\Temp) and network shares (\\\\host\\share).
* Rule 7: text passed through a shell can collapse backslash escapes into control characters. Flags bytes
  0x00-0x1F other than tab, LF and the CR of CR LF.

Files marked -text in .gitattributes (recorded program output, byte-exact test programs), other recorded
output (RECORDED below) and binary files are not checked for control bytes; every file is checked for paths. A line that must show such a path as an example
(as CLAUDE.md does) is allowed by ALLOWED below, by file and text.

Usage: python tools/repo_check/check_repo.py      (from anywhere in the repo; exit status 1 on findings)
"""

import fnmatch
import re
import subprocess
import sys

PATH_PATTERNS = [
    (re.compile(rb"\b[A-Za-z]:[\\/]+(Users|code)[\\/]", re.IGNORECASE), "local path"),
    (re.compile(rb"(?<![\w.])/[a-z]/(Users|code)/", re.IGNORECASE), "local path (POSIX form)"),
    (re.compile(rb"AppData[\\/]+Local[\\/]+Temp", re.IGNORECASE), "temp folder"),
    (re.compile(rb"(?<![\\\w])\\\\[A-Za-z][\w.-]*\\[\w$]"), "network share"),
]

# (file, bytes the line contains): examples of what not to write, in the rules themselves.
ALLOWED = [
    ("CLAUDE.md", rb"no full local paths"),
    ("tools/repo_check/check_repo.py", rb""),
]

BINARY_SUFFIXES = (".bin", ".exe", ".png", ".ico", ".7z", ".zip", ".vsix")

# Output recorded from the old compiler or its programs: control bytes there are measured behaviour (NULs after
# `INF` in v02, a 0x01 in a qb64pe include message, the `-x` progress bar's CRs).
RECORDED = ["verification/*.out.txt", "vscode/test-fixtures/compiler/*.out.txt"]


def git(*args: str, stdin: bytes = b"", cwd: str | None = None) -> bytes:
    return subprocess.run(["git", *args], input=stdin, cwd=cwd, check=True, capture_output=True).stdout


def unchecked_text(root: str, paths: list[str]) -> set[str]:
    """Paths whose `text` attribute is unset (-text)."""
    out = git("check-attr", "--stdin", "-z", "text", stdin="\0".join(paths).encode(), cwd=root).split(b"\0")
    triples = zip(out[0::3], out[1::3], out[2::3])
    return {p.decode() for p, _, v in triples if v == b"unset"}


def allowed(path: str, line: bytes) -> bool:
    return any(path == f and text in line for f, text in ALLOWED)


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
    paths = [p for p in git("ls-files", "-z", cwd=root).decode().split("\0") if p]
    skip_bytes = unchecked_text(root, paths) if paths else set()
    findings = []
    for path in paths:
        try:
            data = open(f"{root}/{path}", "rb").read()
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
