"""Tests of qb45.py: the source as DOS reads it, the DOSBox configuration and the result text.

Run: python -m unittest discover -s tools/qb45 -p "test_*.py"
"""

import pathlib
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import qb45

BC_CLEAN = """Microsoft (R) QuickBASIC Compiler Version 4.50
(C) Copyright Microsoft Corporation 1982-1988.
All rights reserved.
Simultaneously published in the U.S. and Canada.

44045 Bytes Available
43272 Bytes Free

    0 Warning Error(s)
    0 Severe  Error(s)
"""

BC_ERRORS = """Microsoft (R) QuickBASIC Compiler Version 4.50
(C) Copyright Microsoft Corporation 1982-1988.
All rights reserved.
Simultaneously published in the U.S. and Canada.
 00DC   0038    PRINT #1, a(1)
                          ^\x07 Wrong number of dimensions

44045 Bytes Available
43415 Bytes Free

    0 Warning Error(s)
    1 Severe  Error(s)
"""


class DosTextTest(unittest.TestCase):
    def test_line_ends_become_crlf(self):
        self.assertEqual(qb45.dos_text("a\nb\n"), b"a\r\nb\r\n")
        self.assertEqual(qb45.dos_text("a\r\nb"), b"a\r\nb\r\n")


class ConfTest(unittest.TestCase):
    def test_mounts_builds_and_exits(self):
        text = qb45.conf_text(pathlib.Path("Q"), pathlib.Path("W"))
        self.assertIn('mount c "Q"', text)
        self.assertIn('mount d "W"', text)
        self.assertIn("bc P.BAS /O /E /X /D; > P.CMP", text)
        self.assertTrue(text.rstrip().endswith("exit"))


class ResultTest(unittest.TestCase):
    def test_a_clean_build_shows_only_the_output(self):
        self.assertEqual(qb45.compile_messages(BC_CLEAN), [])
        self.assertEqual(qb45.result_text(BC_CLEAN, "x\r\n", False, 60), "x\n")

    def test_errors_are_kept_without_the_banner(self):
        messages = qb45.compile_messages(BC_ERRORS)
        self.assertEqual(messages[0], " 00DC   0038    PRINT #1, a(1)")
        self.assertEqual(messages[1].strip(), "^ Wrong number of dimensions")
        self.assertIn("    1 Severe  Error(s)", messages)
        self.assertFalse(any("Bytes" in m or "Microsoft" in m for m in messages))

    def test_a_failed_link_is_shown_without_its_banner(self):
        link = (
            "\nMicrosoft (R) Overlay Linker  Version 3.69\n"
            "Copyright (C) Microsoft Corp 1983-1988.  All rights reserved.\n\n"
            "LINK : fatal error L1093: BCOM45.LIB : object not found\n"
        )
        text = qb45.result_text(BC_CLEAN, None, False, 60, link)
        self.assertEqual(
            text,
            "link (no executable):\nLINK : fatal error L1093: BCOM45.LIB : object not found\n"
            "(no OUT.TXT)\n",
        )

    def test_no_output_and_a_timeout_are_said(self):
        self.assertEqual(qb45.result_text(BC_CLEAN, None, False, 60), "(no OUT.TXT)\n")
        self.assertIn("timed out after 5 s", qb45.result_text(BC_CLEAN, "", True, 5))


if __name__ == "__main__":
    unittest.main()
