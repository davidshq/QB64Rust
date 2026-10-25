"""Tests of old_wording.py's rules that need no compiler: python -m unittest discover -s tools/diagnostics"""

import unittest

import old_wording


class NamesFile(unittest.TestCase):
    def test_words_joined_by_a_slash_are_no_path(self):
        for old in (
            "Illegal SUB/FUNCTION parameter",
            "Expected variable/value after 'NOT'",
            "Expected STRING * number/constant",
            "Embed-Handle 'h' has invalid chars, use Aa-Zz/0-9 only",
        ):
            self.assertFalse(old_wording.names_file(old), old)

    def test_recorded_paths_are_files(self):
        for old in (
            "File v16_only_main.bi not found in line 2 of <verification>\\v16_inc/x2.bi included",
            "File '<verification>\\v29_inc/missing.ico' not found",
            "Could not find sub/function 'nothing_here' in 'internal\\c\\parts\\core\\core.a'",
            "Syntax error in line 1 of x.bm included",
        ):
            self.assertTrue(old_wording.names_file(old), old)


class PatternOf(unittest.TestCase):
    def test_a_line_number_is_a_placeholder(self):
        for number in ("10", "010&", "10.5", "30#", "12345678901234567890123456789012345678901"):
            message = f"`{number}` is not a valid label"
            self.assertEqual(old_wording.pattern_of(message), ("`{}` is not a valid label", [number]), number)

    def test_keywords_and_signs_stay(self):
        self.assertEqual(old_wording.pattern_of("`STRING * 0` needs `<>`"), ("`STRING * 0` needs `<>`", []))


class OneProgram(unittest.TestCase):
    def test_a_number_the_pattern_does_not_carry(self):
        for old in ("Syntax error in line 4", "1st sub argument requires a number", "Value 1.5 out of range"):
            self.assertTrue(old_wording.one_program(old, "a number is required here"), old)

    def test_a_name_the_pattern_does_not_carry(self):
        self.assertTrue(old_wording.one_program("Cannot convert type (FOO)", "a value of `{}`"))

    def test_an_ordinal_the_pattern_holds(self):
        old = "2nd sub argument requires a number"
        self.assertFalse(old_wording.one_program(old, "a number is required for the 2nd argument"))
        self.assertTrue(old_wording.one_program(old, "a number is required for the 3rd argument"))

    def test_placeholders_and_plain_text(self):
        self.assertFalse(old_wording.one_program("Variable '{1}' ({2:U}) not defined", "variable `{}` is `{}`"))


if __name__ == "__main__":
    unittest.main()
