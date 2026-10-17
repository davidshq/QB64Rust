#!/usr/bin/env python3
"""Tests of close.py: how each kind of log is judged, when the run without folding is asked for, and what the
plan holds.

    python -m unittest discover -s tools/close -p "test_*.py"

A change to a closing step or to how its log is judged comes with a change here.
"""

from __future__ import annotations

import io
import sys
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))

import close
from close import judge, needs_nofold, steps

CARGO_OK = """test a ... ok
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
"""
CARGO_FAIL = """test resource_files_equal_the_old_compilers ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
test result: ok. 46 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
error: 1 target failed:
"""


class Judges(unittest.TestCase):
    def test_cargo_pass_and_counts(self):
        self.assertEqual(judge("cargo", CARGO_OK, set()), (True, "18 passed, 0 failed"))

    def test_cargo_failure_unless_known(self):
        ok, summary = judge("cargo", CARGO_FAIL, set())
        self.assertFalse(ok)
        self.assertIn("not known: resource_files_equal_the_old_compilers", summary)
        ok, summary = judge("cargo", CARGO_FAIL, {"resource_files_equal_the_old_compilers"})
        self.assertTrue(ok)
        self.assertIn("known: resource_files_equal_the_old_compilers", summary)

    def test_cargo_failing_doctest(self):
        log = (
            "   Doc-tests qb64rust_base\n"
            "test crates\\base\\src\\lib.rs - show_bytes (line 3) ... FAILED\n"
            "test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s\n"
        )
        ok, summary = judge("cargo", log, set())
        self.assertFalse(ok)
        self.assertIn("not known: crates\\base\\src\\lib.rs - show_bytes (line 3)", summary)

    def test_cargo_failure_without_its_name(self):
        log = "test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s\n"
        ok, summary = judge("cargo", log, set())
        self.assertFalse(ok)
        self.assertIn("1 of them not named", summary)

    def test_cargo_binary_that_stopped(self):
        # The second binary crashed: a header, no result; cargo goes on (`--no-fail-fast`).
        log = (
            "     Running unittests src\\lib.rs (target\\debug\\deps\\a.exe)\n"
            + CARGO_OK.splitlines(keepends=True)[1]
            + "     Running tests\\inputs.rs (target\\debug\\deps\\inputs.exe)\n"
            "error: test failed, to rerun pass `-p qb64rust-driver --test inputs`\n"
            "error: 1 target failed:\n"
        )
        ok, summary = judge("cargo", log, set())
        self.assertFalse(ok)
        self.assertIn("1 test binaries stopped without a result", summary)

    def test_cargo_build_failure_and_no_result(self):
        self.assertFalse(judge("cargo", "error[E0425]: cannot find value\n" + CARGO_OK, set())[0])
        self.assertFalse(judge("cargo", "error: could not compile `x`\n", set())[0])
        self.assertFalse(judge("cargo", "", set())[0])

    def test_runner(self):
        ok_log = '[1/2] PASS a\n{"corpus": {"PASS": 449, "FAIL": 0, "KFAIL": 0}}\nresults: x.json\n'
        self.assertEqual(judge("runner", ok_log, set()), (True, "449 pass, 0 fail, 0 known"))
        self.assertFalse(judge("runner", '{"compile_tests": {"PASS": 3, "FAIL": 4, "KFAIL": 0}}\n', set())[0])
        # Refused for the clone's lock: no result line.
        self.assertFalse(judge("runner", "Another run holds the reference clone\n", set())[0])

    def test_callsite(self):
        self.assertTrue(judge("callsite", "1871 programs: 1871 equal, 0 differ, 0 not compared\n", set())[0])
        self.assertFalse(judge("callsite", "7 programs: 6 equal, 1 differ, 0 not compared\n", set())[0])
        self.assertFalse(judge("callsite", "Another run holds the reference clone\n", set())[0])

    def test_clippy_fmt_build_wording(self):
        self.assertTrue(judge("clippy", "    Finished `dev` profile\n", set())[0])
        self.assertFalse(judge("clippy", "warning: unused variable\n", set())[0])
        self.assertTrue(judge("empty", "", set())[0])
        self.assertFalse(judge("empty", "Diff in x.rs\n", set())[0])
        self.assertFalse(judge("build", "error: could not compile\n", set())[0])
        self.assertTrue(judge("wording", "828 notes on recorded rejections, 0 not the recorded message\n", set())[0])
        self.assertFalse(judge("wording", "828 notes on recorded rejections, 2 not the recorded message\n", set())[0])
        self.assertEqual(judge(None, "", set()), (True, "ran"))


