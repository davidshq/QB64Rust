#!/usr/bin/env python3
"""How much of the work on this project is spent running tests: every test run of every Claude
Code session, with how long it took, from the sessions' own transcripts.

    python tools/testtime/test_time.py                      # read the transcripts, write the files
    python tools/testtime/test_time.py --transcripts DIR    # another folder of *.jsonl transcripts

Claude Code keeps a transcript of each session on the machine it ran on (one .jsonl file, outside
this repo): every command Claude ran, with the time it was started and the time its result came
back. This script reads them and writes, beside itself:

  runs.csv      one row per command that is a test run, a measurement with the old compiler or a
                build (README.md, "Kinds"): session, start, bucket, kind, seconds, background,
                result, repeat_after
  sessions.csv  one row per session: start, end, active seconds, seconds inside commands
  changes.csv   one row per session and OpenSpec change: its times, waits and idle stretches
  activity.csv  one row per session, change and kind of work: actions and bytes written
  REPORT.md     the summary made from them: per day, per kind, repeats, the longest runs, per change,
                per kind of work

Nothing is measured by the tests themselves, so nothing has to be remembered while working: a run
is on record because it was run. The script only adds: rows of sessions whose transcript is gone
(Claude Code deletes old ones) stay in the CSV files. No command text, path or prompt is written,
only the kind of each run, its times and what is known of its result.
"""

from __future__ import annotations

import argparse
import bisect
import csv
import itertools
import json
import re
import statistics
import sys
from collections import defaultdict
from datetime import datetime, timedelta
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
RUNS, SESSIONS, REPORT = HERE / "runs.csv", HERE / "sessions.csv", HERE / "REPORT.md"
# Added 2026-10-16 (study\archive\31 item 4); the files above keep their columns and meaning.
CHANGES, ACTIVITY = HERE / "changes.csv", HERE / "activity.csv"
# The format of the two files: a later change to what a column means writes another number.
WORK_VERSION = "1"
CHANGE_FIELDS = [
    "version",
    "session",
    "change",
    "first",
    "last",
    "active_seconds",
    "test_seconds",
    "measure_seconds",
    "build_seconds",
    "background_seconds",
    "repeats",
    "longest_wait",
    "idle_stretches",
    "idle_seconds",
    "longest_idle",
    "last_code_edit",
    "archived",
]
ACTIVITY_FIELDS = ["version", "session", "change", "category", "actions", "bytes"]
RUN_FIELDS = [
    "session",
    "start",
    "bucket",
    "kind",
    "seconds",
    "background",
    "result",
    "repeat_after",
]

# A gap between two lines of a transcript that no command covers counts as work (Claude reading,
# thinking, writing) up to this long; a longer one is the user being away, and counts as nothing.
IDLE = timedelta(minutes=5)
# A command in the foreground is moved to the background when its time limit is reached: ten minutes,
# or the limit the command asked for when that is longer. One whose result came later than that
# (and a little, for the result to be written) was waiting for the user to allow it, for a time
# that is not known: it is left out, and so is its time.
LIMIT = timedelta(minutes=10)
SLACK = timedelta(seconds=15)
# How a Python script is started: the interpreter, its options (-X and -W take a value), the folder
# of the script. The interpreter is a word of its own ("callsite.py x.py" starts nothing), and the
# script's name follows the folder at once ("test_run_legacy_tests.py" is not "run_legacy_tests.py").
PY = r"(?<![\w./\\-])(python[\w.]*|py)\s+(-[XW]\s+\S+\s+|-\w+\s+)*(\S*[/\\\"'])?"
# How cargo is started, up to its subcommand: a word of its own. It is looked for outside quotes
# (`grep 'cargo test' README.md` and `git commit -m "run cargo test"` start nothing).
CARGO = r"(?<![\w./\\-])cargo\s+(\+\S+\s+)?"
QUOTED = re.compile(r"\"[^\"]*\"|'[^']*'")
# The kinds in the order `classify` gives them for one command; the runner's kinds come first.
ORDER = [
    "call-site",
    "tier 1",
    "clippy",
    "fmt",
    "tests of the Python tools",
    "extension tests",
    "repo check",
    "openspec validate",
    "cargo build",
    "old compiler run directly",
    "extension build",
]

SHELLS = {"Bash", "PowerShell"}
EDITS = {"Edit", "Write", "MultiEdit", "NotebookEdit"}
BACKGROUND_ID = re.compile(r"running in background with ID: (\w+)")
# A command that reached its time limit went on in the background.
MOVED_ID = re.compile(r"moved to the background \(ID: (\w+)\)")
NOTIFICATION = re.compile(r"<task-id>(\w+)</task-id>(.*?)(?=<task-id>|$)")
EXIT_CODE = re.compile(r"\(exit code (\d+)\)|with exit code (\d+)")
NOT_RUN = ("The user doesn't want to proceed with this tool use", "User rejected tool use")

TEST, MEASURE, BUILD = "test", "measure", "build"
PASS, FAIL = "pass", "fail"
OLD_DIRECT = "old compiler run directly"

HEREDOC = re.compile(r"(<<-?\s*['\"]?(\w+)['\"]?[^\n]*)\n.*?^\s*\2\s*$", re.DOTALL | re.MULTILINE)
HERE_STRING = re.compile(r"@(['\"])\s*\n.*?\n\1@", re.DOTALL)


def statements(command: str) -> list[str]:
    """The statements of a command line: text given to a program as its input (a heredoc, a
    PowerShell here-string) is no statement, and a line continued on the next is one."""
    c = HEREDOC.sub(r"\1", command)
    c = HERE_STRING.sub("", c)
    c = re.sub(r"[\\`]\r?\n", " ", c)
    return [s.strip() for s in re.split(r"\n|;|&&|\|\|", c) if s.strip()]


