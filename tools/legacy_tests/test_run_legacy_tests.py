#!/usr/bin/env python3
"""Tests of run_legacy_tests.py: the rules that decide what "pass" means and what gets recorded.

Run:  python -m unittest discover -s tools/legacy_tests -p "test_*.py"

The runner judges tier 2 and records every expected output (study/29 section 5), so its comparison
and recording rules are pinned here. Two kinds of test:
  * the pure functions (normalisation, the new compiler's rejection rule, .normalize files, exit
    codes as Git Bash reports them, the --list and known-failures files, what a copy of the clone
    leaves out);
  * corpus_test with the compiler and the program replaced by a stand-in for run_proc, so every
    outcome of one corpus program is reached without a compiler, a clone or a C++ build.
Not covered here: the locks and the private copies of qb64pe (they need the clone; the full runs
cover them), and the compile, qbasic, format and verification suites' own functions.
"""

from __future__ import annotations

import os
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest import mock

import run_legacy_tests as rlt


class Normalisation(unittest.TestCase):
    def test_output_treats_crlf_as_lf_and_drops_trailing_line_ends(self):
        self.assertEqual(rlt.norm_output(b"a\r\nb\r\n\r\n"), b"a\nb")
        self.assertEqual(rlt.norm_output(b"a\nb"), b"a\nb")
        # A blank line inside the output stays.
        self.assertEqual(rlt.norm_output(b"a\r\n\r\nb\n"), b"a\n\nb")

    def test_output_keeps_a_bare_cr(self):
        self.assertNotEqual(rlt.norm_output(b"a\rb\n"), rlt.norm_output(b"a\nb\n"))
        self.assertEqual(rlt.norm_output(b"a\rb\n"), b"a\rb")

    def test_output_keeps_leading_and_inner_blanks(self):
        self.assertEqual(rlt.norm_output(b" 1 \r\n"), b" 1 ")

    def test_text_ignores_every_cr(self):
        self.assertEqual(rlt.norm_text(b"x\r\ny\r\n\n"), ["x", "y"])
        self.assertEqual(rlt.norm_text(b"x\ny"), ["x", "y"])

    def test_old_compiler_by_name(self):
        self.assertTrue(rlt.is_old_compiler(Path("somewhere") / "QB64PE.exe"))
        self.assertTrue(rlt.is_old_compiler(Path("qb64pe")))
        self.assertFalse(rlt.is_old_compiler(Path("target") / "release" / "qb64rust.exe"))
        self.assertEqual(rlt.include_root_args(Path("qb64pe.exe")), [])
        self.assertEqual(rlt.include_root_args(Path("qb64rust.exe"))[0], "--include-root")


class Rejection(unittest.TestCase):
    """new_compiler_rejection: "" when a failed compile counts as rejecting the program."""

    def test_a_real_error_is_a_rejection(self):
        self.assertEqual(rlt.new_compiler_rejection(1, b"p.bas:1:1: error: x\r\n1 error\r\n"), "")
        self.assertEqual(rlt.new_compiler_rejection(1, b"3 errors (2 not supported yet)\n"), "")

    def test_only_marked_errors_are_no_rejection(self):
        why = rlt.new_compiler_rejection(1, b"2 errors (2 not supported yet)\n")
        self.assertIn("not supported yet", why)
        self.assertNotEqual(rlt.new_compiler_rejection(1, b"1 error (1 not supported yet)"), "")

    def test_an_internal_error_is_no_rejection(self):
        self.assertIn("internal compiler error", rlt.new_compiler_rejection(3, b"1 error\n"))

    def test_a_failed_build_is_no_rejection(self):
        # The C++ compiler's "error:" lines are not the front end's summary.
        out = b"qbx.cpp:5:1: error: expected ';'\nC++ build failed\n"
        self.assertIn("without an error summary", rlt.new_compiler_rejection(1, out))
        self.assertIn("without an error summary", rlt.new_compiler_rejection(1, b""))

    def test_the_summary_is_the_last_line(self):
        self.assertNotEqual(rlt.new_compiler_rejection(1, b"1 error\nsomething after\n"), "")
        self.assertEqual(rlt.new_compiler_rejection(1, b"1 error\n\n  \n"), "")


