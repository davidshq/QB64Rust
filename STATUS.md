# Status and next steps

Updated 2026-10-13 (session 55).

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

M0 (baseline) and M1 (VS Code extension on the old compiler) are complete. **M2 (front end) is complete**: all 11
steps of the order of work are done and its exit criterion is met (`ROADMAP.md`); the last change,
`m2-console-screenhide`, was archived 2026-10-12 with all 24 tasks done. **M3 is under way**: its first step,
OpenSpec change `m3-window-output`, is done (all 25 tasks; committed and archived 2026-10-12):
a program whose output is a window compiles, with the text-screen, drawing and pacing statements and the screen
functions. Its second step, OpenSpec change `m3-get-put-bload`, is built (all tasks; in the working tree, not
committed or archived): what Civil War Strategy needs to build apart from sound, so that the game's report
names only `SOUND`, `PLAY`, `BEEP` and `$EXEICON`; 51 of the 52 screen programs pass by the state of their
screen. Step 9 of M2, built-in statements, was OpenSpec change
`m2-builtin-statements` (archived 2026-10-10); after it came the eighth review (`study\29`) and `m2-demand-report-and-runner-tests`
(archived 2026-10-10: `qb64rust --report unsupported`, and tests of the test runner). Step 9a, the screen-state
check, was OpenSpec change `m2-screen-state-check` (all 22 tasks done, archived 2026-10-10).
A program that draws can now be tested: `tools\screen\screen_state.cpp`, linked into a test program of either
compiler, writes the state of the screen at exit and gives the program its keys; the corpus runner records and
compares it (`<name>.screen`, `<name>.keys`); 52 programs are recorded in `tests\corpus\screen\`
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
in `$IF`, and, with the first step of M3: programs with no console at all (a window), `SCREEN`, `CLS`, `COLOR`,
`LOCATE`, `WIDTH`, `VIEW PRINT`, `PSET`, `PRESET`, `LINE`, `CIRCLE`, `PAINT`, `DRAW`, `VIEW`, `WINDOW`,
`PALETTE`, `PCOPY`, `_LIMIT`, `_DELAY`, `SLEEP`, `_DISPLAY`, and the functions `INKEY$`, `CSRLIN`, `POS`,
`POINT`, `SCREEN`, and, with its second step: a line that is a SUB's name and a colon (a call, or a label when
a jump names the word), `TAB` and `SPC`, `DEF SEG`, `POKE`, `PEEK`, `BLOAD`, `BSAVE`, `VARPTR` and `VARSEG` of an
array of numbers of the main module (which then lies in conventional memory), graphics `GET` and `PUT`,
`_RGB32`, `_RGBA32`, `_FULLSCREEN`, `DATE$`, `TIME$` (`crates\README.md`). There is
a thin language server (`qb64rust lsp`). A plain built-in now comes in as a row, a call-site program and a slice
line: the call-site check is kept and part of tier 2 (`DECISIONS.md` 2026-10-10).

## Open items

- None from CI: `rust.yml` (`test`, `tier2`, `deny`), `vscode-extension.yml` and `repo-check.yml` are green on
  GitHub on `main` at `25e4ff6`, the merge of `m2-console-screenhide` (seen 2026-10-13; `tier2` 20 min 58 s; at
  `d9f5e95` on the branch it was seen with `s66_window_builtins` and `glut/icon` passing, so the runner builds
  the libqb variant with the icon part).
- No branch is waiting: `m2-console-screenhide` is merged into `main` (pull request 4, 2026-10-13).
- `m3-window-output` is committed on branch `m3` (`9ae4d40`) and archived; the branch has no upstream yet, so
  nothing of it has run on GitHub: CI is looked at when the user asks, not a gate
  (`CLAUDE.md` working rule 9), and the `tier2` job now runs by hand and weekly, so it must be started to see
  this. `rust.yml` itself changed (that trigger), which shows on its first run. **One thing to see there:** the `tier2` job runs screen programs with a shown window
  for the first time, and GitHub's Windows runner may not give them one (`study\00` §5, "The GitHub Windows
  runner"). If they fail there and pass here, the fallback is the design's: a `screen.list` run locally and not
  by the job.
- `m3-get-put-bload` is in the working tree of branch `m3`, not committed; all its tasks are done and it can be
  archived after the commit. Four things in it go beyond what was agreed and are for the user to look at
  (`DECISIONS.md` 2026-10-13, second row): dynamic arrays in conventional memory, `DATE$` and `TIME$`, five
  changed `cpp` snapshots, programs cut from the game by line ranges.

Every question of the seventh and eighth reviews is answered (`study\28` §9, `study\29` §8, `DECISIONS.md`
2026-10-09 and 2026-10-10): the local reference clone (`16f629784e`) is the one reference for the old compiler;
Civil War Strategy is in `tests\programs` as the real-program graphics target; the formatter is out of M2's exit
criterion; the screen-state check needs no copy of libqb.

## Next

M2's order was set by the fifth and sixth reviews (`study\26` §6, `study\27` §3) and changed by the eighth
(`study\29` §3: the last two steps swapped). Its last steps:

9. Done: built-in statements and functions by demand (`m2-builtin-statements`). Further plain built-ins come in
   as programs name them, each with a call-site program (`crates\README.md`); what was left marked and why is in
   task 7.4 of the change.
9a. Done: the screen-state check (`m2-screen-state-check`; `tools\screen\README.md`). What a set of screen
    programs needs: `qb64rust --report unsupported tests\corpus\screen`.
10. Done: (was 11) default types, `COMMON`, dynamic arrays, arrays in procedures, array parameters, the rest
    of `TYPE` and member arrays (`m2-defxxx-common-arrays`, all 29 tasks done, archived 2026-10-12).
    What each task found is in the change's `tasks.md`, how each group was built in its `design.md` ("As
    built"), the measured facts in `study\00` §5 (`verification\v24_*`, `v25_*`). Still marked in the
    member-array layer, each listed in `SOMEDAY.md`: a bound that names a `CONST`, a member array of an array
    element beside other targets of one `ERASE` or `REDIM`, an element of a member array as a target of
    `INPUT`, `READ` or `MID$`, `REDIM t.v(0) AS type` under `OPTION BASE 1`.
11. Done: (was 10) `$CONSOLE` with `_DEST _CONSOLE` and `$SCREENHIDE` (`study\26` §5), with the wider scope
    the user decided (`DECISIONS.md` 2026-10-12): the window built-ins of the 19 upstream `glut` tests as
    compiled, never-run code, the constants of the auto-included file, the precompiler flags in `$IF`
    (`m2-console-screenhide`, all 24 tasks done, archived 2026-10-12). What each task found is in the change's
    `tasks.md`, how each group was built in its `design.md`, the measured facts in `study\00` §5
    (`verification\v26_*`, 140 programs). The GitHub Windows runner starts a program with a hidden window. What
    stayed marked is in `SOMEDAY.md` ("Left marked by `m2-console-screenhide`").

**M3's order of work** (user, `DECISIONS.md` 2026-10-12):

1. Done: a program with a window as its output, the text-screen, drawing and pacing statements, the screen
   functions (`m3-window-output`, all 25 tasks, archived 2026-10-12). What each task found is in the
   change's `tasks.md`, the call of every form in its `design.md` D2, the measured facts in `study\00` §5
   (`verification\v27_*`, 65 programs), what stayed marked in `SOMEDAY.md`.
2. Built: graphics `GET` and `PUT`, `DEF SEG` with `BLOAD`, data files beside a corpus program, `TAB` and `SPC`,
   labels named like a SUB: what Civil War Strategy needs to build; more programs cut from it
   (`m3-get-put-bload`, all 24 tasks; to commit, then archive). What each task found is in the change's
   `tasks.md`, how it was built in its `design.md` ("As built"), the measured facts in `study\00` §5
   (`verification\v28_*`, 100 programs), what stayed marked in `SOMEDAY.md`.
3. **Next:** sound (`PLAY`, `SOUND`, `BEEP`: compiled, and what of them can be checked); with it `$EXEICON` and
   the first try at building the whole game (`tests\programs\README.md`, stage 2).
4. The console programs of the corpus that still wait (`PRINT USING`, random and binary files).
5. The plain copy of `internal\c`, when libqb must change.

## Numbers at the last full runs

2026-10-12, release build against the reference clone, after `m2-defxxx-common-arrays` (its task 10.1, session
48: every run of tier 1 and tier 2 and both full suites, with `--jobs 8 --build-cache`). The two rows of the old
compiler (the screen programs, the corpus of 336) are from session 40, 2026-10-10. Newer, from session 53
(2026-10-12, task 7.1 of `m2-console-screenhide`, the runs that close step 11, with `--jobs 8 --build-cache`):
tier 1, the runner's tests, the call-site check, the slice list, the upstream pass list, the differential pass
list and the two full suites (the full corpus, the upstream suite of 404). The upstream part of the demand row
is from session 52. Newest, from session 54 (2026-10-12, task 6.1 of `m3-window-output`, each run once with
`--jobs 8 --build-cache`): every row below but the last two and the upstream demand. Newest of all, from session 55
(2026-10-13, task 7.1 of `m3-get-put-bload`, each run once with `--jobs 8 --build-cache`): tier 1, the runner's
tests, the call-site check, the slice list with and without folding, the upstream and differential pass lists and
the two full suites; the rows of the old compiler and of `--pass-cache` are older.

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | 357 of 357 (51 of them screen programs), also with `QB64RUST_NO_FOLD=1`; 326 s and 260 s with the build cache at the close of `m3-get-put-bload`, where the C++ of every program with a dynamic array had changed (about 2 min each when little C++ changes) |
| Full corpus (395 programs) | 376 pass (the 357 of the slice list, 19 `.err` programs), none wrong at run time; 13 rejected with "not supported yet" (11 console programs, the screen program `g03_capitol`, `239_lprint`) and 1 `.err` program with only marked errors; 5 known failures (`tests\corpus\README.md`); 61 s after the slice runs had filled the build cache |
| Upstream | **140 of 279** (`tests\upstream\pass.list`: 140, all passing, run by itself); the full suite again at the close of `m3-get-put-bload` with the same result, none wrong (404 programs: 140 pass, 225 rejected, 38 `.err` programs with only marked errors, 1 known failure) |
| Call-site check | 810 of 810 programs equal (`tools\callsite\README.md`; 147 of them for `m3-get-put-bload`); 55 s with the old compiler's answers kept (`target\callsite-cache`); not timed without them since the 147 came in (5 min 44 s for the 663 before) |
| Real programs (`tests\programs`) | Civil War Strategy: the front end reads it with no real error; 66 marked errors in 4 messages are left (`SOUND`, `PLAY`, `BEEP`, `$EXEICON`), 135 in 15 before (`tests\programs\README.md`) |
| Differential (`tests\differential\pass.list`) | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 53 only-marked rejections (`tests\upstream\README.md`) |
| Tier 1 (`cargo test`) | passes (292 tests; not timed again since the 147 call-site programs, 7 front-end tests and 11 corpus programs of `m3-get-put-bload` came in, 28 s before); the 13 command-line tests that build an executable (`--ignored`) were not run again at this close (nothing of the driver changed) |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 68 pass, under a second |
| `slice.list` while working, with `--pass-cache` | 24 s when no program's executable changed, 92 s when all are run (`--jobs 8 --build-cache`, 2026-10-12) |
| Screen programs (`tests\corpus\screen`, 52) | Old compiler: 52 pass, each recorded from two equal runs. New compiler: 51 pass, each on its first run; `g03_capitol` is rejected with only "not supported yet" errors (`PLAY`) |
| Demand (`qb64rust --report unsupported`; upstream with `--include-root tests\upstream\root`) | Corpus: 11 console programs and 1 screen program held back. Civil War Strategy: `SOUND` 33, `PLAY` 31, `BEEP` 1, `$EXEICON` 1. Upstream, as of session 52 (the full suite's result did not change) (348 without an `.err`): 122 without errors, 216 held back, 10 with other errors (the ten that include the old compiler's sources); `_DYNAMIC`/`_STATIC` members 86 programs (deferred), `$Unstable` 85, an array used after a marked declaration 60, `_MEM` 30, `REDIM _RETAIN` 27, `_MEMFREE` 22 (no line names `_DEST`, `$CONSOLE`, a window built-in, `_TRUE` or a precompiler flag any more) |
| Tier 2 as CI runs it, locally | about 12 min at the last full build; about 2 min with `--jobs 8 --build-cache` when little C++ changed |
| Full corpus with the old compiler (2026-10-10, 336 programs) | 331 pass, 5 known failures, `--jobs 8`; the 305 before the screen programs took about 12 min with one job, 3 min 44 s with `--jobs 8` (`tools\legacy_tests\README.md`) |

How to read them: the corpus's 11 rejected console programs wait on `PRINT USING`, random and binary files
and a few functions (step 4 of M3's order); its screen programs pass but for `g03_capitol` (`PLAY`). The game
waits on sound alone (step 3). Upstream
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
| M3 | The image built-ins that run (`_LOADIMAGE`, `_PUTIMAGE`, …), fonts, the mouse: in none of the five steps of M3's order yet |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
