#!/usr/bin/env python3
"""The call-site check (OpenSpec change m2-builtin-statements, design D8; spec testing/call-site-check).

For each program of tests/callsite, both compilers write their C++ without building it (-z). The lines
of the statement under test (the last statement before the closing END or SYSTEM) are taken from each
compiler's main-module fragment, normalised and compared. Whole files are never compared.

Output: one line per program (equal, differ with both token lists, not compared with the reason) and
a summary line. No local path is printed. Nothing is written inside this repository; qb64pe writes its
C++ into the clone's git-ignored internal/temp (emptied first, as the corpus runner does), qb64rust
into a scratch folder that is removed at the end.

Usage:
    python tools/callsite/callsite.py [--qb-root DIR] [--qb64rust EXE] [--glob PATTERN] [--verbose]
                                      [--wait]
    python tools/callsite/callsite.py --self-test

Exit code: 0 when every program compared equal, 1 otherwise; 2 when a compiler is missing; 3 when
another run is building in the clone (tools/legacy_tests/clone_lock.py; --wait waits for it
instead).
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]

# One run at a time builds in the clone; the lock is the test runner's.
sys.path.insert(0, str(REPO / "tools" / "legacy_tests"))
from clone_lock import (  # pylint: disable=wrong-import-position,import-error
    BUSY_EXIT,
    CLONE_BUSY,
    lock_clone,
)

# ---- normalisation -------------------------------------------------------------------------------

# Lines of the statement frame, which the emitter's design calls free: the statement loop, its event
# check, the old compiler's statement label, the cleanup of string temporaries.
FRAME = [
    re.compile(r"^do\{$"),
    re.compile(r"^if\(!qbevent\)break;evnt\(.*\);\}while\(r\);$"),
    re.compile(r"^S_\d+:;$"),
    re.compile(r"^qbs_cleanup\(qbs_tmp_base,0\);$"),
    re.compile(r"^$"),
]

TOKEN = re.compile(
    r"""
    "(?:\\.|[^"\\])*"                         # a string literal
  | \d+(?:\.\d+)?(?:[eE][+-]?\d+)?[A-Za-z]*   # a number, with a C suffix
  | [A-Za-z_]\w*                              # a name
  | ->|==|!=|<=|>=|&&|\|\||<<|>>|::
  | \S
    """,
    re.VERBOSE,
)

# C types an explicit cast may name. A cast to an integer type around a whole call argument is dropped
# (rule 4): C++ converts the argument to the parameter's type anyway.
INT_TYPES = {
    "int8",
    "uint8",
    "int16",
    "uint16",
    "int32",
    "uint32",
    "int64",
    "uint64",
    "ptrszint",
    "uptrszint",
}
TYPES = INT_TYPES | {"float", "double", "long", "qbs"}
# Words before a `(` that do not make it a call.
NOT_CALLS = {"if", "while", "for", "switch", "return", "else", "goto"}
# Numbered temporaries and labels: renumbered in order of first use (rule 2).
NUMBERED = re.compile(r"^(pass|skip|sc_|RETURN_|temp_\w*?|L_)(\d+)$")
# An integer literal with the `long long` suffix (rule 5).
LONG_LONG = re.compile(r"^\d+ll$")


def tokenize(text: str) -> list[str]:
    return TOKEN.findall(text)


class Group:
    """A parenthesised token sequence. `call` is true when a name stands before it."""

    def __init__(self, items: list, call: bool) -> None:
        self.items = items
        self.call = call


def parse(tokens: list[str]) -> list:
    """Tokens to a tree of tokens and groups."""

    def seq(i: int) -> tuple[list, int]:
        out: list = []
        while i < len(tokens):
            t = tokens[i]
            if t == ")":
                return out, i
            if t == "(":
                prev = out[-1] if out else None
                call = (
                    isinstance(prev, str)
                    and re.match(r"[A-Za-z_]\w*$", prev) is not None
                    and prev not in NOT_CALLS
                )
                inner, j = seq(i + 1)
                out.append(Group(inner, call and prev not in TYPES))
                i = j + 1
            else:
                out.append(t)
                i += 1
        return out, i

    tree, _ = seq(0)
    return tree


def split_args(items: list) -> list[list]:
    args: list[list] = [[]]
    for it in items:
        if it == ",":
            args.append([])
        else:
            args[-1].append(it)
    return args


def is_simple(items: list) -> bool:
    """One token that is no type name, or a minus sign and a number."""
    if len(items) == 1 and isinstance(items[0], str):
        return items[0] not in TYPES
    return (
        len(items) == 2 and items[0] == "-" and isinstance(items[1], str) and items[1][:1].isdigit()
    )


def is_cast(node) -> bool:
    return (
        isinstance(node, Group)
        and not node.call
        and all(isinstance(t, str) and t in TYPES for t in node.items)
    )


def norm_seq(items: list, whole_arg: bool) -> list:
    """Normalises a token sequence. `whole_arg`: the sequence is one whole argument of a call."""
    while True:
        # Rule 3a: parentheses around a whole argument.
        if whole_arg and len(items) == 1 and isinstance(items[0], Group) and not items[0].call:
            items = items[0].items
            continue
        # Rule 4: an integer cast around a whole argument: `(T)(x)`, `(T)x` or `(T)f(...)`.
        if whole_arg and len(items) >= 2 and is_cast(items[0]) and set(items[0].items) <= INT_TYPES:
            rest = items[1:]
            one = len(rest) == 1
            call = (
                len(rest) == 2
                and isinstance(rest[0], str)
                and isinstance(rest[1], Group)
                and rest[1].call
            )
            if one or call:
                items = rest
                continue
        break
    # Rule 5: the `ll` suffix of an integer literal that is a whole argument.
    if whole_arg and is_simple(items) and LONG_LONG.match(items[-1]):
        items = [*items[:-1], items[-1][:-2]]
    # Rule 6: a whole argument that is integer literals joined by `|` is their value (`0|1` is `1`).
    if (
        whole_arg
        and len(items) >= 3
        and len(items) % 2 == 1
        and all(isinstance(t, str) and t.isdigit() for t in items[0::2])
        and all(t == "|" for t in items[1::2])
    ):
        value = 0
        for t in items[0::2]:
            value |= int(t)
        items = [str(value)]
    out: list = []
    for it in items:
        if not isinstance(it, Group):
            out.append(it)
        elif it.call:
            args = [norm_seq(a, True) for a in split_args(it.items)] if it.items else []
            flat: list = []
            for k, a in enumerate(args):
                if k:
                    flat.append(",")
                flat.extend(a)
            out.append(Group(flat, True))
        elif is_cast(it):
            out.append(it)
        else:
            inner = norm_seq(it.items, False)
            # Rule 3b: parentheses around a single token or a negative number.
            if is_simple(inner):
                out.extend(inner)
            else:
                out.append(Group(inner, False))
    return out


def flatten(items: list) -> list[str]:
    out: list[str] = []
    for it in items:
        if isinstance(it, Group):
            out.append("(")
            out.extend(flatten(it.items))
            out.append(")")
        else:
            out.append(it)
    return out


def rename(tokens: list[str]) -> list[str]:
    """Rule 2: numbered temporaries and labels become <prefix>#1, #2, ... in order of first use."""
    seen: dict[str, str] = {}
    out = []
    for t in tokens:
        m = NUMBERED.match(t)
        if m:
            if t not in seen:
                seen[t] = f"{m.group(1)}#{len(seen) + 1}"
            t = seen[t]
        out.append(t)
    return out


