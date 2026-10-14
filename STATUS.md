# Status and next steps

Updated 2026-10-14 (session 59).

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
functions. Its second step, OpenSpec change `m3-get-put-bload`, is done (all 24 tasks; committed and
archived 2026-10-13): what Civil War Strategy needs to build apart from sound, so that the game's report
names only `SOUND`, `PLAY`, `BEEP` and `$EXEICON`. Its third step, OpenSpec change `m3-sound`, is done (all 18
tasks; committed and archived 2026-10-13): sound, `$EXEICON`, and a built-in statement with a template
after `CALL`. **Civil War Strategy compiles with no diagnostic and builds**, and all 52 screen programs pass by
the state of their screen. Its fourth step, OpenSpec change `m3-console-files`, is done (all 22 tasks,
2026-10-13; committed, archived 2026-10-14): the console programs of the corpus that waited. **No program of
the corpus that the old compiler accepts is held back any more.** Its fifth step, OpenSpec change
`m3-scripted-game`, is done (all 14 tasks, 2026-10-14; not committed or archived yet): **Civil War Strategy's
screens are compared in a scripted game**, a copy of the game with listed edits played for two turns from a keys
file, 23 states of its screen equal to the old compiler's; the new compiler's build passed on its first run.
Step 9 of M2, built-in statements, was OpenSpec change `m2-builtin-statements` (archived 2026-10-10); after it came the eighth review (`study\29`) and `m2-demand-report-and-runner-tests`
(archived 2026-10-10: `qb64rust --report unsupported`, and tests of the test runner). Step 9a, the screen-state
check, was OpenSpec change `m2-screen-state-check` (all 22 tasks done, archived 2026-10-10).
A program that draws can now be tested: `tools\screen\screen_state.cpp`, linked into a test program of either
compiler, writes the state of the screen at exit and gives the program its keys; the corpus runner records and
compares it (`<name>.screen`, `<name>.keys`); 52 programs are recorded in `tests\corpus\screen\`
(`tools\screen\README.md`).
The compiler parses every program the old compiler accepts with no false error, and compiles scalars, procedures,
error handling, every operator, `CONST`, control flow, `SELECT CASE`, static arrays and `TYPE` of the main module,
107 built-in functions, all 17 numeric types, fixed-length strings, sequential files, `DATA`/`READ`/`RESTORE`,
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
`_RGB32`, `_RGBA32`, `_FULLSCREEN`, `DATE$`, `TIME$`, and, with its third step: `SOUND`, `PLAY` (statement and
function) and `BEEP` with the audio part of libqb, `$EXEICON` with a name that starts with `./` or is
absolute, and every built-in statement that has a template after `CALL` (`CALL LINE((1, 2)-(3, 4), 5, BF)`),
and, with its fourth step: `PRINT USING` to the screen and to a file, `GET` and `PUT` on a file open `FOR
RANDOM` or `FOR BINARY`, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`, `EXIT SELECT` and `EXIT CASE`, the functions
`_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`, `_SHR`, `_ROL`, `_ROR`, and `CHAIN` and `LPRINT` as code that no test
runs (`crates\README.md`). There is
a thin language server (`qb64rust lsp`). A plain built-in now comes in as a row, a call-site program and a slice
line: the call-site check is kept and part of tier 2 (`DECISIONS.md` 2026-10-10).

## Open items

- None from CI: `rust.yml` (`test`, `tier2`, `deny`), `vscode-extension.yml` and `repo-check.yml` are green on
  GitHub on `main` at `25e4ff6`, the merge of `m2-console-screenhide` (seen 2026-10-13; `tier2` 20 min 58 s; at
  `d9f5e95` on the branch it was seen with `s66_window_builtins` and `glut/icon` passing, so the runner builds
  the libqb variant with the icon part).
