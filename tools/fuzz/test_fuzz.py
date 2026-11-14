#!/usr/bin/env python3
"""Tests of tools/fuzz/fuzz.py: the commands built for each target, the summary read from libFuzzer's output, the
seeds and the failing inputs found.

    python -m unittest discover -s tools/fuzz -p "test_*.py"
"""

from __future__ import annotations

import os
import sys
import tempfile
import time
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import fuzz

# A run with one job (cut from a real run, 2026-11-12).
ONE_JOB = """\
#517	INITED cov: 22364 ft: 88059 corp: 490/516Kb exec/s: 129 rss: 274Mb
#7234	DONE   cov: 22999 ft: 90000 corp: 600/600Kb lim: 11781 exec/s: 60 rss: 300Mb
Done 7234 runs in 121 second(s)
stat::number_of_executed_units: 7234
stat::average_exec_per_sec:     59
"""

# A run in fork mode (cut from a real run, 2026-11-12): no final stats.
FORK = """\
INFO: -fork=2: 442 seed inputs, starting to fuzz in /tmp/libFuzzerTemp.FuzzWithFork1334.dir
#637: cov: 2527 ft: 10277 corp: 442 exec/s: 318 oom/timeout/crash: 0/0/0 time: 3s job: 1 dft_time: 0
#9181: cov: 3323 ft: 13616 corp: 1132 exec/s: 257 oom/timeout/crash: 0/0/0 time: 24s job: 7 dft_time: 0
INFO: fuzzed for 24 seconds, wrapping up soon
INFO: exiting: 0 time: 24s
"""


class Commands(unittest.TestCase):
    def test_flags_hold_the_spec_limits(self):
        flags = fuzz.libfuzzer_flags(600, 1)
        self.assertIn("-max_total_time=600", flags)
        self.assertIn("-max_len=16384", flags)
        self.assertIn("-timeout=10", flags)
        self.assertIn("-rss_limit_mb=2048", flags)
        self.assertIn("-dict=fuzz/target/keywords.dict", flags)
        self.assertFalse(any(f.startswith("-fork") for f in flags))

    def test_more_than_one_job_forks(self):
        self.assertIn("-fork=4", fuzz.libfuzzer_flags(60, 4))
        self.assertFalse(any(f.startswith("-jobs") for f in fuzz.libfuzzer_flags(60, 4)))

    def test_fork_mode_stops_at_any_first_failure(self):
        flags = fuzz.libfuzzer_flags(60, 2)
        for flag in ("-ignore_crashes=0", "-ignore_timeouts=0", "-ignore_ooms=0"):
            self.assertIn(flag, flags)
        self.assertFalse(any(f.startswith("-ignore") for f in fuzz.libfuzzer_flags(60, 1)))

    @unittest.skipIf(os.name == "nt", "process groups are Linux's")
    def test_run_bounded_stops_the_command_and_its_children(self):
        t0 = time.time()
        # A shell whose child sleeps: both must go.
        self.assertIsNone(fuzz.run_bounded(["sh", "-c", "sleep 30; true"], 0.5))
        self.assertLess(time.time() - t0, 10)
        self.assertEqual(fuzz.run_bounded(["sh", "-c", "exit 3"], 10), 3)

    def test_run_command_for_each_target(self):
        for target in fuzz.TARGETS:
            cmd = fuzz.run_command("nightly-x", target, 60, 2, "none")
            self.assertEqual(cmd[:9], [
                "cargo", "+nightly-x", "fuzz", "run", "--sanitizer", "none", target,
                f"fuzz/corpus/{target}", "fuzz/target/seeds",
            ])  # fmt: skip
            self.assertEqual(cmd[9], "--")
            self.assertEqual(cmd[10:], fuzz.libfuzzer_flags(60, 2))

    def test_tmin_and_cmin(self):
        tmin = fuzz.tmin_command("n", "parse", "fuzz/artifacts/parse/crash-1", "none")
        self.assertEqual(
            tmin[2:8],
            ["fuzz", "tmin", "--sanitizer", "none", "parse", "fuzz/artifacts/parse/crash-1"],
        )
        self.assertIn("-timeout=10", tmin)
        cmin = fuzz.cmin_command("n", "server", "none")
        self.assertEqual(
            cmin[2:8], ["fuzz", "cmin", "--sanitizer", "none", "server", "fuzz/corpus/server"]
        )

    def test_nightly_is_read_from_the_fuzz_toolchain_file(self):
        self.assertTrue(fuzz.nightly().startswith("nightly-"))

    def test_wsl_command_runs_this_script_in_the_copy(self):
        cmd = fuzz.wsl_command("C:/tree", ["parse", "1", "--jobs", "2"])
        self.assertEqual(cmd[:6], ["wsl", "--cd", "C:/tree", "-e", "bash", "-lc"])
        self.assertIn('cd "$(linux_copy "$tree")"', cmd[6])
        self.assertIn('python3 tools/fuzz/fuzz.py parse 1 --jobs 2 --copy-to "$tree"', cmd[6])