class NormalizeFiles(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)

    def rules(self, text: str):
        path = self.dir / "p.normalize"
        path.write_text(text, encoding="latin-1")
        return rlt.load_normalize(path)

    def test_no_file_no_rules(self):
        self.assertEqual(rlt.load_normalize(self.dir / "none.normalize"), [])
        self.assertEqual(rlt.apply_normalize(b"a\r\n", []), b"a\r\n")

    def test_rules_apply_in_order_to_each_line(self):
        rules = self.rules("# a comment\n\n\\d+ ms\t<time>\n<time>\tT\n")
        self.assertEqual(len(rules), 2)
        self.assertEqual(rlt.apply_normalize(b"took 12 ms\nand 7 ms\n", rules), b"took T\nand T\n")

    def test_the_line_end_stays_outside_the_rule(self):
        rules = self.rules("x$\tend\n")
        self.assertEqual(rlt.apply_normalize(b"ax\r\nbx\n", rules), b"aend\r\nbend\n")

    def test_a_line_without_a_tab_is_an_error(self):
        with self.assertRaises(ValueError):
            self.rules("no tab here\n")


class ExitCodes(unittest.TestCase):
    def test_as_git_bash_reports_them(self):
        self.assertEqual(rlt.bash_exit_code(0), 0)
        self.assertEqual(rlt.bash_exit_code(1), 1)
        self.assertEqual(rlt.bash_exit_code(256 + 7), 7)
        self.assertEqual(rlt.bash_exit_code(0xC0000005), 139)
        self.assertEqual(rlt.bash_exit_code(0xC000001D), 132)
        self.assertEqual(rlt.bash_exit_code(0xC000013A), 130)
        self.assertEqual(rlt.bash_exit_code(0xC0000094), 127)
        # Python may hand a Windows status over as a negative number.
        self.assertEqual(rlt.bash_exit_code(0xC0000005 - (1 << 32)), 139)


class Recording(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)

    def test_git_sees_text(self):
        self.assertTrue(rlt.git_sees_text(b"plain\r\ntext\n"))
        self.assertFalse(rlt.git_sees_text(b"a\0b"))
        self.assertFalse(rlt.git_sees_text(b"bare\rcr"))

    def test_write_recorded_leaves_a_line_end_difference_alone(self):
        path = self.dir / "v.out.txt"
        self.assertEqual(rlt.write_recorded(path, b"a\r\nexit=0\n"), "recorded new v.out.txt")
        path.write_bytes(b"a\nexit=0\n")  # as git checks it out
        self.assertEqual(rlt.write_recorded(path, b"a\r\nexit=0\n"), "")
        self.assertEqual(path.read_bytes(), b"a\nexit=0\n")
        self.assertEqual(rlt.write_recorded(path, b"b\r\nexit=0\n"), "warning: v.out.txt changed")
        self.assertEqual(path.read_bytes(), b"b\r\nexit=0\n")

    def test_write_recorded_compares_binary_byte_for_byte(self):
        path = self.dir / "v.out.txt"
        path.write_bytes(b"\0a\n")
        self.assertEqual(rlt.write_recorded(path, b"\0a\r\n"), "warning: v.out.txt changed")

    def test_write_expected_replaces_the_other_kind(self):
        out, err = self.dir / "p.output", self.dir / "p.err"
        self.assertEqual(rlt.write_expected(out, b"1\n", err), "recorded new p.output")
        self.assertEqual(rlt.write_expected(out, b"1\n", err), "")
        self.assertEqual(rlt.write_expected(out, b"2\n", err), "warning: p.output changed")
        self.assertEqual(
            rlt.write_expected(err, b"bad\n", out), "warning: p.output replaced by p.err"
        )
        self.assertFalse(out.exists())
        self.assertEqual(err.read_bytes(), b"bad\n")