# A statement that only sets a shell variable (`R="python …"`, `export X=1`), or a PowerShell environment
# variable, or removes one.
ASSIGN = re.compile(r"(export\s+)?([A-Za-z_]\w*)=(\"[^\"]*\"|'[^']*'|\S*)")
ENV_SET = re.compile(r"\$env:(\w+)\s*=\s*(\"[^\"]*\"|'[^']*'|\S+)", re.IGNORECASE)
# Matched at the start of a statement: options may follow (`-ErrorAction SilentlyContinue`).
ENV_UNSET = re.compile(r"Remove-Item\s+(?:-Path\s+)?Env:\\?(\w+)|unset\s+(\w+)", re.IGNORECASE)
# What sets a PowerShell variable to nothing, which removes it.
NOTHING = {"", "$null"}
VARIABLE = re.compile(r"\$\{(\w+)\}|\$(\w+)")


def unquote(value: str) -> str:
    return value[1:-1] if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'" else value


def as_run(command: str) -> list[tuple[str, str]]:
    """(statement, what makes it the same run) for each statement of a command line that is not an
    assignment. A shell variable set by a statement of its own (`R="python …run_legacy_tests.py …"`)
    stands for its value in the statements after it (`$R --suite corpus`); the assignment runs nothing.
    An environment variable set by a statement of its own (`$env:QB64RUST_NO_FOLD='1'`, `export X=1`)
    is part of what makes the statements after it the same run, until it is removed: a run without
    folding is not a repeat of one with it."""
    values: dict[str, str] = {}
    env: dict[str, str] = {}
    out = []
    for s in statements(command):
        if m := ENV_SET.fullmatch(s):
            if unquote(m[2]).lower() in NOTHING:
                env.pop(m[1], None)
            else:
                env[m[1]] = unquote(m[2])
            continue
        if m := ENV_UNSET.match(s):
            env.pop(m[1] or m[2], None)
            continue
        if m := ASSIGN.fullmatch(s):
            values[m[2]] = unquote(m[3])
            if m[1]:
                env[m[2]] = values[m[2]]
            continue
        s = VARIABLE.sub(lambda v: values.get(v[1] or v[2], v[0]), s)
        out.append((s, "".join(f"{k}={v} " for k, v in sorted(env.items())) + s))
    return out


def classify_statement(c: str) -> list[tuple[str, str]]:
    """The (bucket, kind) of what one statement runs, in the order of the rules below."""
    found: list[tuple[str, str]] = []

    def has(pattern: str) -> bool:
        return re.search(pattern, c) is not None

    # What is started, without what its output is piped into: an option there is another program's.
    head = c.split("|")[0]
    bare = QUOTED.sub('""', head)
    # Asking a tool for its options runs nothing.
    asks_help = re.search(r"\s(--help|-h)(\s|$)", head) is not None
    runner = has(PY + r"run_legacy_tests\.py") and not asks_help
    if runner or has(r"(^|\b(sh|bash)\s+)\S*verification[/\\]run\.sh"):
        new = has(r"--qb64\s+\S*qb64rust")
        if has(r"--check-list\b"):
            # Runs nothing: judges a pass list from the last full run's results.
            found.append((TEST, "pass list checked from a full run"))
        elif has(r"--suite\s+verification|run\.sh"):
            found.append((MEASURE, "verification programs (old compiler)"))
        elif has(r"--record"):
            found.append((MEASURE, "recording expected results (old compiler)"))
        elif has(r"--suite\s+corpus") or not has(r"--suite"):
            if has(r"--corpus-root\s+\S*differential"):
                kind = "differential pass list"
            elif has(r"--list\s+\S*slice\.list"):
                kind = "slice list"
            elif has(r"--list|--glob|--category|--corpus-root"):
                # Another corpus root is a few programs made for a trial, not the corpus.
                kind = "corpus, selected programs"
            else:
                kind = "full corpus"
            found.append((TEST if new else MEASURE, kind if new else kind + " (old compiler)"))
        elif has(r"--suite\s+compile"):
            if has(r"--list\s+\S*pass\.list"):
                kind = "upstream pass list"
            elif has(r"--list|--glob|--category"):
                kind = "upstream, selected programs"
            else:
                kind = "full upstream suite"
            found.append((TEST if new else MEASURE, kind if new else kind + " (old compiler)"))
        else:
            found.append(
                (TEST if new else MEASURE, "other suite" if new else "other suite (old compiler)")
            )
    if has(PY + r"callsite\.py") and not asks_help:
        found.append((TEST, "call-site self-test" if has(r"--self-test") else "call-site check"))

    def cargo(pattern: str) -> bool:
        return re.search(CARGO + pattern, bare) is not None

    if cargo(r"(insta\s+)?test\b"):
        # `--test-threads` is an option of the tests, not a choice of them.
        part = cargo(r"test\b.*\s(-p|--test|--lib)(\s|=|$)")
        found.append((TEST, "tier 1, part (cargo test -p)" if part else "tier 1 (cargo test)"))
    if cargo(r"clippy\b"):
        found.append((TEST, "clippy"))
    if cargo(r"fmt\b"):
        found.append((TEST, "fmt"))
    # A test file started by itself; this script is named like one and is none.
    if has(r"-m\s+unittest|\bpytest\b") or has(PY + r"test_(?!time\.py)\w+\.py"):
        found.append((TEST, "tests of the Python tools"))
    if has(r"\bnpm\s+(run\s+)?(test|lint|test:\w+)\b|vscode-test|\bnpx\s+(mocha|eslint|tsc)\b"):
        found.append((TEST, "extension tests (npm)"))
    if has(PY + r"check_repo\.py") and not asks_help:
        found.append((TEST, "repo check"))
    if has(r"openspec\s+validate"):
        found.append((TEST, "openspec validate"))
    if cargo(r"(build|check)\b"):
        found.append((BUILD, "cargo build"))
    if has(r"qb64pe(\.exe)?['\")]*\s+-[xzyqm]\b") and not found:
        found.append((MEASURE, OLD_DIRECT))
    if has(r"\bnpm\s+run\s+(compile|build|package)\b|\bvsce\s+package\b"):
        found.append((BUILD, "extension build (npm)"))
    return found


