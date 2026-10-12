# Status and next steps

Updated 2026-10-12 (session 47).

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
1 to 9a of the order of work are done. Step 9, built-in statements, was OpenSpec change `m2-builtin-statements`
(archived 2026-10-10); after it came the eighth review (`study\29`) and `m2-demand-report-and-runner-tests`
(archived 2026-10-10: `qb64rust --report unsupported`, and tests of the test runner). Step 9a, the screen-state
check, was OpenSpec change `m2-screen-state-check` (all 22 tasks done, archived 2026-10-10).
A program that draws can now be tested: `tools\screen\screen_state.cpp`, linked into a test program of either
compiler, writes the state of the screen at exit and gives the program its keys; the corpus runner records and
compares it (`<name>.screen`, `<name>.keys`); 29 programs are recorded in `tests\corpus\screen\`
(`tools\screen\README.md`).
The compiler parses every program the old compiler accepts with no false error, and compiles scalars, procedures,
error handling, every operator, `CONST`, control flow, `SELECT CASE`, static arrays and `TYPE` of the main module,
70 built-in functions, all 17 numeric types, fixed-length strings, sequential files, `DATA`/`READ`/`RESTORE`,
`SWAP`, the `MID$` statement, `RANDOMIZE`, console `INPUT`/`LINE INPUT`, `SHELL`, the default types (`DEFINT`
to `DEFSTR`, `_DEFINE`), `OPTION BASE`, the type before the names (`DIM AS LONG a, b(3)`), dynamic arrays of the
main module (`DIM` with a bound that is not constant, `$DYNAMIC`, `REDIM`, `REDIM _PRESERVE`, `ERASE`), `_BIT`
arrays, arrays used without `DIM`, arrays in procedures (`DIM`, `REDIM`, `STATIC a(n)`, `STATIC a()`, `SHARED
a()`, `SUB … STATIC`), array parameters and `COMMON`, `STRING` members of a `TYPE`, whole `TYPE` values (`q =
p`), parameters of a user type and member arrays (`v(0 TO 3) AS LONG` in a `TYPE`: elements through any chain,
`LBOUND`/`UBOUND`, `REDIM` and `ERASE` as a clear, a whole member array as an array argument)
(`crates\README.md`). There is
a thin language server (`qb64rust lsp`). A plain built-in now comes in as a row, a call-site program and a slice
line: the call-site check is kept and part of tier 2 (`DECISIONS.md` 2026-10-10).

## Open items

- **The GitHub CI run after the next push has not been seen.** It settles task 10.2 of `m2-numeric-types` and task
  5.1 of `m2-language-server` (both changes were archived with that task open), and it is the first run of the
  changed `tier2` job (QB64pe's sources at the reference clone's commit, `DECISIONS.md` 2026-10-09), which now
  also builds `qb64pe.exe` there for the call-site check (`mingw32-make OS=win BUILD_QB64=y`, never tried on a
  GitHub runner). Work is checked locally; the user does not treat CI as a concern for now.

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
10. **In progress:** (was 11) OpenSpec change `m2-defxxx-common-arrays`, 27 of 29 tasks done (`tasks.md` of the
    change says what each found). Done: the measurements (`verification\v24_*`, 371 programs; findings in
    `study\00` §5, the design and spec deltas corrected by them), group 2, the default types (`DEFINT` to
    `DEFSTR`, `_DEFINE`), group 3 (the array shape, `OPTION BASE`, `$DYNAMIC`/`$STATIC` as state, the type
    before the names) and group 4 (dynamic arrays in the main module: `Allocate`, `Clear` and `Release` in the
    typed tree, the IR and the emitter; `REDIM`, `REDIM _PRESERVE`, `ERASE`; `_BIT` arrays) and group 5 (arrays
    used without `DIM`: static in the main module, dynamic below `$DYNAMIC` and allocated as part of the
    statement that names them first, in a procedure never allocated, as QB64pe; what stays marked is in design
    D5, "As built") and group 6 (arrays in procedures: a local dynamic array per call for `DIM` and `REDIM`,
    released on every way out; `STATIC a(n)`, a dynamic array allocated when the program starts; `STATIC a()`;
    `SHARED a()`; `STATIC` after a procedure header, which keeps variables only down to the procedure's first
    `STATIC` statement, as QB64pe; design D6, "As built in group 6") and group 7 (array parameters: a whole
    array passed by reference, its number of dimensions found from every use and every call after the bodies
    are checked; `ERASE` of a parameter; `COMMON` and `COMMON SHARED`, with `COMMON a()` a listed dynamic array;
    design D6, "As built in group 7") and group 8 (the rest of `TYPE`: `STRING` members, made and released with
    each value of the type; `q = p` as `CopyWhole`, not guarded, its two places found again for every part of a
    type that holds a `STRING`, as QB64pe and listed in `SOMEDAY.md`; a parameter of a user type by reference;
    design D9, "As built in group 8") and group 9 (member arrays: `ArrayRef` in the typed tree and the IR, the
    elements inline in the value, `OPTION BASE` as it stands at the `TYPE` block; `LBOUND`, `UBOUND` and `LEN`
    never evaluate the indexes of the place, `ERASE` and `REDIM` clear and are not guarded, a whole member array
    is an array argument, all as QB64pe; **a store through a member with an index on the way evaluates a number
    before the indexes and a string after them**, measured in this group, which corrected the string stores of
    the groups before; a store with an index outside stores nothing, `DIVERGENCES.md` D-004; design D8, "As
    built in group 9"). Task 1.4's questions are answered (`DECISIONS.md` 2026-10-11: both behaviours as QB64pe;
    member arrays are inline and need no decision). **Next in the change: group 10, integration** (its full
    runs: the slice list, the full corpus and the full upstream suite were run after group 9 and are in the
    table below; the call-site check and the differential pass list were run on 2026-10-12 and are equal and
    passing; the upstream pass list by itself and the runner's own tests were not run again). Still marked in
    the member-array layer: a bound that names a `CONST`, a member array of an array element beside other
    targets of one `ERASE` or `REDIM`, an element of a member array as a target of `INPUT`, `READ` or `MID$`
    (`SOMEDAY.md`). Groups 1 to 8 are committed; group 9 is in the working tree.
11. (was 10) `$CONSOLE` with `_DEST _CONSOLE` and `$SCREENHIDE`, measured first (`study\26` §5); check once that
    the GitHub Windows runner can start the hidden window.

## Numbers at the last full runs

2026-10-10, release build against the reference clone: session 39 after `m2-builtin-statements` (its task 8.2);
the rows that name the screen programs, the runner's tests and the corpus of 336 are from session 40, after
`m2-screen-state-check`.

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | 287 of 287, also with `QB64RUST_NO_FOLD=1` (2026-10-11, after group 9 of `m2-defxxx-common-arrays`) |
| Full corpus (357 programs, run 2026-10-12 after group 9) | 306 pass (the 287 of the slice list, 19 `.err` programs), none wrong at run time; 45 rejected with "not supported yet" (16 and the 29 screen programs) and 1 `.err` program with only marked errors (`tests\corpus\README.md`) |
| Upstream | **114 of 279** (`tests\upstream\pass.list`: 114, all passing in the full run); none wrong in the full run of the suite (404 programs, 2026-10-11 after group 9: 114 pass, 251 rejected, 38 `.err` programs with only marked errors, 1 known failure) |
| Call-site check | 256 of 256 programs equal (`tools\callsite\README.md`); about 1 min |
| Real programs (`tests\programs`) | Civil War Strategy: the front end reads it with no real error; every diagnostic is "not supported yet" (what it needs next: `tests\programs\README.md`) |
| Differential | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 53 only-marked rejections (2026-10-11 after group 9; `tests\upstream\README.md`) |
| Tier 1 (`cargo test`) | about 20 s |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 56 pass, under a second |
| Screen programs (`tests\corpus\screen`, 29) | Old compiler: 29 pass, each recorded from two equal runs. New compiler: all 29 rejected with only "not supported yet" errors (by programs: no `$CONSOLE:ONLY` 29, `SCREEN` 23, `LINE` 20, `PSET` 17) |
| Demand (`qb64rust --report unsupported`, 2026-10-11 after group 9) | Corpus: 16 programs held back beside the 29 screen programs. Upstream (348 without an `.err`): 96 without errors, 242 held back, 10 with other errors (the ten that include the old compiler's sources); `_DYNAMIC`/`_STATIC` members 86 programs (deferred), `$Unstable` 85, an array used after a marked declaration 60, `_DEST` 54, `$CONSOLE` 34, `_MEM` 30, `REDIM _RETAIN` 27 (no "arrays as `TYPE` members" line is left). Civil War Strategy: `LINE` 899, `COLOR` 190, `LOCATE` 145, as before |
| Tier 2 as CI runs it, locally | about 12 min at the last full build; about 2 min with `--jobs 8 --build-cache` when little C++ changed |
| Full corpus with the old compiler (2026-10-10, 336 programs) | 331 pass, 5 known failures, `--jobs 8`; the 305 before the screen programs took about 12 min with one job, 3 min 44 s with `--jobs 8` (`tools\legacy_tests\README.md`) |

How to read them: the corpus has given what it can before M3 (its 16 rejected console programs wait on `PRINT
USING`/`TAB`/`SPC`, random and binary files and a few functions; the 29 screen programs on drawing). Upstream
went from 64 to 114 with member arrays (group 9 of step 10). Its next blockers outside `deferred.list` are
`_DEST` and `$CONSOLE` (step 11, `study\27` §4) and `_MEM`; the `_DYNAMIC`/`_STATIC` markers and `$Unstable` are
deferred (`SOMEDAY.md`).

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