class Lists(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)

    def test_load_list(self):
        (self.dir / "g").mkdir()
        (self.dir / "g" / "one.bas").write_bytes(b"")
        (self.dir / "g" / "two.bas").write_bytes(b"")
        lst = self.dir / "pass.list"
        lst.write_text("# header\n\ng/one  # why\n  g/two\n", encoding="utf-8")
        want = {(self.dir / "g" / n).resolve() for n in ("one.bas", "two.bas")}
        self.assertEqual(rlt.load_list(lst, self.dir), want)

    def test_load_list_names_a_missing_program(self):
        lst = self.dir / "pass.list"
        lst.write_text("g/none\n", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "line 1: no program g/none.bas"):
            rlt.load_list(lst, self.dir)

    def test_known_failures(self):
        path = self.dir / "known_failures.txt"
        self.assertEqual(rlt.load_known_failures(path), {})
        path.write_text("# c\ncorpus:g/p   hangs  # note\ncompile:a/b\n", encoding="utf-8")
        self.assertEqual(rlt.load_known_failures(path), {"corpus:g/p": "hangs", "compile:a/b": ""})

    def test_what_a_copy_of_the_clone_leaves_out(self):
        self.assertTrue(rlt.written_by_builds(("libqb", "src"), "qbs.o"))
        self.assertTrue(rlt.written_by_builds((), "qbx2.cpp"))
        self.assertTrue(rlt.written_by_builds(("parts", "core", "temp"), "gl.h"))
        self.assertFalse(rlt.written_by_builds((), "qbx.cpp"))
        self.assertFalse(rlt.written_by_builds((), "libqb.cpp"))
        self.assertFalse(rlt.written_by_builds(("c_compiler", "lib"), "libm.a"))


class Tools:
    """Stands in for run_proc: the compiler (a command with -x) and the program built by it."""

    def __init__(
        self,
        rc=0,
        compile_out=b"",
        exe=True,
        runs=(b"",),
        run_rc=0,
        run_times_out=False,
        states=(),
    ):
        self.rc, self.compile_out, self.exe = rc, compile_out, exe
        self.runs, self.run_rc, self.run_times_out = list(runs), run_rc, run_times_out
        # The screen state each run writes to the file QB64_SCREEN_STATE names (the last one again
        # for further runs); none, or None for a run, writes no file.
        self.states = list(states)
        self.compile_cmd: list[str] = []
        self.run_cmds: list[list[str]] = []
        self.run_stdin: list[Path | None] = []
        self.run_envs: list[dict] = []

    def __call__(self, args, cwd, stdout_path, timeout, env=None, stdin_path=None, **_):
        if "-x" in args:
            self.compile_cmd = list(args)
            stdout_path.write_bytes(self.compile_out)
            if self.exe:
                Path(args[args.index("-o") + 1]).write_bytes(b"")
            return self.rc, False
        self.run_stdin.append(stdin_path)
        self.run_cmds.append(list(args))
        self.run_envs.append(dict(env or {}))
        stdout_path.write_bytes(self.runs[min(len(self.run_cmds), len(self.runs)) - 1])
        state = self.states[min(len(self.run_cmds), len(self.states)) - 1] if self.states else None
        if state is not None and "QB64_SCREEN_STATE" in (env or {}):
            Path(env["QB64_SCREEN_STATE"]).write_bytes(state)
        if self.run_times_out:
            return None, True
        return self.run_rc, False


