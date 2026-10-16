#!/usr/bin/env python3
"""Tests of test_time.py's own rules: which commands are on record and as what, what is known of
a run's result, and how a transcript's lines become runs and times.

    python -m unittest discover -s tools/testtime -p "test_*.py"

A change to what counts as a test run, a measurement or a build comes with a change here.
"""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import test_time
from test_time import BUILD, FAIL, MEASURE, PASS, TEST, classify, result_of, said

RUNNER = "python tools/legacy_tests/run_legacy_tests.py"
NEW = "--qb64 target/release/qb64rust.exe"
SLICE = f"{RUNNER} {NEW} --list tests/corpus/slice.list"
UPSTREAM = f"{RUNNER} {NEW} --suite compile --list tests/upstream/pass.list"


def kinds(command: str) -> list[str]:
    return [kind for _, kind in classify(command)]


class Classify(unittest.TestCase):
    def test_commands_on_record(self):
        differential = "--corpus-root tests/differential --list tests/differential/pass.list"
        cases = [
            ("cargo test", [(TEST, "tier 1 (cargo test)")]),
            ("cargo test --workspace --locked 2>&1 | tail -5", [(TEST, "tier 1 (cargo test)")]),
            ("cargo +stable test", [(TEST, "tier 1 (cargo test)")]),
            ("cargo test -p qb64rust-sema", [(TEST, "tier 1, part (cargo test -p)")]),
            ("cargo test --test frontend", [(TEST, "tier 1, part (cargo test -p)")]),
            ("cargo clippy --workspace --all-targets -- -D warnings", [(TEST, "clippy")]),
            ("cargo fmt --all -- --check", [(TEST, "fmt")]),
            ("cargo build --release", [(BUILD, "cargo build")]),
            ("cargo check", [(BUILD, "cargo build")]),
            (f"{SLICE} --jobs 8", [(TEST, "slice list")]),
            (f"{RUNNER} {NEW} --suite corpus", [(TEST, "full corpus")]),
            (f"{RUNNER} {NEW} --glob 's7*'", [(TEST, "corpus, selected programs")]),
            (f"{RUNNER} {NEW} {differential}", [(TEST, "differential pass list")]),
            (UPSTREAM, [(TEST, "upstream pass list")]),
            (f"{RUNNER} {NEW} --suite compile", [(TEST, "full upstream suite")]),
            (
                f"{RUNNER} {NEW} --suite compile --category arrays",
                [(TEST, "upstream, selected programs")],
            ),
            (f"{RUNNER} {NEW} --suite format", [(TEST, "other suite")]),
            (f"{RUNNER} --suite corpus", [(MEASURE, "full corpus (old compiler)")]),
            (
                f"{RUNNER} --suite corpus --record --glob 'g07*'",
                [(MEASURE, "recording expected results (old compiler)")],
            ),
            (f"{RUNNER} --suite verification", [(MEASURE, "verification programs (old compiler)")]),
            ("bash verification/run.sh", [(MEASURE, "verification programs (old compiler)")]),
            ("python tools/callsite/callsite.py --qb-root ../QB64pe", [(TEST, "call-site check")]),
            ("python tools/callsite/callsite.py --self-test", [(TEST, "call-site self-test")]),
            (
                'python -m unittest discover -s tools/legacy_tests -p "test_*.py"',
                [(TEST, "tests of the Python tools")],
            ),
            (
                "python tools/legacy_tests/test_run_legacy_tests.py 2>&1 | tail -2",
                [(TEST, "tests of the Python tools")],
            ),
            ("python tools/repo_check/check_repo.py --untracked", [(TEST, "repo check")]),
            ("openspec validate m3-get-put-bload", [(TEST, "openspec validate")]),
            ("npm test", [(TEST, "extension tests (npm)")]),
            ("npm run lint", [(TEST, "extension tests (npm)")]),
            ("npm run compile", [(BUILD, "extension build (npm)")]),
            ("../QB64pe/qb64pe.exe -x t.bas -o t.exe", [(MEASURE, "old compiler run directly")]),
        ]
        for command, expected in cases:
            with self.subTest(command=command):
                self.assertEqual(classify(command), expected)

    def test_commands_not_on_record(self):
        for command in [
            "git status --short",
            "ls tests/corpus/screen",
            "grep -rn 'cargo test' crates/README.md | head",
            "python tools/testtime/test_time.py",
            "target/debug/qb64rust.exe t.bas --dump ir",
            "cat tools/legacy_tests/run_legacy_tests.py",
            # Asking a tool for its options.
            f"{RUNNER} --help 2>&1 | head -50",
            "python tools/callsite/callsite.py -h",
            # "callsite.py" ends in "py", and a path follows it.
            "git diff -- tools/callsite/callsite.py tools/legacy_tests/run_legacy_tests.py",
            # Text in quotes, wherever it stands in them.
            'git commit -m "run cargo test before the close"',
        ]:
            with self.subTest(command=command):
                self.assertEqual(classify(command), [])

    def test_an_option_of_the_program_the_output_goes_to_is_not_the_runs(self):
        self.assertEqual(kinds(f"{SLICE} 2>&1 | grep -h FAIL"), ["slice list"])
        self.assertEqual(kinds("python tools/repo_check/check_repo.py | sort -h"), ["repo check"])
        self.assertEqual(kinds("cargo test -q 2>&1 | grep -p x"), ["tier 1 (cargo test)"])

    def test_an_option_of_the_tests_is_no_choice_of_them(self):
        self.assertEqual(kinds("cargo test -- --test-threads=1"), ["tier 1 (cargo test)"])
        self.assertEqual(kinds("cargo test --test=frontend"), ["tier 1, part (cargo test -p)"])

    def test_an_option_of_python_with_a_value(self):
        self.assertEqual(kinds("python -X utf8 tools/callsite/callsite.py"), ["call-site check"])
        self.assertEqual(kinds("python -I tools/repo_check/check_repo.py"), ["repo check"])

    def test_one_command_with_several_runs(self):
        got = classify("cargo fmt --all -- --check && cargo clippy --workspace && cargo test")
        self.assertEqual(got, [(TEST, "tier 1 (cargo test)"), (TEST, "clippy"), (TEST, "fmt")])

    def test_each_statement_of_a_line_is_read_by_itself(self):
        # Read as one text, the second run's list would make both the same kind.
        self.assertEqual(
            kinds(f"{UPSTREAM} | tail -2; {SLICE}"), ["upstream pass list", "slice list"]
        )
        self.assertEqual(
            kinds(f"cargo build -q 2>&1 | tail -1 && {RUNNER} --help"), ["cargo build"]
        )

    def test_a_continued_line_is_one_statement(self):
        command = f"{RUNNER} {NEW} --suite corpus \\\n  --list tests/corpus/slice.list"
        self.assertEqual(kinds(command), ["slice list"])

    def test_text_given_to_a_program_is_no_statement(self):
        script = "p='tools/legacy_tests/run_legacy_tests.py'\nprint('cargo test')"
        self.assertEqual(kinds(f"python - <<'EOF'\n{script}\nEOF\ncargo fmt"), ["fmt"])

    def test_a_run_through_a_shell_variable(self):
        # The assignment runs nothing; each use of the variable is a run of its own kind.
        command = (
            f'R="{RUNNER} {NEW} --jobs 8"; $R --suite corpus --list tests/corpus/slice.list | tail -2; '
            "QB64RUST_NO_FOLD=1 ${R} --suite compile --list tests/upstream/pass.list"
        )
        self.assertEqual(kinds(command), ["slice list", "upstream pass list"])
        self.assertEqual(kinds('export R="cargo test -q"\n$R'), ["tier 1 (cargo test)"])
        self.assertEqual(kinds("X=1; echo $Y"), [])

    def test_an_environment_variable_set_before_a_run_makes_it_another_run(self):
        set_, unset = "$env:QB64RUST_NO_FOLD='1'", "Remove-Item Env:QB64RUST_NO_FOLD"
        got = [
            same
            for _, _, same in test_time.classify_parts(
                f"{SLICE}; {set_}; {SLICE}; {unset}; {SLICE}"
            )
        ]
        self.assertEqual(got, [SLICE, f"QB64RUST_NO_FOLD=1 {SLICE}", SLICE])
        self.assertEqual(kinds(f"{set_}; {SLICE}"), ["slice list"])
        # Other ways to remove it: with options after it, set to nothing, unset in bash.
        for unset in (
            "Remove-Item Env:QB64RUST_NO_FOLD -ErrorAction SilentlyContinue",
            "$env:QB64RUST_NO_FOLD = $null",
            "$env:QB64RUST_NO_FOLD=''",
            "unset QB64RUST_NO_FOLD",
        ):
            got = test_time.classify_parts(f"{set_}; {unset}; {SLICE}")
            self.assertEqual([same for _, _, same in got], [SLICE], unset)

    def test_a_trial_corpus_is_not_the_full_corpus(self):
        got = kinds(f'{RUNNER} --suite corpus --corpus-root "$S/corpus21"')
        self.assertEqual(got, ["corpus, selected programs (old compiler)"])

    def test_a_build_beside_tier_1_is_part_of_it(self):
        self.assertEqual(classify("cargo build && cargo test"), [(TEST, "tier 1 (cargo test)")])

    def test_the_old_compiler_inside_another_run_is_not_counted_again(self):
        got = classify("cargo test && ../QB64pe/qb64pe.exe -z t.bas")
        self.assertEqual(got, [(TEST, "tier 1 (cargo test)")])

    def test_the_old_compiler_by_hand_is_one_run_of_a_command(self):
        self.assertEqual(
            kinds("./qb64pe.exe -x a.bas; ./qb64pe.exe -x b.bas"), ["old compiler run directly"]
        )


