"""What the hooks share: the repo root, the open tasks of the active OpenSpec changes, and the answer to Claude.

A hook reads one JSON object on stdin (Claude Code's hook input) and answers with its exit code: 0 lets the
action through, 2 refuses it and sends stderr to Claude as the reason. Nothing here asks the user anything.
"""

from __future__ import annotations

import json
import os
import re
import sys
import time
from pathlib import Path

ROOT = Path(os.environ.get("CLAUDE_PROJECT_DIR") or Path(__file__).resolve().parents[2]).resolve()
CHANGES = ROOT / "openspec" / "changes"
# A change whose files were last touched longer ago than this is not being worked on; its open tasks do not
# hold a session (a parked change, or one waiting on the user).
ACTIVE_SECONDS = 24 * 3600
UNCHECKED = re.compile(r"^\s*- \[ \] (.*)$")
# The closing task itself stays unchecked until the close has run; it never counts as open work.
CLOSING_TASK = re.compile(r"close\.py|closing runs", re.IGNORECASE)


def read_input() -> dict:
    try:
        return json.load(sys.stdin)
    except (json.JSONDecodeError, OSError):
        return {}


def refuse(reason: str) -> int:
    """Refuses the action: the reason reaches Claude, not the user."""
    sys.stderr.write(reason.rstrip() + "\n")
    return 2


def newest_mtime(folder: Path) -> float:
    newest = 0.0
    for p in folder.rglob("*"):
        try:
            if p.is_file():
                newest = max(newest, p.stat().st_mtime)
        except OSError:
            pass
    return newest


def open_tasks(now: float | None = None) -> list[tuple[str, str]]:
    """(change name, task text) for every unchecked task of a change worked on in the last day, the closing
    task left out. Archived changes (`archive\\`) are not looked at."""
    now = time.time() if now is None else now
    found: list[tuple[str, str]] = []
    if not CHANGES.is_dir():
        return found
    for change in sorted(CHANGES.iterdir()):
        if not change.is_dir() or change.name == "archive":
            continue
        tasks = change / "tasks.md"
        if not tasks.is_file():
            continue
        if now - newest_mtime(change) > ACTIVE_SECONDS:
            continue
        try:
            lines = tasks.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            continue
        for line in lines:
            m = UNCHECKED.match(line)
            if m and not CLOSING_TASK.search(m.group(1)):
                found.append((change.name, m.group(1).strip()))
    return found


def state_dir() -> Path:
    d = ROOT / "target" / "scratch" / "hooks"
    d.mkdir(parents=True, exist_ok=True)
    return d