class CorpusFixture(unittest.TestCase):
    """One program g/p of a corpus in a scratch folder, and corpus_test run on it."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        base = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)
        self.corpus, self.results, self.clone = base / "corpus", base / "results", base / "clone"
        self.group = self.corpus / "g"
        self.group.mkdir(parents=True)
        self.results.mkdir()
        self.clone.mkdir()
        self.bas = self.group / "p.bas"
        self.bas.write_bytes(b"PRINT 1\n")

    def sidecar(self, ext: str, data: bytes) -> Path:
        path = self.group / f"p.{ext}"
        path.write_bytes(data)
        return path

    def run_test(self, tools: Tools, qb64="qb64rust.exe", record=False, cpp_opt=False, known=None):
        args = SimpleNamespace(qb64=Path(qb64), record=record, cpp_opt=cpp_opt, copies=None)
        with mock.patch.object(rlt, "run_proc", tools):
            return rlt.corpus_test(
                self.bas, args, self.results, self.clone, self.corpus, known or {}
            )

    def assert_fail(self, r, stage: str, detail: str = ""):
        self.assertEqual((r.status, r.stage), ("FAIL", stage), r.detail)
        self.assertIn(detail, r.detail)


class CorpusProgram(CorpusFixture):
    """corpus_test: every outcome of one program g/p of a corpus."""

    def test_equal_output_passes_and_the_work_folder_goes(self):
        self.sidecar("output", b"1\n2\n")
        tools = Tools(runs=[b"1\r\n2\r\n"])
        r = self.run_test(tools)
        self.assertEqual((r.status, r.kind, r.test), ("PASS", "output", "g/p"), r.detail)
        self.assertFalse((self.results / "g-p").exists())
        self.assertEqual(len(tools.run_cmds), 1)
        self.assertEqual(tools.compile_cmd[0], "qb64rust.exe")
        self.assertIn("--include-root", tools.compile_cmd)
        self.assertNotIn("-f:OptimizeCppProgram=true", tools.compile_cmd)

    def test_different_output_fails_and_the_work_folder_stays(self):
        self.sidecar("output", b"1\n")
        r = self.run_test(Tools(runs=[b"2\r\n"]))
        self.assert_fail(r, "result", "differs from .output")
        self.assertTrue((self.results / "g-p" / "p.bas").is_file())

    def test_a_bare_cr_in_the_output_counts(self):
        self.sidecar("output", b"a\nb\n")
        self.assert_fail(self.run_test(Tools(runs=[b"a\rb\r\n"])), "result")

    def test_run_failures(self):
        self.sidecar("output", b"1\n")
        self.assert_fail(self.run_test(Tools(runs=[b"1\n"], run_rc=5)), "run", "exit code 5")
        self.assert_fail(self.run_test(Tools(runs=[b"1\n"], run_times_out=True)), "run", "timeout")

    def test_compile_failures(self):
        self.sidecar("output", b"1\n")
        self.assert_fail(self.run_test(Tools(rc=1, exe=False)), "compile", "exit code 1")
        self.assert_fail(self.run_test(Tools(rc=0, exe=False)), "exe exists", "no executable")
        self.assert_fail(self.run_test(Tools(rc=1, exe=True)), "exe exists", "although")

    def test_expected_files(self):
        self.assert_fail(self.run_test(Tools()), "expected", "record it with --record")
        self.sidecar("output", b"1\n")
        self.sidecar("err", b"bad\n")
        self.assert_fail(self.run_test(Tools()), "expected", "more than one")

    def test_the_options_of_the_compile(self):
        self.sidecar("output", b"\n")
        tools = Tools()
        self.run_test(tools, qb64="qb64pe.exe", cpp_opt=True)
        self.assertNotIn("--include-root", tools.compile_cmd)
        self.assertEqual(tools.compile_cmd[1], "-f:OptimizeCppProgram=true")
        self.assertEqual(tools.compile_cmd[-3:-1], ["p.bas", "-o"])

    def test_err_with_the_new_compiler_needs_a_real_error(self):
        self.sidecar("err", b"Syntax error\n")
        real = b"p.bas:1:1: error: x\n2 errors (1 not supported yet)\n"
        self.assertEqual(self.run_test(Tools(rc=1, compile_out=real, exe=False)).status, "PASS")
        marked = b"p.bas:1:1: error: not supported yet: x\n1 error (1 not supported yet)\n"
        r = self.run_test(Tools(rc=1, compile_out=marked, exe=False))
        self.assert_fail(r, "error result", "not supported yet")
        r = self.run_test(Tools(rc=3, compile_out=b"internal compiler error\n", exe=False))
        self.assert_fail(r, "error result", "internal compiler error")
        self.assert_fail(self.run_test(Tools()), "compile", "expected an error")

    def test_err_with_the_old_compiler_compares_the_text(self):
        self.sidecar("err", b"Syntax error\nLINE 1:x\n")
        same = Tools(rc=1, compile_out=b"Syntax error\r\nLINE 1:x\r\n", exe=False)
        self.assertEqual(self.run_test(same, qb64="qb64pe.exe").status, "PASS")
        other = Tools(rc=1, compile_out=b"Type mismatch\r\nLINE 1:x\r\n", exe=False)
        self.assert_fail(self.run_test(other, qb64="qb64pe.exe"), "error result", "differs")

    def test_norun_compiles_and_never_runs(self):
        self.sidecar("norun", b"waits for a key\n")
        tools = Tools()
        r = self.run_test(tools)
        self.assertEqual((r.status, r.kind), ("PASS", "compile-only"), r.detail)
        self.assertEqual(tools.run_cmds, [])

    def test_a_known_failure_without_an_expected_file_is_compiled_only(self):
        tools = Tools()
        r = self.run_test(tools, known={"corpus:g/p": "loops for ever"})
        self.assert_fail(r, "run", "not run: known failure")
        self.assertEqual(tools.run_cmds, [])

    def test_stdin_is_a_copy_beside_the_results(self):
        self.sidecar("output", b"1\n")
        self.sidecar("stdin", b"42\r\n")
        tools = Tools(runs=[b"1\n"])
        self.assertEqual(self.run_test(tools).status, "PASS")
        copy = self.results / "g-p-stdin.txt"
        self.assertEqual(copy.read_bytes(), b"42\r\n")
        # On Windows the key-press helper is handed the file, elsewhere the program itself.
        given = tools.run_cmds[0][-1] if os.name == "nt" else str(tools.run_stdin[0])
        self.assertEqual(given, str(copy))

    def test_normalize_rules_apply_to_the_actual_output(self):
        self.sidecar("output", b"took <t> ms\n")
        self.sidecar("normalize", b"\\d+ ms\t<t> ms\n")
        self.assertEqual(self.run_test(Tools(runs=[b"took 31 ms\r\n"])).status, "PASS")

    def test_record_writes_the_output_of_two_equal_runs(self):
        tools = Tools(runs=[b"1\r\n", b"1\r\n"])
        r = self.run_test(tools, qb64="qb64pe.exe", record=True)
        self.assertEqual((r.status, r.detail), ("PASS", "recorded new p.output"))
        self.assertEqual((self.group / "p.output").read_bytes(), b"1\r\n")
        self.assertEqual(len(tools.run_cmds), 2)

    def test_record_refuses_two_different_runs(self):
        r = self.run_test(Tools(runs=[b"1\n", b"2\n"]), qb64="qb64pe.exe", record=True)
        self.assert_fail(r, "record", "non-deterministic")
        self.assertFalse((self.group / "p.output").exists())

    def test_record_normalises_before_comparing_the_runs(self):
        self.sidecar("normalize", b"\\d+\tN\n")
        r = self.run_test(Tools(runs=[b"t 1\n", b"t 2\n"]), qb64="qb64pe.exe", record=True)
        self.assertEqual(r.status, "PASS", r.detail)
        self.assertEqual((self.group / "p.output").read_bytes(), b"t N\n")

    def test_record_of_a_rejection_replaces_the_output_file(self):
        self.sidecar("output", b"1\n")
        tools = Tools(rc=1, compile_out=b"Syntax error\r\n", exe=False)
        r = self.run_test(tools, qb64="qb64pe.exe", record=True)
        self.assertEqual((r.status, r.kind), ("PASS", "error"), r.detail)
        self.assertEqual(r.detail, "warning: p.output replaced by p.err")
        self.assertEqual((self.group / "p.err").read_bytes(), b"Syntax error\r\n")
        self.assertFalse((self.group / "p.output").exists())

    def test_record_with_the_new_compiler_takes_only_a_real_rejection(self):
        marked = Tools(rc=1, compile_out=b"1 error (1 not supported yet)\n", exe=False)
        self.assert_fail(self.run_test(marked, record=True), "record", "not recorded")
        self.assertFalse((self.group / "p.err").exists())
        real = Tools(rc=1, compile_out=b"p.bas:1:1: error: x\n1 error\n", exe=False)
        self.assertEqual(self.run_test(real, record=True).status, "PASS")
        self.assertTrue((self.group / "p.err").is_file())


STATE = b"screen-state 1\npages 1 display 0 write 0 read 0\nend\n"


class ScreenProgram(CorpusFixture):
    """corpus_test for a program with a .screen file (spec testing/screen-state)."""

    def setUp(self):
        super().setUp()
        self.sidecar("output", b"")
        self.screen = self.sidecar("screen", STATE)

    def test_equal_state_passes(self):
        tools = Tools(states=[STATE.replace(b"\n", b"\r\n")])
        r = self.run_test(tools)
        self.assertEqual(r.status, "PASS", r.detail)
        flag = [a for a in tools.compile_cmd if a.startswith("-f:ExtraLinkerFlags=")]
        self.assertEqual(len(flag), 1, tools.compile_cmd)
        self.assertTrue(flag[0].endswith("/tools/screen/screen_state.cpp"), flag)
        self.assertEqual(
            tools.run_envs[0]["QB64_SCREEN_STATE"], str(self.results / "g-p-run1-screen.txt")
        )
        self.assertNotIn("QB64_SCREEN_KEYS", tools.run_envs[0])

    def test_a_program_without_screen_gets_neither_the_flag_nor_the_variable(self):
        self.screen.unlink()
        tools = Tools(states=[STATE])
        self.assertEqual(self.run_test(tools).status, "PASS")
        self.assertFalse([a for a in tools.compile_cmd if "ExtraLinkerFlags" in a])
        self.assertNotIn("QB64_SCREEN_STATE", tools.run_envs[0])

    def test_a_different_state_fails_and_is_kept(self):
        other = STATE.replace(b"display 0", b"display 1")
        r = self.run_test(Tools(states=[other]))
        self.assert_fail(r, "screen", "differs from .screen")
        self.assertEqual((self.results / "g-p-run1-screen.txt").read_bytes(), other)

    def test_the_output_is_still_compared(self):
        self.assert_fail(self.run_test(Tools(runs=[b"x\n"], states=[STATE])), "result")

    def test_no_state_written_fails(self):
        self.assert_fail(self.run_test(Tools()), "screen", "no screen state written")

    def test_a_state_left_by_an_earlier_run_does_not_count(self):
        (self.results / "g-p-run1-screen.txt").write_bytes(STATE)
        self.assert_fail(self.run_test(Tools()), "screen", "no screen state written")

    def test_keys_are_a_copy_beside_the_results(self):
        self.sidecar("keys", b"text a\nkey enter\n")
        tools = Tools(states=[STATE])
        self.assertEqual(self.run_test(tools).status, "PASS")
        copy = self.results / "g-p-keys.txt"
        self.assertEqual(copy.read_bytes(), b"text a\nkey enter\n")
        self.assertEqual(tools.run_envs[0]["QB64_SCREEN_KEYS"], str(copy))

    def test_screen_beside_err_or_norun_is_an_error_in_the_test(self):
        (self.group / "p.output").unlink()
        self.sidecar("norun", b"why\n")
        self.assert_fail(self.run_test(Tools()), "expected", ".screen beside .norun")
        (self.group / "p.norun").unlink()
        self.sidecar("err", b"Syntax error\n")
        self.assert_fail(self.run_test(Tools(rc=1, exe=False)), "expected", ".screen beside .err")

    def test_keys_without_screen_is_an_error_in_the_test(self):
        self.screen.unlink()
        self.sidecar("keys", b"text a\n")
        tools = Tools()
        self.assert_fail(self.run_test(tools), "expected", ".keys without .screen")
        self.assertEqual(tools.compile_cmd, [])

    def test_a_screen_program_that_does_not_compile_is_never_recorded_as_an_error(self):
        tools = Tools(rc=1, compile_out=b"Syntax error\r\n", exe=False)
        r = self.run_test(tools, qb64="qb64pe.exe", record=True)
        self.assert_fail(r, "compile", "exit code 1")
        self.assertFalse((self.group / "p.err").exists())

    def test_record_writes_the_state_of_two_equal_runs(self):
        self.screen.write_bytes(b"")
        (self.group / "p.output").unlink()
        r = self.run_test(Tools(states=[STATE, STATE]), qb64="qb64pe.exe", record=True)
        self.assertEqual(
            (r.status, r.detail), ("PASS", "recorded new p.output; recorded new p.screen")
        )
        self.assertEqual(self.screen.read_bytes(), STATE)

    def test_record_reports_a_changed_state(self):
        other = STATE.replace(b"pages 1", b"pages 2")
        r = self.run_test(Tools(states=[other]), qb64="qb64pe.exe", record=True)
        self.assertEqual((r.status, r.detail), ("PASS", "warning: p.screen changed"))
        self.assertEqual(self.screen.read_bytes(), other)

    def test_record_refuses_two_different_states(self):
        other = STATE.replace(b"pages 1", b"pages 2")
        r = self.run_test(Tools(states=[STATE, other]), qb64="qb64pe.exe", record=True)
        self.assert_fail(r, "record", "screen states of the two runs differ")
        self.assertEqual(self.screen.read_bytes(), STATE)

    def test_record_refuses_a_run_without_a_state(self):
        r = self.run_test(Tools(states=[STATE, None]), qb64="qb64pe.exe", record=True)
        self.assert_fail(r, "screen", "no screen state written")

    def test_a_blank_in_the_path_of_the_state_file_is_refused(self):
        with mock.patch.object(rlt, "SCREEN_STATE_SOURCE", Path("my repo/screen_state.cpp")):
            self.assert_fail(self.run_test(Tools(states=[STATE])), "screen", "a blank in the path")


if __name__ == "__main__":
    unittest.main()