def classify_parts(command: str) -> list[tuple[str, str, str]]:
    """(bucket, kind, statement) of each thing a command runs that is on record; the statement as it ran,
    with the environment variables set before it (`as_run`)."""
    found = [(b, k, same) for s, same in as_run(command) for b, k in classify_statement(s)]
    # A build before a `cargo test` of the same line is part of that test run.
    if any(k.startswith("tier 1") for _, k, _ in found):
        found = [f for f in found if f[1] != "cargo build"]
    # The old compiler started by hand is on record once for a command, and only when the command
    # runs nothing else that is.
    direct = [f for f in found if f[1] == OLD_DIRECT]
    others = [f for f in found if f[1] != OLD_DIRECT]

    def place(f) -> int:
        return next((i for i, start in enumerate(ORDER, 1) if f[1].startswith(start)), 0)

    return sorted(others, key=place) or direct[:1]


def classify(command: str) -> list[tuple[str, str]]:
    """The (bucket, kind) of each thing a command runs that is on record; empty for any other
    command. A command that runs several (fmt, clippy and tests in one line) gives several; its
    time is then split between them (`share_out`)."""
    return [(b, k) for b, k, _ in classify_parts(command)]


def said(kind: str, text: str, shared: bool = False) -> str:
    """What the output of a run says of its result: PASS, FAIL, or '' when it says nothing.
    `shared`: the output is that of several runs of one line."""

    def has(pattern: str) -> bool:
        return re.search(pattern, text, re.MULTILINE) is not None

    summaries = [(int(p), int(f)) for p, f in re.findall(r'\{"PASS": (\d+), "FAIL": (\d+)', text)]
    if (
        "corpus" in kind
        or "upstream" in kind
        or "differential" in kind
        or "slice" in kind
        or "suite" in kind
        or "pass list" in kind
    ):
        if any(f for _, f in summaries) or has(r"^\[\d+/\d+\] FAIL "):
            return FAIL
        return PASS if summaries else ""
    if kind == "call-site check":
        counts = re.findall(r"\d+ programs: \d+ equal, (\d+) differ, (\d+) not compared", text)
        if any(int(a) or int(b) for a, b in counts):
            return FAIL
        return PASS if counts else ""
    if kind == "call-site self-test":
        counts = re.findall(r"self-test: .*?(\d+) failures", text)
        return "" if not counts else FAIL if any(int(n) for n in counts) else PASS
    if kind.startswith("tier 1"):
        if has(r"test result: FAILED|error: test failed|\.\.\. FAILED|panicked at"):
            return FAIL
        if has(r"test result: ok"):
            return PASS
        # No line of a test is shown. A compile error is the tests' own when they are the only run
        # of the line; beside a clippy or build run it may be that run's, with the tests passing
        # and their output cut down to nothing.
        return FAIL if not shared and has(r"could not compile|^error(\[E\d+\])?:") else ""
    if kind in ("clippy", "cargo build"):
        if has(r"could not compile|^error(\[E\d+\])?:") or (
            kind == "clippy" and has(r"^warning: (?!build failed)")
        ):
            return FAIL
        return PASS if has(r"^\s*Finished ") else ""
    if kind == "fmt":
        return FAIL if has(r"^Diff in ") else ""
    if kind == "repo check":
        counts = re.findall(r"\d+ files checked, (\d+) findings", text)
        return "" if not counts else FAIL if any(int(n) for n in counts) else PASS
    if kind == "openspec validate":
        if has(r"is invalid|has issues|\bfailed\b"):
            return FAIL
        return PASS if has(r"is valid") else ""
    if kind == "tests of the Python tools":
        if has(r"^FAILED \(|^Traceback "):
            return FAIL
        return PASS if has(r"^OK\b") else ""
    if kind == "extension tests (npm)":
        if has(r"\d+ failing|\d+ problems? \("):
            return FAIL
        return PASS if has(r"\d+ passing") else ""
    return ""


# The OpenSpec change a moment of work belongs to (README.md, "Changes"). A folder of a change, not of
# the archive: reading an archived change is not working on it.
CHANGE_PATH = re.compile(
    r"openspec[/\\]+changes[/\\]+(?!archive(?![\w-]))([a-z0-9][a-z0-9-]*)(?![\w.-])"
)
CHANGE_NAME = re.compile(r"[a-z0-9][a-z0-9-]*")
OPENSPEC = re.compile(r"(?<![\w./\\-])openspec\s+(.*)")
# A change archived by moving its folder (as some were before `openspec archive` was used).
MOVED_TO_ARCHIVE = re.compile(
    r"(?<![\w-])(?:mv|Move-Item)\s.*archive[/\\]+\d{4}-\d\d-\d\d-([a-z0-9][a-z0-9-]*)",
    re.IGNORECASE,
)
# The fields of a tool's input that name a file or a folder.
PATH_FIELDS = ("file_path", "notebook_path", "path")
NO_CHANGE = ""


def changes_named(command: str) -> tuple[list[str], list[str]]:
    """The changes a command names, and of those the ones it archives: a path of a change's folder,
    `openspec new change NAME`, `openspec archive NAME`, `openspec validate NAME`, `--change NAME`."""
    named: list[str] = []
    archived: list[str] = []
    for s in statements(command):
        head = s.split("|")[0]
        named += CHANGE_PATH.findall(head)
        if m := MOVED_TO_ARCHIVE.search(head):
            named.append(m[1])
            archived.append(m[1])
            continue
        m = OPENSPEC.search(head)
        if not m:
            continue
        words = [unquote(w) for w in m[1].split()]
        name = ""
        if "--change" in words[:-1]:
            name = words[words.index("--change") + 1]
        elif words[:2] == ["new", "change"] or words[:1] in (["archive"], ["validate"]):
            rest = words[2:] if words[0] == "new" else words[1:]
            name = next((w for w in rest if not w.startswith("-")), "")
        if CHANGE_NAME.fullmatch(name):
            named.append(name)
            if words[0] == "archive":
                archived.append(name)
    return named, archived


def changes_in(tool: str, arg: dict) -> list[str]:
    """The changes a tool's input names: by a path of a change's folder, or by the command."""
    if tool in SHELLS:
        return changes_named(str(arg.get("command") or ""))[0]
    named = []
    for field in (*PATH_FIELDS, "pattern" if tool == "Glob" else ""):
        value = arg.get(field) if field else None
        if isinstance(value, str):
            named += CHANGE_PATH.findall(value)
    return named


