# 00 — Synthesis: archived parts

Parts of `study\00-synthesis.md` moved here unchanged on 2026-10-18 because a current document now holds what
they held, and their figures had gone out of date: §1 (now `DECISIONS.md`), the milestone table of §2 (now
`ROADMAP.md`), §10 (now `crates\README.md`, the `tests\*\README.md` files and `CLAUDE.md` working rule 9) and
the first row of §13's table (closed by the M3 changes). Read only when needed (`CLAUDE.md` working rule 10).

## 1. Decisions

All taken 2026-10-02; the authoritative list is in `DECISIONS.md`.

| Decision | Consequence |
|---|---|
| Ground-up rewrite of QB64pe | Understand the old code first (done: `01`–`05`, checked in `09`–`10`) |
| Do not port the text-mode IDE; reach parity with a VS Code extension | Compiler must be a library with an analysis API, a separate formatter, column-accurate diagnostics, debug symbols (`06`) |
| New compiler in **Rust** | Self-hosting is not a goal; compiling the old `qb64pe.bas` remains a large test case |
| **New error messages** (columns, several errors per run) | The 56 `.err` tests compare old texts; use them loosely (must fail, right line). Old texts optional during a transition |
| No strict QuickBASIC 4.5 mode | What one would need is recorded in `08` |
| Target dialect = upstream QB64pe 4.7.0 | The "fork" features turned out to be upstream work. Keep `$USELIBRARY`, `$ERRORLOCATION`, GLFW; defer TYPE member arrays, `_ARRAYCOPY`, whole-array assignment, `REDIM _RETAIN` (`SOMEDAY.md`) |

## 2. Plan: architecture and roadmap (the milestone table)

