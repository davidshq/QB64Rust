#!/usr/bin/env python3
"""Stop hook: Claude does not stop in the middle of an OpenSpec change.

When Claude is about to end its turn and a change worked on today still has unchecked tasks, the stop is
refused (exit 2) with the next open task as the reason, and Claude goes on. The stop is allowed when:

- the last message asks the user something (ends in a question, or `AskUserQuestion` was used), or hands over
  the files to stage ("to stage", "files to stage"), or says it is blocked ("blocked on", "cannot continue");
- no change has open tasks (the closing task does not count: `common.CLOSING_TASK`);
- this session has already been kept going `MAX_CONTINUES` times, which guards against a loop when a task can
  never be checked off (then the reason says so and Claude must tell the user).

Why: the loop of apply, review, fix and commit was driven by the user's prompts, one task group at a time, and a
prompt saying "do all the steps without stopping" was gone at the next /clear (`study\\41`).
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import open_tasks, read_input, refuse, state_dir

MAX_CONTINUES = 25
HANDOVER = re.compile(
    r"to stage\b|files to stage|blocked on|cannot continue|can't continue|waiting on the user|needs your decision",
    re.IGNORECASE,
)


def asks_the_user(message: str) -> bool:
    tail = message.rstrip()[-600:]
    return "?" in tail or bool(HANDOVER.search(tail))


def continues_so_far(session: str) -> int:
    f = state_dir() / f"stop-{session or 'nosession'}.count"
    try:
        return int(f.read_text().strip() or 0)
    except (OSError, ValueError):
        return 0


def record_continue(session: str, n: int) -> None:
    f = state_dir() / f"stop-{session or 'nosession'}.count"
    try:
        f.write_text(str(n))
    except OSError:
        pass


def decide(message: str, tasks: list[tuple[str, str]], so_far: int) -> str | None:
    """The reason to refuse the stop, or None to allow it."""
    if not tasks or asks_the_user(message):
        return None
    if so_far >= MAX_CONTINUES:
        return None
    name, text = tasks[0]
    return (
        f"Stop refused by tools/hooks/stop_continue.py ({so_far + 1} of {MAX_CONTINUES} this session): change "
        f"{name} has {len(tasks)} open task(s). Go on with the next one: {text!r}. Stop only to ask the user "
        "something, to hand over the files to stage, or when blocked (say on what)."
    )


def main() -> int:
    data = read_input()
    if "last_assistant_message" not in data:
        # Without the message the hook cannot tell a question to the user from a stop mid-change; it must not
        # block a question, so it lets the stop through (an older Claude Code, or an unexpected input).
        return 0
    session = re.sub(r"[^A-Za-z0-9_-]", "", str(data.get("session_id", "")))[:64]
    message = str(data.get("last_assistant_message", ""))
    so_far = continues_so_far(session)
    reason = decide(message, open_tasks(), so_far)
    if reason is None:
        return 0
    record_continue(session, so_far + 1)
    return refuse(reason)


if __name__ == "__main__":
    sys.exit(main())
