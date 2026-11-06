"""Which keyword and built-in names the old compiler accepts as variable names with a suffix (m4-qb64pe-sources,
design D1; the sibling of v15_builtin_names.py, which measured no suffix, `&` and a required `$`).

For every reserved word of crates/syntax/src/parser/keywords.rs and every name in tools/builtins/builtins.json,
compiles a two-line program that assigns and prints a variable of that name with `qb64pe.exe -z` (C++ only), once
per suffix that v15 did not measure: `$` (for a name that does not require it), `%`, `!`, `#`, `%%`, `&&`, `##`,
`~%`, `~&`, `~%%`, `~&&` and `` ` ``. Each program that compiles is then built and run, to see whether the name
really became a variable: it then prints the value it was given (`"a"` for `$`, -1 for `` ` ``, else 5). A form
that compiles but prints something else is a statement or a function of the same name. Programs and executables
go into a scratch folder given on the command line, never into the repo; each job has a folder of its own there,
and each instance of the old compiler takes a temporary folder of its own in the clone. The script takes no lock
on the clone: run it while nothing else uses the clone (a call-site check run beside it got answers cut short,
and kept them in its cache). Side by side, some forms fail spuriously; those are measured again one at a time.

Usage (from the repo root):
  python verification/v52_builtin_name_suffixes.py <scratch-folder> [--jobs N]   measure
  python verification/v52_builtin_name_suffixes.py --from-log <log>              rebuild the .txt from a run's lines
Writes verification/v52_builtin_name_suffixes.txt: one line per form, `<form> variable|statement|rejected
<detail>`, in the order of the names and suffixes.

The run of 2026-10-21: 5,788 forms in 800 s with --jobs 8, then 59 spurious lines (7 not built, 52 "UNEXPECTED
INTERNAL COMPILER ERROR!") measured again one at a time in about a minute; each gave an ordinary verdict.
"""

import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

from names_common import HERE, compile_only, from_log, reserved_names, run

SUFFIXES = ["$", "%", "!", "#", "%%", "&&", "##", "~%", "~&", "~%%", "~&&", "`"]


def forms(seen):
    out = []
    for name, musthave in sorted(seen.items()):
        low = name.lower()
        for suffix in SUFFIXES:
            if suffix == "$" and musthave == "$":
                continue  # measured by v15
            out.append(low + suffix)
    return out


def value(form: str) -> str:
    if form.endswith("$"):
        return '"a"'
    if form.endswith("`"):
        return "-1"
    return "5"


def expected(form: str) -> str:
    return value(form).strip('"')


def classify(form: str, printed: str) -> str:
    """The line for a form that compiled, from what its program printed."""
    if printed == expected(form):
        return f"{form} variable prints: {printed}"
    return f"{form} statement prints: {printed}"


def measure(job):
    i, form, scratch = job
    folder = scratch / f"j{i % 64}"
    folder.mkdir(parents=True, exist_ok=True)
    src = folder / f"n{i}.bas"
    src.write_text(f"$CONSOLE:ONLY\n{form} = {value(form)}\nPRINT {form}\nSYSTEM\n")
    msg = compile_only(src)
    if msg:
        return f"{form} rejected {msg}", f"{form} rejected {msg}"
    printed = run(src)
    for p in (src, src.with_suffix(".exe")):
        p.unlink(missing_ok=True)
    return f"{form} accepted prints: {printed}", classify(form, printed)


def spurious(line: str) -> bool:
    """A line that a run side by side with other instances may give for any form (measured again)."""
    return line.endswith(("(not built)", "(timed out)")) or "UNEXPECTED INTERNAL COMPILER ERROR" in line


def main():
    if sys.argv[1] == "--from-log":
        # In the order of the forms: a form measured again comes later in the log, and its last line counts.
        by_form = from_log(Path(sys.argv[2]), classify)
        results = [by_form[f] for f in forms(reserved_names()) if f in by_form]
    else:
        scratch = Path(sys.argv[1])
        jobs = int(sys.argv[sys.argv.index("--jobs") + 1]) if "--jobs" in sys.argv else 1
        all_forms = forms(reserved_names())
        work = [(i, f, scratch) for i, f in enumerate(all_forms)]
        results = []
        with ThreadPoolExecutor(max_workers=jobs) as pool:
            # map keeps the order of the forms; each printed line is the log `--from-log` reads.
            for printed, line in pool.map(measure, work):
                print(printed, flush=True)
                results.append(line)
        # Instances side by side sometimes fail: with no message (a form that compiled and was not built) or with
        # "UNEXPECTED INTERNAL COMPILER ERROR!". Those are measured again, one at a time.
        for k, line in enumerate(results):
            if spurious(line):
                printed, results[k] = measure(work[k])
                print(printed, flush=True)
    (HERE / "v52_builtin_name_suffixes.txt").write_text("\n".join(results) + "\n")


if __name__ == "__main__":
    main()