| # | Milestone | Done when | State |
|---|---|---|---|
| M0 | Baseline | Old compiler builds on Windows; Windows runner; baseline recorded; dialect settled; study gaps closed | **done** |
| M1 | VS Code extension v0 on the old compiler | Highlighting, build/run, diagnostics from `-z`, formatting via `-y`, CP437 default | **done** (`vscode\`) |
| M2 | Front end | Lossless parser with recovery over the whole language (no false syntax error on any program the old compiler accepts: corpus, upstream tests, `qbasic_testcases`, its own sources), resolution, type checker, a thin language server (the formatter matching `-y` left this criterion on 2026-10-09, `DECISIONS.md`) | **done 2026-10-12**: golden corpus; Rust workspace and end-to-end slice; procedures and error handling; upstream tests; control flow and constants; arrays and `TYPE`; parser breadth (every accepted program parses, no false error); core built-ins; the language server; the numeric types; built-in statements; the screen-state check; default types, dynamic arrays, arrays in procedures, `COMMON`, the rest of `TYPE` and member arrays; a console and a window (`$CONSOLE`, `$SCREENHIDE`, `_DEST _CONSOLE`), the window built-ins as compiled code, the auto-included constants and the precompiler flags (`crates\`). No step of the order of work is left. Order of work and the numbers of the last full runs: `STATUS.md` |
| M3 | Code generation to the existing ABI | Emits `qbx.cpp` fragments, links with libqb, passes expected-output and differential tests; programs without `$CONSOLE:ONLY` with a screen-state oracle | under way: the slice links with libqb since M2; since `m3-window-output` (2026-10-12) a program whose output is a window compiles, with the text-screen, drawing and pacing statements, and 42 screen programs pass by their screen state |
| M4 | Parity | The corpus (275) and the upstream tests in reach (279 of 404; 125 need the deferred array features) match; `qb64pe.bas` compiles and the result passes the suite (exit criterion, `study\archive\20`) | |
| M5 | Debugger | Debug symbol file + DAP adapter | |
| M6 | Runtime modernisation | ABI owned by the new compiler; error model, string heap, threading redesigned behind golden tests | |

## 6. Bug-compatibility choices (the "Fixed" list)

Replaced in the active file by a pointer to `DIVERGENCES.md`, whose rows D-003 and D-005 to D-014 hold the same
fixes. The paragraph went on with the programs rejected by both compilers, which stays in the active file.

**Fixed (no compatibility value):** `RETURN label` with no `GOSUB` pending breaking the `GOSUB` stack (2026-10-06,
D-003); `a IMP b IMP c` computing `a OR b OR c` (D-005); the smallest LONG/`_INTEGER64` `\ -1` and `MOD -1`
crashing the program (D-006); `CONST 1 / 0` giving 0 and `CONST 2 ^ 70` wrapping while `1E+19 / 1` does not
(D-007); `label: CONST …` on one line failing to compile (D-008); `_BIT * n` with n > 32 overwriting 4 bytes
of the `_BIT` scalar allocated before it (D-009; the victim measured by `verification\v21_b_bit_overlap`); `INF` printed with padding and a stray `D` (D-010); the console `tab()` hang and the
`CONOUT$` handle leak (D-011); runtime errors exiting 0 (D-012); `_LogMinLevel` and `_ScreenExists` registered
without a return type (D-013); two calls of a FUNCTION `f$n` in one expression reading memory the FUNCTION had
released (2026-10-09, D-014).

## 10. Testing position

- **Runner:** `tools\legacy_tests\run_legacy_tests.py` re-implements the bash suites on Windows (compile,
  qbasic, format) for the old or the new compiler; `known_failures.txt` marks expected failures.
- **Baseline** (`baselines\`, old compiler at `16f629784e`): compile_tests 330 of 331 expected-output, 56 of 56
  expected-error, 17 of 17 compile-only; qbasic_testcases 143 of 143 (compile only); format_tests 24 of 24. The one
  failure (`http/read_example`) depends on a web page that changed.
- **Reusable as black-box conformance:** the 331 expected-output tests (357 of 404 programs are `$CONSOLE:ONLY`)
  and the 143 qbasic programs. Tied to the old implementation: the `.err` texts, `qb64pe/*` internals tests,
  formatter tests, licence tests.
- **Planned layers (R11):** (1) existing suite; (2) golden corpus: run the 143 qbasic programs and QB64Fresh's 261
  `runtime_comparison` programs through the old compiler and freeze their output (**the 261 done 2026-10-03**:
  `tests\corpus\`, 263 programs with `verification\` v11 and v12; 237 `.output`, 20 `.err`, 1 compile-only, 5
  known failures; with the 12 `slice\` programs written for the new compiler, 275; baseline `baselines\qb64pe-16f629784e-win64-corpus.json`; how often it runs: `study\archive\19`. The 143
  qbasic programs are still compile-only); (3) differential testing of
  random expressions old vs. new; (4) formatter goldens from `-y` over all available `.bas`; plus golden images for
  LINE/CIRCLE/PAINT/DRAW/GET/PUT (only 12 image tests exist).
- **Not covered by anything yet:** the IDE, real graphics output, audio, interactive input, run-time behaviour of
  the 143 qbasic programs, and classic QBasic areas (PRINT USING, file modes, string functions) beyond light use.
- **Test hygiene:** run every program with `QB64PE_NOPROMPT=y`; detect fatal errors from output, not exit code;
  never use screen `PRINT` with a comma under a redirected `$CONSOLE`.
- `verification\` holds 77 small programs with recorded outputs behind `09`, `10`, `16`, the slice's design
  (`v13`, `v13b`: type suffixes on DIMmed names) and `m2-procedures-and-errors` (`v14_*`: scopes, reserved names,
  compile errors, `ERROR` values; `v15_*`: names of built-ins, unary `+`) (`run.sh` reruns them).
- **New compiler (2026-10-03, `crates\README.md`):** tier 1 `cargo test` runs unit and snapshot tests (`insta`),
  `tests\frontend\` by mode line (`' TEST: parse-ok|check-ok|check-fail|typed|ir|cpp`), and the front end over every
  corpus program (no panic, exact byte round trip; no diagnostics for `tests\corpus\slice.list`; at least one error
  for every `.err` program). Tier 2 runs the programs of `slice.list` end to end with the corpus runner's
  `--list` (70 on 2026-10-06): all pass, also with constant folding off. The full corpus with `qb64rust`: those
  pass, and so do the `.err` programs that get a real error; the 5 known failures are not run; everything else is
  rejected with a diagnostic (`tests\corpus\README.md`).
  Intentional differences from the old compiler are in `DIVERGENCES.md` (D-001, D-002: integer overflow wraps);
  the numeric rules are the spec `openspec\specs\language\numeric-semantics`, procedures and error handling the
  specs `language\procedures` and `language\error-handling`.
- **The call-site check (kept 2026-10-10, `DECISIONS.md`; `tools\callsite\README.md`):** for one statement per
  program of `tests\callsite` (256 on 2026-10-10), the libqb call the new compiler writes is compared with the
  old compiler's (`-z` on both, six normalisation rules, none naming a built-in). Part of tier 2, about a minute.
  It is how a plain built-in comes in: a row in the list, a program here, a line in a slice program. It does not
  see behaviour, element addressing, stores or the cleanup of string temporaries; the corpus does.

## 13. Remaining study gaps (the closed row)

| Gap | Milestone | Where |
|---|---|---|
| ON TIMER/KEY/STRIG, FIELD, `_MEM*` statements (skimmed; FOR/DO/WHILE closed by `verification\v17_*` and the `FOR` temporaries at 6510–6522, 2026-10-06) | M2/M3 | `qb64pe.bas` 6212–8765 |