def normalise(lines: list[str]) -> list[str]:
    """The normalised tokens of a statement's C++ lines."""
    kept = [ln.strip() for ln in lines]
    kept = [ln for ln in kept if not any(f.match(ln) for f in FRAME)]
    tokens = tokenize("\n".join(kept))
    return rename(flatten(norm_seq(parse(tokens), False)))


# ---- extraction ----------------------------------------------------------------------------------

LINE = re.compile(r'^#line (\d+) "(.*)"$')


def statement_line(source: bytes) -> int:
    """The 1-based line of the statement under test: the last line with code before the closing END or
    SYSTEM line."""
    lines = source.decode("latin-1").splitlines()

    def code(s: str) -> str:
        s = s.strip()
        return "" if s.startswith("'") or s.upper().startswith("REM") else s

    last = max(
        (i for i, ln in enumerate(lines) if code(ln).upper() in ("END", "SYSTEM")), default=None
    )
    if last is None:
        raise ValueError("no closing END or SYSTEM line")
    for i in range(last - 1, -1, -1):
        if code(lines[i]):
            return i + 1
    raise ValueError("no statement before the closing END")


def extract(fragment: str, line: int, name: str) -> list[str]:
    """The C++ lines of source line `line` of file `name` in a main-module fragment."""
    out = []
    here = False
    for ln in fragment.splitlines():
        m = LINE.match(ln.strip())
        if m:
            file = m.group(2).replace("\\\\", "/").replace("\\", "/")
            here = int(m.group(1)) == line and file.rsplit("/", 1)[-1] == name
        elif here:
            out.append(ln)
    return out


