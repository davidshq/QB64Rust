#!/usr/bin/env python3
"""Writes crates/base/src/old_wording.tsv: for each message of this compiler, the old compiler's wording of the same
error, taken from recorded output (m3-old-wording-notes). `qb64rust_base::old_wording` reads it.

Run from the repo root after `cargo build --release`:  python tools/diagnostics/old_wording.py
Then, after building again with the new table:        python tools/diagnostics/old_wording.py --check

The pairs come from every program the old compiler rejected with its message recorded: verification/*.compile.txt
(exit not 0), tests/corpus/**/*.err and tests/upstream/compile_tests/**/*.err. Each is compiled by this compiler
with -z (nothing built); its first error that is not "not supported yet", when it is in the main file on the line
the old message names ("LINE n:"), is paired with the old message's first line.

A message becomes a pattern: a backtick span whose words are all keywords or built-in names (`_MEM`, `SELECT
CASE`, a metacommand) stays as written; any other span (a name or a line number as the program writes it) becomes
`{}`, and a number outside the spans becomes {#}. The compiler makes the pattern and the forms of each placeholder
(`qb64rust --old-wording-keys`), and gives the word list (`--old-wording-words`): the script holds no copy of
either rule, so the compiler's lookup and this table never differ. In the old message each placeholder's text is looked for as a
whole word, as written, in capitals, without its parentheses or type suffix, and replaced by {n} or {n:U} and so on
(n counts the placeholders in order). A pattern is kept only when every pair of it gives the same old template, the template
differs from the message (ignoring case and backticks), names no file and holds no text of one program (a
number or a name in capitals that is no keyword and not in the message). So a note is the old compiler's own
wording of that error, never a guess.
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
import tempfile
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
QB64RUST = ROOT / "target" / "release" / ("qb64rust.exe" if sys.platform == "win32" else "qb64rust")
OUT = ROOT / "crates" / "base" / "src" / "old_wording.tsv"
WORDS = ROOT / "crates" / "base" / "src" / "old_wording_words.txt"
OURS = re.compile(r"^(.*?):(\d+):(\d+): error: (.*)$")

# A placeholder's text in each form the table can name, as the compiler gives it: (code, text) pairs in the
# compiler's order of the codes (`=` as written, `U` capitals, `P` no `()`, `S` no `()` and no suffix, ...).
Forms = list[tuple[str, str]]


def keywords() -> list[str]:
    """The reserved words of the parser and the names of the built-in table, in capitals, from the compiler."""
    run = subprocess.run([str(QB64RUST), "--old-wording-words"], capture_output=True, check=True, timeout=60)
    return run.stdout.decode("utf-8").split()


def patterns(messages: list[str]) -> list[tuple[str, list[Forms]]]:
    """Each message's pattern and its placeholders' forms, in order, from the compiler."""
    if not messages:
        return []
    if any("\n" in m or "\r" in m for m in messages):
        raise ValueError("a message spans lines")
    run = subprocess.run(
        [str(QB64RUST), "--old-wording-keys"],
        input="".join(m + "\n" for m in messages).encode("utf-8"),
        capture_output=True,
        check=True,
        timeout=60,
    )
    out = []
    for line in run.stdout.decode("utf-8").splitlines():
        got = json.loads(line)
        out.append((got["key"], [[(code, text) for code, text in value] for value in got["values"]]))
    if len(out) != len(messages):
        raise RuntimeError(f"{len(messages)} messages, {len(out)} keys")
    return out


def old_message(text: str) -> tuple[str, int] | None:
    """The old compiler's message and line from its recorded output, or None."""
    body = [line.rstrip("\r") for line in text.split("\n")]
    body = [line for line in body if line.strip() and not line.startswith("exit=")]
    if not body:
        return None
    for line in body[1:]:
        m = re.match(r"^LINE (\d+):", line)
        if m:
            return body[0].strip(), int(m.group(1))
    return None