# Kinds of work, by what a tool did (README.md, "Kinds of work"). Counts of actions and bytes
# written, not time: the transcript does not say how long a piece of writing took.
PRODUCT, TESTS, PREPARATION, DOCUMENTS, GENERATED = (
    "product code",
    "tests and fixtures",
    "measurement preparation",
    "documents and planning",
    "generated files",
)
SCRATCH = "scratch files"
READING, TEST_RUNS, MEASURE_RUNS, BUILDS = "reading", "test runs", "measurement runs", "builds"
OTHER_COMMANDS, OTHER = "other commands", "other"
CATEGORIES = [
    PRODUCT,
    TESTS,
    PREPARATION,
    DOCUMENTS,
    GENERATED,
    SCRATCH,
    READING,
    TEST_RUNS,
    MEASURE_RUNS,
    BUILDS,
    OTHER_COMMANDS,
    OTHER,
]
READING_TOOLS = {"Read", "Grep", "Glob", "LSP", "WebFetch", "WebSearch", "ToolSearch"}


def file_category(path: str) -> str:
    """What kind of file an edit wrote, by its path (the first rule that fits)."""
    p = "/" + path.replace("\\", "/").lower()
    # Scripts and programs written to a temporary folder for one task (an edit, a trial, a measurement).
    if re.search(r"/te?mp/", p):
        return SCRATCH
    if re.search(
        r"\.(snap|output|err|screen)$|/tools/testtime/(runs|sessions|changes|activity)\.csv$"
        r"|/tools/testtime/report\.md$|/builtins\.json$|/old_wording\.tsv$",
        p,
    ):
        return GENERATED
    if "/verification/" in p:
        return PREPARATION
    if re.search(
        r"/crates/[^/]+/tests/|/tests/|/test_[^/]*\.py$|/vscode/src/test/|/tools/|/\.github/", p
    ):
        return TESTS
    if re.search(r"/crates/|/vscode/|\.(rs|cpp|h|ts|toml)$", p):
        return PRODUCT
    if re.search(r"\.(md|ya?ml)$|/openspec/|/study/", p):
        return DOCUMENTS
    return OTHER


def written(tool: str, arg: dict) -> int:
    """The bytes an edit wrote: what it put in (a whole file, or each replacement)."""
    if tool == "Write":
        return len(str(arg.get("content") or "").encode("utf-8"))
    if tool == "MultiEdit":
        edits = arg.get("edits") or []
        return sum(
            len(str(e.get("new_string") or "").encode("utf-8"))
            for e in edits
            if isinstance(e, dict)
        )
    return len(str(arg.get("new_string") or arg.get("new_source") or "").encode("utf-8"))


def category(tool: str, arg: dict) -> str:
    """The kind of work a tool call is."""
    if tool in EDITS:
        path = next((arg[f] for f in PATH_FIELDS if isinstance(arg.get(f), str)), "")
        return file_category(path)
    if tool in SHELLS:
        buckets = {b for b, _ in classify(str(arg.get("command") or ""))}
        if TEST in buckets:
            return TEST_RUNS
        if MEASURE in buckets:
            return MEASURE_RUNS
        return BUILDS if BUILD in buckets else OTHER_COMMANDS
    return READING if tool in READING_TOOLS else OTHER


def result_of(
    bucket: str, kind: str, text: str, exit_code: int | None, alone: bool, shared: bool = False
) -> str:
    """PASS, FAIL or '' (not known) for a run. `text` is its output as the transcript has it;
    `exit_code` that of the whole command (None: not known); `alone`: the command is this one
    statement, with no pipe after it, so the exit code is the run's own; `shared`: the command
    started several runs, which have one output."""
    if bucket == MEASURE:
        return ""  # finding out what the old compiler does has no pass or fail
    by_output = said(kind, text, shared)
    if by_output:
        return by_output
    if alone and exit_code is not None:
        return PASS if exit_code == 0 else FAIL
    return ""


def parse_time(text: str) -> datetime:
    # Python before 3.11 does not read the "Z" of a transcript's times.
    return datetime.fromisoformat(text.replace("Z", "+00:00"))  # noqa: FURB162


def text_of(content) -> str:
    """The text of a tool result: a string, or a list of text parts."""
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        return "\n".join(
            i.get("text", "")
            for i in content
            if isinstance(i, dict) and isinstance(i.get("text"), str)
        )
    return ""


def default_folders() -> list[Path]:
    """The transcript folders of sessions started in this repo or in the folder above it (which
    has the same CLAUDE.md): Claude Code names a project's folder after its path, every character
    that is no letter or digit turned into '-'."""
    base = Path.home() / ".claude" / "projects"
    wanted = {re.sub(r"[^A-Za-z0-9]", "-", str(p)).lower() for p in (REPO, REPO.parent)}
    if not base.is_dir():
        return []
    return [d for d in base.iterdir() if d.is_dir() and d.name.lower() in wanted]


