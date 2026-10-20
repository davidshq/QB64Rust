#!/usr/bin/env python3
"""Tests of run_legacy_tests.py: the rules that decide what "pass" means and what gets recorded.

Run:  python -m unittest discover -s tools/legacy_tests -p "test_*.py"

The runner judges tier 2 and records every expected output (study/archive/29 section 5), so its comparison
and recording rules are pinned here. Two kinds of test:
  * the pure functions (normalisation, the new compiler's rejection rule, .normalize files, exit
    codes as Git Bash reports them and as press_any_key.py passes them on, the --list and
    known-failures files, what a copy of the clone leaves out);
  * corpus_test with the compiler and the program replaced by a stand-in for run_proc, so every
    outcome of one corpus program is reached without a compiler, a clone or a C++ build.
Not covered here: the locks and the private copies of qb64pe (they need the clone; the full runs
cover them), and the compile, qbasic, format and verification suites' own functions.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
import tempfile
import time
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

    @unittest.skipUnless(os.name == "nt", "press_any_key.py runs on Windows only")
    def test_press_any_key_passes_on_every_32_bit_code(self):
        # A child process ends as the helper does, with sys.exit(exit_status(code)); a code from 2**31
        # up came out as 4294967295 before (sys.exit takes a 32-bit C long on Windows).
        here = str(Path(__file__).resolve().parent)
        for code in (0, 3, 2**31 - 1, 2**31, 4294967294, 4294967295):
            script = (
                f"import sys; sys.path.insert(0, {here!r}); "
                f"from press_any_key import exit_status; sys.exit(exit_status({code}))"
            )
            p = subprocess.run([sys.executable, "-c", script], check=False, capture_output=True)
            self.assertEqual(p.returncode, code, p.stderr)

    @unittest.skipUnless(os.name == "nt", "press_any_key.py runs on Windows only")
    def test_press_any_key_on_a_hidden_desktop_passes_stdin_stdout_and_the_exit_code(self):
        # As the runner starts it: a console of its own, output to a file. cmd.exe reads its
        # commands from the stdin file and writes to the helper's stdout.
        with tempfile.TemporaryDirectory() as tmp:
            commands, out = Path(tmp, "in.txt"), Path(tmp, "out.txt")
            commands.write_bytes(b"echo from-the-hidden-desktop\r\nexit 7\r\n")
            helper = Path(__file__).with_name("press_any_key.py")
            cmd = os.path.join(os.environ.get("SystemRoot", r"C:\Windows"), "System32", "cmd.exe")
            with open(out, "wb") as f:
                p = subprocess.run(
                    [sys.executable, str(helper), "--hidden-desktop", cmd, str(commands)],
                    stdout=f,
                    stderr=subprocess.STDOUT,
                    stdin=subprocess.DEVNULL,
                    creationflags=subprocess.CREATE_NO_WINDOW,
                    timeout=60,
                    check=False,
                )
            self.assertEqual(p.returncode, 7, out.read_bytes())
            self.assertIn(b"from-the-hidden-desktop", out.read_bytes())


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


class DivergenceRows(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)

    def test_allowed_rows(self):
        self.assertEqual(rlt.allowed_rows(self.dir / "none.divergences"), set())
        path = self.dir / "p.divergences"
        path.write_text("# why\n\nD-004\n  D-019 # a member block\n", encoding="latin-1")
        self.assertEqual(rlt.allowed_rows(path), {"D-004", "D-019"})

    def test_unexpected_rows_in_the_order_taken(self):
        trace = self.dir / "t.txt"
        self.assertEqual(rlt.unexpected_rows(trace, set()), [])
        trace.write_bytes(b"D-022\r\nD-004\n\n")
        self.assertEqual(rlt.unexpected_rows(trace, set()), ["D-022", "D-004"])
        self.assertEqual(rlt.unexpected_rows(trace, {"D-022"}), ["D-004"])

    def test_a_compile_suite_sidecar_is_outside_the_copied_folder(self):
        path = rlt.compile_sidecar("arrays", "p")
        self.assertEqual(
            path.parts[-5:], ("tests", "upstream", "divergences", "arrays", "p.divergences")
        )

    def test_set_trace_clears_the_file(self):
        trace = self.dir / "t.txt"
        trace.write_bytes(b"D-004\n")
        env: dict[str, str] = {}
        rlt.set_trace(env, trace)
        self.assertFalse(trace.exists())
        self.assertEqual(env[rlt.TRACE_ENV], str(trace))


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


class RunAgainAlone(unittest.TestCase):
    """timing_sensitive and run_again_alone (test-parallel-runs, design D3): a program that failed
    beside other builds runs once more alone, and only a timing-sensitive one that then passes
    leaves the run passing."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)

    def program(self, name: str, source: str, screen: bool = False) -> Path:
        bas = self.dir / f"{name}.bas"
        bas.write_text(source, encoding="latin-1")
        if screen:
            bas.with_suffix(".screen").write_bytes(b"")
        return bas

    def test_timing_sensitive(self):
        self.assertTrue(rlt.timing_sensitive(self.program("a", "SOUND 440, 2\n")))
        self.assertTrue(rlt.timing_sensitive(self.program("b", "x = _MOUSEX\n")))
        self.assertTrue(rlt.timing_sensitive(self.program("c", "on key(1) gosub h\n")))
        self.assertTrue(rlt.timing_sensitive(self.program("d", "t! = TIMER\n")))
        self.assertTrue(rlt.timing_sensitive(self.program("e", "PRINT 1\n", screen=True)))
        self.assertFalse(rlt.timing_sensitive(self.program("f", "PRINT SOUNDS%\nDISPLAY = 1\n")))
        self.assertFalse(rlt.timing_sensitive(self.dir / "missing.bas"))

    def outcomes(self, tests, first, second, old_compiler=False):
        by_test = {i: [r] for i, r in enumerate(first)}
        again = dict(zip([t for t, r in zip(tests, first) if r.status == "FAIL"], second))
        ran = []

        def run_one(bas):
            ran.append(bas)
            return [again[bas]]

        with mock.patch("builtins.print"):
            result = rlt.run_again_alone(tests, by_test, run_one, old_compiler)
        return by_test, ran, result

    def test_only_failures_run_again(self):
        tests = [self.program("p", "PRINT 1\n"), self.program("q", "PRINT 2\n")]
        known = rlt.Result("corpus", "g/q", "output", "KFAIL", "output", "differs")
        by_test, ran, result = self.outcomes(
            tests, [rlt.Result("corpus", "g/p", "output"), known], []
        )
        self.assertEqual(ran, [])
        self.assertEqual(result, ([], []))
        self.assertIs(by_test[1][0], known)

    def test_a_settled_failure_does_not_run_again(self):
        tests = [self.program("p", "PRINT 1\n"), self.program("q", "PRINT 2\n")]
        wrong_err = rlt.Result(
            "corpus", "g/p", "error", "FAIL", "error result", "differs from .err"
        )
        layout = rlt.Result("corpus", "g/q", "output", "FAIL", "expected", "no .output")
        by_test, ran, result = self.outcomes(tests, [wrong_err, layout], [])
        self.assertEqual(ran, [])
        self.assertEqual(result, ([], []))
        self.assertIs(by_test[0][0], wrong_err)
        self.assertIs(by_test[1][0], layout)

    def test_with_qb64pe_a_settled_failure_runs_again(self):
        tests = [self.program("p", "PRINT 1\n")]
        before = rlt.Result("corpus", "g/p", "error", "FAIL", "error result", "differs from .err")
        _, ran, _ = self.outcomes(
            tests, [before], [rlt.Result("corpus", "g/p", "error")], old_compiler=True
        )
        self.assertEqual(ran, tests)

    def test_a_timing_sensitive_pass_alone_does_not_fail_the_run(self):
        tests = [self.program("s", "BEEP\n")]
        before = rlt.Result("corpus", "g/s", "output", "FAIL", "output", "differs")
        after = rlt.Result("corpus", "g/s", "output")
        by_test, ran, result = self.outcomes(tests, [before], [after])
        self.assertEqual(ran, tests)
        self.assertEqual(result, ([], ["corpus:g/s"]))
        self.assertEqual(by_test[0][0].status, "PASS")
        self.assertIn("failed with --jobs (output: differs)", by_test[0][0].detail)
        self.assertIn("timing-sensitive", by_test[0][0].detail)

    def test_another_pass_alone_fails_the_run(self):
        tests = [self.program("p", "PRINT 1\n")]
        before = rlt.Result("corpus", "g/p", "output", "FAIL", "build", "make")
        _, _, result = self.outcomes(tests, [before], [rlt.Result("corpus", "g/p", "output")])
        self.assertEqual(result, (["corpus:g/p"], []))

    def test_with_qb64pe_every_pass_alone_fails_the_run(self):
        tests = [self.program("s", "BEEP\n")]
        before = rlt.Result("corpus", "g/s", "output", "FAIL", "output", "differs")
        by_test, _, result = self.outcomes(
            tests, [before], [rlt.Result("corpus", "g/s", "output")], old_compiler=True
        )
        self.assertEqual(result, (["corpus:g/s"], []))
        self.assertIn("passed alone in the clone", by_test[0][0].detail)

    def test_a_failure_alone_keeps_both_outcomes(self):
        tests = [self.program("s", "BEEP\n")]
        before = rlt.Result("corpus", "g/s", "output", "FAIL", "output", "differs")
        after = rlt.Result("corpus", "g/s", "output", "FAIL", "run", "timed out")
        by_test, _, result = self.outcomes(tests, [before], [after])
        self.assertEqual(result, ([], []))
        self.assertEqual(by_test[0][0].status, "FAIL")
        self.assertEqual(by_test[0][0].detail, "timed out (alone; with --jobs: output: differs)")


