#!/usr/bin/env python3
"""PreToolUse hook: refuses the actions the rules forbid, so that they do not depend on Claude remembering.

Refused (exit 2, the reason sent to Claude):

- Bash / PowerShell: staging and stashing (`git add`, `git rm --cached`, `git reset`, `git restore --staged`,
  `git stash`, `git commit -a`/`-am`/`--all`) (CLAUDE.md human rule 1); a plain `mv`/`Move-Item`/`rm`/`Remove-Item`
  of a tracked file (use `git mv`; a deletion is the user's call); a redirect that writes a file at the top of
  `target\\` (scratch goes under `target\\scratch\\`); a program list run, the call-site check or `close.py`
  while the active change still has open tasks (the close runs once, when the change is done).
- Write / Edit: a file at the top of `target\\` or under it outside `target\\scratch\\` and `target\\close\\`; any
  file under the read-only reference clone `..\\QB64pe\\`.

Allowed through untouched: everything else, including `--glob` runs and `cargo test -p` while working.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import ROOT, open_tasks, read_input, refuse

STAGING = re.compile(
    r"\bgit\s+(?:-C\s+\S+\s+)?(?:add|stash|reset|rm\s+(?:-r\s+)?--cached|restore\s+(?:\S+\s+)*--staged"
    r"|commit\s+(?:\S+\s+)*(?:-a|-am|--all))\b"
)
# A move or deletion by the shell: the first path argument is checked against `git ls-files`.
MOVE_OR_DELETE = re.compile(
    r"(?:^|[;&|]\s*)(?:mv|rm|Move-Item|Remove-Item|mi|ri|del|rmdir)\s+((?:-\S+\s+)*)(\"[^\"]+\"|'[^']+'|\S+)",
    re.MULTILINE,
)
# A redirect to a file directly under target/ (no further separator after `target/`).
TARGET_ROOT_WRITE = re.compile(
    r">{1,2}\s*\"?(?:\./)?target[\\/](?![\\/])([^\\/\s\"]+)\"?(?=\s|$)", re.MULTILINE
)
# The runs that belong to a close and must wait for it.
CLOSE_RUNS = re.compile(
    r"close\.py\s+--(?:plan|check)(?!.*--merge)|run_legacy_tests\.py[^\n]*--(?:list|pass-cache)\b"
    r"|tools[\\/]callsite[\\/]|tools[\\/]sandbox[\\/]run\.py|tools[\\/]linux[\\/]sync"
)
# Working runs the gate lets through even when they mention a list.
WORKING_RUN = re.compile(
    r"--glob\b|--check-list\b|--list-only\b|unittest|test_close|test_run_legacy_tests"
)
TARGET_ALLOWED = ("scratch", "close")


def tracked(path: str) -> bool:
    p = path.strip("\"'")
    try:
        r = subprocess.run(
            ["git", "ls-files", "--error-unmatch", "--", p],
            check=False,
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=10,
        )
    except (OSError, subprocess.SubprocessError):
        return False
    return r.returncode == 0


def check_command(cmd: str) -> str | None:
    """The reason to refuse a shell command, or None."""
    if STAGING.search(cmd):
        return (
            "Refused by tools/hooks/guard_tool.py: staging, stashing or resetting is the user's (CLAUDE.md human "
            "rule 1). Leave the change in the working tree and tell the user which files to stage."
        )
    for m in MOVE_OR_DELETE.finditer(cmd):
        if tracked(m.group(2)):
            return (
                f"Refused by tools/hooks/guard_tool.py: {m.group(2)} is tracked by git. Move or rename it with "
                "`git mv` (human rule 1); a deletion is the user's call: say which file and why."
            )
    m = TARGET_ROOT_WRITE.search(cmd)
    if m:
        return (
            f"Refused by tools/hooks/guard_tool.py: target/{m.group(1)} would be written at the top of target/. "
            "Scratch output goes under target/scratch/<date>/ (mkdir -p it first); the close's logs under "
            "target/close/."
        )
    if CLOSE_RUNS.search(cmd) and not WORKING_RUN.search(cmd):
        tasks = open_tasks()
        if tasks:
            name, text = tasks[0]
            return (
                f"Refused by tools/hooks/guard_tool.py: this is a closing run, and change {name} still has "
                f"{len(tasks)} open task(s), the first: {text!r}. The close runs once, when the change's tasks are "
                "done (CLAUDE.md working rule 6). While working, run `cargo test -p <crate>` and the change's own "
                "programs with --glob."
            )
    return None


def check_path(file_path: str) -> str | None:
    """The reason to refuse a Write or Edit of file_path, or None."""
    p = Path(file_path)
    if not p.is_absolute():
        p = ROOT / p
    try:
        p = p.resolve()
    except OSError:
        return None
    clone = (ROOT.parent / "QB64pe").resolve()
    if clone in p.parents:
        return "Refused by tools/hooks/guard_tool.py: ../QB64pe is the read-only reference clone (CLAUDE.md working rule 2)."
    target = (ROOT / "target").resolve()
    if target in p.parents:
        rel = p.relative_to(target).parts
        if len(rel) == 1 or rel[0] not in TARGET_ALLOWED:
            return (
                f"Refused by tools/hooks/guard_tool.py: {p.relative_to(ROOT).as_posix()} is under target/ but not "
                "under target/scratch/ or target/close/. Scratch files go under target/scratch/<date>/."
            )
    return None


def main() -> int:
    data = read_input()
    tool = data.get("tool_name", "")
    inp = data.get("tool_input") or {}
    reason = None
    if tool in ("Bash", "PowerShell"):
        reason = check_command(str(inp.get("command", "")))
    elif tool in ("Write", "Edit"):
        reason = check_path(str(inp.get("file_path", "")))
    return refuse(reason) if reason else 0


if __name__ == "__main__":
    sys.exit(main())
