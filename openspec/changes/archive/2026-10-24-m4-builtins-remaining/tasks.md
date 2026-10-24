# Tasks

Standing rules: CLAUDE.md. What this change does differently: the runner's copy of the release (design D8) is
built first, so that every recording with `qb64pe.exe` (the family `v58`, the libraries among them) and the
reading of the old compiler's C++ (`-z`) are batched in group 2, before the code. They may change the delta specs
and the design before anything is built.

## 1. Recording with a compiler root of its own

- [x] 1.1 The runner (design D8, `tools\legacy_tests\run_legacy_tests.py`): a verification program with a
  `<name>.root` sidecar is recorded with `qb64pe.exe` in `target\qb64pe-root\<folder>\` (a copy of the clone's
  `qb64pe.exe`, a junction `internal` to the clone's, a copy of the folder's `libraries\`, remade when a stamp of
  their hashes changes), under the clone's lock; another compiler compiles it with `--include-root` of the copied
  folder in the run's own folder. `tests\verification\README.md` and `tools\legacy_tests\README.md` say how.
  Verify: `test_run_legacy_tests.py` has a test of the sidecar for each compiler and of the stamp; a program
  `v58_u00` with a one-line library records its output; `git -C ..\QB64pe status` is clean and the clone has no
  `libraries\`.

## 2. Measurements and records

- [x] 2.1 The family `verification\v58_*` of design D1, with its support folder `v58_root\libraries\` (the test
  libraries, each descriptor and its files) and `.root` sidecars, recorded with `qb64pe.exe` (the verification
  suite), with one line in `verification\README.md`. Where a recording differs from a delta spec or the design, the
  spec or design is changed here. A machine-dependent value (`_DESKTOPREFRESHRATE`, `_UPTIME`, the lock keys) is
  printed as a comparison, and `FILES`'s free space is masked by a `.normalize` sidecar. Verify: every program has
  its recorded output or error, two recordings equal; `tools\repo_check\check_repo.py --untracked` finds nothing
  in them; `openspec validate m4-builtins-remaining --strict` passes after any edit.
- [x] 2.2 The old compiler's C++ (`qb64pe.exe -z`) of `$ERRORLOCATION:ON` over a program with every kind of
  statement and block, a joined line, a single-line `IF` and an included file; of `INSTR(, a$, b$)`; of `DATE$ =`;
  and of a program with each kind of library file. Design D5, D6 and D7 name the C++ lines they rest on and are
  corrected where they differ. Verify: the clone is unchanged (`git -C ..\QB64pe status`).
- [x] 2.3 `DECISIONS.md`: a row for the change (step 4b of the plan of 2026-10-22, split on 2026-10-23;
  `$USELIBRARY` in full, its success path measured in a copy of the release under `target\`, user, 2026-10-24;
  the built-ins never run). It names `openspec\changes\m4-builtins-remaining\design.md`. Verify: the row is there
  and names the design.

## 3. The built-ins

- [x] 3.1 The functions (design D2): the rows of `SUPPORTED` for the math, bit, MBF and system functions, `LPOS`,
  `INP`, the lock keys, the console, display and dialog functions, `_BLINK` and `_DISPLAY`, each with the rule `v58`
  shows. Verify: the `v58` function programs pass (`--suite verification --glob`); one `typed` front-end test
  `tests\frontend\builtins_more_typed.bas` calls each; a slice program `tests\corpus\slice\s1xx_builtins_last`
  (the next free number) calls each that runs, recorded from the old compiler's build, on `slice.list`; a program
  per form in `tests\callsite\`, the call-site check equal; the seven math functions agree with the `CONST` of the
  same literal (D2).
- [x] 3.2 The statements (design D2, D4): rows of `STATEMENTS` for `OUT`, `WAIT`, `FILES`, `_BLINK`, the lock keys,
  the console and display statements, `_NOTIFYPOPUP`, `_SCREENCLICK`, `_SCREENPRINT`; `DATE$ =` and `TIME$ =`
  through the assignment checker (`check\io.rs`). Verify: the `v58` statement programs pass; an `ir` test
  `tests\frontend\statements_more_ir.bas`; the slice program of 3.1 runs those that run; a screen program
  `tests\corpus\screen\` for `_BLINK OFF` with its state recorded from the old compiler's build, on `slice.list`; a
  call-site program per form, equal.
- [x] 3.3 `NEVER_RUN` (design D3) takes the nine names with their reasons. Verify: the coverage check of tier 1
  passes (each compiled and not reached in a slice program, and in a call-site program).
- [x] 3.4 The rejected forms `v58` records (too many or too few arguments, a string for a number, the statements'
  forms) are the old compiler's errors; the wording table by `python tools\diagnostics\old_wording.py`. Verify:
  those `v58` programs are errors with no mark, each with the old compiler's message as its note (`qb64rust -z`);
  `old_wording.py --check` passes; `tests\frontend\builtins_more_errors.bas` (`check-fail`).
- [x] 3.5 `INSTR` with its first argument left out (design D5). Verify: `v57_o07` and the `v58` `INSTR` programs
  pass; the scenario "The first argument of INSTR" has a test; a call-site program, equal.
- [x] 3.6 A built-in with its result's suffix (`_UFONTHEIGHT&`), as `v58` records it: accepted, then compiled as
  the name without the suffix; rejected, then the old compiler's error. Verify: the `v58` neighbour programs pass.

## 4. `$ERRORLOCATION`

- [x] 4.1 `sema` (`check\mod.rs` `meta`): the setting recorded, a second that differs is the old compiler's error,
  the forms it does not know are its errors; `ir::Program` holds the setting; codegen (`codegen-cpp\src\stmt.rs`,
  `procs.rs`) writes `error_track_line(…)` where 2.2 found it (design D6). Verify: the `v58` `$ERRORLOCATION`
  programs pass; `tests\frontend\errorlocation_cpp.bas` (`cpp`) and `errorlocation_errors.bas` (`check-fail`); a
  call-site program, equal; the slice list with `--pass-cache` (a program without the setting is unchanged).

## 5. `$USELIBRARY`

- [x] 5.1 Reading (design D7): `syntax\src\library.rs` reads a descriptor's three keys by `ReadSetting$`'s rules;
  `Loader::library(name)` and `FileLoader` find it under the compiler root; the line's errors. Verify: unit tests
  of the reader (sections, quotes, comments, a missing key); the `v58` error programs are errors with the old
  compiler's notes.
- [x] 5.2 Including (design D7): the library list across rounds, the top, after-main and bottom trees in the old
  compiler's order, the after-main trees after the main module's implicit `END` in `sema`'s stream
  (`check\walk.rs`). Verify: the `v58` library programs pass with their `.root` sidecars; a front-end test with a
  compiler root of its own (`tests\frontend\inc\root\libraries\`) for each kind of file; the language server
  reports a library's errors at the `$USELIBRARY` line (`crates\lsp` test).

## 6. The examples and the table test

- [x] 6.1 The examples of "not supported yet" (design D9): `tests\frontend\unsupported.bas`, the parser test of
  `DO: FILES: LOOP`, `unsupported.check-fail.snap` and the metacommand test of `$ERRORLOCATION:ON` take forms left
  marked. Verify: `cargo test`; each scenario of the delta specs that names an example has its test.
- [x] 6.2 `crates\sema\tests\every_builtin.rs` (design D10). Verify: it passes, and fails when a row of 3.1 is taken
  out (tried once, put back).

## 7. Lists and closing

- [x] 7.1 `SOMEDAY.md`: rows that name a form of this change as "not supported yet" are removed or narrowed; rows
  of "QB64pe behaviours to review" for what is kept QB64pe's way (`DATE$ =` changing nothing, a library named twice,
  anything else `v58` shows). Verify: no row says a form of this change is "not supported yet" unless it still is.
- [x] 7.2 `tests\verification\known.list` and `pass.list`: `v57_o07` and every `v58` program that passes move to
  `pass.list`; the rest keep a reason. Verify: the tier 1 tests of the lists; the demand report over `known.list`'s
  "not supported yet" programs names only the 9 that steps 2 and 3 left; the probe of `proposal.md`, rerun, marks
  nothing.
- [x] 7.3 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes. Then the counts of the test READMEs by `tools\legacy_tests\counts.py`, and
  `STATUS.md`'s "Next" names step 5 of the plan (`SUB _GL`). Then `python tools\testtime\test_time.py`. Verify:
  `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
