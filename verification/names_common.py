"""Helpers shared by the scripts that measure which reserved names the old compiler accepts:
v15_builtin_names.py, v19_proc_names.py and v52_builtin_name_suffixes.py.

They give the names to measure (every reserved word of crates/syntax/src/parser/keywords.rs and every name in
tools/builtins/builtins.json), the two ways a program is put through `qb64pe.exe`, and the reading of a run's
printed lines back into result lines.
"""

import json
import os
import re
import subprocess
from collections.abc import Callable
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
QB = REPO.parent / "QB64pe" / "qb64pe.exe"


def reserved_names() -> dict[str, str | None]:
    """Every reserved word and built-in name, in upper case, with the suffix its name must be written with
    (builtins.json's `musthave`, such as `$` for `LEFT$`), or None."""
    kw_src = (REPO / "crates" / "syntax" / "src" / "parser" / "keywords.rs").read_text(encoding="utf-8")
    block = kw_src.split("const KEYWORDS")[1].split("];")[0]
    keywords = re.findall(r'"([^"]+)"', block)
    doc = json.loads((REPO / "tools" / "builtins" / "builtins.json").read_text(encoding="utf-8"))
    seen: dict[str, str | None] = {}
    for k in keywords:
        seen.setdefault(k.upper(), None)
    for e in doc["entries"]:
        key = e["name"].upper()
        if seen.get(key) is None:
            seen[key] = e.get("musthave")
    return seen


def compile_only(src: Path) -> str:
    """The first line `qb64pe.exe -z` (C++ only) prints for src: its error, or "" when it compiles."""
    r = subprocess.run(
        [str(QB), "-z", "-q", src.name],
        check=False,
        cwd=src.parent,
        capture_output=True,
        text=True,
        errors="replace",
    )
    lines = [l.strip() for l in (r.stdout + r.stderr).splitlines() if l.strip()]
    return lines[0] if lines else ""


def run(src: Path, timeout: int = 60) -> str:
    """Builds src and runs it; what it printed, on one line. `(not built)` when no executable came out and
    `(timed out)` when it ran longer than timeout seconds."""
    exe = src.with_suffix(".exe")
    subprocess.run(
        [str(QB), "-x", "-q", "-m", src.name, "-o", exe.name],
        check=False,
        cwd=src.parent,
        capture_output=True,
    )
    if not exe.exists():
        return "(not built)"
    env = dict(os.environ, QB64PE_NOPROMPT="y")
    try:
        r = subprocess.run(
            [str(exe)],
            check=False,
            cwd=src.parent,
            capture_output=True,
            text=True,
            errors="replace",
            stdin=subprocess.DEVNULL,
            env=env,
            timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        return "(timed out)"
    return " ".join(r.stdout.split())


def from_log(log: Path, classify: Callable[[str, str], str]) -> dict[str, str]:
    """The result line of each form, rebuilt from the lines a run printed (`<form> rejected <msg>` or
    `<form> accepted prints: <output>`), in the order of the log. classify turns a form and what it
    printed into its line."""
    by_form = {}
    for line in log.read_text(encoding="utf-8").splitlines():
        form, verdict, rest = line.split(" ", 2)
        if verdict == "rejected":
            by_form[form] = line
        else:
            by_form[form] = classify(form, rest.removeprefix("prints:").strip())
    return by_form
