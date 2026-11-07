#!/usr/bin/env python3
"""Writes the keyword lists of the VS Code grammar from the built-in table.

    python tools/builtins/grammar_lists.py           # rewrite vscode/syntaxes/qb64rust.tmLanguage.json
    python tools/builtins/grammar_lists.py --check   # exit 1 when the grammar's lists differ from the table

The grammar keeps four lists written by hand, the words of the language itself: `keyword.control`,
`storage.modifier`, `storage.type` and `keyword.operator.word`. Four lists are made from `builtins.json` here:

- `support.function`: every built-in SUB and FUNCTION of `entries`, upper case, without a `$`, less the words
  of the hand-written lists (`END`, `READ`, `_MEM`, ... are colored as those);
- `constant.language`: the constants of `include/beforefirstline.bi` (`_TRUE`, `_PI`, ...);
- `support.constant.color`: the constants of `color/color0.bi` and `color/color32.bi`;
- the lookahead of the label pattern: every word of the hand-written lists and of `support.function`, so that
  `PRINT:` is a statement, not a label.

Each list is sorted longest first, then alphabetically. The order does not change what matches: each list is
followed by a lookahead that ends the word, so a shorter word that is a prefix of a longer one does not stop it.
The file is written as `json.dumps(indent=2)` writes it, which is how it was written by hand.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TABLE = ROOT / "tools" / "builtins" / "builtins.json"
GRAMMAR = ROOT / "vscode" / "syntaxes" / "qb64rust.tmLanguage.json"

HAND = (
    "keyword.control.qb64rust",
    "storage.modifier.qb64rust",
    "storage.type.qb64rust",
    "keyword.operator.word.qb64rust",
)
FUNCTIONS = "support.function.qb64rust"
CONSTANTS = "constant.language.qb64rust"
COLORS = "support.constant.color.qb64rust"

# The first group of a pattern's match: the alternation of its words.
WORDS = re.compile(r"\(\?:([A-Z0-9_|]+)\)")


def ordered(words) -> list[str]:
    return sorted(set(words), key=lambda w: (-len(w), w))


def words_of(match: str) -> list[str]:
    m = WORDS.search(match)
    if not m:
        raise SystemExit(f"grammar_lists.py: no list of words in the pattern {match[:60]!r}")
    return m.group(1).split("|")


def with_words(match: str, words: list[str]) -> str:
    m = WORDS.search(match)
    return match[: m.start(1)] + "|".join(words) + match[m.end(1) :]


def lists(table: dict, hand: set[str]) -> dict[str, list[str]]:
    """The generated lists, by the scope name of their pattern ("label" for the label's lookahead)."""
    functions = {e["name"].upper().rstrip("$") for e in table["entries"]} - hand
    consts = [a for a in table["auto_include"] if a["kind"] == "const"]
    return {
        FUNCTIONS: ordered(functions),
        CONSTANTS: ordered(a["name"].upper() for a in consts if a["file"] == "include/beforefirstline.bi"),
        COLORS: ordered(a["name"].upper() for a in consts if a["file"].startswith("color/")),
        "label": ordered(hand | functions),
    }


def generate(grammar: dict, table: dict) -> dict:
    """The grammar with its generated lists written from the table (a copy; the argument is not changed)."""
    grammar = json.loads(json.dumps(grammar))
    keywords = {p["name"]: p for p in grammar["repository"]["keywords"]["patterns"]}
    missing = [n for n in (*HAND, FUNCTIONS, CONSTANTS, COLORS) if n not in keywords]
    if missing:
        raise SystemExit(f"grammar_lists.py: the grammar has no keyword pattern {', '.join(missing)}")
    hand = set().union(*(words_of(keywords[n]["match"]) for n in HAND))
    made = lists(table, hand)
    for name in (FUNCTIONS, CONSTANTS, COLORS):
        keywords[name]["match"] = with_words(keywords[name]["match"], made[name])
    labels = [p for p in grammar["repository"]["label"]["patterns"] if WORDS.search(p["match"])]
    if len(labels) != 1:
        raise SystemExit(f"grammar_lists.py: {len(labels)} label patterns hold a list of words, not 1")
    labels[0]["match"] = with_words(labels[0]["match"], made["label"])
    return grammar


def text(grammar: dict) -> str:
    return json.dumps(grammar, indent=2) + "\n"


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true", help="only compare; exit 1 when the grammar differs")
    a = ap.parse_args(argv)
    old = GRAMMAR.read_bytes().decode("utf-8")
    new = text(generate(json.loads(old), json.loads(TABLE.read_bytes())))
    rel = GRAMMAR.relative_to(ROOT).as_posix()
    if a.check:
        if new != old.replace("\r\n", "\n"):
            print(f"{rel}: its keyword lists differ from {TABLE.name}; run python tools/builtins/grammar_lists.py")
            return 1
        print(f"{rel}: its keyword lists match {TABLE.name}")
        return 0
    if new != old.replace("\r\n", "\n"):
        GRAMMAR.write_bytes(new.encode("utf-8"))
        print(f"{rel}: written")
    else:
        print(f"{rel}: unchanged")
    return 0


if __name__ == "__main__":
    sys.exit(main())
