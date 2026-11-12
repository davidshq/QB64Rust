"""Tests of reference.py (tools/reference/README.md).

Run: python -m unittest discover -s tools/reference -p "test_*.py"

The pure functions are tested alone. The page check and the overlap check run on pages written for the test under
target/scratch, with the module's folders pointed there; the inventory and the sources are the repository's own.
"""

import pathlib
import shutil
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import reference as r

GOOD_PAGE = """# LEFT$

`LEFT$` gives the first characters of a string.

## Syntax

<!-- begin generated: syntax -->
<!-- end generated -->

## Behavior

A count of 0 gives `""`.
<!-- src: verification\\v20_c_string_edges -->

## Errors

These are compile errors:

- One argument.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
<!-- end generated -->

## See also

`RIGHT$`.
"""


class Words(unittest.TestCase):
    def test_page_names(self):
        self.assertEqual(r.page_name("LEFT$"), "left-str")
        self.assertEqual(r.page_name("$INCLUDE"), "meta-include")
        self.assertEqual(r.page_name("_MEMCOPY"), "_memcopy")
        self.assertEqual(r.page_name("LINE INPUT"), "line-input")
        self.assertEqual(r.page_name("INPUT$"), "input-str")

    def test_a_compound_is_one_word(self):
        self.assertEqual(r.words_in("LINE INPUT a$"), {"LINE INPUT", "A$"})
        self.assertEqual(r.words_in("LINE (1, 2)-(3, 4)"), {"LINE"})

    def test_in_a_spec_only_capitals_count(self):
        self.assertEqual(r.code_words("`RUN line` and `PRINT x`"), {"RUN", "PRINT"})

    def test_the_statement_of_a_callsite_program(self):
        text = "$CONSOLE:ONLY\n' comment\nDIM n AS LONG\nLINE (n, n)-(n, n)\nEND\n"
        self.assertEqual(r.callsite_statement(text), "LINE (n, n)-(n, n)")

    def test_the_form_of_a_special_format(self):
        entry = {"name": "Foo", "kind": "sub", "arg_types": ["LONG", "STRING"], "specialformat": "?{To}?"}
        self.assertEqual(r.form(entry), "FOO LONGToSTRING")
        entry = {"name": "Bar", "kind": "function", "arg_types": ["LONG"], "ret": "STRING", "musthave": "$"}
        self.assertEqual(r.form(entry), "BAR$(LONG) AS STRING")

    def test_the_syntax_writes_keywords_in_capitals_and_one_form_per_line(self):
        entry = {"name": "_ScreenMove", "kind": "sub", "hr_syntax": "_SCREENMOVE x, y or _SCREENMOVE _Middle"}
        self.assertEqual(r.syntax_forms(entry), ["_SCREENMOVE x, y", "_SCREENMOVE _MIDDLE"])
        entry = {"name": "Key", "kind": "sub", "hr_syntax": "KEY(number) {On|Off|Stop}"}
        self.assertEqual(r.syntax_forms(entry), ["KEY(number) {ON|OFF|STOP}"])
        # An argument name keeps its spelling, `Array` of graphics PUT included.
        entry = {"name": "Put", "kind": "sub", "hr_syntax": "PUT (column, row), Array([index]), omitColor"}
        self.assertEqual(r.syntax_forms(entry), ["PUT (column, row), Array([index]), omitColor"])

    def test_a_hand_written_syntax_keeps_the_words_of_the_table(self):
        forms = ["_PUTIMAGE [STEP](dx1, dy1)[, source][_SMOOTH]"]
        self.assertEqual(r.syntax_words_left_out("_PUTIMAGE (dx1, dy1)[, source]", forms), ["STEP", "_SMOOTH"])
        self.assertEqual(r.syntax_words_left_out("_PUTIMAGE [STEP](x, y)[, s][, _SMOOTH]", forms), [])
        # Capitals that are expressions or a string's text are not required.
        self.assertEqual(r.syntax_words_left_out("RANDOMIZE [seed]", ["RANDOMIZE [USING] {seednumber|TIMER}"]), [])


