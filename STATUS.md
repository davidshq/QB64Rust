# Status and next steps

Updated 2026-10-12 (session 52).

**This file holds the current state only**: where we are, what is open, what comes next, the numbers of the last
full runs, known bugs. It holds no history. When a step is done, replace its line; do not add a paragraph about it.
Where everything else lives:

| Kind of information | Home |
|---|---|
| What a change did, task by task, and what was found on the way | the change's `tasks.md` and `design.md` under `openspec\changes\` (archived ones in `archive\`), and `git log` |
| A measured fact about the old compiler | `study\00` §5 (one home per fact) |
| A decision | `DECISIONS.md`; a decided difference from QB64pe: `DIVERGENCES.md` |
| What the compiler supports, how to build and test it | `crates\README.md`; per test set: `tests\*\README.md` |
| Milestones, done and not done | `ROADMAP.md` |
| History up to 2026-10-09 (what this file held before) | `study\progress-record.md` (closed) |

## Where we are

M0 (baseline) and M1 (VS Code extension on the old compiler) are complete. **M2 (front end) is in progress**: steps
1 to 10 of the order of work are done; step 11 is the last. Step 9, built-in statements, was OpenSpec change
`m2-builtin-statements` (archived 2026-10-10); after it came the eighth review (`study\29`) and `m2-demand-report-and-runner-tests`
(archived 2026-10-10: `qb64rust --report unsupported`, and tests of the test runner). Step 9a, the screen-state
check, was OpenSpec change `m2-screen-state-check` (all 22 tasks done, archived 2026-10-10).
A program that draws can now be tested: `tools\screen\screen_state.cpp`, linked into a test program of either
compiler, writes the state of the screen at exit and gives the program its keys; the corpus runner records and
compares it (`<name>.screen`, `<name>.keys`); 29 programs are recorded in `tests\corpus\screen\`
(`tools\screen\README.md`).
The compiler parses every program the old compiler accepts with no false error, and compiles scalars, procedures,
error handling, every operator, `CONST`, control flow, `SELECT CASE`, static arrays and `TYPE` of the main module,
83 built-in functions, all 17 numeric types, fixed-length strings, sequential files, `DATA`/`READ`/`RESTORE`,
`SWAP`, the `MID$` statement, `RANDOMIZE`, console `INPUT`/`LINE INPUT`, `SHELL`, the default types (`DEFINT`
to `DEFSTR`, `_DEFINE`), `OPTION BASE`, the type before the names (`DIM AS LONG a, b(3)`), dynamic arrays of the
main module (`DIM` with a bound that is not constant, `$DYNAMIC`, `REDIM`, `REDIM _PRESERVE`, `ERASE`), `_BIT`
arrays, arrays used without `DIM`, arrays in procedures (`DIM`, `REDIM`, `STATIC a(n)`, `STATIC a()`, `SHARED
a()`, `SUB … STATIC`), array parameters and `COMMON`, `STRING` members of a `TYPE`, whole `TYPE` values (`q =
p`), parameters of a user type and member arrays (`v(0 TO 3) AS LONG` in a `TYPE`: elements through any chain,
`LBOUND`/`UBOUND`, `REDIM` and `ERASE` as a clear, a whole member array as an array argument)
(step 10, OpenSpec change `m2-defxxx-common-arrays`, archived 2026-10-12; `crates\README.md`), and, with step
11: programs with `$CONSOLE` (a console and a window), `$SCREENHIDE`/`$SCREENSHOW`, `_DEST`, `_SOURCE`,
`_CONSOLE`, `_ECHO`, the 20 window built-ins of the upstream `glut` tests as code that is compiled and never
run, 231 of the 310 constants of QB64pe's auto-included file (`_TRUE`, `_FALSE`, ...) and the precompiler flags
in `$IF`. There is
a thin language server (`qb64rust lsp`). A plain built-in now comes in as a row, a call-site program and a slice
line: the call-site check is kept and part of tier 2 (`DECISIONS.md` 2026-10-10).

## Open items

- None from CI: `rust.yml` (`test`, `tier2`, `deny`), `vscode-extension.yml` and `repo-check.yml` are green on
  GitHub (seen 2026-10-12 at `3ba82c2`, and before it at `cda1268` and `cf4fb1f`). That settled task 10.2 of
  `m2-numeric-types` and task 5.1 of `m2-language-server`, and showed that the changed `tier2` job builds
  `qb64pe.exe` on the runner for the call-site check.

Every question of the seventh and eighth reviews is answered (`study\28` §9, `study\29` §8, `DECISIONS.md`
2026-10-09 and 2026-10-10): the local reference clone (`16f629784e`) is the one reference for the old compiler;
Civil War Strategy is in `tests\programs` as the real-program graphics target; the formatter is out of M2's exit
criterion; the screen-state check needs no copy of libqb.

## Next

The order was set by the fifth and sixth reviews (`study\26` §6, `study\27` §3) and changed by the eighth
(`study\29` §3: the last two steps swapped).

9. Done: built-in statements and functions by demand (`m2-builtin-statements`). Further plain built-ins come in
   as programs name them, each with a call-site program (`crates\README.md`); what was left marked and why is in
   task 7.4 of the change.
9a. Done: the screen-state check (`m2-screen-state-check`; `tools\screen\README.md`). The new
    compiler compiles none of the 29 screen programs yet: it builds console programs only, and the first drawing
    statement is M3 work. What a set of them needs: `qb64rust --report unsupported tests\corpus\screen`.
10. Done: (was 11) default types, `COMMON`, dynamic arrays, arrays in procedures, array parameters, the rest
    of `TYPE` and member arrays (`m2-defxxx-common-arrays`, all 29 tasks done, archived 2026-10-12).
    What each task found is in the change's `tasks.md`, how each group was built in its `design.md` ("As
    built"), the measured facts in `study\00` §5 (`verification\v24_*`, `v25_*`). Still marked in the
    member-array layer, each listed in `SOMEDAY.md`: a bound that names a `CONST`, a member array of an array
    element beside other targets of one `ERASE` or `REDIM`, an element of a member array as a target of
    `INPUT`, `READ` or `MID$`, `REDIM t.v(0) AS type` under `OPTION BASE 1`.
11. **Next:** (was 10) `$CONSOLE` with `_DEST _CONSOLE` and `$SCREENHIDE` (`study\26` §5). Measured 2026-10-12
    (`verification\v26_*`, 140 programs; the facts in `study\00` §5). OpenSpec change `m2-console-screenhide`
    (proposed 2026-10-12; 7 task groups; 21 of 24 tasks done: groups 1 to 6), with the wider scope the user
    decided (`DECISIONS.md` 2026-10-12). Built: a `$CONSOLE` program is compiled and built with a console and
    a window, the four metacommands are matched exactly, `_DEST`, `_SOURCE`, `_CONSOLE` and `_ECHO` are
    compiled (groups 2, 3); the window built-ins of the 19 upstream `glut` tests are compiled and never run
    (group 4); the constants of the auto-included file are known in every program (group 5); the precompiler
    flags in `$IF` have the old compiler's values (group 6). `slice.list` has 297 programs, all passing, and
    upstream is at 140 of 279. The GitHub Windows runner starts a program with a window: `tier2` is green at
    `ab418be` with `s63_console_window` passing, the job 18 min 22 s against 17 min 25 s before (task 2.6).
    Groups 3 to 6 are not pushed yet. **Open: group 7**: the two full suites with the new compiler (`--suite
    compile`, `--suite corpus`; the lists and tier 1 were run after group 6), the closing documents, and
    reading CI after the push (the first run there of `s66`, whose `_ICON` links a libqb variant CI has not
    built yet, so `tier2` will take a little longer once); then `/opsx:archive`. What stayed marked is in
    `SOMEDAY.md` ("Left marked by `m2-console-screenhide`").

## Numbers at the last full runs

2026-10-12, release build against the reference clone, after `m2-defxxx-common-arrays` (its task 10.1, session
48: every run of tier 1 and tier 2 and both full suites, with `--jobs 8 --build-cache`). The two rows of the old
compiler (the screen programs, the corpus of 336) are from session 40, 2026-10-10. Newer, from session 52
(2026-10-12, after group 6 of `m2-console-screenhide`): the slice list, the upstream pass list, the differential
pass list and the call-site check, each run in full, tier 1, and the demand row; the two full-suite rows (the
full corpus, the upstream suite of 404) are as they were and are task 7.1 of the change.

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | 297 of 297, also with `QB64RUST_NO_FOLD=1`; about 3 min 30 s each from an empty build cache, under a minute when the cache holds the unchanged programs |
| Full corpus (357 programs) | 306 pass (the 287 of the slice list, 19 `.err` programs), none wrong at run time; 45 rejected with "not supported yet" (16 and the 29 screen programs) and 1 `.err` program with only marked errors; 5 known failures (`tests\corpus\README.md`) |
| Upstream | **140 of 279** (`tests\upstream\pass.list`: 140, all passing, run by itself); at the last full run of the suite, with 114 on the list, none wrong (404 programs: 114 pass, 251 rejected, 38 `.err` programs with only marked errors, 1 known failure) |
| Call-site check | 362 of 362 programs equal (`tools\callsite\README.md`); about 1 min 20 s |
| Real programs (`tests\programs`) | Civil War Strategy: the front end reads it with no real error; every diagnostic is "not supported yet" (what it needs next: `tests\programs\README.md`) |
| Differential (`tests\differential\pass.list`) | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 53 only-marked rejections (`tests\upstream\README.md`) |
| Tier 1 (`cargo test`) | passes (290 tests), about 28 s (23 to 26 s before step 11's groups 4 to 6, which added 156 programs and 7 tests; the front end over the same inputs takes 2.3 s before and after); the 13 command-line tests that build an executable (`--ignored`) pass |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 56 pass, under a second |
| Screen programs (`tests\corpus\screen`, 29) | Old compiler: 29 pass, each recorded from two equal runs. New compiler: all 29 rejected with only "not supported yet" errors (by programs: no `$CONSOLE:ONLY` 29, `SCREEN` 23, `LINE` 20, `PSET` 17) |
| Demand (`qb64rust --report unsupported`; upstream with `--include-root tests\upstream\root`) | Corpus: 16 programs held back beside the 29 screen programs. Upstream (348 without an `.err`): 122 without errors, 216 held back, 10 with other errors (the ten that include the old compiler's sources); `_DYNAMIC`/`_STATIC` members 86 programs (deferred), `$Unstable` 85, an array used after a marked declaration 60, `_MEM` 30, `REDIM _RETAIN` 27, `_MEMFREE` 22 (no line names `_DEST`, `$CONSOLE`, a window built-in, `_TRUE` or a precompiler flag any more). Civil War Strategy: `LINE` 899, `COLOR` 190, `LOCATE` 145, as before |
| Tier 2 as CI runs it, locally | about 12 min at the last full build; about 2 min with `--jobs 8 --build-cache` when little C++ changed |
| Full corpus with the old compiler (2026-10-10, 336 programs) | 331 pass, 5 known failures, `--jobs 8`; the 305 before the screen programs took about 12 min with one job, 3 min 44 s with `--jobs 8` (`tools\legacy_tests\README.md`) |

How to read them: the corpus has given what it can before M3 (its 16 rejected console programs wait on `PRINT
USING`/`TAB`/`SPC`, random and binary files and a few functions; the 29 screen programs on drawing). Upstream
went from 50 to 114 over step 10 (64 before its member arrays) and is at 140 after group 6 of step 11 (115
after `_DEST` and `$CONSOLE`; all 19 `glut` tests with the window built-ins and `_TRUE`/`_FALSE`; four of the
nine `precomp-flags` tests). Its next blockers outside `deferred.list` are `_MEM`, `$ASSERTS`/`$DEBUG` and the
image built-ins that run; the
`_DYNAMIC`/`_STATIC` markers and `$Unstable` are deferred (`SOMEDAY.md`).

## Known bugs

None known. (The two of 2026-10-11, a `+` after another sign and a `DIM` of several dynamic arrays, are fixed:
`tasks.md` of `m2-defxxx-common-arrays`, above its group 7; slice programs `s17_signs` and `s52_dim_list`.)

## Queued for later milestones

| When | Item |
|---|---|
| M3 | Plain copy of `..\QB64pe\internal\c` at the reference clone's commit, `16f629784e` (`DECISIONS.md` 2026-10-02 and 2026-10-09) |
| M3 | Civil War Strategy builds and its screens match (`tests\programs\README.md`, stages 2 and 3) |
| Any time | A formatter that matches `-y` (out of M2's exit criterion, `DECISIONS.md` 2026-10-09) |
| M3 | Programs that draw (without `$CONSOLE:ONLY` and not writing to `_DEST _CONSOLE`): the check exists (`tools\screen`), with 29 recorded programs; the new compiler has to build a program with a window and compile the drawing statements, each coming in with screen programs that pass |
| M3 | Data files beside a corpus program, for the cut programs that load the game's `*.VGA` pictures with `BLOAD` (`SOMEDAY.md`) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