class Result(unittest.TestCase):
    def test_the_runner_says_it_in_its_summary(self):
        self.assertEqual(
            said("slice list", '{"corpus": {"PASS": 46, "FAIL": 0, "KFAIL": 0}}'), PASS
        )
        self.assertEqual(
            said("slice list", '{"corpus": {"PASS": 27, "FAIL": 21, "KFAIL": 0}}'), FAIL
        )
        self.assertEqual(said("slice list", "[3/9] FAIL corpus:slice/s01 (output, 0.1s)"), FAIL)
        self.assertEqual(said("slice list", "Command running in background"), "")

    def test_cargo_test(self):
        ok = "test result: ok. 2 passed; 0 failed"
        self.assertEqual(said("tier 1 (cargo test)", ok), PASS)
        self.assertEqual(
            said("tier 1 (cargo test)", ok + "\ntest result: FAILED. 1 passed; 1 failed"), FAIL
        )
        self.assertEqual(said("tier 1 (cargo test)", "error[E0425]: cannot find value"), FAIL)
        self.assertEqual(said("tier 1 (cargo test)", ""), "")

    def test_a_compile_error_beside_another_run_is_not_known_to_be_the_tests(self):
        error = "error: this could be written as `let...else`"
        self.assertEqual(said("tier 1 (cargo test)", error, shared=True), "")
        self.assertEqual(said("clippy", error, shared=True), FAIL)
        failed = error + "\ntest result: FAILED. 1 passed; 1 failed"
        self.assertEqual(said("tier 1 (cargo test)", failed, shared=True), FAIL)

    def test_clippy_and_build(self):
        done = "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s"
        self.assertEqual(said("clippy", done), PASS)
        self.assertEqual(said("clippy", "warning: unused import: `x`\n" + done), FAIL)
        self.assertEqual(
            said("cargo build", "error: could not compile `qb64rust-sema` (lib)"), FAIL
        )

    def test_the_tools_that_count(self):
        self.assertEqual(
            said("call-site check", "24 programs: 22 equal, 2 differ, 0 not compared"), FAIL
        )
        self.assertEqual(
            said("call-site check", "810 programs: 810 equal, 0 differ, 0 not compared"), PASS
        )
        self.assertEqual(said("repo check", "5096 files checked, 0 findings"), PASS)
        self.assertEqual(said("repo check", "5096 files checked, 2 findings"), FAIL)
        self.assertEqual(said("openspec validate", "Change 'm3-get-put-bload' is valid"), PASS)

    def test_the_exit_code_counts_only_for_a_run_alone(self):
        self.assertEqual(result_of(TEST, "fmt", "", 0, True), PASS)
        self.assertEqual(result_of(TEST, "fmt", "", 1, True), FAIL)
        self.assertEqual(result_of(TEST, "fmt", "", 0, False), "")
        self.assertEqual(result_of(TEST, "fmt", "", None, True), "")

    def test_a_measurement_has_no_result(self):
        self.assertEqual(
            result_of(MEASURE, "full corpus (old compiler)", '{"PASS": 1, "FAIL": 2', 1, True), ""
        )


