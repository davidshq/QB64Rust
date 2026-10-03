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

- [ ] 2.1 Add `--suite corpus` and `--corpus-root` to `tools\legacy_tests\run_legacy_tests.py`: compile with
  `-q -m -x` in a fresh scratch folder per program under the results folder, run with no arguments, null stdin,
  `QB64PE_NOPROMPT=y`, 60 s timeout; compare `.output` / `.err` as the compile suite does; for `.norun`, compile
  only; delete the folder on pass. Update the docstring and `tools\legacy_tests\README.md`. Verify: on 3
  hand-written expected files (one pass, one wrong output, one `.err`) the runner reports PASS, FAIL `result`, PASS; the repo's `git status` is unchanged after the run.
- [ ] 2.2 Add `.normalize` support (D4) applied before comparison and recording. Verify: a sidecar for
  `234_timer_value` makes two runs compare equal.
- [ ] 2.3 Add `--record` (compile once, run twice, write only on agreement, warn on change, report
  non-deterministic). Verify:
  recording `--glob "0*.bas"` twice leaves `git status` unchanged after the second run.
- [ ] 2.4 Add `--cpp-opt` (adds `-f:OptimizeCppProgram=true`). Verify: on `v11_wrap_o2` with an expected file
  recorded in the default build, `--cpp-opt` fails with `result`.

## 3. Record and check (design D3, D7, D8)

- [ ] 3.1 Record the whole corpus with `..\QB64pe\qb64pe.exe` (`16f629784e`). Add `.normalize` sidecars for every
  program reported non-deterministic or found in 1.1 (at least 212, 234; check 217, 218, 237); add
  `known_failures.txt` entries for programs that cannot run headless, with the reason. Rerun `--record` until it
  reports no non-deterministic programs. Verify: a second `--record` run changes no file.
- [ ] 3.2 Privacy check (D7): `grep -rniE '[a-z]:[\\/]|users|<user name>|<host name>' tests/corpus` finds nothing
  in `.output`/`.err` files. Verify: the grep output is empty.
- [ ] 3.3 Check run: `--suite corpus` passes for every program not in `known_failures.txt`. Save its
  `results.json` as `baselines\qb64pe-16f629784e-win64-corpus.json`; add a line to `baselines\README.md`.
  Verify: the JSON's counts match the run summary.
- [ ] 3.4 `-O2` run (D6): `--suite corpus --cpp-opt`; list each failing program with the differing behaviour in
  `tests\corpus\README.md`. Note whether any `_INTEGER64` program is on the list (`study\16` §8). Verify:
  `v11_wrap_o2` is on the list.

## 4. Documentation

- [ ] 4.1 Write `tests\corpus\README.md`: purpose, layout, how to check, how to record, counts (`.output`, `.err`,
  KFAIL), the `-O2` list. Verify: the commands run as written.
- [ ] 4.2 Update `STATUS.md` (golden corpus done; next: `m2-workspace-and-slice`), `study\00` §10 (layer 2 status
  and counts), `CLAUDE.md` layout table (`tests\corpus\`), and `study\16` §8 if 3.4 answered the `_INTEGER64`
  question. Verify: `openspec validate m2-golden-corpus --strict` passes.