class CheckList(unittest.TestCase):
    """check_list (--check-list, study/archive/31 item 3): a pass list judged from a full run's results."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        base = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)
        self.root = base / "corpus"
        (self.root / "g").mkdir(parents=True)
        self.programs = {}
        for name in ("a", "b"):
            bas = self.root / "g" / f"{name}.bas"
            bas.write_bytes(b"PRINT 1\n")
            bas.with_suffix(".output").write_bytes(b"1\n")
            self.programs[name] = bas.resolve()
        self.compiler = base / "qb64rust.exe"
        self.compiler.write_bytes(b"compiler build 1")

    def data(self, statuses: dict, **run) -> dict:
        record = {
            "version": rlt.RUN_RECORD_VERSION,
            "suite": "corpus",
            "selection": {},
            "roots": {"corpus": rlt.repo_relative(self.root.resolve())},
            "no_fold": False,
            "cpp_opt": False,
            "record": False,
            "pass_cache": False,
            "compiler_sha256": rlt.file_sha256(self.compiler),
            "runner_sha256": rlt.file_sha256(Path(rlt.__file__).resolve()),
            "started": time.time() + 60,
        }
        record.update(run)
        results = [
            {
                "suite": "corpus",
                "test": f"g/{n}",
                "kind": kind,
                "status": status,
                "stage": "run",
                "detail": detail,
            }
            for n, (status, kind, detail) in statuses.items()
        ]
        return {"run": record, "results": results}

    def check(self, data: dict, names=("a", "b")):
        wanted = {self.programs[n] for n in names}
        return rlt.check_list("corpus", wanted, self.root.resolve(), data, self.compiler)

    def test_every_listed_program_passed(self):
        data = self.data(
            {
                "a": ("PASS", "output", ""),
                "b": ("PASS", "compile", ""),
                "c": ("FAIL", "output", "x"),
            }
        )
        # A compile-only program passes as the run judged it; a failure off the list does not count.
        self.assertEqual(self.check(data), ([], [], 2))

    def test_a_program_with_no_result_fails(self):
        problems, failures, passed = self.check(self.data({"a": ("PASS", "output", "")}))
        self.assertEqual((problems, passed), ([], 1))
        self.assertEqual(failures, [("corpus:g/b", "no result in the full run")])

    def test_a_failure_and_a_known_failure_fail_the_list(self):
        data = self.data(
            {"a": ("FAIL", "output", "differs"), "b": ("KFAIL", "output", "(known: hangs)")}
        )
        _, failures, passed = self.check(data)
        self.assertEqual(passed, 0)
        self.assertEqual(failures[0], ("corpus:g/a", "run: differs"))
        self.assertIn("a pass list needs a pass", failures[1][1])

    def test_a_program_the_pass_cache_kept_from_running_fails(self):
        data = self.data({"a": ("PASS", "output", rlt.PASSED_BEFORE), "b": ("PASS", "output", "")})
        self.assertEqual(
            self.check(data)[1], [("corpus:g/a", "not run in the full run (--pass-cache)")]
        )

    def test_an_input_changed_after_the_run_started_fails(self):
        data = self.data(
            {"a": ("PASS", "output", ""), "b": ("PASS", "output", "")}, started=time.time() - 60
        )
        for path in (self.root / "g").iterdir():
            os.utime(path, (time.time() - 120, time.time() - 120))
        self.assertEqual(self.check(data, names=("b",))[1], [])
        os.utime(self.programs["b"].with_suffix(".output"))
        self.assertEqual(
            self.check(data, names=("b",))[1],
            [("corpus:g/b", "changed after the run started: b.output")],
        )

    def test_a_run_of_another_kind_fails_every_program(self):
        ok = {"a": ("PASS", "output", ""), "b": ("PASS", "output", "")}
        for run, why in [
            ({"compiler_sha256": "0"}, "another build of the compiler"),
            ({"runner_sha256": "0"}, "the runner has changed"),
            ({"no_fold": True}, "QB64RUST_NO_FOLD=1"),
            ({"pass_cache": True}, "--pass-cache"),
            ({"selection": {"list": "tests/corpus/slice.list"}}, "not a full run"),
            ({"suite": "compile"}, "not a full run"),
            ({"roots": {"corpus": "elsewhere"}}, "another root"),
        ]:
            problems, failures, passed = self.check(self.data(ok, **run))
            self.assertEqual((failures, passed), ([], 0), run)
            self.assertTrue(any(why in p for p in problems), (run, problems))

    def test_results_without_a_run_record_fail(self):
        problems, _, passed = self.check({"results": []})
        self.assertEqual(passed, 0)
        self.assertIn("no run record", problems[0])


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
        exe_bytes=b"",
        trace=None,
        counts=None,
    ):
        self.exe_bytes = exe_bytes
        # What each run writes to the coverage counts (the file QB64RUST_COVERAGE names).
        self.counts = counts
        self.compiles = 0
        # What each run writes to the trace of divergences (the file QB64RUST_DIVERGENCES names).
        self.trace = trace
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
            self.compiles += 1
            self.compile_cmd = list(args)
            stdout_path.write_bytes(self.compile_out)
            if self.exe:
                Path(args[args.index("-o") + 1]).write_bytes(self.exe_bytes)
            return self.rc, False
        self.run_stdin.append(stdin_path)
        self.run_cmds.append(list(args))
        self.run_envs.append(dict(env or {}))
        stdout_path.write_bytes(self.runs[min(len(self.run_cmds), len(self.runs)) - 1])
        state = self.states[min(len(self.run_cmds), len(self.states)) - 1] if self.states else None
        if state is not None and "QB64_SCREEN_STATE" in (env or {}):
            Path(env["QB64_SCREEN_STATE"]).write_bytes(state)
        if self.trace is not None and rlt.TRACE_ENV in (env or {}):
            Path(env[rlt.TRACE_ENV]).write_bytes(self.trace)
        if self.counts is not None and "QB64RUST_COVERAGE" in (env or {}):
            Path(env["QB64RUST_COVERAGE"]).write_bytes(self.counts)
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

    def run_test(
        self,
        tools: Tools,
        qb64="qb64rust.exe",
        record=False,
        cpp_opt=False,
        known=None,
        pass_cache=None,
        bas=None,
        **more,
    ):
        args = SimpleNamespace(
            qb64=Path(qb64),
            record=record,
            cpp_opt=cpp_opt,
            copies=None,
            pass_cache=pass_cache,
            **more,
        )
        with mock.patch.object(rlt, "run_proc", tools):
            return rlt.corpus_test(
                bas or self.bas, args, self.results, self.clone, self.corpus, known or {}
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

    def test_an_exit_code_is_compared_with_the_exitcode_file(self):
        self.sidecar("output", b"1\n")
        self.sidecar("exitcode", b"3\n")
        self.assertEqual(self.run_test(Tools(runs=[b"1\n"], run_rc=3)).status, "PASS")
        self.assert_fail(
            self.run_test(Tools(runs=[b"1\n"], run_rc=0)), "run", "exit code 0, expected 3"
        )
        self.assert_fail(self.run_test(Tools(runs=[b"1\n"], run_rc=4)), "run", "expected 3")

    def test_an_exitcode_file_beside_err_or_norun_is_an_error_in_the_test(self):
        self.sidecar("exitcode", b"3\n")
        err = self.sidecar("err", b"bad\n")
        self.assert_fail(self.run_test(Tools(rc=1, exe=False)), "expected", ".exitcode beside .err")
        err.unlink()
        self.sidecar("norun", b"why\n")
        self.assert_fail(self.run_test(Tools()), "expected", ".exitcode beside .norun")

    def test_record_writes_an_exit_code_that_is_not_0_and_removes_one_that_is(self):
        r = self.run_test(Tools(runs=[b"1\r\n"], run_rc=4294967295), record=True)
        self.assertEqual(r.status, "PASS", r.detail)
        self.assertEqual((self.group / "p.exitcode").read_bytes(), b"4294967295\n")
        r = self.run_test(Tools(runs=[b"1\r\n"], run_rc=0), record=True)
        self.assertIn("removed p.exitcode", r.detail)
        self.assertFalse((self.group / "p.exitcode").exists())
        self.run_test(Tools(runs=[b"1\r\n"]), record=True)
        self.assertFalse((self.group / "p.exitcode").exists())

    def test_record_refuses_two_different_exit_codes(self):
        tools = Tools(runs=[b"1\r\n"], run_rc=3)
        calls = iter([3, 4])
        original = tools.__call__

        def two_codes(args, cwd, stdout_path, timeout, **kw):
            rc, out = original(args, cwd, stdout_path, timeout, **kw)
            return (rc, out) if "-x" in args else (next(calls), out)

        with mock.patch.object(rlt, "run_proc", two_codes):
            args = SimpleNamespace(
                qb64=Path("qb64rust.exe"), record=True, cpp_opt=False, copies=None, pass_cache=None
            )
            r = rlt.corpus_test(self.bas, args, self.results, self.clone, self.corpus, {})
        self.assert_fail(r, "record", "exit codes 3 and 4")

    def test_every_run_has_a_trace_of_divergences(self):
        self.sidecar("output", b"1\n")
        tools = Tools(runs=[b"1\n"])
        self.assertEqual(self.run_test(tools).status, "PASS")
        self.assertIn(rlt.TRACE_ENV, tools.run_envs[0])

    def test_a_row_the_program_does_not_list_fails(self):
        self.sidecar("output", b"1\n")
        r = self.run_test(Tools(runs=[b"1\n"], trace=b"D-004\r\nD-022\r\n"))
        self.assert_fail(r, "divergences", "took D-004, D-022 of DIVERGENCES.md")
        self.sidecar("divergences", b"# the index outside on purpose\nD-004\n")
        r = self.run_test(Tools(runs=[b"1\n"], trace=b"D-004\nD-022\n"))
        self.assert_fail(r, "divergences", "took D-022 of")
        self.sidecar("divergences", b"D-004\nD-022  # and this\n")
        self.assertEqual(self.run_test(Tools(runs=[b"1\n"], trace=b"D-004\n")).status, "PASS")

    def test_a_trace_of_an_earlier_run_is_not_read(self):
        self.sidecar("output", b"1\n")
        self.run_test(Tools(runs=[b"1\n"], trace=b"D-004\n"))
        self.assertEqual(self.run_test(Tools(runs=[b"1\n"])).status, "PASS")

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

    def data_file(self, name: str, data: bytes) -> Path:
        path = self.group / "p.files" / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        return path

    def test_data_files_are_copied_beside_the_program(self):
        self.sidecar("output", b"1\n")
        self.data_file("MTN.VGA", b"\xfd\x00\r\n\x01")
        self.data_file("sub/notes.txt", b"a\n")
        # A failing run keeps the folder the program ran in.
        self.assert_fail(self.run_test(Tools(runs=[b"2\n"])), "result")
        work = self.results / "g-p"
        self.assertEqual((work / "MTN.VGA").read_bytes(), b"\xfd\x00\r\n\x01")
        self.assertEqual((work / "sub" / "notes.txt").read_bytes(), b"a\n")
        self.assertTrue((work / "p.bas").is_file())

    def test_data_files_are_copied_for_a_recording_too(self):
        self.data_file("d.bin", b"x")
        r = self.run_test(Tools(runs=[b"1\n", b"2\n"]), qb64="qb64pe.exe", record=True)
        self.assertEqual(r.status, "FAIL", r.detail)
        self.assertEqual((self.results / "g-p" / "d.bin").read_bytes(), b"x")

    def test_data_files_beside_err_or_norun_are_an_error_in_the_test(self):
        self.data_file("d.bin", b"x")
        self.sidecar("norun", b"why\n")
        tools = Tools()
        self.assert_fail(self.run_test(tools), "expected", ".files beside .norun")
        self.assertEqual(tools.compile_cmd, [])
        (self.group / "p.norun").unlink()
        self.sidecar("err", b"Syntax error\n")
        self.assert_fail(self.run_test(Tools(rc=1, exe=False)), "expected", ".files beside .err")

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


class CompilePassCache(unittest.TestCase):
    """--pass-cache with the compile suite (test-parallel-runs): the same rule as a corpus program's,
    with every file of the program's category folder counted."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        base = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)
        self.category = base / "tests" / "no-such-category-here"
        self.category.mkdir(parents=True)
        self.results, self.cache = base / "results", base / "passes"
        self.results.mkdir()
        self.bas = self.category / "p.bas"
        self.bas.write_bytes(b"PRINT 1\n")
        (self.category / "p.output").write_bytes(b"1\n")

    def go(self, runs=(b"1\n",), exe_bytes=b"exe one"):
        tools = Tools(runs=list(runs), exe_bytes=exe_bytes)
        args = SimpleNamespace(qb64=Path("qb64rust.exe"), pass_cache=self.cache)
        with mock.patch.object(rlt, "run_proc", tools):
            r = rlt.compile_test(self.bas, args, self.results, self.results, "windows")
        return r, len(tools.run_cmds)

    def test_a_program_that_passed_is_not_run(self):
        r, ran = self.go()
        self.assertEqual((r.status, r.detail, ran), ("PASS", "", 1))
        r, ran = self.go(runs=[b"wrong\n"])
        self.assertEqual((r.status, r.detail, ran), ("PASS", rlt.PASSED_BEFORE, 0))

    def test_another_executable_or_file_of_the_folder_runs(self):
        self.go()
        r, ran = self.go(exe_bytes=b"exe two")
        self.assertEqual((r.status, ran), ("PASS", 1))
        (self.category / "data.txt").write_bytes(b"x")
        r, ran = self.go(exe_bytes=b"exe two")
        self.assertEqual((r.status, ran), ("PASS", 1))

    def test_a_failure_is_not_remembered(self):
        r, _ = self.go(runs=[b"2\n"])
        self.assertEqual(r.status, "FAIL")
        r, ran = self.go()
        self.assertEqual((r.status, r.detail, ran), ("PASS", "", 1))


