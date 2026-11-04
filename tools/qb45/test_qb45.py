"""Tests of qb45.py: the source as DOS reads it, the DOSBox configuration and the result text.

Run: python -m unittest discover -s tools/qb45 -p "test_*.py"
"""

import contextlib
import io
import pathlib
import sys
import tempfile
import unittest
from unittest import mock

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


class BcFinishedTest(unittest.TestCase):
    def test_bc_finished_when_its_summary_is_there(self):
        self.assertTrue(qb45.bc_finished(BC_CLEAN))
        self.assertTrue(qb45.bc_finished(BC_ERRORS))

    def test_bc_did_not_finish_without_its_summary(self):
        # No P.CMP (read as ""), an empty one, or BC's banner alone: the run of 2026-11-02 that gave
        # neither messages nor OUT.TXT.
        banner_only = "\n".join(BC_CLEAN.split("\n")[:4]) + "\n"
        for log in ("", "\r\n", banner_only):
            self.assertFalse(qb45.bc_finished(log), repr(log))

    def test_the_message_says_whether_dosbox_hung(self):
        hung = qb45.Run("", False, True, False)
        ended = qb45.Run("", False, False, False)
        self.assertIn("timed out after 60 s", qb45.no_build_message(hung, 60))
        self.assertIn("DOSBox ended first", qb45.no_build_message(ended, 60))


NO_BUILD = qb45.Run("", False, False, False)


class MainTest(unittest.TestCase):
    def main_with(self, results):
        """Runs main --record on a program with run() giving results; returns (status, record or
        None, stderr, the number of runs)."""
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root / "BC.EXE").write_bytes(b"")
            dosbox = root / "DOSBox.exe"
            dosbox.write_bytes(b"")
            bas = root / "p.bas"
            bas.write_text("PRINT 1\n", encoding="utf-8")
            err = io.StringIO()
            with (
                mock.patch.object(qb45, "run", side_effect=results) as run,
                contextlib.redirect_stdout(io.StringIO()),
                contextlib.redirect_stderr(err),
            ):
                status = qb45.main(
                    ["--qb45", str(root), "--dosbox", str(dosbox), "--record", str(bas)]
                )
            record = root / "p.qb45.txt"
            text = record.read_text(encoding="utf-8") if record.exists() else None
            return status, text, err.getvalue(), run.call_count

    def test_a_run_with_no_build_is_tried_twice_and_not_recorded(self):
        status, record, err, runs = self.main_with([NO_BUILD, NO_BUILD])
        self.assertEqual((status, record, runs), (1, None, 2))
        self.assertIn("nothing recorded", err)

    def test_what_bc_wrote_is_shown_when_it_did_not_finish(self):
        # BC stopped after a message, before its summary: the message goes to stderr, not a record.
        stopped = "\n".join(BC_ERRORS.split("\n")[:6]) + "\n"
        self.assertFalse(qb45.bc_finished(stopped))
        result = qb45.Run("", False, False, False, stopped)
        status, record, err, runs = self.main_with([result, result])
        self.assertEqual((status, record, runs), (1, None, 2))
        self.assertIn("^ Wrong number of dimensions", err)
        self.assertNotIn("Microsoft", err)

    def test_the_log_says_when_bc_wrote_nothing_but_its_banner(self):
        banner_only = "\n".join(BC_CLEAN.split("\n")[:4]) + "\n"
        self.assertIn("no messages from BC", qb45.unfinished_log(banner_only))
        self.assertIn("no P.CMP", qb45.unfinished_log(""))

    def test_a_second_run_that_builds_is_recorded(self):
        status, record, err, runs = self.main_with([NO_BUILD, qb45.Run("1\n", True, False, False)])
        self.assertEqual((status, record, runs), (0, "1\n", 2))
        self.assertNotIn("warning", err)

    def test_a_run_with_no_out_txt_is_recorded_with_a_warning(self):
        silent = qb45.Run("(no OUT.TXT)\n", True, False, True)
        status, record, err, runs = self.main_with([silent])
        self.assertEqual((status, record, runs), (0, "(no OUT.TXT)\n", 1))
        self.assertIn("wrote no OUT.TXT", err)


if __name__ == "__main__":
    unittest.main()
