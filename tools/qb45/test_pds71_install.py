"""Tests of pds71_install.py: the expander, the file names and LIB's response file.

Run: python -m unittest discover -s tools/qb45 -p "test_*.py"
"""

import pathlib
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import pds71_install as pds


def qb_file(size: int, body: bytes) -> bytes:
    """A file in the form on PDS 7.1's disks: the 8-byte header, the size, the LZSS data."""
    return pds.MAGIC_QB + size.to_bytes(4, "little") + body


class ExpandTest(unittest.TestCase):
    def test_literals(self):
        # Flag byte FF: eight literal bytes follow.
        self.assertEqual(pds.expand_file(qb_file(8, b"\xffABCDEFGH")), b"ABCDEFGH")

    def test_a_back_reference_repeats_what_was_written(self):
        # "AB" as literals (flags 0b011), then a reference to their window position, 4 bytes long.
        # The window starts 18 bytes before its end: "A" is at 4078 = 0xFEE.
        offset = 4096 - 18
        ref = bytes([offset & 0xFF, ((offset >> 4) & 0xF0) | (4 - 3)])
        self.assertEqual(pds.expand_file(qb_file(6, b"\x03AB" + ref)), b"ABABAB")

    def test_the_window_starts_as_blanks(self):
        # A reference before anything is written reads the window's blanks.
        self.assertEqual(pds.expand_file(qb_file(3, b"\x00\x00\x00")), b"   ")

    def test_an_uncompressed_file_is_not_expanded(self):
        self.assertIsNone(pds.expand_file(b"MZ not compressed"))


class NameTest(unittest.TestCase):
    NAMES = pds.packing_names(
        "BC.EXE\t\tMicrosoft BASIC Compiler\n"
        "B71ORN.LIB\t\tBASIC run-time constituent library\n"
        "\t\tprograms that use some devices\n"
    )

    def test_packing_list_gives_the_last_letter(self):
        self.assertEqual(pds.real_name("BC.EX$", self.NAMES), "BC.EXE")
        self.assertEqual(pds.real_name("b71orn.li$", self.NAMES), "B71ORN.LIB")

    def test_a_two_letter_extension_loses_its_dollar(self):
        self.assertEqual(pds.real_name("QBX.BI$", self.NAMES), "QBX.BI")
        self.assertEqual(pds.real_name("MXADSTC.C$$", self.NAMES), "MXADSTC.C")
        self.assertEqual(pds.real_name("MXSHKF.FR$", self.NAMES), "MXSHKF.FOR")

    def test_a_plain_name_stays(self):
        self.assertEqual(pds.real_name("SETUP.EXE", self.NAMES), "SETUP.EXE")


class InstallTest(unittest.TestCase):
    def test_files_are_expanded_and_named(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            disk = root / "disks" / "Disk 1"
            disk.mkdir(parents=True)
            (disk / "PACKING.LST").write_text("BC.EXE\t\tcompiler\n", encoding="cp437")
            (disk / "BC.EX$").write_bytes(qb_file(2, b"\x03MZ"))
            (disk / "DATIM.BI$").write_bytes(b"' not compressed")
            out = root / "out"
            self.assertEqual(pds.install(root / "disks", out), 1)
            self.assertEqual((out / "BC.EXE").read_bytes(), b"MZ")
            self.assertEqual((out / "DATIM.BI").read_bytes(), b"' not compressed")


class LibraryTest(unittest.TestCase):
    def test_the_response_file_joins_every_part(self):
        text = pds.lib_response()
        self.assertTrue(text.startswith("BCL71ENR.LIB\r\ny\r\n"))
        for part in pds.PARTS:
            self.assertIn(f"+{part}.LIB", text)
        # A continued line ends with &, and the next one starts with + (not "++").
        self.assertNotIn("+&", text)
        self.assertTrue(text.endswith("NUL;\r\n"))
        self.assertTrue(all(len(line) < 120 for line in text.split("\r\n")))

    def test_a_hung_dosbox_is_said_not_raised(self):
        hang = pds.subprocess.TimeoutExpired("dosbox", pds.LIB_TIMEOUT)
        with mock.patch.object(pds.subprocess, "run", side_effect=hang):
            log = pds.build_library(pathlib.Path("out"), pathlib.Path("DOSBox.exe"))
        self.assertIn(f"DOSBox hung: stopped after {pds.LIB_TIMEOUT} s", log)
        self.assertIn("(no log)", log)

    def test_no_run_time_module_part_is_taken(self):
        # The t parts are for the run-time module; mixed in, the program hangs at start.
        for part in ("B71RCN", "B71RLN", "B71RORN", "B71ORN", "B71RORJ"):
            self.assertNotIn(part, pds.PARTS)


if __name__ == "__main__":
    unittest.main()
