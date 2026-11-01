"""What the hooks share: the repo root and the answer to Claude.

A hook reads one JSON object on stdin (Claude Code's hook input) and answers with its exit code: 0 lets the
action through, 2 refuses it and sends stderr to Claude as the reason. Nothing here asks the user anything.
"""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path

ROOT = Path(os.environ.get("CLAUDE_PROJECT_DIR") or Path(__file__).resolve().parents[2]).resolve()


def read_input() -> dict:
    try:
        return json.load(sys.stdin)
    except (json.JSONDecodeError, OSError):
        return {}


def refuse(reason: str) -> int:
    """Refuses the action: the reason reaches Claude, not the user."""
    sys.stderr.write(reason.rstrip() + "\n")
    return 2