class PassCache(CorpusFixture):
    """--pass-cache: a program that passed is not run again while its executable, its source and
    its expected and input files are the same; anything else runs it."""

    def setUp(self):
        super().setUp()
        self.cache = Path(self.tmp.name) / "passes"
        self.sidecar("output", b"1\n")

    def go(self, tools: Tools, **more):
        return self.run_test(tools, pass_cache=self.cache, **more)

    def passes_once(self) -> None:
        tools = Tools(runs=[b"1\n"], exe_bytes=b"exe one")
        r = self.go(tools)
        self.assertEqual((r.status, r.detail, len(tools.run_cmds)), ("PASS", "", 1))

    def again(self, exe_bytes=b"exe one", runs=(b"1\n",)):
        tools = Tools(runs=list(runs), exe_bytes=exe_bytes)
        return self.go(tools), len(tools.run_cmds)

    def test_a_program_that_passed_is_compiled_and_not_run(self):
        self.passes_once()
        tools = Tools(runs=[b"wrong\n"], exe_bytes=b"exe one")
        r = self.go(tools)
        self.assertEqual((r.status, r.detail), ("PASS", rlt.PASSED_BEFORE))
        self.assertEqual(len(tools.run_cmds), 0)
        self.assertTrue(tools.compile_cmd)
        self.assertFalse((self.results / "g-p").exists())

    def test_another_executable_runs(self):
        self.passes_once()
        r, runs = self.again(exe_bytes=b"exe two", runs=[b"2\n"])
        self.assertEqual(runs, 1)
        self.assert_fail(r, "result")

    def test_a_changed_expected_or_input_file_runs(self):
        for ext, data in (
            ("output", b"2\n"),
            ("stdin", b"5\n"),
            ("normalize", b"x\ty\n"),
            ("noprompt", b"continue\n"),
        ):
            with self.subTest(ext):
                self.cache = Path(self.tmp.name) / f"passes-{ext}"
                self.sidecar("output", b"1\n")
                self.passes_once()
                self.sidecar(ext, data)
                self.assertEqual(self.again()[1], 1)
                (self.group / f"p.{ext}").unlink()

    def test_a_changed_added_or_renamed_data_file_runs(self):
        folder = self.group / "p.files"
        folder.mkdir()
        (folder / "a.bin").write_bytes(b"one")
        self.passes_once()
        self.assertEqual(self.again()[1], 0)
        (folder / "a.bin").write_bytes(b"two")
        self.assertEqual(self.again()[1], 1)
        self.assertEqual(self.again()[1], 0)
        (folder / "b.bin").write_bytes(b"")
        self.assertEqual(self.again()[1], 1)
        (folder / "b.bin").rename(folder / "c.bin")
        self.assertEqual(self.again()[1], 1)
        # The same files under no folder at all are another run.
        shutil.rmtree(folder)
        self.assertEqual(self.again()[1], 1)

    def test_a_changed_source_runs(self):
        self.passes_once()
        self.bas.write_bytes(b"PRINT 1 ' the same program, another file\n")
        self.assertEqual(self.again()[1], 1)

    def test_a_changed_screen_state_or_keys_file_runs(self):
        self.sidecar("screen", b"state\n")
        tools = Tools(runs=[b"1\n"], states=[b"state\n"], exe_bytes=b"exe one")
        self.assertEqual(self.go(tools).status, "PASS")
        tools = Tools(runs=[b"1\n"], states=[b"state\n"], exe_bytes=b"exe one")
        self.assertEqual((self.go(tools).detail, len(tools.run_cmds)), (rlt.PASSED_BEFORE, 0))
        self.sidecar("keys", b"key enter\n")
        tools = Tools(runs=[b"1\n"], states=[b"state\n"], exe_bytes=b"exe one")
        self.go(tools)
        self.assertEqual(len(tools.run_cmds), 1)
        self.sidecar("screen", b"another\n")
        tools = Tools(runs=[b"1\n"], states=[b"state\n"], exe_bytes=b"exe one")
        self.assert_fail(self.go(tools), "screen")

    def test_a_failure_is_not_remembered(self):
        r, runs = self.again(runs=[b"2\n"])
        self.assert_fail(r, "result")
        r, runs = self.again(runs=[b"2\n"])
        self.assertEqual(runs, 1)
        self.assert_fail(r, "result")

    def test_record_runs_and_remembers_nothing(self):
        self.passes_once()
        tools = Tools(runs=[b"1\n", b"1\n"], exe_bytes=b"exe one")
        self.go(tools, record=True)
        self.assertEqual(len(tools.run_cmds), 2)
        self.assertEqual(len(list(self.cache.iterdir())), 1)

    def test_without_the_option_every_program_runs(self):
        self.passes_once()
        tools = Tools(runs=[b"1\n"], exe_bytes=b"exe one")
        self.run_test(tools)
        self.assertEqual(len(tools.run_cmds), 1)


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

    @unittest.skipUnless(os.name == "nt", "the helper and its desktop are Windows only")
    def test_a_screen_program_runs_on_a_hidden_desktop_unless_windows_are_asked_for(self):
        with mock.patch.dict(os.environ, {"QB64RUST_SHOW_WINDOWS": ""}):
            tools = Tools(states=[STATE])
            self.assertEqual(self.run_test(tools).status, "PASS")
            self.assertIn("--hidden-desktop", tools.run_cmds[0])
        with mock.patch.dict(os.environ, {"QB64RUST_SHOW_WINDOWS": "1"}):
            tools = Tools(states=[STATE])
            self.assertEqual(self.run_test(tools).status, "PASS")
            self.assertNotIn("--hidden-desktop", tools.run_cmds[0])

    def test_a_program_without_screen_gets_neither_the_flag_nor_the_variable(self):
        self.screen.unlink()
        tools = Tools(states=[STATE])
        self.assertEqual(self.run_test(tools).status, "PASS")
        self.assertFalse([a for a in tools.compile_cmd if "ExtraLinkerFlags" in a])
        self.assertNotIn("QB64_SCREEN_STATE", tools.run_envs[0])
        self.assertNotIn("--hidden-desktop", tools.run_cmds[0])

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