def line(stamp: str, content: list[dict], session: str = "abcdef0123456789") -> str:
    return json.dumps({"timestamp": stamp, "sessionId": session, "message": {"content": content}})


def at(clock: str, content: list[dict]) -> str:
    return line(f"2026-10-13T10:{clock}Z", content)


def use(ident: str, command: str, tool: str = "Bash", background: bool = False) -> dict:
    arg = {"command": command, "run_in_background": background}
    return {"type": "tool_use", "id": ident, "name": tool, "input": arg}


def result(ident: str, text: str = "ok", error: bool = False) -> dict:
    return {"type": "tool_result", "tool_use_id": ident, "content": text, "is_error": error}


def says(text: str) -> dict:
    return {"type": "text", "text": text}


class ReadSession(unittest.TestCase):
    def read(self, lines: list[str]):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "s.jsonl"
            path.write_text("\n".join(lines) + "\n", encoding="utf-8")
            return test_time.read_session(path)

    def test_a_run_and_the_session_times(self):
        runs, row = self.read(
            [
                at("00:00", [says("start")]),
                at("01:00", [use("a", "cargo test")]),
                at("01:30", [result("a")]),
                at("02:00", [use("b", "git status")]),
                at("02:05", [result("b")]),
            ]
        )
        got = [(r["kind"], r["seconds"], r["background"]) for r in runs]
        self.assertEqual(got, [("tier 1 (cargo test)", "30.0", "0")])
        self.assertEqual(runs[0]["session"], "abcdef01")
        # 60 s before the first command, 30 s inside it, 30 s after it, 5 s inside the second.
        self.assertEqual((row["active_seconds"], row["command_seconds"]), ("125", "35"))

    def test_time_is_split_evenly_between_the_runs_of_one_command(self):
        runs, _ = self.read(
            [
                at("00:00", [use("a", "cargo fmt --check && cargo clippy")]),
                at("00:40", [result("a")]),
            ]
        )
        got = sorted((r["kind"], r["seconds"]) for r in runs)
        self.assertEqual(got, [("clippy", "20.0"), ("fmt", "20.0")])

    def test_and_by_what_each_kind_takes_alone_where_that_is_known(self):
        runs, _ = self.read(
            [
                at("00:00", [use("a", "cargo fmt")]),
                at("00:01", [result("a", "")]),
                at("00:10", [use("b", "cargo test -q")]),
                at("00:29", [result("b", "test result: ok.")]),
                at("01:00", [use("c", "cargo fmt; cargo test -q")]),
                at("01:40", [result("c", "test result: ok.")]),
            ]
        )
        got = sorted((r["kind"], r["seconds"]) for r in runs[2:])
        self.assertEqual(got, [("fmt", "2.0"), ("tier 1 (cargo test)", "38.0")])
        self.assertEqual(sorted(runs[0]), sorted(test_time.RUN_FIELDS))

    def test_a_long_gap_is_the_user_being_away(self):
        _, row = self.read(
            [
                line("2026-10-13T10:00:00Z", [says("a")]),
                line("2026-10-13T10:04:00Z", [says("b")]),
                line("2026-10-13T11:00:00Z", [says("c")]),
            ]
        )
        self.assertEqual(row["active_seconds"], "240")

    def test_a_foreground_command_past_the_limit_waited_for_the_user(self):
        runs, row = self.read([at("00:00", [use("a", "cargo test")]), at("30:00", [result("a")])])
        self.assertEqual(runs, [])
        self.assertEqual(row["command_seconds"], "0")

    def test_unless_it_asked_for_that_long(self):
        asked = use("a", "cargo test")
        asked["input"]["timeout"] = 1200000
        runs, _ = self.read([at("00:00", [asked]), at("15:00", [result("a")])])
        self.assertEqual([r["seconds"] for r in runs], ["900.0"])
        runs, _ = self.read([at("00:00", [asked]), at("30:00", [result("a")])])
        self.assertEqual(runs, [])

    def test_a_time_ending_in_z_is_read(self):
        self.assertEqual(test_time.parse_time("2026-10-13T10:00:00.5Z").utcoffset().seconds, 0)

    def test_a_command_the_user_refused_is_no_run(self):
        refused = test_time.NOT_RUN[0] + ". The tool use was rejected."
        runs, _ = self.read(
            [at("00:00", [use("a", "cargo test")]), at("00:40", [result("a", refused, True)])]
        )
        self.assertEqual(runs, [])

    def test_a_background_run_ends_at_its_notification(self):
        runs, _ = self.read(
            [
                at("00:00", [use("a", "cargo test", background=True)]),
                at("00:01", [result("a", "Command running in background with ID: bg1")]),
                at("03:00", [says("<task-id>bg1</task-id> done")]),
            ]
        )
        self.assertEqual([(r["seconds"], r["background"]) for r in runs], [("180.0", "1")])

    def test_a_background_run_the_session_did_not_see_end_is_left_out(self):
        runs, _ = self.read(
            [
                at("00:00", [use("a", "cargo test", background=True)]),
                at("00:01", [result("a", "Command running in background with ID: bg1")]),
            ]
        )
        self.assertEqual(runs, [])

    def test_a_run_that_reached_its_time_limit_went_on_in_the_background(self):
        moved = "Command did not complete within its 600s timeout and was moved to the background (ID: b1)."
        note = (
            '<task-id>b1</task-id><summary>Background command "x" completed (exit code 0)</summary>'
        )
        runs, _ = self.read(
            [
                at("00:00", [use("a", "cargo build --release")]),
                at("10:00", [result("a", moved)]),
                at("12:00", [says(note)]),
            ]
        )
        got = [(r["seconds"], r["background"], r["result"]) for r in runs]
        self.assertEqual(got, [("600.0", "0", ""), ("120.0", "1", PASS)])

    def test_a_repeat_is_the_same_statement_with_nothing_edited_since(self):
        ok = "test result: ok. 3 passed; 0 failed"
        failed = "Exit code 101\ntest result: FAILED. 2 passed; 1 failed"
        runs, _ = self.read(
            [
                at("00:00", [use("a", "cargo test -q")]),
                at("00:20", [result("a", ok)]),
                at("00:30", [use("b", "cargo test -q | tail -3")]),
                at("00:50", [result("b", ok)]),
                at("01:00", [{"type": "tool_use", "id": "e", "name": "Edit", "input": {}}]),
                at("01:10", [use("c", "cargo test -q")]),
                at("01:30", [result("c", failed, True)]),
                at("01:40", [use("d", "cargo test -q -p qb64rust-sema")]),
                at("01:50", [result("d", ok)]),
            ]
        )
        self.assertEqual([r["result"] for r in runs], [PASS, PASS, FAIL, PASS])
        self.assertEqual([r["repeat_after"] for r in runs], ["", PASS, "", ""])

    def test_the_rows_of_a_session_do_not_depend_on_other_sessions(self):
        chained = [at("01:00", [use("c", "cargo fmt; cargo test -q")]), at("01:40", [result("c")])]
        runs, _ = self.read(chained)
        self.assertEqual(sorted(r["seconds"] for r in runs), ["20.0", "20.0"])

    def test_the_report_reads_rows_written_before_the_result_columns(self):
        runs, row = self.read([at("00:00", [use("a", "cargo test")]), at("00:30", [result("a")])])
        old = {k: v for k, v in runs[0].items() if k not in ("result", "repeat_after")}
        text = test_time.report([old, *runs], [row])
        self.assertIn(
            "| test | tier 1 (cargo test) | 2 | 1 min 00 s | 30 s | 30 s | 1 | 0 | 1 |", text
        )
        self.assertIn("| | **All** | 0 |", text)

    def test_a_tool_that_is_no_shell_is_no_run(self):
        runs, _ = self.read(
            [at("00:00", [use("a", "cargo test", tool="Read")]), at("00:09", [result("a")])]
        )
        self.assertEqual(runs, [])

    def test_a_transcript_without_a_time_is_none(self):
        self.assertIsNone(self.read(["not json", json.dumps({"message": {}})]))


if __name__ == "__main__":
    unittest.main()