def read_session(path: Path, work: dict | None = None):
    """One transcript: its runs and its row for sessions.csv; None when it holds no time. With
    `work`, also its rows for changes.csv and activity.csv, as work["changes"] and work["activity"]."""
    times: list[datetime] = []
    actions: list[tuple[datetime, str, str, dict]] = []  # time, tool_use id, tool, its input
    # tool_use id: start, command, background, files edited before it, the time limit asked for
    calls: dict[str, tuple[datetime, str, bool, int, timedelta]] = {}
    ends: dict[str, datetime] = {}
    outputs: dict[
        str, tuple[str, bool]
    ] = {}  # tool_use id: text of the result, whether it is an error
    background: dict[str, str] = {}  # background task id: tool_use id
    moved: dict[str, str] = {}  # the same, for a command that reached its time limit
    notified: dict[str, datetime] = {}
    exit_codes: dict[str, int] = {}  # background task id: exit code
    edits = 0
    session = path.stem
    with path.open(encoding="utf-8", errors="replace") as f:
        for raw in f:
            try:
                d = json.loads(raw)
            except json.JSONDecodeError:
                continue
            stamp = d.get("timestamp")
            if not isinstance(stamp, str):
                continue
            try:
                t = parse_time(stamp)
            except ValueError:
                continue
            times.append(t)
            session = d.get("sessionId") or session
            if "<task-id>" in raw:
                for task, rest in NOTIFICATION.findall(raw):
                    if task not in notified:
                        notified[task] = t
                        code = EXIT_CODE.search(rest)
                        if code:
                            exit_codes[task] = int(code.group(1) or code.group(2))
            content = (d.get("message") or {}).get("content")
            if not isinstance(content, list):
                continue
            for item in content:
                if not isinstance(item, dict):
                    continue
                if item.get("type") == "tool_use":
                    arg = item.get("input") or {}
                    # A line the transcript holds twice is one action.
                    known = item.get("id") in calls
                    if isinstance(arg, dict) and isinstance(item.get("name"), str) and not known:
                        actions.append((t, item.get("id", ""), item["name"], arg))
                    if item.get("name") in EDITS:
                        edits += 1
                    command = arg.get("command") if item.get("name") in SHELLS else None
                    asked = arg.get("timeout")
                    calls[item.get("id", "")] = (
                        t,
                        command or "",
                        bool(arg.get("run_in_background")),
                        edits,
                        timedelta(milliseconds=asked) if isinstance(asked, (int, float)) else LIMIT,
                    )
                elif item.get("type") == "tool_result":
                    use = item.get("tool_use_id", "")
                    if use in ends:
                        continue
                    ends[use] = t
                    text = text_of(item.get("content"))
                    outputs[use] = (text, bool(item.get("is_error")))
                    m = BACKGROUND_ID.search(text)
                    if m:
                        background[m.group(1)] = use
                    m = MOVED_ID.search(text)
                    if m:
                        moved[m.group(1)] = use
    if not times:
        return None
    times.sort()
    task_of = {use: task for task, use in list(background.items()) + list(moved.items())}
    # Seconds inside commands of any kind, and the gaps outside them that are short enough to be work.
    waited_for_user = {
        use
        for use, (start, _, flag, _, asked) in calls.items()
        if use in ends
        and not flag
        and use not in background.values()
        and ends[use] - start > max(LIMIT, asked) + SLACK
    }
    spans = sorted(
        (start, ends[use])
        for use, (start, *_) in calls.items()
        if use in ends and use not in waited_for_user
    )
    covered = timedelta()
    merged: list[list[datetime]] = []
    for start, end in spans:
        if merged and start <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], end)
        else:
            merged.append([start, end])
    for start, end in merged:
        covered += end - start
    active = covered
    for a, b in itertools.pairwise(times):
        if b - a <= IDLE and not any(s <= a and b <= e for s, e in merged):
            active += b - a
    key = session[:8]
    runs = []
    last: dict[
        tuple[str, str], tuple[int, str]
    ] = {}  # (kind, statement): files edited before it, its result
    for use, (start, command, flag, edited, _) in calls.items():
        parts = classify_parts(command) if command else []
        if not parts or use in waited_for_user or use not in ends:
            continue
        text, is_error = outputs[use]
        if text.startswith(NOT_RUN):
            continue
        task = task_of.get(use)
        in_background = flag or use in background.values()
        # Each stretch of the command: its end, whether it was waited for.
        stretches = []
        if in_background:
            if task in notified:
                stretches.append((start, notified[task], True))
        else:
            stretches.append((start, ends[use], False))
            if use in moved.values() and task in notified:
                stretches.append((ends[use], notified[task], True))
        if not stretches:
            continue  # the session ended before the run did
        if task is not None:
            output, exit_code = "", exit_codes.get(task)
        else:
            code = re.match(r"(?:Error: )?Exit code (\d+)", text)
            output, exit_code = text, int(code.group(1)) if code else 1 if is_error else 0
        alone = len(statements(command)) == 1 and "|" not in command
        for bucket, kind, statement in parts:
            result = result_of(bucket, kind, output, exit_code, alone, len(parts) > 1)
            # The same statement run again with no file edited since (by the Edit or Write tool: a
            # file changed by a shell command is not seen).
            same = (kind, re.sub(r"\s+", " ", statement.split("|")[0]).strip())
            before = last.get(same)
            repeat_after = (before[1] or "unknown") if before and before[0] == edited else ""
            last[same] = (edited, result)
            for number, (a, b, behind) in enumerate(stretches):
                runs.append(
                    {
                        "at": a,
                        "session": key,
                        "start": a.astimezone().strftime("%Y-%m-%d %H:%M:%S"),
                        "bucket": bucket,
                        "kind": kind,
                        "seconds": f"{max((b - a).total_seconds(), 0.0) / len(parts):.1f}",
                        "background": "1" if behind else "0",
                        # A run that went on in the background has its result on that row.
                        "result": result if number == len(stretches) - 1 else "",
                        "repeat_after": repeat_after if number == 0 else "",
                        "group": (key, use, number),
                        "total": max((b - a).total_seconds(), 0.0),
                    }
                )
    row = {
        "session": key,
        "start": times[0].astimezone().strftime("%Y-%m-%d %H:%M:%S"),
        "end": times[-1].astimezone().strftime("%Y-%m-%d %H:%M:%S"),
        "active_seconds": f"{active.total_seconds():.0f}",
        "command_seconds": f"{covered.total_seconds():.0f}",
    }
    share_out(runs)
    if work is not None:
        archived = [
            (ends[use], name)
            for _, use, tool, arg in actions
            if tool in SHELLS
            and use in ends
            and not outputs[use][1]
            and not outputs[use][0].startswith(NOT_RUN)
            for name in changes_named(str(arg.get("command") or ""))[1]
        ]
        work["changes"], work["activity"] = work_rows(key, times, merged, actions, archived, runs)
    for r in runs:
        del r["at"]
    return runs, row


def local(t: datetime) -> str:
    return t.astimezone().strftime("%Y-%m-%d %H:%M:%S")