class Plan(unittest.TestCase):
    def test_nofold_by_files(self):
        self.assertTrue(needs_nofold(["crates/sema/src/consteval.rs"]))
        self.assertTrue(needs_nofold(["crates/sema/src/check/constants.rs"]))
        self.assertFalse(needs_nofold(["crates/sema/src/check/mod.rs", "STATUS.md"]))

    def test_order(self):
        names = [s.name for s in steps(True)]
        # What changes the compiler or the lists comes before the build, the build before every run.
        self.assertLess(names.index("wording"), names.index("build"))
        self.assertLess(names.index("lists"), names.index("build"))
        self.assertEqual(names[names.index("build") + 1], "tier1")
        self.assertIn("nofold", names)
        self.assertNotIn("nofold", [s.name for s in steps(False)])

    def test_commands(self):
        by = {s.name: s.command for s in steps(True)}
        self.assertIn("--no-fail-fast", by["tier1"])
        self.assertIn("--locked", by["tier1"])
        self.assertIn("--pass-cache", by["slice"])
        self.assertIn("--pass-cache", by["upstream"])
        self.assertNotIn("--pass-cache", by["nofold"])
        self.assertIn("--wait", by["callsite"])
        for s in steps(True):
            # Each command writes its own log, and no command holds a backslash (CLAUDE.md working rule 7).
            self.assertIn(f"> target/close/{s.name}.log 2>&1", s.command)
            self.assertNotIn("\\", s.command)

    def test_plan_prints_every_step(self):
        out = io.StringIO()
        with mock.patch.object(close, "changed_files", return_value=["crates/sema/src/consteval.rs"]):
            with redirect_stdout(out):
                close.plan(False)
        text = out.getvalue()
        for s in steps(True):
            self.assertIn(s.command, text)

    def test_inputs(self):
        self.assertTrue(close.INPUTS.match("crates/sema/src/lib.rs"))
        self.assertTrue(close.INPUTS.match("tests/corpus/slice.list"))
        self.assertTrue(close.NOT_INPUTS.search("tests/upstream/README.md"))
        self.assertTrue(close.NOT_INPUTS.search("tools/testtime/runs.csv"))
        self.assertFalse(close.INPUTS.match("STATUS.md"))


