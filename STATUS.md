# Status and next steps

Updated 2026-10-10 (session 39).

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
1 to 9 of the order of work are done, the last being OpenSpec change `m2-builtin-statements` (all 41 tasks done,
archived 2026-10-10). After it, the eighth review (`study\29`) and the small change it asked for before step 9a,
`m2-demand-report-and-runner-tests` (archived 2026-10-10): `qb64rust --report unsupported` says what a set of
programs still needs, and the test runner has tests of its own.
The compiler parses every program the old compiler accepts with no false error, and compiles scalars, procedures,
error handling, every operator, `CONST`, control flow, `SELECT CASE`, static arrays and `TYPE` of the main module,
70 built-in functions, all 17 numeric types, fixed-length strings, sequential files, `DATA`/`READ`/`RESTORE`,
`SWAP`, the `MID$` statement, `RANDOMIZE`, console `INPUT`/`LINE INPUT` and `SHELL` (`crates\README.md`). There is
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
criterion; the screen-state check comes next and needs no copy of libqb.

## Next

The order was set by the fifth and sixth reviews (`study\26` §6, `study\27` §3) and changed by the eighth
(`study\29` §3: the last two steps swapped).

9. Done: built-in statements and functions by demand (`m2-builtin-statements`). Further plain built-ins come in
   as programs name them, each with a call-site program (`crates\README.md`); what was left marked and why is in
   task 7.4 of the change.
9a. **Next: the screen-state check** (`DECISIONS.md` 2026-10-10, `study\29` §2): one C++ file of this repo that
    writes the screen state to a file at exit (`atexit`) when a test asks, linked into the programs of both
    compilers through the extra flags that exist (`-f:ExtraLinkerFlags=`, `CXXLIBS_EXTRA`), the clone untouched;
    the runner records it with the old compiler and compares it for the new one; a key helper for windowed
    programs. One small OpenSpec change, design first, **measured first**: that the hook sees the pages from its
    own file and runs at every kind of exit (`SYSTEM`, `END`, the window closed, a runtime error), what is written
    per screen mode, and that keys posted to the window reach `INKEY$` and `END`'s wait. Test programs end with
    `SYSTEM` (`END` prints its prompt into the window first). First recordings need only the old compiler:
    upstream's 47 programs without `$CONSOLE:ONLY`, then small drawing programs cut from Civil War Strategy
    (`tests\programs\README.md`, stage 2), then `qbasic_testcases`. No copy of libqb and no fixed-seed session of
    the whole game in this change.
10. (was 11) `DEFxxx` and `COMMON`, then the rest of arrays and `TYPE`: arrays in procedures, `REDIM`, dynamic
    arrays, `OPTION BASE`, plain member arrays (`study\26` §4; their `_STATIC`/`_DYNAMIC` markers stay in
    `SOMEDAY.md`). The only step that moves the corpus, upstream and Civil War Strategy at once; scope it with
    `qb64rust --report unsupported` over the three.
11. (was 10) `$CONSOLE` with `_DEST _CONSOLE` and `$SCREENHIDE`, measured first (`study\26` §5); check once that
    the GitHub Windows runner can start the hidden window.

## Numbers at the last full runs

All of them: 2026-10-10, session 39, release build against the reference clone, after `m2-builtin-statements`
(its task 8.2).

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | 255 of 255, also with `QB64RUST_NO_FOLD=1` |
| Full corpus (307 programs) | 271 pass, none wrong at run time; 27 rejected with "not supported yet" and 4 `.err` programs with only marked errors (`tests\corpus\README.md`) |
| Upstream | **50 of 279** (`tests\upstream\pass.list`); none wrong in the full run of the suite (404 programs) |
| Call-site check | 256 of 256 programs equal (`tools\callsite\README.md`); about 1 min |
| Real programs (`tests\programs`) | Civil War Strategy: the front end reads it with no real error; every diagnostic is "not supported yet" (what it needs next: `tests\programs\README.md`) |
| Differential | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 64 only-marked rejections (`tests\upstream\README.md`) |
| Tier 1 (`cargo test`) | about 20 s |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 41 pass, under a second |
| Demand (`qb64rust --report unsupported`, 2026-10-10) | Corpus: 27 programs held back, 14 of them by one message each. Upstream (348 without an `.err`): 300 held back; arrays as `TYPE` members 152 programs, `REDIM` 148, `$Unstable` 85, `LBOUND` of an expression 63, `_DEST` 54. Civil War Strategy: 1,740 marked errors in 38 messages (`LINE` 899, `COLOR` 190, `LOCATE` 145) |
| Tier 2 as CI runs it, locally | about 12 min at the last full build; about 2 min with `--jobs 8 --build-cache` when little C++ changed |
| Full corpus with the old compiler (2026-10-10) | 300 pass, 5 known failures; about 12 min with one job, 3 min 44 s with `--jobs 8` (`tools\legacy_tests\README.md`) |

How to read them: the corpus has given most of what it can before step 10 (11 of its 27 rejected programs wait
on arrays, `TYPE` and `DEFINT`; the rest on `PRINT USING`/`TAB`/`SPC`, random files and a few functions). Upstream
will stay near 50 until steps 10 and 11: its blockers are the rest of arrays and `_DEST` (`study\27` §4).

## Known bugs

None open.

## Queued for later milestones

| When | Item |
|---|---|
| M3 | Plain copy of `..\QB64pe\internal\c` at the reference clone's commit, `16f629784e` (`DECISIONS.md` 2026-10-02 and 2026-10-09) |
| M3 | Civil War Strategy builds and its screens match (`tests\programs\README.md`, stages 2 and 3) |
| Any time | A formatter that matches `-y` (out of M2's exit criterion, `DECISIONS.md` 2026-10-09) |
| M3 | Programs that draw (without `$CONSOLE:ONLY` and not writing to `_DEST _CONSOLE`), with an oracle such as a screen-state dump at exit compared between old and new compiler (`study\22` §3.2, `study\28` §3) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