def work_rows(key: str, times, merged, actions, archived, runs) -> tuple[list[dict], list[dict]]:
    """The rows of one session for changes.csv and activity.csv. Each moment belongs to the change
    named last before it in the session (`changes_in`); before the first, to none (NO_CHANGE). A gap
    between two lines of the transcript is active time by the rule of sessions.csv (inside a command,
    or no longer than IDLE); a longer one is an idle stretch of the change it falls in."""
    # Once a change is archived, the work after it belongs to none until another is named.
    marks = sorted(
        [(t, 0, name) for t, _, tool, arg in actions for name in changes_in(tool, arg)]
        + [(t, 1, NO_CHANGE) for t, _ in archived]
    )
    stamps = [t for t, _, _ in marks]

    def change_at(t: datetime) -> str:
        i = bisect.bisect_right(stamps, t)
        return marks[i - 1][2] if i else NO_CHANGE

    rows: dict[str, dict] = {}

    def row(change: str, t: datetime) -> dict:
        if change not in rows:
            rows[change] = {
                "version": WORK_VERSION,
                "session": key,
                "change": change,
                "first": t,
                "last": t,
                **dict.fromkeys(CHANGE_FIELDS[5:15], 0.0),
                "last_code_edit": None,
                "archived": None,
            }
        r = rows[change]
        r["first"], r["last"] = min(r["first"], t), max(r["last"], t)
        return r

    for a, b in itertools.pairwise(times):
        r = row(change_at(a), a)
        gap = (b - a).total_seconds()
        if b - a <= IDLE or any(s <= a and b <= e for s, e in merged):
            r["active_seconds"] += gap
            r["last"] = max(r["last"], b)
        else:
            r["idle_stretches"] += 1
            r["idle_seconds"] += gap
            r["longest_idle"] = max(r["longest_idle"], gap)
    for run in runs:
        r = row(change_at(run["at"]), run["at"])
        seconds = float(run["seconds"])
        if run["background"] == "1":
            r["background_seconds"] += seconds
        else:
            r[f"{run['bucket']}_seconds"] += seconds
            r["longest_wait"] = max(r["longest_wait"], seconds)
        r["repeats"] += 1 if run.get("repeat_after") else 0
    for t, name in archived:
        r = row(name, t)
        r["archived"] = max(r["archived"] or t, t)
    activity: dict[tuple[str, str], list[int]] = defaultdict(lambda: [0, 0])
    for t, _, tool, arg in actions:
        change, kind = change_at(t), category(tool, arg)
        row(change, t)
        counts = activity[(change, kind)]
        counts[0] += 1
        counts[1] += written(tool, arg) if tool in EDITS else 0
        if kind == PRODUCT:
            rows[change]["last_code_edit"] = max(rows[change]["last_code_edit"] or t, t)
    out = []
    for r in rows.values():
        out.append(
            {
                **r,
                **{f: f"{r[f]:.0f}" for f in CHANGE_FIELDS[5:15]},
                "first": local(r["first"]),
                "last": local(r["last"]),
                "last_code_edit": local(r["last_code_edit"]) if r["last_code_edit"] else "",
                "archived": local(r["archived"]) if r["archived"] else "",
            }
        )
    acts = [
        {
            "version": WORK_VERSION,
            "session": key,
            "change": change,
            "category": kind,
            "actions": str(n),
            "bytes": str(size),
        }
        for (change, kind), (n, size) in sorted(activity.items())
    ]
    return out, acts


def share_out(runs: list[dict]) -> None:
    """Splits the time of a command that started several runs (fmt, clippy and tests in one line)
    between them in proportion to what each kind takes when run alone (the median of those runs);
    the even split stays where that is not known for one of them. `runs` are those of one
    session: what a kind takes is taken from that session alone, so the rows of a session that
    is over do not change when later sessions are read."""
    groups: dict[tuple, list[dict]] = defaultdict(list)
    for r in runs:
        groups[r["group"]].append(r)
    alone: dict[str, list[float]] = defaultdict(list)
    for members in groups.values():
        if len(members) == 1 and members[0]["background"] == "0":
            alone[members[0]["kind"]].append(members[0]["total"])
    usual = {kind: statistics.median(values) for kind, values in alone.items()}
    for members in groups.values():
        weights = [usual.get(r["kind"], 0.0) for r in members]
        if not all(weights):
            weights = [1.0] * len(members)
        for r, w in zip(members, weights):
            r["seconds"] = f"{r['total'] * w / sum(weights):.1f}"
    for r in runs:
        del r["group"], r["total"]


def load(path: Path) -> list[dict]:
    if not path.is_file():
        return []
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def save(path: Path, rows: list[dict], fields: list[str]) -> None:
    with path.open("w", newline="\n", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=fields, lineterminator="\n", restval="")
        w.writeheader()
        w.writerows(rows)


def clock(seconds: float) -> str:
    """Hours and minutes, or minutes and seconds below an hour."""
    s = round(seconds)
    if s >= 3600:
        return f"{s // 3600} h {s % 3600 // 60:02d} min"
    if s >= 60:
        return f"{s // 60} min {s % 60:02d} s"
    return f"{s} s"


def share(part: float, whole: float) -> str:
    return f"{100 * part / whole:.0f}%" if whole else "-"


def stamp(text: str) -> datetime:
    return datetime.strptime(text, "%Y-%m-%d %H:%M:%S")


def archived_folders() -> set[str]:
    """The names of the changes in this repo's archive (`2026-10-14-m3-x` is `m3-x`)."""
    folder = REPO / "openspec" / "changes" / "archive"
    return {p.name[11:] for p in folder.iterdir() if p.is_dir()} if folder.is_dir() else set()