def template(old: str, values: list[Forms]) -> tuple[str, list[tuple[int, tuple[str, ...]]]]:
    """The old message with each placeholder's text found in it replaced by a sentinel, and per sentinel the
    placeholder's number and every form that gives the text found, in the compiler's order: a name without a
    suffix fits `=` and `S` alike, and only a pair with a suffix tells them apart."""
    marks: list[tuple[int, tuple[str, ...]]] = []
    for n, forms in enumerate(values, 1):
        found = sorted({text for _, text in forms if text}, key=len, reverse=True)
        for t in found:
            pat = r"(?<![A-Za-z0-9_])" + re.escape(t) + r"(?![A-Za-z0-9_])"
            if re.search(pat, old):
                # A sentinel, so that a later text is never found inside an earlier mark.
                sentinel = "\x00" + str(len(marks)) + "\x00"
                marks.append((n, tuple(code for code, text in forms if text == t)))
                old = re.sub(pat, sentinel, old, count=1)
                break
    return old, marks


def merge(pairs: list[tuple[str, list[tuple[int, tuple[str, ...]]]]]) -> str | None:
    """One template from the templates of every pair of a pattern: the same text and placeholders, each with the
    forms all pairs allow (`{n:=|S}`, in the compiler's order of the codes); None when they differ or a
    placeholder has no form left."""
    texts = {text for text, _ in pairs}
    shapes = {tuple(n for n, _ in marks) for _, marks in pairs}
    if len(texts) != 1 or len(shapes) != 1:
        return None
    text, first = next(iter(texts)), pairs[0][1]
    out = []
    for k, (n, order) in enumerate(first):
        allowed = frozenset.intersection(*(frozenset(marks[k][1]) for _, marks in pairs))
        if not allowed:
            return None
        codes = "|".join(c for c in order if c in allowed)
        out.append("{" + str(n) + ("" if codes == "=" else ":" + codes) + "}")
    return re.sub("\x00(\\d+)\x00", lambda m: out[int(m.group(1))], text)


def names_file(old: str) -> bool:
    """Whether an old message names a file, so its note would be wrong for another program: a masked folder
    (`<verification>`), a backslash, or a BASIC file's extension. A `/` alone is no path: the old compiler writes
    "SUB/FUNCTION" and "variable/value" (m4-upstream-rejections); every recorded path also has one of these."""
    return re.search(r"<[a-z]+>|\.bas\b|\.bi\b|\.bm\b|\\", old) is not None


def one_program(old: str, pattern: str, keywords: frozenset[str]) -> bool:
    """Whether an old template holds text of one program that the pattern does not carry: a number ("line 4"), a
    word with a digit ("1st") or a name in capitals that is no keyword ("Cannot convert type (FOO)"); its note
    would be wrong for another program. A word with a digit that the pattern holds as a word is the same in every
    message of the pattern (`for the 2nd argument`, "2nd sub argument requires a number", m4-sub-gl)."""
    rest = re.sub(r"\{\d+(:[A-Z=|]+)?\}", "", old)
    words = re.compile(r"(?<![A-Za-z0-9_$.])[A-Za-z0-9_$]*\d[A-Za-z0-9_$]*(?![A-Za-z0-9_$.])")
    carried = set(words.findall(pattern))
    if any(w not in carried for w in words.findall(rest)) or re.search(r"\d", words.sub("", rest)):
        return True
    names = [w for w in re.findall(r"(?<![A-Za-z0-9_$])[A-Z_][A-Z0-9_$]+(?![A-Za-z0-9_$])", rest) if w not in keywords]
    return any(w not in pattern.upper() for w in names)


def recorded() -> list[tuple[Path, str, int, list[str]]]:
    """Every recorded rejection: the program, the old message, its line, extra compiler arguments."""
    out = []
    for txt in sorted((ROOT / "verification").glob("*.compile.txt")):
        text = txt.read_text(encoding="latin-1")
        if re.search(r"^exit=0\s*$", text, re.M):
            continue
        bas = txt.with_name(txt.name[: -len(".compile.txt")] + ".bas")
        got = old_message(text)
        if bas.is_file() and got:
            out.append((bas, got[0], got[1], []))
    for err in sorted((ROOT / "tests" / "corpus").rglob("*.err")):
        got = old_message(err.read_text(encoding="latin-1"))
        if err.with_suffix(".bas").is_file() and got:
            out.append((err.with_suffix(".bas"), got[0], got[1], []))
    up = ROOT / "tests" / "upstream"
    for err in sorted((up / "compile_tests").rglob("*.err")):
        got = old_message(err.read_text(encoding="latin-1"))
        if err.with_suffix(".bas").is_file() and got:
            out.append((err.with_suffix(".bas"), got[0], got[1], ["--include-root", str(up / "root")]))
    return out