class Scripts(CorpusFixture):
    """The scripts of a screen program, <name>.<script>.keys (spec testing/screen-state, "Scripts of a
    screen program"), --full-states and --coverage."""

    def setUp(self):
        super().setUp()
        self.sidecar("output", b"")
        self.sidecar("screen", STATE)
        self.script = self.group / "p.s1.bas"  # the test of script s1; no such file
        self.sidecar("s1.keys", b"text a\n")
        self.sidecar("s1.output", b"")

    def test_a_script_is_a_test_judged_by_its_own_files(self):
        self.sidecar("s1.screen", STATE)
        tools = Tools(states=[STATE])
        r = self.run_test(tools, bas=self.script)
        self.assertEqual((r.status, r.test), ("PASS", "g/p.s1"), r.detail)
        self.assertIn("p.bas", tools.compile_cmd)
        self.assertEqual((self.results / "g-p.s1-keys.txt").read_bytes(), b"text a\n")
        other = STATE.replace(b"display 0", b"display 1")
        self.assert_fail(self.run_test(Tools(states=[other]), bas=self.script), "screen")

    def test_a_script_without_its_screen_file(self):
        # The spec's scenario: that test fails at stage "expected", and the program's own test runs.
        r = self.run_test(Tools(states=[STATE]), bas=self.script)
        self.assert_fail(r, "expected", "p.s1.screen")
        self.assertEqual(self.run_test(Tools(states=[STATE])).status, "PASS")

    def test_record_writes_a_new_scripts_screen(self):
        (self.group / "p.s1.output").unlink()
        r = self.run_test(
            Tools(states=[STATE, STATE]), qb64="qb64pe.exe", record=True, bas=self.script
        )
        self.assertEqual(
            (r.status, r.detail), ("PASS", "recorded new p.s1.output; recorded new p.s1.screen")
        )
        self.assertEqual((self.group / "p.s1.screen").read_bytes(), STATE)

    def test_the_tests_of_a_program_share_one_build(self):
        # The spec's scenario "One build, two scripts".
        self.sidecar("s1.screen", STATE)
        self.sidecar("s2.keys", b"text b\n")
        self.sidecar("s2.output", b"")
        self.sidecar("s2.screen", STATE)
        tools = Tools(states=[STATE], exe_bytes=b"exe")
        builds = rlt.SharedBuilds()
        tests = rlt.corpus_tests(self.corpus, "*.bas")
        self.assertEqual([t.name for t in tests], ["p.bas", "p.s1.bas", "p.s2.bas"])
        results = [self.run_test(tools, bas=t, builds=builds) for t in tests]
        self.assertEqual(
            [(r.status, r.test) for r in results],
            [("PASS", "g/p"), ("PASS", "g/p.s1"), ("PASS", "g/p.s2")],
        )
        self.assertEqual(tools.compiles, 1)
        # Without the shared builds (one test alone), each test builds.
        self.run_test(tools, bas=self.script)
        self.assertEqual(tools.compiles, 2)

    def test_glob_and_lists_take_the_test_name(self):
        def names(pattern):
            return [t.name for t in rlt.corpus_tests(self.corpus, pattern)]

        self.assertEqual(names("p.bas"), ["p.bas"])
        self.assertEqual(names("p.s*.bas"), ["p.s1.bas"])
        lst = self.corpus / "pass.list"
        lst.write_text("g/p\ng/p.s1\n", encoding="utf-8")
        self.assertEqual(
            rlt.load_list(lst, self.corpus), {self.bas.resolve(), self.script.resolve()}
        )
        lst.write_text("g/p.s9\n", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "no program g/p.s9.bas"):
            rlt.load_list(lst, self.corpus)
        self.assertEqual(rlt.result_key("corpus", self.script, self.corpus), ("corpus", "g/p.s1"))

    def test_a_scripts_inputs_hold_its_program(self):
        inputs = {p.name for p in rlt.program_inputs("corpus", self.script)}
        self.assertEqual(inputs, {"p.s1.keys", "p.s1.output", "p.bas"})

    def test_the_pass_cache_reads_the_programs_source(self):
        self.sidecar("s1.screen", STATE)
        cache = Path(self.tmp.name) / "passes"

        def runs():
            tools = Tools(states=[STATE], exe_bytes=b"exe")
            self.assertEqual(
                self.run_test(tools, bas=self.script, pass_cache=cache).status, "PASS"
            )
            return len(tools.run_cmds)

        self.assertEqual(runs(), 1)
        self.assertEqual(runs(), 0)
        self.bas.write_bytes(b"PRINT 2\n")
        self.assertEqual(runs(), 1)

    def test_full_states_for_one_test(self):
        self.sidecar("s1.screen", STATE)
        tools = Tools(states=[STATE])
        r = self.run_test(tools, bas=self.script, full_states="g/p.s1")
        self.assert_fail(r, "screen", "--full-states: the states are in g-p.s1-run1-screen.txt")
        self.assertEqual(tools.run_envs[0]["QB64_SCREEN_DIGEST_AS_STATE"], "1")
        tools = Tools(states=[STATE])
        self.assertEqual(self.run_test(tools, full_states="g/p.s1").status, "PASS")
        self.assertNotIn("QB64_SCREEN_DIGEST_AS_STATE", tools.run_envs[0])

    def test_coverage_builds_with_the_option_and_appends_each_runs_counts(self):
        self.sidecar("s1.screen", STATE)
        counts = Path(self.tmp.name) / "counts.txt"
        for bas, record in ((self.bas, b"one\n"), (self.script, b"two\n")):
            tools = Tools(states=[STATE], counts=record)
            self.assertEqual(self.run_test(tools, bas=bas, coverage=counts).status, "PASS")
            self.assertIn("--coverage", tools.compile_cmd)
        self.assertEqual(counts.read_bytes(), b"one\ntwo\n")
        tools = Tools(states=[STATE], counts=b"x\n")
        self.run_test(tools)
        self.assertNotIn("--coverage", tools.compile_cmd)
        self.assertNotIn("QB64RUST_COVERAGE", tools.run_envs[0])

    def test_coverage_leaves_out_the_counts_of_a_run_that_timed_out(self):
        # A run killed by the timeout may have been writing its record: half a record would stop the merge.
        self.sidecar("s1.screen", STATE)
        counts = Path(self.tmp.name) / "counts.txt"
        tools = Tools(states=[STATE], counts=b"coverage 1\nfile", run_times_out=True)
        self.assert_fail(self.run_test(tools, bas=self.bas, coverage=counts), "run", "timeout")
        self.assertFalse(counts.exists())