def by_change(changes: list[dict], in_archive: set[str] = frozenset()) -> list[str]:
    """The table of changes.csv's rows summed per change, in the order the changes were started.
    `in_archive`: changes whose folder is in the archive, archived where no transcript shows it."""
    groups: dict[str, list[dict]] = defaultdict(list)
    for c in changes:
        groups[c["change"]].append(c)

    def num(rows, field, how=sum) -> float:
        return how([float(r[field] or 0) for r in rows] or [0.0])

    out = [
        (
            "| Change | State | Sessions | Started | Ended | Elapsed | Active | Waiting: tests | measurements | builds "
            "| In the background | Repeats | Longest wait | Idle stretches | Longest idle | Closing stretch |"
        ),
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|",
    ]
    named = sorted(
        (n for n in groups if n != NO_CHANGE), key=lambda n: min(r["first"] for r in groups[n])
    )
    for name in [*named, NO_CHANGE] if NO_CHANGE in groups else named:
        rows = groups[name]
        first = min(r["first"] for r in rows)
        archived = max((r["archived"] for r in rows if r.get("archived")), default="")
        end = archived or max(r["last"] for r in rows)
        code = max((r["last_code_edit"] for r in rows if r.get("last_code_edit")), default="")
        closing = (
            clock((stamp(archived) - stamp(code)).total_seconds())
            if archived and code and code < archived
            else ""
        )
        idle = num(rows, "idle_seconds")
        state = (
            ""
            if not name
            else "archived"
            if archived
            else "archived, not seen"
            if name in in_archive
            else "open"
        )
        out.append(
            f"| {name or '(no change named)'} | {state} "
            f"| {len({r['session'] for r in rows})} | {first[:16]} | {end[:16]}{'' if archived or not name else ' (last seen)'} "
            f"| {clock((stamp(end) - stamp(first)).total_seconds()) if name else ''} "
            f"| {clock(num(rows, 'active_seconds'))} | {clock(num(rows, 'test_seconds'))} "
            f"| {clock(num(rows, 'measure_seconds'))} | {clock(num(rows, 'build_seconds'))} "
            f"| {clock(num(rows, 'background_seconds'))} | {num(rows, 'repeats'):.0f} "
            f"| {clock(num(rows, 'longest_wait', max))} "
            f"| {num(rows, 'idle_stretches'):.0f} ({clock(idle)}) | {clock(num(rows, 'longest_idle', max))} "
            f"| {closing} |"
        )
    return out


def by_kind_of_work(activity: list[dict]) -> list[str]:
    """Actions and bytes written per kind of work, in all and per change."""
    totals: dict[str, list[int]] = defaultdict(lambda: [0, 0])
    per_change: dict[str, dict[str, int]] = defaultdict(lambda: defaultdict(int))
    for a in activity:
        totals[a["category"]][0] += int(a["actions"])
        totals[a["category"]][1] += int(a["bytes"])
        per_change[a["change"]][a["category"]] += int(a["actions"])
    edits = sum(totals[c][1] for c in CATEGORIES)
    out = [
        "| Kind of work | Actions | Bytes written | Share of bytes written |",
        "|---|---|---|---|",
    ]
    for c in CATEGORIES:
        if c in totals:
            out.append(
                f"| {c} | {totals[c][0]} | {totals[c][1]} | {share(totals[c][1], edits) if totals[c][1] else ''} |"
            )
    columns = [c for c in CATEGORIES if c in totals]
    out += [
        "",
        "Actions per change:",
        "",
        "| Change | " + " | ".join(columns) + " |",
        "|---|" + "---|" * len(columns),
    ]
    for name in sorted(per_change, key=lambda n: (n == NO_CHANGE, n)):
        counts = per_change[name]
        out.append(
            f"| {name or '(no change named)'} | "
            + " | ".join(str(counts.get(c, 0)) for c in columns)
            + " |"
        )
    return out


