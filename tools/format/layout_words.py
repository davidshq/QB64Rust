"""The keyword spellings of the formatter (OpenSpec change m5b-formatter, design D3).

QB64pe writes each keyword of a statement's layout through SCase$("...") (or SCase2$("...") for a few), with the
CamelCase spelling as a literal in its sources: SCase$("End" + sp + "If"), SCase$("$Include"), SCase2$("_Dynamic").
This reads every such call in the clone's source/ (*.bas, *.bi, *.bm) whose argument is made of string literals
and the separators sp and sp2 only, splits the text into words at blanks and separators, and writes

- crates/format/src/words.rs: each word in upper case with its spelling, in order of the upper-case word.

  python tools/format/layout_words.py           # write the table
  python tools/format/layout_words.py --check   # say whether the file is what it writes (exit 1 if not)

Calls with another argument (SCase$(RTRIM$(id.cn)), SCase2$(t$)) spell names from tables at run time: the built-in
table (crates/builtins) and SCase2$'s rule, which the formatter applies itself. Two literals that spell one word two
ways stop the tool (none at the reference commit).
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
CLONE = REPO.parent / "QB64pe"
WORDS_RS = REPO / "crates" / "format" / "src" / "words.rs"

# A call and its argument up to the closing parenthesis (no parenthesis inside: a call with one has a computed
# argument and is not a literal).
CALL = re.compile(r'\bSCase2?\$\(([^()\r\n]*)\)', re.IGNORECASE)
LITERAL = re.compile(r'\s*(?:"([^"]*)"|(sp2?))\s*(?:\+|$)', re.IGNORECASE)


class WordsError(Exception):
    """The sources hold something the table cannot take."""


def literal_text(argument: str) -> str | None:
    """The text of an argument made of literals and sp/sp2 joined by +, the separators as one blank; None for
    any other argument."""
    pos, parts = 0, []
    while pos < len(argument):
        m = LITERAL.match(argument, pos)
        if not m or m.end() == pos:
            return None
        parts.append(m.group(1) if m.group(1) is not None else " ")
        pos = m.end()
    return "".join(parts) if parts else None


def words_of(text: str) -> list[str]:
    """The words of a literal: split at blanks (sp and sp2 are blanks too)."""
    return [w for w in text.split(" ") if w]


def scan(sources: list[tuple[str, str]]) -> dict[str, str]:
    """{UPPER: spelling} from (file name, text) pairs; WordsError when a word is spelled two ways."""
    table: dict[str, tuple[str, str]] = {}
    for name, text in sources:
        for n, line in enumerate(text.splitlines(), 1):
            code = strip_comment(line)
            for m in CALL.finditer(code):
                literal = literal_text(m.group(1))
                if literal is None:
                    continue
                for word in words_of(literal):
                    key = word.upper()
                    seen = table.get(key)
                    if seen and seen[0] != word:
                        raise WordsError(f"{name}:{n}: {word!r} and {seen[0]!r} ({seen[1]}) spell one word two ways")
                    table.setdefault(key, (word, f"{name}:{n}"))
    return {k: v[0] for k, v in table.items()}


def strip_comment(line: str) -> str:
    """The line without a ' comment outside quotes."""
    quoted = False
    for i, c in enumerate(line):
        if c == '"':
            quoted = not quoted
        elif c == "'" and not quoted:
            return line[:i]
    return line


def read_sources(clone: Path) -> list[tuple[str, str]]:
    source = clone / "source"
    if not source.is_dir():
        raise WordsError(f"no QB64pe sources at {source}")
    files = sorted(p for p in source.rglob("*") if p.suffix.lower() in (".bas", ".bi", ".bm"))
    return [(p.relative_to(clone).as_posix(), p.read_text(encoding="latin-1")) for p in files]


def render(table: dict[str, str]) -> str:
    rows = "".join(f'    ("{k}", "{v}"),\n' for k, v in sorted(table.items()))
    return (
        "//! The keyword spellings of QB64pe's layout: every word of an `SCase$(\"...\")` or `SCase2$(\"...\")`\n"
        "//! literal of its sources, in upper case with its spelling, in order of the upper-case word (design D3 of\n"
        "//! `m5b-formatter`). Written by `tools\\format\\layout_words.py` from the reference clone; not edited by hand.\n"
        "\n"
        "pub const WORDS: &[(&str, &str)] = &[\n"
        f"{rows}"
        "];\n"
    )


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--check", action="store_true", help="compare, write nothing")
    ap.add_argument("--clone", type=Path, default=CLONE, help="the QB64pe reference clone (default: %(default)s)")
    args = ap.parse_args()
    try:
        text = render(scan(read_sources(args.clone)))
    except WordsError as e:
        print(e, file=sys.stderr)
        return 2
    current = WORDS_RS.read_bytes().decode("utf-8") if WORDS_RS.is_file() else None
    if args.check:
        if current != text:
            print(f"{WORDS_RS.relative_to(REPO).as_posix()} differs from what the clone's sources give")
            return 1
        print(f"{WORDS_RS.relative_to(REPO).as_posix()}: as written")
        return 0
    if current != text:
        WORDS_RS.write_bytes(text.encode("utf-8"))
        print(f"wrote {WORDS_RS.relative_to(REPO).as_posix()}")
    else:
        print(f"{WORDS_RS.relative_to(REPO).as_posix()}: unchanged")
    return 0


if __name__ == "__main__":
    sys.exit(main())