class VerificationText(unittest.TestCase):
    """verification_text: the text of <name>.out.txt, written by the recorder and built by the compare."""

    def test_the_folder_is_masked_in_any_spelling(self):
        folder = Path("Work/Some Folder/verification")
        out = b"work\\some folder\\VERIFICATION/v.exe and Work/Some Folder//verification x\r\n"
        self.assertEqual(
            rlt.verification_text(out, 0, False, folder),
            b"<verification>/v.exe and <verification> x\r\nexit=0\n",
        )

    def test_exit_codes_as_git_bash_reports_them(self):
        self.assertEqual(rlt.verification_text(b"", 0xC0000005, False, Path("x")), b"exit=139\n")
        self.assertEqual(rlt.verification_text(b"a\n", 256 + 3, False, Path("x")), b"a\nexit=3\n")

    def test_a_timeout(self):
        self.assertEqual(
            rlt.verification_text(b"Press", None, True, Path("x")),
            b"Press(timed out after 60 s)\nexit=124\n",
        )


class VerificationSupport(unittest.TestCase):
    def test_family_numbers(self):
        self.assertEqual(rlt.family("v16_m8_a"), "v16")
        self.assertEqual(rlt.family("v37c.a"), "v37")
        self.assertEqual(rlt.family("v02b_idiv_zero"), "v02")
        self.assertEqual(rlt.family("V44_files"), "v44")
        self.assertIsNone(rlt.family("cp437_all_bytes.bin"))

    def test_the_family_and_no_family_without_programs_records_or_scripts(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            for name in (
                "v16_inc/a.bi",
                "v16_y.bi",
                "v16_p.bas",
                "v16_p.out.txt",
                "v16_p.exe",
                "v16_p.exe.qb64rust/x.cpp",
                "v16_p.stdin",
                "v16_p.noprompt",
                "v16_names.py",
                "v160_z.bi",
                "v17_z.bi",
                "cp437_all_bytes.bin",
                "README.md",
                "run.sh",
                "__pycache__/x.pyc",
            ):
                (root / name).parent.mkdir(parents=True, exist_ok=True)
                (root / name).write_bytes(b"")
            got = [p.name for p in rlt.verification_support(root, "v16_m8_a")]
            self.assertEqual(got, ["cp437_all_bytes.bin", "v16_inc", "v16_y.bi"])
            # Another family's item that the source names comes too.
            (root / "v16_m8_a.bas").write_bytes(b"$EXEICON:'./V17_Z.bi'\n")
            got = [p.name for p in rlt.verification_support(root, "v16_m8_a")]
            self.assertEqual(got, ["cp437_all_bytes.bin", "v16_inc", "v16_y.bi", "v17_z.bi"])


class VerificationFixture(unittest.TestCase):
    """One program v99_p of a verification folder in a scratch folder, and the compare run on it."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        base = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)
        self.root, self.results, self.lists = (
            base / "verification",
            base / "results",
            base / "lists",
        )
        for d in (self.root, self.results, self.lists):
            d.mkdir()
        self.bas = self.root / "v99_p.bas"
        self.bas.write_bytes(b"PRINT 1\n")
        patch = mock.patch.object(rlt, "VERIFICATION_LISTS", self.lists)
        patch.start()
        self.addCleanup(patch.stop)
        rlt.support_items.cache_clear()
        self.addCleanup(rlt.support_items.cache_clear)

    def recorded(self, data: bytes) -> None:
        (self.root / "v99_p.out.txt").write_bytes(data)

    def sidecar(self, kind: str, data: bytes) -> None:
        path = self.lists / kind / f"v99_p.{kind}"
        path.parent.mkdir(exist_ok=True)
        path.write_bytes(data)

    def run_test(self, tools: Tools, pass_cache=None):
        rlt.support_items.cache_clear()
        args = SimpleNamespace(qb64=Path("qb64rust.exe"), pass_cache=pass_cache)
        with mock.patch.object(rlt, "run_proc", tools):
            return rlt.run_suite_test(
                "verification",
                self.bas,
                args=args,
                results=self.results,
                qb_root=self.results,
                corpus_root=self.results,
                known={},
            )[0]

    def assert_fail(self, r, stage: str, detail: str = ""):
        self.assertEqual((r.status, r.stage), ("FAIL", stage), r.detail)
        self.assertIn(detail, r.detail)

    def snapshot(self) -> dict:
        return {p: p.read_bytes() for p in sorted(self.root.rglob("*")) if p.is_file()}


class VerificationCompare(VerificationFixture):
    """verification_compare: every rule of the compare (spec testing/verification-suite)."""

    def test_a_rejection_by_both_passes(self):
        self.recorded(b"(not compiled)\n")
        r = self.run_test(Tools(rc=1, compile_out=b"p.bas:1:1: error: x\n1 error\n", exe=False))
        self.assertEqual(
            (r.status, r.kind, r.suite, r.test), ("PASS", "error", "verification", "v99_p")
        )
        self.assertFalse((self.results / "v99_p").exists())

    def test_a_rejection_with_only_marked_errors_fails(self):
        self.recorded(b"(not compiled)\r\n")
        tools = Tools(rc=1, compile_out=b"2 errors (2 not supported yet)\n", exe=False)
        self.assert_fail(
            self.run_test(tools), "error result", "only errors marked not supported yet"
        )

    def test_a_program_the_old_compiler_rejected_must_not_build(self):
        self.recorded(b"(not compiled)\n")
        self.assert_fail(self.run_test(Tools()), "compile", "the old compiler rejected it")
        tools = Tools(rc=1, compile_out=b"1 error\n", exe=True)
        self.assert_fail(self.run_test(tools), "exe exists")

    def test_equal_output_passes_and_the_work_folder_goes(self):
        self.recorded(b"1\n\nexit=0\n")
        tools = Tools(runs=[b"1\r\n\r\n"])
        r = self.run_test(tools)
        self.assertEqual((r.status, r.kind), ("PASS", "output"), r.detail)
        self.assertFalse((self.results / "v99_p").exists())
        self.assertEqual(tools.compile_cmd[:5], ["qb64rust.exe", "-x", "-q", "-m", "v99_p.bas"])
        self.assertNotIn("--include-root", tools.compile_cmd)

    def test_different_output_fails_and_the_work_folder_stays(self):
        self.recorded(b"1\nexit=0\n")
        r = self.run_test(Tools(runs=[b"2\r\n"]))
        self.assert_fail(r, "result", "line 1: recorded b'1', got b'2'")
        self.assertTrue((self.results / "v99_p" / "v99_p.bas").is_file())

    def test_a_different_exit_code_fails(self):
        self.recorded(b"1\nexit=0\n")
        self.assert_fail(self.run_test(Tools(runs=[b"1\n"], run_rc=3)), "result", "exit=3")
        self.recorded(b"exit=139\n")
        self.assertEqual(self.run_test(Tools(runs=[b""], run_rc=0xC0000005)).status, "PASS")

    def test_a_timeout_on_both_sides_passes(self):
        self.recorded(b"Press any key(timed out after 60 s)\nexit=124\n")
        r = self.run_test(Tools(runs=[b"Press any key"], run_times_out=True))
        self.assertEqual(r.status, "PASS", r.detail)
        self.assert_fail(self.run_test(Tools(runs=[b"Press any key"])), "result")
        self.recorded(b"exit=0\n")
        self.assert_fail(self.run_test(Tools(runs=[b""], run_times_out=True)), "result")

    def test_nothing_recorded_fails(self):
        tools = Tools()
        self.assert_fail(self.run_test(tools), "expected", "nothing recorded to compare")
        self.assertEqual(tools.compile_cmd, [])

    def test_the_run_folder_is_masked(self):
        self.recorded(b"<verification>/v99_p.exe\nexit=0\n")
        work = str(self.results / "v99_p").upper().replace("\\", "/")
        r = self.run_test(Tools(runs=[f"{work}/v99_p.exe\r\n".encode()]))
        self.assertEqual(r.status, "PASS", r.detail)

    def test_the_support_files_and_sidecars_are_copied(self):
        self.recorded(b"1\nexit=0\n")
        for name, data in (
            ("v99_inc/a.bi", b"a"),
            ("v99_y.bi", b"y"),
            ("v98_z.bi", b"z"),
            ("cp437_all_bytes.bin", b"c"),
            ("v99_p.stdin", b"42\r\n"),
            ("v99_p.noprompt", b"continue\n"),
            ("v99_q.stdin", b"q"),
        ):
            (self.root / name).parent.mkdir(exist_ok=True)
            (self.root / name).write_bytes(data)
        tools = Tools(runs=[b"2\n"])
        self.assert_fail(self.run_test(tools), "result")
        work = self.results / "v99_p"
        self.assertEqual(
            sorted(p.relative_to(work).as_posix() for p in work.rglob("*") if p.is_file()),
            [
                "cp437_all_bytes.bin",
                "v99_inc/a.bi",
                "v99_p.bas",
                "v99_p.exe",
                "v99_p.noprompt",
                "v99_p.stdin",
                "v99_y.bi",
            ],
        )
        self.assertEqual(tools.run_envs[0]["QB64PE_NOPROMPT"], "continue")
        given = tools.run_cmds[0][-1] if os.name == "nt" else str(tools.run_stdin[0])
        self.assertEqual(given, str(work / "v99_p.stdin"))

    @unittest.skipUnless(os.name == "nt", "press_any_key.py runs on Windows only")
    def test_the_run_is_out_of_sight_without_key_presses(self):
        self.recorded(b"exit=0\n")
        tools = Tools(runs=[b""])
        self.run_test(tools)
        self.assertEqual(tools.run_cmds[0][2:4], ["--no-keys", "--hidden-desktop"])
        self.assertEqual(tools.run_cmds[0][-1], str(self.results / "v99_p" / "v99_p.exe"))

    def test_the_verification_folder_is_left_alone(self):
        self.recorded(b"1\nexit=0\n")
        (self.root / "v99_inc").mkdir()
        (self.root / "v99_inc" / "a.bi").write_bytes(b"a")
        before = self.snapshot()
        self.run_test(Tools(runs=[b"1\n"]))
        self.run_test(Tools(runs=[b"2\n"]))
        self.recorded(b"(not compiled)\n")
        before[self.root / "v99_p.out.txt"] = b"(not compiled)\n"
        self.run_test(Tools(rc=1, compile_out=b"1 error\n", exe=False))
        self.assertEqual(self.snapshot(), before)

    def test_a_normalize_sidecar_applies_to_both_sides(self):
        self.recorded(b"took 12 ms\nexit=0\n")
        self.sidecar("normalize", b"\\d+ ms\t<t> ms\n")
        self.assertEqual(self.run_test(Tools(runs=[b"took 31 ms\r\n"])).status, "PASS")

    def test_a_divergence_not_listed_fails(self):
        self.recorded(b"1\nexit=0\n")
        tools = Tools(runs=[b"1\n"], trace=b"D-026\n")
        self.assert_fail(self.run_test(tools), "divergences", "took D-026 of DIVERGENCES.md")
        self.assertIn(rlt.TRACE_ENV, tools.run_envs[0])
        self.sidecar("divergences", b"D-026  # the math functions\n")
        self.assertEqual(self.run_test(Tools(runs=[b"1\n"], trace=b"D-026\n")).status, "PASS")

    def test_a_v37_program_needs_its_libraries(self):
        self.bas = self.root / "v37_x01.bas"
        self.bas.write_bytes(b"")
        (self.root / "v37_x01.out.txt").write_bytes(b"exit=0\n")
        self.assert_fail(
            self.run_test(Tools(runs=[b""])), "expected", "v37_lib/build_libs.py first"
        )
        for lib in rlt.V37_LIBRARIES:
            (self.root / lib).write_bytes(b"")
        self.assertEqual(self.run_test(Tools(runs=[b""])).status, "PASS")
        self.assertFalse((self.results / "v37_x01").exists())

    def test_the_pass_cache(self):
        cache = Path(self.tmp.name) / "passes"
        self.recorded(b"1\nexit=0\n")
        (self.root / "v99_y.bi").write_bytes(b"y")
        self.assertEqual(self.run_test(Tools(runs=[b"1\n"]), cache).detail, "")
        tools = Tools(runs=[b"wrong\n"])
        r = self.run_test(tools, cache)
        self.assertEqual((r.status, r.detail, len(tools.run_cmds)), ("PASS", rlt.PASSED_BEFORE, 0))
        # A changed support file runs it again.
        (self.root / "v99_y.bi").write_bytes(b"z")
        self.assert_fail(self.run_test(Tools(runs=[b"wrong\n"]), cache), "result")

    def test_the_recorder_still_records_with_qb64pe(self):
        tools = Tools(runs=[b"1\r\n"])
        args = SimpleNamespace(qb64=Path("qb64pe.exe"))
        with mock.patch.object(rlt, "run_proc", tools):
            r = rlt.run_suite_test(
                "verification",
                self.bas,
                args=args,
                results=self.results,
                qb_root=self.results,
                corpus_root=self.results,
                known={},
            )[0]
        self.assertEqual((r.status, r.kind), ("PASS", "recorded"), r.detail)
        self.assertEqual((self.root / "v99_p.out.txt").read_bytes(), b"1\r\nexit=0\n")
        self.assertEqual(tools.run_cmds[0], [str(self.root / "v99_p.exe")])


class VerificationLists(VerificationFixture):
    """The lists of tests/verification as the runner reads them."""

    def test_known_list(self):
        path = self.lists / "known.list"
        path.write_text("# comment\n\nv01_a\tD-026\nv02_b\tbug: PRINT USING ;\n", encoding="utf-8")
        self.assertEqual(
            rlt.load_known_list(path),
            {"verification:v01_a": "D-026", "verification:v02_b": "bug: PRINT USING ;"},
        )
        self.assertEqual(rlt.load_known_list(self.lists / "none.list"), {})

    def test_a_pass_of_a_program_that_varies_is_no_news(self):
        self.assertTrue(rlt.pass_is_news("verification:v01_a", "D-026"))
        self.assertTrue(rlt.pass_is_news("verification:v01_a", "bug: PRINT USING"))
        self.assertFalse(rlt.pass_is_news("verification:v29_b", "varies: SOUND's time"))
        self.assertFalse(rlt.pass_is_news("verification:v29_b", "varies"))
        # known_failures.txt of the other suites keeps its note.
        self.assertTrue(rlt.pass_is_news("corpus:slice/s01", "varies"))

    def test_result_key_and_inputs(self):
        self.assertEqual(
            rlt.result_key("verification", self.bas, self.root), ("verification", "v99_p")
        )
        for name in (
            "v99_p.out.txt",
            "v99_p.compile.txt",
            "v99_p.exe",
            "v99_p.stdin",
            "v99_pq.bas",
        ):
            (self.root / name).write_bytes(b"")
        self.sidecar("normalize", b"")
        self.sidecar("divergences", b"")
        got = sorted(p.name for p in rlt.program_inputs("verification", self.bas))
        self.assertEqual(
            got,
            [
                "v99_p.bas",
                "v99_p.compile.txt",
                "v99_p.divergences",
                "v99_p.normalize",
                "v99_p.out.txt",
                "v99_p.stdin",
            ],
        )

    def test_check_list(self):
        compiler = Path(self.tmp.name) / "qb64rust.exe"
        compiler.write_bytes(b"build")
        run = {
            "version": rlt.RUN_RECORD_VERSION,
            "suite": "verification",
            "selection": {},
            "roots": {"verification": rlt.repo_relative(self.root.resolve())},
            "no_fold": False,
            "compiler_sha256": rlt.file_sha256(compiler),
            "runner_sha256": rlt.file_sha256(Path(rlt.__file__).resolve()),
            "started": time.time() + 60,
        }
        result = {
            "suite": "verification",
            "test": "v99_p",
            "status": "PASS",
            "detail": "",
            "stage": "",
        }
        data = {"run": run, "results": [result]}
        wanted = {self.bas.resolve()}
        self.assertEqual(
            rlt.check_list("verification", wanted, self.root, data, compiler), ([], [], 1)
        )
        result["status"] = "KFAIL"
        _, failures, _ = rlt.check_list("verification", wanted, self.root, data, compiler)
        self.assertEqual(failures[0][0], "verification:v99_p")

    def test_load_list_takes_plain_names(self):
        path = self.lists / "pass.list"
        path.write_text("v99_p  # a comment\n", encoding="utf-8")
        self.assertEqual(rlt.load_list(path, self.root), {self.bas.resolve()})


@unittest.skipUnless(os.name == "nt", "press_any_key.py runs on Windows only")
class NoKeys(unittest.TestCase):
    def test_a_program_waiting_for_a_key_times_out(self):
        # The stand-in waits at its "END" for a console key, as a QB64pe program does; with --no-keys no
        # key press comes, so it runs into the timeout, and what it printed before is kept. python.exe
        # runs the script it reads from its stdin file.
        with tempfile.TemporaryDirectory() as tmp:
            script, out = Path(tmp, "wait.py"), Path(tmp, "out.txt")
            script.write_text(
                "import msvcrt, sys, time\n"
                "print('Press any key', flush=True)\n"
                "try:\n"
                "    msvcrt.getch()\n"
                "except OSError:\n"
                "    pass\n"
                "time.sleep(60)\n",  # without a console getch fails at once; the stand-in waits anyway
                encoding="ascii",
            )
            helper = Path(__file__).with_name("press_any_key.py")
            cmd = [sys.executable, str(helper), "--no-keys", "--hidden-desktop", sys.executable]
            t0 = time.time()
            rc, timed_out = rlt.run_proc(
                cmd + [str(script)], tmp, out, 3, merge_stderr=True, ask_first=True
            )
            self.assertEqual((rc, timed_out), (None, True), out.read_bytes())
            self.assertGreaterEqual(time.time() - t0, 3)
            self.assertIn(b"Press any key", out.read_bytes())


if __name__ == "__main__":
    unittest.main()