# ---- the two compilers ---------------------------------------------------------------------------


def clear_temp(qb_root: Path) -> None:
    """Empties the clone's internal/temp, keeping the tracked temp.bin (as tools/legacy_tests does)."""
    temp = qb_root / "internal" / "temp"
    if not temp.is_dir():
        return
    for p in temp.iterdir():
        if p.name == "temp.bin":
            continue
        if p.is_dir():
            shutil.rmtree(p, ignore_errors=True)
        else:
            try:
                p.unlink()
            except OSError:
                pass


def last_line(text: str) -> str:
    lines = [ln.strip() for ln in text.splitlines() if ln.strip()]
    return lines[-1] if lines else "no output"


def old_cpp(qb_root: Path, work: Path, name: str) -> tuple[str | None, str]:
    clear_temp(qb_root)
    r = subprocess.run(
        [str(qb_root / "qb64pe.exe"), "-z", "-q", "-m", name],
        cwd=work,
        capture_output=True,
        text=True,
        errors="replace",
        timeout=300,
        check=False,
    )
    main0 = qb_root / "internal" / "temp" / "main0.txt"
    if r.returncode != 0 or not main0.is_file():
        return None, "the old compiler rejects it: " + " / ".join(
            ln.strip() for ln in r.stdout.splitlines() if ln.strip()
        )
    return main0.read_text(encoding="latin-1"), ""


def new_cpp(qb64rust: Path, work: Path, name: str) -> tuple[str | None, str]:
    r = subprocess.run(
        [str(qb64rust), "-z", name, "-o", str(work / (name + ".exe"))],
        cwd=work,
        capture_output=True,
        text=True,
        errors="replace",
        timeout=300,
        check=False,
    )
    if r.returncode != 0:
        errors = [ln.strip() for ln in r.stdout.splitlines() if ": error: " in ln]
        return None, errors[0] if errors else last_line(r.stdout + r.stderr)
    folder = Path(last_line(r.stdout))
    return (folder / "main0.txt").read_text(encoding="latin-1"), ""


def compare(path: Path, qb_root: Path, qb64rust: Path, work: Path) -> tuple[str, str]:
    """(verdict, detail) for one program: equal, differ or not compared."""
    name = path.name
    source = path.read_bytes()
    try:
        line = statement_line(source)
    except ValueError as e:
        return "not compared", str(e)
    shutil.copyfile(path, work / name)
    new, why = new_cpp(qb64rust, work, name)
    if new is None:
        return "not compared", why
    old, why = old_cpp(qb_root, work, name)
    if old is None:
        return "not compared", why
    a, b = normalise(extract(old, line, name)), normalise(extract(new, line, name))
    if not a or not b:
        return "not compared", f"no C++ line for the statement (line {line})"
    if a == b:
        return "equal", " ".join(a)
    return "differ", "old: " + " ".join(a) + "\n    new: " + " ".join(b)