def report(
    runs: list[dict],
    sessions: list[dict],
    changes: list[dict] = (),
    activity: list[dict] = (),
    in_archive: set[str] = frozenset(),
) -> str:
    def secs(rows, bucket=None, background=None):
        return sum(
            float(r["seconds"])
            for r in rows
            if (bucket is None or r["bucket"] == bucket)
            and (background is None or (r["background"] == "1") == background)
        )

    active = sum(float(s["active_seconds"]) for s in sessions)
    commands = sum(float(s["command_seconds"]) for s in sessions)
    out = [
        "# Time spent testing",
        "",
        "Made by `python tools\\testtime\\test_time.py` from the transcripts of the Claude Code sessions; do not edit.",
        "What the columns mean, and what the numbers cannot say: `README.md` beside this file.",
        "",
    ]
    if not sessions:
        return "\n".join(out + ["No session on record.", ""])
    first, last = min(s["start"] for s in sessions)[:10], max(s["end"] for s in sessions)[:10]
    waited = {b: secs(runs, b, False) for b in (TEST, MEASURE, BUILD)}
    out += [
        f"{len(sessions)} sessions from {first} to {last}; {len(runs)} runs on record.",
        "",
        "## In all",
        "",
        "| | Time | Share of active time |",
        "|---|---|---|",
        f"| Active time (Claude working, the user not waited for) | {clock(active)} | |",
        f"| Inside commands of any kind | {clock(commands)} | {share(commands, active)} |",
        f"| **Waiting for test runs** | {clock(waited[TEST])} | {share(waited[TEST], active)} |",
        f"| Waiting for measurements with the old compiler | {clock(waited[MEASURE])} | {share(waited[MEASURE], active)} |",
        f"| Waiting for builds | {clock(waited[BUILD])} | {share(waited[BUILD], active)} |",
        (
            f"| Everything else (reading, thinking, writing code and documents, other commands) | "
            f"{clock(active - sum(waited.values()))} | {share(active - sum(waited.values()), active)} |"
        ),
        f"| Test runs and measurements in the background (not waited for) | {clock(secs(runs, None, True))} | |",
        "",
        "## By day",
        "",
        "| Day | Sessions | Active | Tests | Measurements | Builds | Tests, share | Test runs | In the background |",
        "|---|---|---|---|---|---|---|---|---|",
    ]
    days = sorted({s["start"][:10] for s in sessions} | {r["start"][:10] for r in runs})
    for day in days:
        ss = [s for s in sessions if s["start"][:10] == day]
        rr = [r for r in runs if r["start"][:10] == day]
        a = sum(float(s["active_seconds"]) for s in ss)
        t = secs(rr, TEST, False)
        count = sum(1 for r in rr if r["bucket"] == TEST)
        out.append(
            f"| {day} | {len(ss)} | {clock(a)} | {clock(t)} | {clock(secs(rr, MEASURE, False))} | "
            f"{clock(secs(rr, BUILD, False))} | {share(t, a)} | {count} | {clock(secs(rr, None, True))} |"
        )
    out += [
        "",
        "A day's runs are those started on it; a session that runs past midnight has its active time on the day it",
        "started, so a share above 100% can show on the day after.",
        "",
        "## By kind of run",
        "",
        "| Bucket | Kind | Runs | Total | Median | Longest | Passed | Failed | Result not known |",
        "|---|---|---|---|---|---|---|---|---|",
    ]
    groups: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for r in runs:
        groups[(r["bucket"], r["kind"])].append(r)
    order = {TEST: 0, MEASURE: 1, BUILD: 2}

    def total(rows) -> float:
        return sum(float(r["seconds"]) for r in rows)

    for (bucket, kind), rows in sorted(
        groups.items(), key=lambda g: (order[g[0][0]], -total(g[1]))
    ):
        values = [float(r["seconds"]) for r in rows]
        results = [r.get("result") or "" for r in rows]
        known = (
            f"{results.count(PASS)} | {results.count(FAIL)} | {results.count('')}"
            if bucket != MEASURE
            else "| |"
        )
        out.append(
            f"| {bucket} | {kind} | {len(values)} | {clock(sum(values))} | "
            f"{clock(statistics.median(values))} | {clock(max(values))} | {known} |"
        )
    out += [
        "",
        "## Runs repeated with nothing edited",
        "",
        "A run of a statement that the same session had run before, word for word, with no file edited in between",
        '(`README.md`, "Repeats"). By the result of the run before it: after a pass the repeat could tell nothing new,',
        "unless something else had changed.",
        "",
        "| Bucket | Kind | Repeats | Time | After a pass | Time | After a failure | After a result not known |",
        "|---|---|---|---|---|---|---|---|",
    ]
    sums = [0, 0.0, 0, 0.0, 0, 0]
    for (bucket, kind), rows in sorted(
        groups.items(), key=lambda g: (order[g[0][0]], -total(g[1]))
    ):
        again = [r for r in rows if r.get("repeat_after")]
        if not again or bucket == MEASURE:
            continue
        after = {w: [r for r in again if r["repeat_after"] == w] for w in (PASS, FAIL, "unknown")}
        line = [
            len(again),
            total(again),
            len(after[PASS]),
            total(after[PASS]),
            len(after[FAIL]),
            len(after["unknown"]),
        ]
        sums = [a + b for a, b in zip(sums, line)]
        out.append(
            f"| {bucket} | {kind} | {line[0]} | {clock(line[1])} | {line[2]} | {clock(line[3])} | {line[4]} | {line[5]} |"
        )
    out.append(
        f"| | **All** | {sums[0]} | {clock(sums[1])} | {sums[2]} | {clock(sums[3])} | {sums[4]} | {sums[5]} |"
    )
    out += [
        "",
        "## The 20 longest runs",
        "",
        "| Start | Kind | Time | Background |",
        "|---|---|---|---|",
    ]
    for r in sorted(runs, key=lambda r: -float(r["seconds"]))[:20]:
        out.append(
            f"| {r['start']} | {r['kind']} | {clock(float(r['seconds']))} | {'yes' if r['background'] == '1' else ''} |"
        )
    if changes:
        out += [
            "",
            "## By change",
            "",
            'Each moment of a session belongs to the OpenSpec change named last before it (`README.md`, "Changes"):',
            "a guess from the workflow's own commands and the files read and edited, not a record kept by hand. *Elapsed*",
            "is calendar time from the first moment to the archive (or the last moment seen), nights and other work",
            'included; *Active* is Claude working on it (the rule of "Active time"); an *idle stretch* is a gap of more',
            "than 5 minutes inside a session (the user away or reading). *Closing stretch*: calendar time from the last",
            "edit of product code to the archive. *Archived, not seen*: the change is in the archive, but no transcript",
            "shows it archived (by `openspec archive` or by moving its folder); it ends where it was last seen.",
            "",
            *by_change(changes, in_archive),
        ]
    if activity:
        out += [
            "",
            "## By kind of work",
            "",
            'What each tool call did (`README.md`, "Kinds of work"): counts of actions and the bytes edits wrote. These',
            "are not shares of time: the transcript does not say how long a piece of writing took.",
            "",
            *by_kind_of_work(activity),
        ]
    out.append("")
    return "\n".join(out)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument(
        "--transcripts", type=Path, action="append", help="a folder of *.jsonl transcripts"
    )
    ap.add_argument(
        "--check",
        action="store_true",
        help="classify the commands on standard input, write nothing",
    )
    args = ap.parse_args()
    if args.check:
        for line in sys.stdin:
            print(classify(line.rstrip("\n")), line.rstrip("\n"))
        return 0
    folders = args.transcripts or default_folders()
    files = sorted(p for d in folders for p in d.glob("*.jsonl"))
    new_runs: list[dict] = []
    new_sessions: list[dict] = []
    new_changes: list[dict] = []
    new_activity: list[dict] = []
    for path in files:
        work: dict = {}
        got = read_session(path, work)
        if got is None:
            continue
        runs, row = got
        new_runs += runs
        new_sessions.append(row)
        new_changes += work["changes"]
        new_activity += work["activity"]
    seen = {s["session"] for s in new_sessions}
    # Sessions whose transcript is gone keep the rows they have.
    runs = [r for r in load(RUNS) if r["session"] not in seen] + new_runs
    sessions = [s for s in load(SESSIONS) if s["session"] not in seen] + new_sessions
    changes = [c for c in load(CHANGES) if c["session"] not in seen] + new_changes
    activity = [a for a in load(ACTIVITY) if a["session"] not in seen] + new_activity
    runs.sort(key=lambda r: (r["start"], r["kind"]))
    sessions.sort(key=lambda s: s["start"])
    changes.sort(key=lambda c: (c["first"], c["change"]))
    activity.sort(key=lambda a: (a["session"], a["change"], a["category"]))
    save(RUNS, runs, RUN_FIELDS)
    save(SESSIONS, sessions, ["session", "start", "end", "active_seconds", "command_seconds"])
    save(CHANGES, changes, CHANGE_FIELDS)
    save(ACTIVITY, activity, ACTIVITY_FIELDS)
    text = report(runs, sessions, changes, activity, archived_folders())
    REPORT.write_text(text, encoding="utf-8", newline="\n")
    print(f"{len(files)} transcripts read; {len(sessions)} sessions and {len(runs)} runs on record")
    return 0


if __name__ == "__main__":
    sys.exit(main())