def compile_lines(bas: Path, extra: list[str], scratch: Path) -> list[str]:
    """This compiler's output for a program (-z, nothing built), line by line."""
    run = subprocess.run(
        [str(QB64RUST), "-z", *extra, bas.name, "-o", str(scratch / "x.exe")],
        cwd=bas.parent,
        capture_output=True,
        check=False,
        timeout=120,
    )
    return run.stdout.decode("utf-8", "replace").splitlines()


def first_error(bas: Path, lines: list[str]) -> tuple[int, str] | None:
    """This compiler's first real error in its output for bas, when it is in the main file: its line and
    message."""
    for line in lines:
        m = OURS.match(line.rstrip("\r"))
        if not m or m.group(4).startswith("not supported yet: "):
            continue
        if Path(m.group(1)).name != bas.name:
            return None
        return int(m.group(2)), m.group(4)
    return None


def check() -> int:
    """--check: every recorded rejection whose first error is on the old message's line and gets a note gets the
    old message itself (the compiler's lookup agrees with this script's templates)."""
    notes = wrong = 0
    note = re.compile(r"^.*?:(\d+):\d+: note: QB64pe: (.*)$")
    with tempfile.TemporaryDirectory() as tmp:
        for bas, old, line, extra in recorded():
            lines = compile_lines(bas, extra, Path(tmp))
            ours = first_error(bas, lines)
            if not ours or ours[0] != line:
                continue
            for k, text in enumerate(lines):
                m = OURS.match(text.rstrip("\r"))
                if m and m.group(4) == ours[1] and k + 1 < len(lines):
                    n = note.match(lines[k + 1].rstrip("\r"))
                    if n:
                        notes += 1
                        if n.group(2) != old:
                            wrong += 1
                            print(f"{bas.relative_to(ROOT)}: note {n.group(2)!r}, recorded {old!r}")
                    break
    print(f"{notes} notes on recorded rejections, {wrong} not the recorded message")
    return 1 if wrong else 0


def main() -> int:
    if not QB64RUST.is_file():
        print(f"build it first: {QB64RUST.relative_to(ROOT)} is missing", file=sys.stderr)
        return 1
    words = keywords()
    pairs: list[tuple[str, str]] = []
    other_line = 0
    with tempfile.TemporaryDirectory() as tmp:
        for bas, old, line, extra in recorded():
            ours = first_error(bas, compile_lines(bas, extra, Path(tmp)))
            if not ours:
                continue
            if ours[0] != line:
                other_line += 1
                continue
            pairs.append((ours[1], old))
    paired = len(pairs)
    found: dict[str, list] = defaultdict(list)
    for (pattern, values), (_, old) in zip(patterns([m for m, _ in pairs]), pairs):
        found[pattern].append(template(old, values))
    keyword_set = frozenset(words)
    table, disagree, same, files, specific = {}, 0, 0, 0, 0
    for pattern, pairs in found.items():
        old = merge(pairs)
        if old is None:
            disagree += 1
            continue
        if names_file(old):
            files += 1
            continue
        if one_program(old, pattern, keyword_set):
            specific += 1
            continue
        if old.replace("`", "").lower() == pattern.replace("`", "").lower():
            same += 1
            continue
        table[pattern] = old
    lines = [
        "# Generated by tools/diagnostics/old_wording.py from recorded output; do not edit by hand.\n",
        "# <this compiler's message: `{}` any name, {#} a number>TAB<the old compiler's wording: {n} the n-th of them,\n",
        "# {n:U|SU} in one of these forms (= as written, U capitals, P no (), S no () and no suffix; no note when the\n",
        "# forms give different texts)>\n",
    ]
    lines += [f"{k}\t{v}\n" for k, v in sorted(table.items())]
    OUT.write_text("".join(lines), encoding="utf-8", newline="\n")
    # The words a span may hold and stay literal, for the compiler's lookup: the list the keys above were made with.
    WORDS.write_text(
        "# Generated by tools/diagnostics/old_wording.py: reserved words and built-in names.\n"
        + "".join(f"{w}\n" for w in sorted(words)),
        encoding="utf-8",
        newline="\n",
    )
    print(
        f"{paired} pairs on the same line ({other_line} on another line); {len(found)} patterns: "
        f"{len(table)} kept, {disagree} whose pairs disagree, {files} naming a file, {specific} with text of one "
        f"program, {same} worded the same"
    )
    return 0


if __name__ == "__main__":
    sys.exit(check() if sys.argv[1:] == ["--check"] else main())