# ---- self-test -----------------------------------------------------------------------------------

# Recorded pairs: the old compiler's lines and the new compiler's for one statement, as each wrote
# them (qb64pe -z and qb64rust -z, 2026-10-10). Each must compare equal.
PAIRS = [
    (
        "KILL s + CHR$(d)",
        [
            "do{",
            "sub_kill(qbs_add(__STRING_S,func_chr(qbr(*__DOUBLE_D))));",
            "qbs_cleanup(qbs_tmp_base,0);",
        ],
        ["do{", "sub_kill(qbs_add(__STRING_S,func_chr(((int32)(qbr(*__DOUBLE_D))))));"],
    ),
    (
        "SEEK 1.4, 2.5",
        ["sub_seek(qbr( 1.4E+0 ),qbr( 2.5E+0 ));", "if(!qbevent)break;evnt(3);}while(r);"],
        ["sub_seek(((int32)(qbr(1.4E+0))),qbr(2.5E+0));"],
    ),
    (
        "SEEK 1, 2",
        ["sub_seek( 1 , 2 );"],
        ["sub_seek(1,2ll);"],
    ),
    (
        "sg = RND(n)",
        ["*__SINGLE_SG=func_rnd(*__LONG_N,0|1);"],
        ["*__SINGLE_SG=func_rnd(*__LONG_N,1);"],
    ),
    (
        "SEEK n, -2",
        ["sub_seek(*__LONG_N, -2 );"],
        ["sub_seek(*__LONG_N,(-2ll));"],
    ),
    (
        "OPEN f FOR OUTPUT AS #1",
        ['sub_open(qbs_new_txt_len("t.txt",5), 4 ,NULL,NULL, 1 ,NULL,0);'],
        ['sub_open(qbs_new_txt_len("t.txt",5),4,NULL,NULL,1,NULL,0);'],
    ),
    (
        "NAME a AS b",
        ["S_12:;", "sub_name(__STRING_A,__STRING_B);"],
        ["sub_name(__STRING_A,__STRING_B);"],
    ),
    (
        "KILL CHR$(-1)",
        ["sub_kill(func_chr( -1 ));"],
        ["sub_kill(func_chr((-1)));"],
    ),
    (
        "PRINT with numbered labels",
        ["if (is_error_pending()) goto skip7;", "skip7:"],
        ["if (is_error_pending()) goto skip2;", "skip2:"],
    ),
]


def seeded(lines: list[str]) -> list[tuple[str, list[str]]]:
    """Deliberate mistakes made on the new compiler's lines: (what, the changed lines)."""
    text = "\n".join(lines)
    out = []
    if "qbr(" in text:
        out.append(("a missing rounding call", text.replace("qbr(", "(", 1).split("\n")))
    m = re.search(r",(\d+)\);", text)
    if m:
        wrong = str(int(m.group(1)) ^ 1)
        out.append(("a wrong mask", (text[: m.start(1)] + wrong + text[m.end(1) :]).split("\n")))
    m = re.search(r"^(\w+)\((.*)\);$", text, re.MULTILINE)
    if m:
        args = split_args(parse(tokenize(m.group(2))))
        if len(args) >= 2 and flatten(args[0]) != flatten(args[1]):
            args[0], args[1] = args[1], args[0]
            swapped = m.group(1) + "(" + ",".join("".join(flatten(a)) for a in args) + ");"
            out.append(
                ("swapped arguments", (text[: m.start()] + swapped + text[m.end() :]).split("\n"))
            )
    return out


