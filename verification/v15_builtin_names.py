"""Which keyword and built-in names the old compiler accepts as variable names (m2-procedures-and-errors, design D3).

For every reserved word of crates/syntax/src/parser/keywords.rs and every name in tools/builtins/builtins.json,
compiles a two-line program that assigns and prints a variable of that name with `qb64pe.exe -z` (C++ only):
- the bare name (`width = 5`);
- the name with `&` (`width& = 5`: does a suffix free a reserved name?);
- for a built-in whose name must be written with `$` (`LEFT$`), the name with `$` (`left$ = "a"`).
Each program that compiles is then built and run, to see whether the name really became a variable: it then
prints the value it was given. A form that compiles but prints something else is a statement with a function of
the same name (`date$ = "a"` sets the date, `PRINT date$` reads it). Running such a program executes that
statement with `5` or `"a"` (`DATE$`, `TIME$`, `_CLIPBOARD$`); the clock is not changed (invalid value, and
Windows needs administrator rights), the clipboard is. Programs and executables go into a scratch folder given
on the command line, never into the repo.

Usage (from the repo root):
  python verification/v15_builtin_names.py <scratch-folder>     measure (about 15 minutes)
  python verification/v15_builtin_names.py --from-log <log>     rebuild the .txt from the printed lines of a run
Writes verification/v15_builtin_names.txt: one line per form, `<form> variable|statement|rejected <detail>`.
"""

import json
import os
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
QB = REPO.parent / "QB64pe" / "qb64pe.exe"


def names():
    kw_src = (REPO / "crates" / "syntax" / "src" / "parser" / "keywords.rs").read_text()
    block = kw_src.split("const KEYWORDS")[1].split("];")[0]
    keywords = re.findall(r'"([^"]+)"', block)
    doc = json.loads((REPO / "tools" / "builtins" / "builtins.json").read_text())
    seen = {}
    for k in keywords:
        seen.setdefault(k.upper(), None)
    for e in doc["entries"]:
        key = e["name"].upper()
        if seen.get(key) is None:
            seen[key] = e.get("musthave")
    return seen


def forms(seen):
    out = []
    for name, musthave in sorted(seen.items()):
        low = name.lower()
        out.append(low)
        out.append(low + "&")
        if musthave == "$":
            out.append(low + "$")
    return out


def compile_only(src: Path) -> str:
    r = subprocess.run([str(QB), "-z", "-q", src.name], cwd=src.parent, capture_output=True, text=True,
                       errors="replace")
    lines = [l.strip() for l in (r.stdout + r.stderr).splitlines() if l.strip()]
    return lines[0] if lines else ""


def run(src: Path) -> str:
    exe = src.with_suffix(".exe")
    subprocess.run([str(QB), "-x", "-q", "-m", src.name, "-o", exe.name], cwd=src.parent, capture_output=True)
    if not exe.exists():
        return "(not built)"
    env = dict(os.environ, QB64PE_NOPROMPT="y")
    r = subprocess.run([str(exe)], cwd=src.parent, capture_output=True, text=True, errors="replace",
                       stdin=subprocess.DEVNULL, env=env, timeout=60)
    return " ".join(r.stdout.split())


# Statements that read back what they set, so they print the assigned value like a variable would:
# `_CLIPBOARD$ = "a"` puts "a" on the clipboard and `_CLIPBOARD$` returns it.
ROUND_TRIP_STATEMENTS = {"_clipboard$"}


def classify(form: str, printed: str) -> str:
    """The line for a form that compiled, from what its program printed."""
    expected = "a" if form.endswith("$") else "5"
    if printed == expected and form not in ROUND_TRIP_STATEMENTS:
        return f"{form} variable prints: {printed}"
    return f"{form} statement prints: {printed}"


def from_log(log: Path) -> list[str]:
    """Rebuilds the result lines from the lines a run printed (`<form> rejected <msg>` or `<form> accepted
    prints: <output>`)."""
    out = []
    for line in log.read_text().splitlines():
        form, verdict, rest = line.split(" ", 2)
        if verdict == "rejected":
            out.append(line)
        else:
            out.append(classify(form, rest.removeprefix("prints:").strip()))
    return out


def main():
    if sys.argv[1] == "--from-log":
        results = from_log(Path(sys.argv[2]))
    else:
        scratch = Path(sys.argv[1])
        scratch.mkdir(parents=True, exist_ok=True)
        results = []
        for i, form in enumerate(forms(names())):
            value = '"a"' if form.endswith("$") else "5"
            src = scratch / f"n{i}.bas"
            src.write_text(f"$CONSOLE:ONLY\n{form} = {value}\nPRINT {form}\nSYSTEM\n")
            msg = compile_only(src)
            if msg:
                line = f"{form} rejected {msg}"
                results.append(line)
            else:
                printed = run(src)
                line = f"{form} accepted prints: {printed}"
                results.append(classify(form, printed))
            # The printed lines are the log `--from-log` reads.
            print(line, flush=True)
    (HERE / "v15_builtin_names.txt").write_text("\n".join(results) + "\n")


if __name__ == "__main__":
    main()