- No branch is waiting: `m2-console-screenhide` is merged into `main` (pull request 4, 2026-10-13).
- `m3-window-output` (`9ae4d40`), `m3-get-put-bload` (`3258d56`, archived in `bf59892`) and `m3-sound`
  (`556724c`, archived in the commit after it) are committed on branch `m3`; what CI made of them has not been
  looked at: CI is looked at when the user asks, not a gate (`CLAUDE.md` working rule 9), and the `tier2` job
  runs by hand and weekly, so it must be started to see this. `rust.yml` itself changed (that trigger), which
  shows on its first run.
  **Two things to see there:** the `tier2` job runs screen programs with a shown window for the first time, and
  GitHub's Windows runner may not give them one (`study\00` §5, "The GitHub Windows runner"); and it now builds
  libqb's audio part and runs two programs that need a sound device (`s75_sound_wait` and the second part of
  `s76_sound_errors`: without a device libqb queues and checks nothing, `SOMEDAY.md`). If either kind fails
  there and passes here, the fallback is the design's: a list run locally and not by the job.
- `m3-sound` is committed and archived (2026-10-13, its specs merged). One thing in it goes beyond its proposal
  and is for the user to look at: a built-in statement with a template after `CALL` is compiled for every
  statement, not for `SOUND` and `PLAY` alone, with errors for the forms the old
  compiler rejects (task 2.1 of the change: design D1 asks for the general fix, and the rule was read in
  `qb64pe.bas` and measured with twelve programs). One question from the session: whether the full corpus is
  still worth a closing run, since all but 37 of its programs are on the slice list, which runs twice at the
  close.
- `m3-console-files` is committed (`4fe8c76`) and archived (2026-10-14, its specs merged; `language/print-using`
  and `language/record-files` are new main specs). For the user to look at, each in the change's `tasks.md`:
  - **One accepted form stays "not supported yet" that task 1.2 did not name**: a whole array `e()` as the
    string of `FIELD`, `LSET` or `RSET` (the old compiler takes the array's first element with no bounds
    check; `SOMEDAY.md`).
  - **Three things differ from the design**: the arguments of `_MIN`, `_MAX`, `_CLAMP`, the parts of `_IIF` and
    the numbers of `PRINT USING` are not converted in the tree (the old compiler leaves that to C++, and a cast
    written out fails the call-site check); nothing in this compiler takes a part out of an expression, so the
    validation of `_IIF` checks its shape, not a hoisted part; `sema` keeps no stack of `SELECT`s (the parser
    rejects every misplaced `EXIT SELECT`).
  - **Beyond the proposal**: the runtime part of the printer (`DEP_PRINTER=y`, without which `LPRINT` does not
    link); two rules of the call-site tool extended (`ull` literals; `skip_pu` and `print_using_format`
    numbers); six upstream programs on `pass.list` (148); `238_chain` on `slice.list` (its output is equal),
    so one program does reach `sub_chain`, with a name that is not there.
  - **Two questionable behaviours of the old compiler kept and listed** (`SOMEDAY.md`, "QB64pe behaviours to
    review"): `EXIT CASE` before a `SELECT` in the same `CASE` goes on after that inner `SELECT`; a field
    string stays a field after `=`.
  - **Not of this change, met on the way**: a literal above the `_INTEGER64` range (`18446744073709551615`) is
    "not supported yet: unsigned literals".
- `m3-scripted-game` is done and waits to be committed and archived. For the user to look at, each in
  the change's `tasks.md`:
  - **Two things changed from the design while built**: a `prompt` *counts* the program's announcements (a menu
    announces again after every arrow key, so "wait for a change" returned too early; task 1.1); a `prompt` that
    fails writes its reason as the last line of the state file, since libqb points stderr at the console the
    program was started from (task 2.3).
  - **Beyond the tasks as written**: `c12_prompts` is in `screen\`, not `slice\s78_prompts` (task 2.2); the test
    of a `prompt` that never comes is a command-line test in `cli.rs`, not a runner test (task 2.3);
    `state_to_png.py --all`; `g12_game.bas` marked `-text` in `.gitattributes` (the game's CP437 bytes).
  - **The script reaches** recruiting, moves, a ship, a railroad move, three reports, Fortify, the Utility menu,
    a save, two computer turns with a battle each, and Quit; **not** a naval action, a retreat, the end of a
    game, loading, the alternative map, two players (task 4.1; `SOMEDAY.md`).
- Four things in `m3-get-put-bload` go beyond what was agreed and are for the user to look at
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
2. Done: graphics `GET` and `PUT`, `DEF SEG` with `BLOAD`, data files beside a corpus program, `TAB` and `SPC`,
   labels named like a SUB: what Civil War Strategy needs to build; more programs cut from it
   (`m3-get-put-bload`, all 24 tasks, archived 2026-10-13). What each task found is in the change's
   `tasks.md`, how it was built in its `design.md` ("As built"), the measured facts in `study\00` §5
   (`verification\v28_*`, 100 programs), what stayed marked in `SOMEDAY.md`.
3. Done: sound (`SOUND`, `PLAY`, `BEEP`), `$EXEICON`, a statement with a template after `CALL`, and the whole
   game compiled and built (`m3-sound`, all 18 tasks, archived 2026-10-13). What each task found is in the
   change's `tasks.md`, the measured facts in `study\00` §5
   (`verification\v29_*`, 130 programs), what stayed marked in `SOMEDAY.md`, the game's numbers in
   `tests\programs\README.md`.
4. Done: the console programs of the corpus that waited: `PRINT USING`, `GET` and `PUT` on a file (random and
   binary), `FIELD`, `LSET`, `RSET`, `LOCK`/`UNLOCK`, `EXIT SELECT`, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`,
   `_SHR`, `_ROL`, `_ROR`, and `CHAIN` and `LPRINT` compiled and never run (`m3-console-files`, all 22 tasks,
   archived 2026-10-14). What each task found is in the change's `tasks.md`, the measured
   facts in `study\00` §5 (`verification\v30_*`, 177 programs and the table of result types), what stayed
   marked in `SOMEDAY.md` ("Left marked by `m3-console-files`").