def self_test() -> int:
    failures = []
    kinds: set[str] = set()
    for what, old, new in PAIRS:
        a, b = normalise(old), normalise(new)
        if a != b:
            failures.append(f"{what}: not equal\n    old: {' '.join(a)}\n    new: {' '.join(b)}")
        for kind, changed in seeded(new):
            kinds.add(kind)
            if normalise(changed) == a:
                failures.append(f"{what}: {kind} is not reported")
    for kind in ("a missing rounding call", "a wrong mask", "swapped arguments"):
        if kind not in kinds:
            failures.append(f"no recorded pair is tried with {kind}")
    # The normaliser must not hide these.
    distinct = [
        (["f(a,b);"], ["f(b,a);"]),
        (["f((double)(x));"], ["f(x);"]),
        (["f(g(x));"], ["f(x);"]),
        (["x=(int32)(y);"], ["x=y;"]),
        (["f((a+b)*c);"], ["f(a+b*c);"]),
        (["f(1,NULL,2);"], ["f(1,0,2);"]),
        (["f(a*2ll);"], ["f(a*2);"]),
        (["x=2ll;"], ["x=2;"]),
        (["f(x,0|2);"], ["f(x,1);"]),
        (["f(a|1);"], ["f(1);"]),
    ]
    for x, y in distinct:
        if normalise(x) == normalise(y):
            failures.append(f"{x[0]} and {y[0]} compare equal")
    src = b"$CONSOLE:ONLY\nDIM s AS STRING\n' a comment\nKILL s\n\nEND\n"
    if statement_line(src) != 4:
        failures.append("statement_line: expected line 4")
    frag = '#line 4 "t.bas"\ndo{\n#line 4 "t.bas"\nsub_kill(__STRING_S);\n#line 5 "t.bas"\nsub_end();\n'
    if extract(frag, 4, "t.bas") != ["do{", "sub_kill(__STRING_S);"]:
        failures.append("extract: wrong lines")
    for f in failures:
        print("self-test FAILED:", f)
    print(
        f"self-test: {len(PAIRS)} recorded pairs, {len(distinct)} distinct pairs, {len(failures)} failures"
    )
    return 1 if failures else 0


# ---- main ----------------------------------------------------------------------------------------


def main() -> int:
    ap = argparse.ArgumentParser(
        description="Compare the libqb calls both compilers write for one statement."
    )
    ap.add_argument("--qb-root", type=Path, default=REPO.parent / "QB64pe", help="the QB64pe clone")
    ap.add_argument("--qb64rust", type=Path, default=REPO / "target" / "release" / "qb64rust.exe")
    ap.add_argument("--programs", type=Path, default=REPO / "tests" / "callsite")
    ap.add_argument("--glob", default="*.bas")
    ap.add_argument(
        "--verbose", action="store_true", help="print the normalised tokens of equal programs too"
    )
    ap.add_argument(
        "--wait",
        action="store_true",
        help="wait for another run to leave the clone instead of stopping with exit code 3",
    )
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    programs = sorted(args.programs.glob(args.glob))
    if not programs:
        print("no programs")
        return 1
    # Before the lock: --wait must not wait for a run that cannot start.
    for exe in (args.qb_root / "qb64pe.exe", args.qb64rust):
        if not exe.is_file():
            print(f"compiler not found: {exe}", file=sys.stderr)
            return 2
    clone_lock = lock_clone(args.wait)  # held until the process ends
    if clone_lock is None:
        print(CLONE_BUSY, file=sys.stderr)
        return BUSY_EXIT
    counts = {"equal": 0, "differ": 0, "not compared": 0}
    work = Path(tempfile.mkdtemp(prefix="qb64rust-callsite-"))
    try:
        for p in programs:
            verdict, detail = compare(p, args.qb_root.resolve(), args.qb64rust.resolve(), work)
            counts[verdict] += 1
            if verdict == "equal" and not args.verbose:
                print(f"equal         {p.stem}")
            else:
                print(f"{verdict:<13} {p.stem}\n    {detail}")
    finally:
        shutil.rmtree(work, ignore_errors=True)
    print(
        f"{len(programs)} programs: {counts['equal']} equal, {counts['differ']} differ, "
        f"{counts['not compared']} not compared"
    )
    return 0 if counts["equal"] == len(programs) else 1


if __name__ == "__main__":
    sys.exit(main())
