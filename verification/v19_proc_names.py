"""Which keyword and built-in names the old compiler accepts as procedure names (the known bug "a SUB named like a
built-in", STATUS.md; the variable rule is v15_builtin_names.py).

For every reserved word of crates/syntax/src/parser/keywords.rs and every name in tools/builtins/builtins.json,
compiles four small programs with `qb64pe.exe -z` (C++ only):
- `sub`: `SUB <name>` that prints `s`, called by a bare statement `<name>`;
- `call`: the same SUB, called by `CALL <name>`;
- `func`: `FUNCTION <name>` returning 5, printed by `PRINT <name>`;
- `func&`: the same with `&` (`FUNCTION <name>&`).
Each program that compiles is then built and run, to see whether the procedure was really called: it prints `s`
or `5`. A program that compiles but prints something else called a built-in of that name (class `builtin`).
Programs and executables go into a scratch folder given on the command line, never into the repo.

Usage (from the repo root):
  python verification/v19_proc_names.py <scratch-folder>     measure
Writes verification/v19_proc_names.txt: one line per name and form,
`<name> <form> proc|builtin|rejected <detail>`.
"""

import sys
from pathlib import Path

from names_common import HERE, compile_only, reserved_names, run

FORMS = ["sub", "call", "func", "func&"]


def program(name: str, form: str) -> tuple[str, str]:
    """The program for one name and form, and what it prints when the procedure is called."""
    if form == "sub":
        return f'$CONSOLE:ONLY\n{name}\nSYSTEM\nSUB {name}\nPRINT "s"\nEND SUB\n', "s"
    if form == "call":
        return f'$CONSOLE:ONLY\nCALL {name}\nSYSTEM\nSUB {name}\nPRINT "s"\nEND SUB\n', "s"
    n = name + ("&" if form == "func&" else "")
    return f"$CONSOLE:ONLY\nPRINT {n}\nSYSTEM\nFUNCTION {n}\n{n} = 5\nEND FUNCTION\n", "5"


def main():
    scratch = Path(sys.argv[1])
    scratch.mkdir(parents=True, exist_ok=True)
    results = []
    i = 0
    for name in sorted(reserved_names()):
        low = name.lower()
        for form in FORMS:
            text, expected = program(low, form)
            src = scratch / f"p{i}.bas"
            i += 1
            src.write_text(text)
            msg = compile_only(src)
            if msg:
                line = f"{low} {form} rejected {msg}"
            else:
                printed = run(src, timeout=30)
                verdict = "proc" if printed == expected else "builtin"
                line = f"{low} {form} {verdict} prints: {printed}"
            results.append(line)
            print(line, flush=True)
    (HERE / "v19_proc_names.txt").write_text("\n".join(results) + "\n")


if __name__ == "__main__":
    main()
