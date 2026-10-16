"""The type the old compiler gives the result of _MIN, _IIF and _CLAMP for every pair of numeric types, and of _ROL
and _SHL for every type: writes v30_result_types.txt beside this file (step 4 of M3, measured before its design).

    python verification/v30_result_types.py [<QB64pe folder>]      (default: ../QB64pe beside the repository)

It writes one program with a line per pair into a scratch folder, has `qb64pe -z` write its C++ (nothing is built
or run) and reads two things from the line of each `PRINT`: the C++ type between `<` and `>` of the call
(`std::min<int16_t>`), and the type the value is printed in (`qbs_str((int16)(...))`). `_IIF` is a C++ `?:` with no
type of its own, so only the printed type shows it. A form the old compiler rejects is left out of the program and
named at the end of the file: `qb64pe` stops at the first error, so each was found by a run.

Like the test runner it empties nothing and takes no lock: do not run it beside a test run that builds in the clone.
"""

import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
TYPES = [
    ("bt", "_BIT"),
    ("ubt", "_UNSIGNED _BIT"),
    ("b4", "_BIT * 4"),
    ("b", "_BYTE"),
    ("ub", "_UNSIGNED _BYTE"),
    ("i", "INTEGER"),
    ("ui", "_UNSIGNED INTEGER"),
    ("l", "LONG"),
    ("ul", "_UNSIGNED LONG"),
    ("q", "_INTEGER64"),
    ("uq", "_UNSIGNED _INTEGER64"),
    ("s", "SINGLE"),
    ("d", "DOUBLE"),
    ("f", "_FLOAT"),
    ("o", "_OFFSET"),
    ("uo", "_UNSIGNED _OFFSET"),
]
# Rejected by the old compiler: "Unsupported integral type size" for a _BIT type, "Expected non-floating-point
# numeric argument" for a float (v30_x84, v30_x85).
NO_ROL = {"bt": "Unsupported integral type size", "ubt": "Unsupported integral type size",
          "b4": "Unsupported integral type size", "s": "Expected non-floating-point numeric argument",
          "d": "Expected non-floating-point numeric argument", "f": "Expected non-floating-point numeric argument"}


def main() -> int:
    qb = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE.parents[1] / "QB64pe"
    names = dict(TYPES)
    lines = ["$CONSOLE:ONLY"] + [f"DIM {n} AS {t}" for n, t in TYPES]
    tags = {}

    def add(tag, text):
        tags[len(lines) + 1] = tag
        lines.append(text)

    for a, _ in TYPES:
        for b, _ in TYPES:
            add(("_MIN(a, b)", a, b), f"PRINT _MIN({a}, {b})")
            add(("_IIF(1, a, b)", a, b), f"PRINT _IIF(1, {a}, {b})")
            add(("_CLAMP(a, b, b)", a, b), f"PRINT _CLAMP({a}, {b}, {b})")
    for a, _ in TYPES:
        if a not in NO_ROL:
            add(("_ROL(a, 1)", a, ""), f"PRINT _ROL({a}, 1)")
        add(("_SHL(a, 1)", a, ""), f"PRINT _SHL({a}, 1)")
    lines.append("END")
    with tempfile.TemporaryDirectory() as tmp:
        (Path(tmp) / "v30_types.bas").write_text("\n".join(lines) + "\n", encoding="ascii")
        r = subprocess.run(
            [str(qb / "qb64pe.exe"), "-z", "-q", "-m", "v30_types.bas"], cwd=tmp, capture_output=True, text=True
        )
    if r.returncode != 0:
        print(r.stdout[-600:], file=sys.stderr)
        return 1
    text = (qb / "internal" / "temp" / "main0.txt").read_text(encoding="latin-1")
    out = [
        "# Written by v30_result_types.py from the C++ of qb64pe -z; never edited by hand.",
        "# form | type of a | type of b | C++ type of the call | type the result is printed in",
    ]
    cur = None
    seen = set()
    for line in text.split("\n"):
        m = re.match(r'#line (\d+) "v30_types.bas"', line)
        if m:
            cur = int(m.group(1))
            continue
        if cur in tags and cur not in seen and "qbs_set(tqbs" in line:
            seen.add(cur)
            form, a, b = tags[cur]
            call = re.search(r"(?:std::min|func__rol|func_clamp)<([^>]+)>", line)
            printed = re.search(r"qbs_str\(\(([a-z0-9_ ]+)\)", line)
            out.append(
                " | ".join(
                    [form, names[a], names.get(b, "-"), call.group(1) if call else "-", printed.group(1) if printed else "?"]
                )
            )
    if len(seen) != len(tags):
        print(f"{len(tags) - len(seen)} lines of the program were not found in the C++", file=sys.stderr)
        return 1
    out.append("# Rejected:")
    out += [f"# _ROL(a, 1) | {names[a]} | {why}" for a, why in NO_ROL.items()]
    (HERE / "v30_result_types.txt").write_text("\n".join(out) + "\n", encoding="ascii", newline="\n")
    print(f"{len(seen)} forms written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