class Inventory(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.words = r.inventory()

    def test_built_ins_and_keywords(self):
        self.assertEqual(self.words["LEFT$"].page, "left-str")
        self.assertIn("function", self.words["LEFT$"].kinds)
        self.assertIn("keyword", self.words["IF"].kinds)
        self.assertIn("metacommand", self.words["$INCLUDE"].kinds)

    def test_a_word_covered_on_another_page_takes_its_batch(self):
        self.assertEqual(self.words["THEN"].page, "if")
        self.assertEqual(self.words["THEN"].batch, self.words["IF"].batch)

    def test_skipped_words_have_no_page(self):
        self.assertNotIn("OFF", self.words)

    def test_every_word_has_a_batch_with_a_name(self):
        for w in self.words.values():
            self.assertTrue(w.area, w.word)


class Availability(unittest.TestCase):
    def test_texts(self):
        self.assertIn("Not checked", r.availability_text("p", {}))
        one = {"p": {"programs": 1, "ok": 1, "unsupported": [], "rejected": []}}
        self.assertIn("The one program", r.availability_text("p", one))
        part = {"p": {"programs": 3, "ok": 1, "unsupported": ["not supported yet: x"], "rejected": []}}
        self.assertIn("Partly supported: 1 of 3", r.availability_text("p", part))
        rejected = {"p": {"programs": 2, "ok": 0, "unsupported": [], "rejected": ["`FRE` is not implemented"]}}
        self.assertTrue(r.availability_text("p", rejected).startswith("Rejected, as in QB64pe"))


class OpenGl(unittest.TestCase):
    def test_a_routine_with_and_without_a_helper(self):
        text = """
    GlRoutine {
        name: "_glAccum",
        c_name: "call_glAccum",
        ret: None,
        args: &[Ty::U32, Ty::F32],
    },
    GlRoutine {
        name: "_glFlush",
        c_name: "glFlush",
        ret: None,
        args: &[],
    },
    GlRoutine {
        name: "_glGetString",
        c_name: "call_glGetString",
        ret: Some(Ty::Str),
        args: &[Ty::U32],
    },
"""
        self.assertEqual(
            r.gl_routines(text),
            [
                ("_glAccum", True, None, ["_UNSIGNED LONG", "SINGLE"]),
                ("_glFlush", False, None, []),
                ("_glGetString", True, "STRING", ["_UNSIGNED LONG"]),
            ],
        )

    def test_the_table_reads_the_repository_s_list(self):
        block = r.gl_block()
        self.assertIn("| `_glViewport` | SUB | LONG, LONG, LONG, LONG | error 270 |", block)
        self.assertIn("337 routines", block)


class Appendices(unittest.TestCase):
    def test_the_error_messages(self):
        texts = dict(r.error_texts())
        self.assertEqual(texts[5], "Illegal function call")
        self.assertEqual(texts[315], "_ASSERT failed (check console for description)")
        self.assertNotIn(0, texts)

    def test_the_deferred_rows(self):
        rows = r.deferred_rows()
        self.assertTrue(rows)
        self.assertTrue(all(form and form != "Form" for _, form in rows))

    def test_the_index_lists_every_appendix(self):
        index = r.index_md(r.inventory())
        for file, title in r.APPENDICES:
            self.assertIn(f"[{title}]({file})", index)


class Sources(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.reqs = {(q.spec, q.heading) for q in r.load_requirements()}
        cls.rows = {"D-001", "Q-002"}

    def check(self, item):
        return r.check_src(item, self.reqs, self.rows)

    def test_a_requirement(self):
        self.assertIsNone(self.check('language/builtin-functions "Names and arity"'))
        self.assertIn("no requirement", self.check('language/builtin-functions "No such thing"'))

    def test_a_file_with_or_without_bas(self):
        self.assertIsNone(self.check("verification\\v20_c_string_edges"))
        self.assertIsNone(self.check("read: libqb\\internal\\c\\libqb.cpp:8701-8766"))
        self.assertIn("no file", self.check("verification\\v99_nothing"))

    def test_a_row(self):
        self.assertIsNone(self.check("D-001"))
        self.assertIn("no divergence row", self.check("D-999"))


class Pages(unittest.TestCase):
    """The page check on pages written for the test."""

    def setUp(self):
        scratch = r.REPO / "target" / "scratch"
        scratch.mkdir(parents=True, exist_ok=True)
        self.tmp = pathlib.Path(tempfile.mkdtemp(prefix="reference-tests-", dir=scratch))
        base = self.tmp.relative_to(r.REPO)
        self.entries = self.tmp / "entries"
        self.entries.mkdir()
        (self.tmp / "examples").mkdir()
        self.patch = mock.patch.multiple(r, ENTRIES=base / "entries", EXAMPLES=base / "examples")
        self.patch.start()
        self.words = r.inventory()
        self.reqs = {(q.spec, q.heading) for q in r.load_requirements()}

    def tearDown(self):
        self.patch.stop()
        shutil.rmtree(self.tmp)

    def problems(self, text, name="left-str"):
        path = self.entries / f"{name}.md"
        text = r.refresh(text, "LEFT$", self.words, {})
        path.write_text(text, encoding="utf-8")
        return r.check_page(path, self.words, {}, self.reqs, {"Q-037"})

    def test_a_good_page(self):
        self.assertEqual(self.problems(GOOD_PAGE), [])

    def test_a_rule_without_a_source(self):
        text = GOOD_PAGE.replace("<!-- src: verification\\v20_c_string_edges -->", "")
        self.assertTrue(any("names no source" in p for p in self.problems(text)))

    def test_a_source_that_does_not_exist(self):
        text = GOOD_PAGE.replace("v20_c_string_edges", "v20_c_nothing")
        self.assertTrue(any("no file" in p for p in self.problems(text)))

    def test_a_missing_heading_and_a_todo(self):
        text = GOOD_PAGE.replace("## See also\n\n`RIGHT$`.\n", "TODO\n")
        found = self.problems(text)
        self.assertTrue(any("missing `## See also`" in p for p in found))
        self.assertTrue(any("TODO" in p for p in found))

    def test_headings_out_of_order(self):
        text = GOOD_PAGE.replace("## Behavior", "## Tmp").replace("## Errors", "## Behavior").replace("## Tmp", "## Errors")
        self.assertTrue(any("out of order" in p for p in self.problems(text)))

    def test_a_generated_block_edited_by_hand(self):
        path = self.entries / "left-str.md"
        path.write_text(r.refresh(GOOD_PAGE, "LEFT$", self.words, {}).replace("LEFT$(stringValue$", "LEFT$(s$"))
        found = r.check_page(path, self.words, {}, self.reqs, set())
        self.assertTrue(any("out of date" in p for p in found))

    def test_a_link_that_leads_nowhere(self):
        text = GOOD_PAGE.replace("`RIGHT$`.", "[RIGHT$](right-str.md).")
        self.assertTrue(any("leads nowhere" in p for p in self.problems(text)))

    def test_a_page_no_word_has(self):
        found = self.problems(GOOD_PAGE, name="no-such-word")
        self.assertTrue(any("no word of the inventory" in p for p in found))

    def chapter_problems(self, text, name="control-flow"):
        path = self.tmp / f"{name}.md"
        path.write_text(text, encoding="utf-8")
        return r.check_chapter(path, self.reqs, set())

    def test_a_good_chapter(self):
        text = (
            "# Control flow\n\nHow a program chooses and repeats.\n\n## Loops\n\n"
            "A `FOR` loop counts.\n<!-- src: language/control-flow \"FOR loops\" -->\n"
        )
        self.assertEqual(self.chapter_problems(text), [])

    def test_a_chapter_rule_without_a_source_and_a_chapter_of_no_spec(self):
        text = "# Odd\n\nA summary.\n\n## Part\n\nA rule with no source.\n"
        self.assertTrue(any("names no source" in p for p in self.chapter_problems(text)))
        self.assertTrue(any("no spec language/no-such-area" in p for p in self.chapter_problems(text, "no-such-area")))

    def test_an_example_is_embedded_with_its_output(self):
        (self.tmp / "examples" / "left-str_1.bas").write_text('$CONSOLE:ONLY\nPRINT LEFT$("ab", 1)\nEND\n')
        (self.tmp / "examples" / "left-str_1.output").write_bytes(b"a\r\n")
        block = r.example_block("left-str_1")
        self.assertIn('PRINT LEFT$("ab", 1)', block)
        self.assertNotIn("$CONSOLE:ONLY", block)
        self.assertIn("Output:\n\n```text\na\n```", block)

    def test_overlap_finds_copied_prose_and_code(self):
        wiki = self.tmp / "wiki"
        wiki.mkdir()
        (wiki / "1_LEFT$.wikitext").write_text(
            "The [[LEFT$]] function returns a part of a string from the start, counted in bytes always.\n"
            "{{CodeStart}}\n{{Cl|PRINT}} 1\n{{Cl|PRINT}} 2\n{{Cl|PRINT}} 3\n{{Cl|PRINT}} 4\n"
            "{{Cl|PRINT}} 5\n{{Cl|PRINT}} 6\n{{CodeEnd}}\n",
            encoding="utf-8",
        )
        self.problems(GOOD_PAGE)
        self.assertEqual(r.overlap(wiki), [])
        copied = GOOD_PAGE.replace(
            "A count of 0", "The LEFT$ function returns a part of a string from the start, counted in bytes. A count of 0"
        )
        self.problems(copied)
        (self.tmp / "examples" / "left-str_1.bas").write_text("PRINT 1\nPRINT 2\nPRINT 3\nPRINT 4\nPRINT 5\nPRINT 6\n")
        found = r.overlap(wiki)
        self.assertTrue(any("shares 10 words" in p for p in found), found)
        self.assertTrue(any("shares 6 lines" in p for p in found), found)


if __name__ == "__main__":
    unittest.main()
