"""Tests of layout_words.py (design D3 of m5b-formatter): which SCase$ calls give words, and how.

Run: python -m unittest discover -s tools/format -p "test_*.py"
"""

import pathlib
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import layout_words


class LiteralTest(unittest.TestCase):
    def test_a_literal(self):
        self.assertEqual(layout_words.literal_text('"$Include"'), "$Include")

    def test_literals_joined_by_separators(self):
        self.assertEqual(layout_words.literal_text('"End" + sp + "If"'), "End If")
        self.assertEqual(layout_words.literal_text('"Palette" + sp + "Using" + sp'), "Palette Using ")
        self.assertEqual(layout_words.literal_text('"a" + SP2 + "b"'), "a b")

    def test_a_computed_argument_gives_nothing(self):
        for arg in ["t$", "RTRIM$(id.cn)", '"Call" + n$', '"x" + "y" + z$', ""]:
            self.assertIsNone(layout_words.literal_text(arg), arg)


class ScanTest(unittest.TestCase):
    def scan(self, text):
        return layout_words.scan([("a.bas", text)])

    def test_words_from_each_call(self):
        got = self.scan('l$ = SCase$("End" + sp + "If"): x$ = SCase2$("_Dynamic")\nlayout$ = SCase$(" Then")\n')
        self.assertEqual(got, {"END": "End", "IF": "If", "_DYNAMIC": "_Dynamic", "THEN": "Then"})

    def test_metacommand_spellings_keep_their_colon_and_quote(self):
        got = self.scan('l$ = SCase$("$ExeIcon:\'") + x$\nl$ = SCase$("$Resize:Stretch")\n')
        self.assertEqual(got, {"$EXEICON:'": "$ExeIcon:'", "$RESIZE:STRETCH": "$Resize:Stretch"})

    def test_calls_in_comments_and_computed_calls_are_left_out(self):
        got = self.scan("x$ = SCase$(t$) ' SCase$(\"Wrong\")\ny$ = SCase$(RTRIM$(id.cn))\n")
        self.assertEqual(got, {})

    def test_a_word_spelled_two_ways_stops_the_tool(self):
        with self.assertRaises(layout_words.WordsError):
            self.scan('a$ = SCase$("GoTo")\nb$ = SCase$("Goto")\n')

    def test_the_same_spelling_twice_is_one_word(self):
        self.assertEqual(self.scan('a$ = SCase$("To")\nb$ = SCase$("To")\n'), {"TO": "To"})


class RenderTest(unittest.TestCase):
    def test_sorted_by_the_upper_case_word(self):
        text = layout_words.render({"THEN": "Then", "$IF": "$If"})
        self.assertLess(text.index('("$IF", "$If")'), text.index('("THEN", "Then")'))
        self.assertIn("pub const WORDS: &[(&str, &str)] = &[", text)


class CloneTest(unittest.TestCase):
    @unittest.skipUnless((layout_words.CLONE / "source").is_dir(), "needs the QB64pe reference clone")
    def test_the_written_table_is_the_clones(self):
        text = layout_words.render(layout_words.scan(layout_words.read_sources(layout_words.CLONE)))
        self.assertEqual(text, layout_words.WORDS_RS.read_bytes().decode("utf-8"))
        for word in ['("END", "End")', '("GOSUB", "GoSub")', '("_ARRAYCOPY", "_ArrayCopy")']:
            self.assertIn(word, text)


if __name__ == "__main__":
    unittest.main()
