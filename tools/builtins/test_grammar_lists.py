"""Tests of grammar_lists.py, and the check that the VS Code grammar's keyword lists match builtins.json."""

from __future__ import annotations

import json
import unittest

import grammar_lists as gl

TAIL = r")(?![\w.$%&!#~`])"


def pattern(name: str, words: list[str]) -> dict:
    return {"name": name, "match": r"(?i)(?<![\w.])(?:" + "|".join(words) + TAIL}


def grammar() -> dict:
    """A grammar with the patterns grammar_lists.py rewrites, its lists out of date."""
    return {
        "repository": {
            "label": {
                "patterns": [
                    {"match": r"^\s*(\d+)(?=\s|:|$)"},
                    {"match": r"(?i)^\s*(?!(?:OLD)(?![\w.$%&!#~`]))([A-Z_][A-Z0-9_.]*)(:)(?!=)"},
                ]
            },
            "keywords": {
                "patterns": [
                    pattern(gl.HAND[0], ["GOTO", "END"]),
                    pattern(gl.HAND[1], ["DIM", "READ"]),
                    pattern(gl.HAND[2], ["LONG"]),
                    pattern(gl.HAND[3], ["MOD"]),
                    pattern(gl.CONSTANTS, ["OLD"]),
                    pattern(gl.COLORS, ["OLD"]),
                    {
                        "name": gl.FUNCTIONS,
                        "match": r"(?i)(?<![\w.])(?:OLD)(?:\$|(?![\w.$%&!#~`]))",
                    },
                ]
            },
        }
    }


TABLE = {
    "entries": [
        {"name": "Print", "kind": "sub"},
        {"name": "_NewBuiltin", "kind": "function"},
        {"name": "Chr$", "kind": "function"},
        {"name": "Chr$", "kind": "function"},
        {"name": "End", "kind": "sub"},
        {"name": "Read", "kind": "sub"},
    ],
    "auto_include": [
        {"name": "_TRUE", "kind": "const", "file": "include/beforefirstline.bi"},
        {"name": "_PI", "kind": "const", "file": "include/beforefirstline.bi"},
        {"name": "Red", "kind": "const", "file": "color/color0.bi"},
        {"name": "AliceBlue", "kind": "const", "file": "color/color32.bi"},
        {"name": "_WhatIsMyIP", "kind": "function", "file": "include/afterlastline.bm"},
    ],
}


def words(g: dict, name: str) -> list[str]:
    if name == "label":
        return gl.words_of(g["repository"]["label"]["patterns"][1]["match"])
    (p,) = [p for p in g["repository"]["keywords"]["patterns"] if p["name"] == name]
    return gl.words_of(p["match"])


class GenerateTest(unittest.TestCase):
    def setUp(self):
        self.old = grammar()
        self.new = gl.generate(self.old, TABLE)

    def test_functions_are_the_entries_less_the_hand_words(self):
        # A new built-in comes in; END and READ stay colored as the hand-written lists color them; the $ goes.
        self.assertEqual(words(self.new, gl.FUNCTIONS), ["_NEWBUILTIN", "PRINT", "CHR"])

    def test_constants_and_colors_come_from_their_files(self):
        self.assertEqual(words(self.new, gl.CONSTANTS), ["_TRUE", "_PI"])
        self.assertEqual(words(self.new, gl.COLORS), ["ALICEBLUE", "RED"])

    def test_label_lookahead_holds_every_keyword_but_the_constants(self):
        self.assertEqual(
            words(self.new, "label"),
            gl.ordered(["GOTO", "END", "DIM", "READ", "LONG", "MOD", "_NEWBUILTIN", "PRINT", "CHR"]),
        )

    def test_hand_lists_and_the_rest_of_a_pattern_are_kept(self):
        for name in gl.HAND:
            self.assertEqual(words(self.new, name), words(self.old, name))
        fn = [p for p in self.new["repository"]["keywords"]["patterns"] if p["name"] == gl.FUNCTIONS][0]
        self.assertTrue(fn["match"].startswith(r"(?i)(?<![\w.])(?:_NEWBUILTIN|"))
        self.assertTrue(fn["match"].endswith(r")(?:\$|(?![\w.$%&!#~`]))"))
        self.assertEqual(self.new["repository"]["label"]["patterns"][0], self.old["repository"]["label"]["patterns"][0])

    def test_the_argument_is_not_changed(self):
        self.assertEqual(self.old, grammar())

    def test_a_missing_pattern_is_an_error(self):
        g = grammar()
        g["repository"]["keywords"]["patterns"].pop()
        with self.assertRaises(SystemExit):
            gl.generate(g, TABLE)


class TheRealGrammarTest(unittest.TestCase):
    def test_grammar_matches_the_table(self):
        """Fails when a built-in was added to builtins.json and the grammar not written again."""
        old = gl.GRAMMAR.read_bytes().decode("utf-8").replace("\r\n", "\n")
        new = gl.text(gl.generate(json.loads(old), json.loads(gl.TABLE.read_bytes())))
        self.assertEqual(new, old, "run python tools/builtins/grammar_lists.py")


if __name__ == "__main__":
    unittest.main()