class Summary(unittest.TestCase):
    def test_inputs_from_final_stats(self):
        self.assertEqual(fuzz.inputs_tried(ONE_JOB), 7234)

    def test_inputs_from_fork_status_lines(self):
        self.assertEqual(fuzz.inputs_tried(FORK), 9181)

    def test_inputs_unknown(self):
        self.assertIsNone(fuzz.inputs_tried("error: could not compile\n"))

    def test_clean_line(self):
        self.assertEqual(
            fuzz.summary_line("parse", 600.0, 7234, []),
            "fuzz parse: 10.0 min, 7234 inputs, no failure",
        )

    def test_line_with_a_finding(self):
        line = fuzz.summary_line(
            "compile", 90.0, None, ["fuzz/artifacts/compile/minimized-from-ab"]
        )
        self.assertEqual(
            line,
            "fuzz compile: 1.5 min, an unknown number of inputs, 1 failing: fuzz/artifacts/compile/minimized-from-ab",
        )


class Files(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)

    def tearDown(self):
        self.tmp.cleanup()

    def write(self, rel: str, size: int) -> Path:
        p = self.root / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_bytes(b"x" * size)
        return p

    def test_seeds_are_small_corpus_programs_and_regressions(self):
        self.write("tests/corpus/slice/a.bas", 100)
        self.write("tests/corpus/screen/B.BAS", fuzz.MAX_LEN)
        self.write("tests/corpus/screen/big.bas", fuzz.MAX_LEN + 1)
        self.write("tests/corpus/slice/a.output", 10)
        self.write("tests/fuzz/regressions/parse-x.bas", 5)
        self.write("tests/fuzz/regressions/README.md", 5)
        names = [p.relative_to(self.root).as_posix() for p in fuzz.seed_files(self.root)]
        self.assertEqual(
            names,
            [
                "tests/corpus/screen/B.BAS",
                "tests/corpus/slice/a.bas",
                "tests/fuzz/regressions/parse-x.bas",
            ],
        )
        self.assertEqual(fuzz.fill_seeds(self.root), 3)
        copied = sorted(p.name for p in (self.root / fuzz.SEEDS).iterdir())
        self.assertIn("tests__corpus__slice__a.bas", copied)

    def test_new_artifacts_leave_out_old_ones_and_tmin_output(self):
        old = self.write("fuzz/artifacts/parse/crash-old", 3)
        past = time.time() - 100
        os.utime(old, (past, past))
        since = time.time() - 10
        new = self.write("fuzz/artifacts/parse/crash-new", 3)
        self.write("fuzz/artifacts/parse/minimized-from-new", 2)
        self.assertEqual(fuzz.new_artifacts(self.root, "parse", since), [new])
        self.assertEqual(fuzz.new_artifacts(self.root, "server", since), [])

    def test_smallest_is_tmin_output_or_the_artifact(self):
        since = time.time() - 10
        crash = self.write("fuzz/artifacts/parse/crash-a", 30)
        self.assertEqual(fuzz.smallest(self.root, "parse", crash, since), crash)
        self.write("fuzz/artifacts/parse/minimized-from-a1", 20)
        small = self.write("fuzz/artifacts/parse/minimized-from-a2", 5)
        self.assertEqual(fuzz.smallest(self.root, "parse", crash, since), small)


class Arguments(unittest.TestCase):
    def test_target_and_minutes_are_needed(self):
        with self.assertRaises(SystemExit):
            fuzz.main(["parse"])

    def test_unknown_target(self):
        with self.assertRaises(SystemExit):
            fuzz.main(["lexer", "1"])


if __name__ == "__main__":
    unittest.main()