5. Done: Civil War Strategy's screens in a scripted game (`m3-scripted-game`, all 14 tasks, 2026-10-14): the
   keys file's `prompt` and `state` steps, a copy of the game written by `cut_game.py`, `g12_game` with 23
   states and `c12_prompts`, both on `slice.list`. What each task found is in the change's `tasks.md`, the
   measured facts in `study\00` §5 ("Civil War Strategy played from a keys file"), the game's numbers in
   `tests\programs\README.md`.
6. **Next**, for the user to choose (`ROADMAP.md`): the image built-ins that run, fonts, the mouse (after
   `_MEM`, `$ASSERTS` and `$DEBUG` among what holds back the upstream tests); the plain copy of `internal\c`,
   when libqb must change.

## Numbers at the last full runs

From session 57 (2026-10-13, task 8.1 of `m3-console-files`, release build against the reference clone, each
run once with `--jobs 8 --build-cache`): tier 1, the command-line tests that build, the runner's tests, the
call-site check, the slice list with and without folding, the upstream and differential pass lists, the two
full suites and the demand reports. The rows of the real programs, of the old compiler and of `--pass-cache` are
older (sessions 56, 40 and 54).

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | **386 of 386** (54 of them screen programs, `g12_game` the scripted game) at the close of `m3-scripted-game` (5 min 53 s: every screen program linked again with the changed `screen_state.cpp`); 384 of 384 also with `QB64RUST_NO_FOLD=1` at the close of `m3-console-files` (89 s and 144 s with the build cache; not repeated, since no compiler crate changed since) |
| Full corpus (409 programs) | 403 pass (the 384 of the slice list, 19 `.err` programs), none wrong at run time; **none rejected with "not supported yet"**; 1 `.err` program with only marked errors (`218` `FRE`); 5 known failures (`tests\corpus\README.md`); 66 s after the slice runs had filled the build cache |
| Upstream | **148 of 279** (`tests\upstream\pass.list`: 148, all passing, run by itself, 41 s); the full suite at the close of `m3-console-files`: 404 programs, 148 pass, 255 rejected or with only marked errors, 1 known failure, none wrong |
| Call-site check | 1145 of 1145 programs equal (`tools\callsite\README.md`; 223 of them for `m3-console-files`: the eight functions 75, `PRINT USING` and `LPRINT` 55, file `GET`/`PUT` and locks 64, `FIELD`/`LSET`/`RSET` 23, `CHAIN` 6) |
| Real programs (`tests\programs`) | Civil War Strategy: **no diagnostic, and it builds** (3.6 s; the old compiler 6.9 s the same day). Its C++ compared once with the old compiler's, line by line: no call differs on 4,332 lines. **Played for two turns from a keys file (`g12_game`, a copy with listed edits): 23 states equal to the old compiler's**, 1.1–1.9 s a run (`tests\programs\README.md`) |
| Differential (`tests\differential\pass.list`) | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 53 only-marked rejections (`tests\upstream\README.md`) |
| Tier 1 (`cargo test`) | passes; the 17 command-line tests that build an executable (`--ignored`) pass too (10 s), among them the build of Civil War Strategy, the icon in the executable and a `prompt` that never comes (2026-10-14) |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 68 pass, under a second |
| `slice.list` while working, with `--pass-cache` | 24 s when no program's executable changed, 92 s when all are run (`--jobs 8 --build-cache`, 2026-10-12) |
| Screen programs (`tests\corpus\screen`, 54) | Old compiler: 54 recorded, each from two equal runs. New compiler: 54 pass, each on its first run (the 52 before also with the changed `screen_state.cpp`, so a keys file without the new steps gives the same states) |
| Demand (`qb64rust --report unsupported`; upstream with `--include-root tests\upstream\root`) | Corpus: **nothing held back** (389 without an `.err`: 388 without errors, and `s77_exeicon`, whose icon only the runner puts beside it). Civil War Strategy: nothing held back. Upstream (348 without an `.err`): 131 without errors, 206 held back, 11 with other errors (the ten that include the old compiler's sources, and `icon_relative/test`, whose icon lies in the clone); `_DYNAMIC`/`_STATIC` members 86 programs (deferred), `$Unstable` 85, an array used after a marked declaration 60, `_MEM` 30, `REDIM _RETAIN` 27, `_MEMFREE` 22 (no line names `_DEST`, `$CONSOLE`, a window built-in, `_TRUE` or a precompiler flag any more) |
| Tier 2 as CI runs it, locally | about 12 min at the last full build; about 2 min with `--jobs 8 --build-cache` when little C++ changed |
| Full corpus with the old compiler (2026-10-10, 336 programs) | 331 pass, 5 known failures, `--jobs 8`; the 305 before the screen programs took about 12 min with one job, 3 min 44 s with `--jobs 8` (`tools\legacy_tests\README.md`) |

How to read them: every program of the corpus that the old compiler accepts compiles and, where it can be
compared, passes; its screen programs all pass. The game compiles and builds, and a copy of it played for two
turns shows the same 23 screens as the old compiler's build (stage 3 of `tests\programs\README.md`). Upstream
went from 50 to 114 over step 10 (64 before its member arrays) and is at 140 after group 6 of step 11 (115
after `_DEST` and `$CONSOLE`; all 19 `glut` tests with the window built-ins and `_TRUE`/`_FALSE`; four of the
nine `precomp-flags` tests), at 142 after `m3-sound` and at 148 after `m3-console-files`. Its next blockers outside `deferred.list` are `_MEM`, `$ASSERTS`/`$DEBUG` and the
image built-ins that run; the
`_DYNAMIC`/`_STATIC` markers and `$Unstable` are deferred (`SOMEDAY.md`).

## Known bugs

None known. (The two of 2026-10-11, a `+` after another sign and a `DIM` of several dynamic arrays, are fixed:
`tasks.md` of `m2-defxxx-common-arrays`, above its group 7; slice programs `s17_signs` and `s52_dim_list`.)

## Queued for later milestones

| When | Item |
|---|---|
| M3 | Plain copy of `..\QB64pe\internal\c` at the reference clone's commit, `16f629784e` (`DECISIONS.md` 2026-10-02 and 2026-10-09) |
| Any time | A formatter that matches `-y` (out of M2's exit criterion, `DECISIONS.md` 2026-10-09) |
| M3 | The image built-ins that run (`_LOADIMAGE`, `_PUTIMAGE`, …), fonts, the mouse: in none of the five steps of M3's order yet |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
