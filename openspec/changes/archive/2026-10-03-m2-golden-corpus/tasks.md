# Tasks

## 1. Copy the programs (`specs/testing/golden-corpus`, design D1)

- [x] 1.1 Copy the 261 `.bas` files from `<qb64contain>\QB64Fresh\tests\runtime_comparison\` (commit `85e6df8`)
  to `tests\corpus\runtime_comparison\`, unchanged. Read each in full (rule 5) and note in a scratch list any that
  print machine-dependent values, need a device (`CHAIN`, `LPRINT`) or touch files outside their working folder.
  Verify: `sha256sum` of every copy equals the original's; count is 261. (Done 2026-10-03: hashes equal, 261
  files; findings in `design.md` "What the programs touch"; `LPRINT` would print on the default printer, hence
  `.norun`.)
- [x] 1.2 Write `tests\corpus\runtime_comparison\SOURCE.md` (origin, commit, MIT, rule 5, not copied:
  `README.md`, `run_comparison.sh`, `diff_results.sh`, `DIFFERENCES.md`, QB64Fresh's results). Verify: no local
  paths in it (`<qb64contain>` placeholder only).
- [x] 1.3 Copy `verification\v11_wrap_o2.bas` to `tests\corpus\verification\` with a `SOURCE.md`. Verify: bytes
  equal.
- [x] 1.4 Add `tests/corpus/**/*.output -text` and `tests/corpus/**/*.err -text` to `.gitattributes` (D5).
  Verify: `git check-attr text` on a sample path reports `unset`.
- [x] 1.5 Write `239_lprint.norun` (reason: sends a page to the default printer). Verify: the runner (2.1) never
  starts its executable.

## 2. Runner (design D2–D4, D6)

- [x] 2.1 Add `--suite corpus` and `--corpus-root` to `tools\legacy_tests\run_legacy_tests.py`: compile with
  `-q -m -x` in a fresh scratch folder per program under the results folder, run with no arguments, null stdin,
  `QB64PE_NOPROMPT=y`, 60 s timeout; compare `.output` / `.err` as the compile suite does; for `.norun`, compile
  only; delete the folder on pass. Update the docstring and `tools\legacy_tests\README.md`. Verify: on 3
  hand-written expected files (one pass, one wrong output, one `.err`) the runner reports PASS, FAIL `result`, PASS; the repo's `git status` is unchanged after the run.
  (Done 2026-10-03: PASS, FAIL `result`, PASS; `git status` unchanged; the passing program's folder, holding the
  file it wrote, was deleted. Found: on Windows `END` waits for a console key event and the null device never
  sends one, so every corpus program hung until the timeout. Fixed with `tools\legacy_tests\press_any_key.py`
  (a console of its own with no window, Shift presses); design "What the programs touch" and D2, and the spec,
  updated. Also checked: a `DO: LOOP` program fails at `run: timeout` after 60 s with no process left; a `.norun`
  program compiles and never runs.)
- [x] 2.2 Add `.normalize` support (D4) applied before comparison and recording. Verify: a sidecar for
  `234_timer_value` makes two runs compare equal.
  (Done 2026-10-03, on a scratch copy: without the sidecar `--record` reports it non-deterministic and writes
  nothing (`57226.92` vs `57227.14`); with it both runs give `timer: <TIMER>`, the file is written with CRLF
  kept, and a check run passes. The real sidecar is added in 3.1.)
- [x] 2.3 Add `--record` (compile once, run twice, write only on agreement, warn on change, report
  non-deterministic). Verify:
  recording `--glob "0*.bas"` twice leaves `git status` unchanged after the second run.
  (Done 2026-10-03: 9 `.output` files written by the first run; after the second, `git status` and the files'
  SHA-256 are unchanged.)
- [x] 2.4 Add `--cpp-opt` (adds `-f:OptimizeCppProgram=true`). Verify: on `v11_wrap_o2` with an expected file
  recorded in the default build, `--cpp-opt` fails with `result`.
  (Done 2026-10-03: default build `x + 1 > x : false` / `-2147483648`; `-O2` build `true` / ` 2147483648`, FAIL
  `result`; the default build still passes. `--record` with `--cpp-opt` is refused.)

## 3. Record and check (design D3, D7, D8)

- [x] 3.1 Record the whole corpus with `..\QB64pe\qb64pe.exe` (`16f629784e`). Add `.normalize` sidecars for every
  program reported non-deterministic or found in 1.1 (at least 212, 234; check 217, 218, 237); add
  `known_failures.txt` entries for programs that cannot run headless, with the reason. Rerun `--record` until it
  reports no non-deterministic programs. Verify: a second `--record` run changes no file.
  (Done 2026-10-03: 236 `.output`, 20 `.err`, 1 `.norun`; no program non-deterministic. Five time out and are in
  `known_failures.txt`: 80, 126, 187, 251 (`PRINT` with a comma prints spaces forever when the console's stdout
  is redirected; corrects design "What the programs touch") and 147 (`RESUME 0` re-runs `ERROR 5` forever, by
  design). Sidecars 212, 234, and 238 (`CHAIN` names the program's folder, found by 3.2). 217 (`len: 0`), 237
  (cursor never moves: `csrlin: 1 pos: 1`), 215 and 189 are stable; 218 does not compile (`FRE` not
  implemented). Second `--record`: 257 PASS, 5 KFAIL, nothing written, all 256 SHA-256 unchanged.)
- [x] 3.2 Privacy check (D7): `grep -rniE '[a-z]:[\\/]|users|<user name>|<host name>' tests/corpus` finds nothing
  in `.output`/`.err` files. Verify: the grep output is empty.
  (Done 2026-10-03: first found the scratch folder path in `238_chain.output`; masked as `<DIR>` by
  `238_chain.normalize` and re-recorded; now empty, also for the host name.)
- [x] 3.3 Check run: `--suite corpus` passes for every program not in `known_failures.txt`. Save its
  `results.json` as `baselines\qb64pe-16f629784e-win64-corpus.json`; add a line to `baselines\README.md`.
  Verify: the JSON's counts match the run summary.
  (Done 2026-10-03: 257 PASS, 0 FAIL, 5 KFAIL; JSON counts 236 output, 20 error, 1 compile-only, 5 KFAIL
  match the summary.)
- [x] 3.4 `-O2` run (D6): `--suite corpus --cpp-opt`; list each failing program with the differing behaviour in
  `tests\corpus\README.md`. Note whether any `_INTEGER64` program is on the list (`study\16` §8). Verify:
  `v11_wrap_o2` is on the list.
  (Done 2026-10-03: 256 PASS, 1 FAIL (`v11_wrap_o2`, `result`), 5 KFAIL. The corpus has no `_INTEGER64` program
  and no LONG overflow, so it could not answer the question; `verification\v12_wrap_int64` was written and added
  as `tests\corpus\verification\v12_wrap_int64`: `x + 1 > x` false in the default build, true with `-O2`, so
  `_INTEGER64` has the same exposure (`study\16` §8). It is the second program on the list. Also: before this
  run the runner was changed to compile but not run known failures without an expected file (60 s → 2 s each).
  One `-O2` compile of `v12` failed once with qb64pe's "UNEXPECTED INTERNAL COMPILER ERROR" at line 0; three
  reruns compiled it, not reproduced.)

## 4. Documentation

- [x] 4.1 Write `tests\corpus\README.md`: purpose, layout, how to check, how to record, counts (`.output`, `.err`,
  KFAIL), the `-O2` list. Verify: the commands run as written.
  (Done 2026-10-03: full check 258 PASS, 5 KFAIL; `--glob "1*.bas"` 107 PASS, 3 KFAIL; `--record` ran in 3.1;
  the privacy grep prints nothing.)
- [x] 4.2 Update `STATUS.md` (golden corpus done; next: `m2-workspace-and-slice`), `study\00` §10 (layer 2 status
  and counts), `CLAUDE.md` layout table (`tests\corpus\`), and `study\16` §8 if 3.4 answered the `_INTEGER64`
  question. Verify: `openspec validate m2-golden-corpus --strict` passes.
  (Done 2026-10-03: all four updated, plus `study\19` (test cadence) and the `CLAUDE.md` decision for it; the
  `_INTEGER64` measurement is in `study\16` §8 and the user decided the same day that it wraps too (`CLAUDE.md`); validation passes.)