class Check(unittest.TestCase):
    """`check` over a temporary repo: logs written at chosen times against a build and a changed source."""

    def setUp(self):
        import tempfile

        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "target/release").mkdir(parents=True)
        (self.root / "target/close").mkdir(parents=True)
        (self.root / "crates").mkdir()
        self.exe = self.root / close.EXE
        self.exe.write_text("exe")
        self.source = self.root / "crates/a.rs"
        self.source.write_text("fn a() {}")
        self.at(self.source, 100)
        self.at(self.exe, 200)
        good = {
            "build": "    Finished\n",
            "tier1": CARGO_OK,
            "fmt": "",
            "clippy": "",
            "lists": CARGO_OK,
            "wording-check": "1 notes on recorded rejections, 0 not the recorded message\n",
            "slice": '{"corpus": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
            "upstream": '{"compile_tests": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
            "callsite": "1 programs: 1 equal, 0 differ, 0 not compared\n",
            "cli": CARGO_OK,
        }
        for s in steps(False):
            path = self.root / close.log(s.name)
            path.write_text(good.get(s.name, ""))
            self.at(path, 150 if s.before_build and s.name != "build" else 300)
        self.patches = [
            mock.patch.object(close, "REPO", self.root),
            mock.patch.object(close, "changed_files", return_value=["crates/a.rs"]),
        ]
        for p in self.patches:
            p.start()

    def tearDown(self):
        for p in self.patches:
            p.stop()
        self.tmp.cleanup()

    @staticmethod
    def at(path: Path, t: float):
        import os

        os.utime(path, (t, t))

    def run_check(self, **kw) -> tuple[int, str]:
        out = io.StringIO()
        with redirect_stdout(out):
            code = close.check(False, set(), **kw)
        return code, out.getvalue()

    def test_all_good(self):
        code, text = self.run_check()
        self.assertEqual(code, 0, text)

    def test_missing(self):
        (self.root / close.log("callsite")).unlink()
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("MISSING  callsite", text)

    def test_stale_after_an_edit_and_not_repeated(self):
        self.at(self.source, 400)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    slice: older than crates/a.rs", text)
        reasons = {s.name: "a test-only edit" for s in steps(False)}
        code, text = self.run_check(not_repeated=reasons)
        self.assertEqual(code, 0, text)
        # The reason once, with the steps it covers; each step points to it.
        self.assertEqual(text.count("a test-only edit"), 1, text)
        self.assertIn("(1) not repeated after crates/a.rs (wording, lists, build, tier1,", text)
        self.assertIn("ok       slice: 1 pass, 0 fail, 0 known [", text)
        self.assertIn("; not repeated (1)", text)

    def test_a_run_after_an_edit_but_before_the_build(self):
        # Built at 200, the source edited at 250, the runs at 300, never built again.
        self.at(self.root / close.log("build"), 200)
        self.at(self.source, 250)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    build: target/release/qb64rust.exe is older than crates/a.rs", text)

    def test_a_build_log_newer_than_its_executable(self):
        # The log written after the edit, the executable not (a build that stopped, another profile).
        self.at(self.source, 250)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    build: target/release/qb64rust.exe is older than crates/a.rs", text)

    def test_a_new_corpus_program_on_no_list(self):
        corpus = self.root / "tests/corpus"
        (corpus / "slice").mkdir(parents=True)
        (corpus / "slice.list").write_text("# a comment\nslice/s1_listed\n")
        for name in ("s1_listed", "s2_new", "s3_rejected"):
            (corpus / f"slice/{name}.bas").write_text("PRINT 1\n")
        (corpus / "slice/s3_rejected.err").write_text("error\n")
        (corpus / "slice/s1_listed.files").mkdir()
        (corpus / "slice/s1_listed.files/inc.bas").write_text("PRINT 2\n")
        changed = [f"tests/corpus/slice/{n}.bas" for n in ("s1_listed", "s2_new", "s3_rejected")]
        changed.append("tests/corpus/slice/s1_listed.files/inc.bas")
        with mock.patch.object(close, "changed_files", return_value=changed):
            code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("UNLISTED tests/corpus/slice/s2_new.bas", text)
        self.assertEqual(text.count("UNLISTED"), 1, text)

    def test_a_failure_is_never_excused_as_not_repeated(self):
        self.at(self.source, 400)
        (self.root / close.log("cli")).write_text(CARGO_FAIL)
        self.at(self.root / close.log("cli"), 300)
        code, text = self.run_check(not_repeated={"cli": "x"})
        self.assertEqual(code, 1)
        self.assertIn("STALE    cli", text)

    def test_a_step_before_the_build_written_after_it(self):
        self.at(self.root / close.log("wording"), 250)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    wording: after the build", text)

    def test_a_step_before_the_build_left_by_an_earlier_change(self):
        # Before the build, but older than this change's edit.
        self.at(self.root / close.log("lists"), 50)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    lists: older than crates/a.rs: run it again, then build", text)

    def test_a_step_before_the_build_is_not_stale_by_what_one_writes(self):
        # `wording` at 150, then `lists` rewrote a list at 160 and ended at 170, then the build at 200.
        listed = self.root / "tests/known_false_errors.list"
        listed.parent.mkdir()
        listed.write_text("x\n")
        self.at(listed, 160)
        self.at(self.root / close.log("lists"), 170)
        with mock.patch.object(close, "changed_files", return_value=["crates/a.rs", "tests/known_false_errors.list"]):
            code, text = self.run_check()
        self.assertEqual(code, 0, text)


if __name__ == "__main__":
    unittest.main()
