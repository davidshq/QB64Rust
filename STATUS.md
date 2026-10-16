# Status and next steps

Updated 2026-10-15 (the session of `m3-whole-array-assign`).

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
`m3-scripted-game`, is done (all 14 tasks, 2026-10-14; committed, archived 2026-10-14): **Civil War Strategy's
screens are compared in a scripted game**, a copy of the game with listed edits played for two turns from a keys
file, 23 states of its screen equal to the old compiler's; the new compiler's build passed on its first run. Its
sixth step, OpenSpec change `m3-images-fonts-mouse`, is done (all 17 tasks, 2026-10-14; committed, archived
2026-10-14): **the image, colour, font and mouse built-ins**, 12 screen programs compared by their state,
the mouse fed by mouse steps of a keys file. The `_MEM` part of its seventh step, OpenSpec change `m3-mem`, is
done (all 14 tasks, 2026-10-14; committed and archived, `96cfcee`): **`_MEM`**, the blocks of references,
addresses, new memory and images, `_MEMGET`, `_MEMPUT`, `_MEMFILL` and `_MEMCOPY` written as the old compiler
writes them, every runtime error 300 to 313, and `MKI$` … `_CV`; upstream went from 153 to 174. Its eighth step,
OpenSpec change `m3-asserts-debug-zlib`, is done (all 12 tasks, 2026-10-14; committed and archived): **`$ASSERTS`,
`_ASSERT`, `$DEBUG` (no debugger code), the checksum, compression and base64 functions, `END n` and `SYSTEM n`, and
`-f:GenerateLicenseFile`**; upstream went from 174 to 186. Its ninth step, OpenSpec change `m3-redim-retain`, is
done (all 9 tasks, 2026-10-14; committed and archived, `cbdfafb`): **`REDIM _RETAIN`**, the elements kept by their
indexes for every kind of element and array, and a `REDIM` whose bound calls a FUNCTION that redims the same
array no longer crashes (`DIVERGENCES.md` D-018); upstream went from 186 of 279 to 193 of 286. Its tenth step,
OpenSpec change `m3-typefields`, is done (all 19 tasks, 2026-10-15; committed, `c84f556`; archived 2026-10-15, its
specs merged): **`$UNSTABLE:TYPEFIELDS`, the `_STATIC`/`_DYNAMIC` member markers and `_DYNAMIC`
member arrays**, of variables, elements and nested values, with their copies, `_MEM`, and whole arrays of their
types in files; three memory misuses of the old compiler fixed as the user decided (D-004 widened, D-019, D-020)
and two more found while built (D-021, for the user to confirm); upstream went from 193 of 286 to 290 of 383. Its
eleventh step, OpenSpec change `m3-whole-array-assign`, is done (all 13 tasks, 2026-10-15; not committed, not
archived): **whole-array assignment `a() = b()`**, of arrays and member arrays (plain and `_DYNAMIC`) of every
element type and place and of arrays of owner types, with the old compiler's compile-time and run-time checks; two
misuses of the old compiler not followed (D-004 widened, D-022, for the user to confirm); upstream went from 290 of
383 to 310 of 399.
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
runs, and, with its sixth step: the colour functions (`_RGB`, `_RED32`, `_HSB32`, …), images (`_NEWIMAGE`,
`_LOADIMAGE`, `_SAVEIMAGE`, `_PUTIMAGE`, `_MAPTRIANGLE`, `_CLEARCOLOR`, `_SETALPHA`, `_BLEND`, palettes, `PMAP`),
fonts (`_LOADFONT`, `_FONT`, `_PRINTSTRING`, `_PRINTMODE`, the `_U…` functions, `_UCHARPOS`, `_MAPUNICODE`), the
mouse (`_MOUSEINPUT`, `_MOUSEX`, `_MOUSEBUTTON`, `_MOUSEWHEEL`, …), and as code that no test runs
`_SCREENIMAGE`, `_CLIPBOARDIMAGE`, `_PRINTIMAGE`, `_DEPTHBUFFER`, `_MOUSEMOVE` and the 3-D `_MAPTRIANGLE`, and,
with its seventh step: the type `_MEM`, `_MEM` and `_MEMELEMENT` of every kind of place and `_MEM(offset,
size)`, `_MEMNEW`, `_MEMIMAGE`, `_MEMEXISTS`, `_MEMFREE`, `_MEMCOPY`, `_MEMGET` (function and statement),
`_MEMPUT`, `_MEMFILL`, `_OFFSET`, `MKI$`, `MKL$`, `MKS$`, `MKD$`, `_MK$`, `CVI`, `CVL`, `CVS`, `CVD`, `_CV`, and
`_MEMSOUND` as code that no test runs, and, with its eighth step: `$ASSERTS`, `$ASSERTS:CONSOLE`, `_ASSERT`,
`$DEBUG` (the flags `_DEBUG_` and `_SOCKETS_` and the warning, no debugger), `_ADLER32`, `_CRC32`, `_DEFLATE$`,
`_INFLATE$`, `_BASE64ENCODE$`, `_BASE64DECODE$`, `END n`, `SYSTEM n` and the licence file, and, with its ninth
and tenth steps: `REDIM _RETAIN`, `$UNSTABLE` (`TYPEFIELDS`, `MIDI`, `HTTP`), the `_STATIC`/`_DYNAMIC` member
markers, `_DYNAMIC` member arrays (`REDIM`, `_PRESERVE`, `_RETAIN`, `ERASE`, elements, bounds, array arguments,
whole-value copies, `_MEM`), and `PUT`/`GET` of whole arrays of a type that owns one, and, with its eleventh step:
whole-array assignment `a() = b()`, `x.v() = e(i).v()` (`crates\README.md`). There is
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
- `m3-scripted-game` is committed (`34026db`) and archived (2026-10-14, its spec merged into
  `testing/screen-state`). For the user to look at, each in
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
- `m3-images-fonts-mouse` is committed (`a536374`) and archived (2026-10-14, its specs merged;
  `language/images`, `language/fonts` and `language/mouse` are new main specs). For the user to look at, each in
  the change's `tasks.md`:
  - **Three forms needed more than a row**, against "every name a plain row" of task 1.3: a function's slot left
    empty (`_LOADIMAGE(f$, , r$)`, task 2.3, `filled_slots`), `_UCHARPOS`'s array as a function argument (a new
    node `ExprKind::Block`, task 3.1), and `_SCREENIMAGE`'s four slots given all or none (task 5.1,
    `template_counts`; it was accepted with two).
  - **One accepted form stays "not supported yet"**, against what task 1.3 told the user: `_UCHARPOS(t$,
    nd&())`, an array not declared, as for graphics `GET` (`SOMEDAY.md`).
  - **Changed while built**: each mouse step waits until libqb has drawn a frame of the screen (task 4.1;
    `_MOUSEX` maps an event by the frame last drawn, and gave -1 right after `SCREEN`).
  - **Beyond the tasks as written**: five upstream programs joined `pass.list` in groups 2 and 5, when tier 1
    asked for them, not in group 6; `_MOUSEMOVE` went on `NEVER_RUN` in group 4.
  - **Behaviours of libqb measured** (`study\00` §5): `_WIDTH` of a number that was never a handle is the
    screen's; `_SAVEIMAGE` with one writes a file; `_PRINTSTRING STEP` counts from another image's last point
    (these three in `SOMEDAY.md`, "QB64pe behaviours to review"); `PRINT` into a 5 by 5 image then
    `_FREEIMAGE` corrupts the heap (exit code 0xC0000374).
  - **A test sensitive to load**: `screen_state_prompt_that_never_comes` (500 ms) failed once in the closing
    runs while other tests ran beside it, and passed alone.
- `m3-whole-array-assign` is done (all 13 tasks, 2026-10-15), not committed, not archived. For the user to look
  at, each in the change's `tasks.md`, `DIVERGENCES.md` or `DECISIONS.md` 2026-10-15:
  - **Two misuses of the old compiler not followed** (Claude's choice, for the user to confirm, as chosen for
    memory misuses on 2026-10-14): after an index outside on the way to a member array nothing is copied (D-004
    widened; the old compiler copies a `STRING` member array through element 0's, `verification\v36_x48`); an
    array of an owner type with other bounds copies nothing after error 9 (D-022; the old compiler copies the
    target's number of elements and reads past the source, `x52`). Each has a CLI test.
  - **The user's two answers kept**: a type suffix on both names stays "not supported yet" (the old compiler
    copies element 0, `SOMEDAY.md`, also under "QB64pe behaviours to review"); `t001_whole_array_core` stays in
    `deferred.list` for its plain scalar beside an array typed by `DIM … AS`.
  - **Beyond the design**: a dynamic member array's descriptor must also have the copy's number of dimensions,
    which a `REDIM` can change (task 3.1; not measured). The messages differ in wording from the old compiler's
    (each rejected form is a real error).
  - **Done earlier than planned**: the 20 upstream programs were run and put on `pass.list` with group 2, as tier 1
    asks for a list for every clean program (task 2.3); `x() = 1` in `arrays_errors.bas` now gets the real error.
  - **Met on the way, not of this change**: `LEN(a())` of a whole array (4 upstream programs) is the next whole-array
    use the report names; `PRINT a()` (element 0) stays marked.
- `m3-typefields` is done (all 19 tasks, 2026-10-15), committed (`c84f556`); the change is
  archived (`openspec\changes\archive\2026-10-15-m3-typefields\`), its specs merged into `openspec\specs\` (a
  new spec, `language\unstable-features`). For the user to look at, each in the change's `tasks.md`, `DIVERGENCES.md` or `DECISIONS.md`
  2026-10-15:
  - **Two more misuses of the old compiler not followed** (`DIVERGENCES.md` D-021, Claude's choice, for the user
    to confirm): `_MEM` of a `_DYNAMIC` member that `ERASE` released crashes the old program (exit code 139),
    here error 9 and a block that does not exist; `GET` of a whole array of an owner type into a released array
    ends the old program with "Out of memory", here error 9; after a `GET` whose read raised an error nothing is
    unpacked (the old compiler unpacks whatever its buffer holds).
  - **Two smaller choices**: a file's bounds check also tests the member's number of dimensions; `_OFFSET` of a
    whole `_DYNAMIC` member stays "not supported yet" (`SOMEDAY.md`, "Left marked by `m3-typefields`").
  - **Beyond the tasks as written**: six more programs measured (`verification\v35_x63`–`x68`; `x68` showed that
    `_OFFSET` of an owner value, which the spec names an error, was accepted: it is the error now); `ERASE` of a
    `_DYNAMIC` member now makes a `_MEM` block over it invalid (group 3's code left it valid; `x66` found it); CLI
    tests for D-019, D-020 (its register row said "not yet") and D-021; stale lines of `crates\README.md` on the
    reads into members (unmarked with task 4.2) corrected.
  - **Twice a heredoc with backslashes went through the shell** (working rule 7): a vertical tab in
    `DIVERGENCES.md` and a tab in `DECISIONS.md`, both repaired; every changed file scanned, no control bytes.
- `m3-asserts-debug-zlib` is done (all 12 tasks, 2026-10-14), committed and archived. For the user to
  look at, each in the change's `tasks.md`, `design.md` ("Found by task …") or `DECISIONS.md`:
  - **Two answers of the user while it was built**: `$DEBUG` sets `_SOCKETS_` as the old compiler's does, and
    vWatch's names are not taken (`DIVERGENCES.md` D-017); a built-in that needs `$` named without it stays "not
    supported yet" (the spec's scenario narrowed, `SOMEDAY.md`).
  - **Corrected by the measurements**: errors 314 and 315 are not trapped by `ON ERROR` (the spec said they were);
    `_ASSERT`'s condition is an INTEGER slot (`Slot::Integer`, new); the exit code is the low 32 bits.
  - **Beyond the tasks as written**: the corpus runner compares an exit code (`<name>.exitcode`, four runner
    tests); `basic/test` passes with the licence file and joined `pass.list` (12 new upstream programs, not
    about 11).
  - **Behaviours of libqb measured** (`study\00` §5): `_INFLATE$` of bytes that are no zlib stream gives 10 MB of
    zero bytes and no error, and with a size larger than the data leaves the bytes past it unset.
  - **Fixed by the review of the change**: `press_any_key.py` passed on every exit code from 2^31 up as
    4294967295 (`sys.exit` takes a 32-bit C long on Windows), so `s111_end_code` (`END -1`) passed by chance;
    a new runner test checks every 32-bit code. A build without `-f:GenerateLicenseFile=true` now removes a
    licence file an earlier build left at the output path.
  - **Twice a heredoc with backslashes went through the shell** (working rule 7), on a scratch file and on
    `tests\corpus\README.md`: both checked, no control bytes (`check_repo.py`).
- `m3-mem` is done (all 14 tasks, 2026-10-14), committed and archived (`96cfcee`). For the user to look at, each
  in the change's `tasks.md`:
  - **One choice was the user's** (task 1.3): `_MEM(p())` of an array parameter whose dimensions nothing above
    has given takes the array's own lock (QB64pe reads the array's data as the lock's id; `DIVERGENCES.md`
    D-016).
  - **Four choices made while built** (`DECISIONS.md` 2026-10-14): the type `_MEM` added when a program names it
    (not before pass 1); `k = _MEMNEW(4)` (a block stored into a number, whose C++ the old compiler fails to
    build) a compile error; `LEN` of a whole member array "not supported yet" (a false error that a marked
    `DIM … AS _MEM` had hidden); a type argument with `* n` in the parser (`_MEMGET(m, o, STRING * 4)`).
  - **Four behaviours of QB64pe kept and listed** (`SOMEDAY.md`, "QB64pe behaviours to review"): a block stays
    valid after `ERASE` (and `_MEMPUT` writes into the released memory of a dynamic array); a block's lock is
    that of the body where `_MEM` stands (a block of a main-module variable taken in a SUB dies when the SUB
    returns); `_MEMFREE` of a `_MEMNEW(0)` block can free a pointer twice (exit code 127, libqb's); the types of
    `_MEMELEMENT` and `_MEM` of members.
  - **Left marked, not measured** (`SOMEDAY.md`, "Left marked by `m3-mem`"): a `TYPE` that holds a `STRING`,
    member arrays not at the end of a place, strings in arrays and members for `_OFFSET` and the statements.
  - **Beyond the tasks as written**: 21 upstream programs joined `pass.list` group by group (153 to 174, the six
    `paint/tile_*` among them); `tests\corpus\README.md` had counts from before `m3-images-fonts-mouse`, now
    brought up to date.
  - **Three times a Python edit through the shell turned `\v` into a vertical tab** (working rule 7), and some
    patch scripts wrote CRLF line ends: all found and repaired before the closing runs.
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
5. Done: Civil War Strategy's screens in a scripted game (`m3-scripted-game`, all 14 tasks, archived 2026-10-14): the
   keys file's `prompt` and `state` steps, a copy of the game written by `cut_game.py`, `g12_game` with 23
   states and `c12_prompts`, both on `slice.list`. What each task found is in the change's `tasks.md`, the
   measured facts in `study\00` §5 ("Civil War Strategy played from a keys file"), the game's numbers in
   `tests\programs\README.md`.
6. Done: the image built-ins that run, fonts, the mouse, and the image built-ins no test runs
   (`m3-images-fonts-mouse`, all 17 tasks, archived 2026-10-14). What each task found is
   in the change's `tasks.md`, the call of every form in its `design.md` D1, the measured facts in `study\00` §5
   (`verification\v31_*`, 113 programs), what stayed marked in `SOMEDAY.md` ("Left marked by
   `m3-images-fonts-mouse`").
7. `_MEM` done (`m3-mem`, all 14 tasks, 2026-10-14): the type, the blocks of references, addresses, new memory
   and images, the statements written inline, every runtime error, `MKI$` … `_CV`. What each task found is in
   the change's `tasks.md`, the C++ of every kind of place in its `design.md` D3, the measured facts in
   `study\00` §5 (`verification\v32_*`, 154 programs), what stayed marked in `SOMEDAY.md` ("Left marked by
   `m3-mem`").
8. Done: `$ASSERTS`, `_ASSERT`, `$DEBUG`, the data-processing functions, `END n`, `SYSTEM n`, the licence file
   (`m3-asserts-debug-zlib`, all 12 tasks, 2026-10-14). What each task found is in the change's `tasks.md` and
   `design.md`, the measured facts in `study\00` §5 (`verification\v33_*`, 99 programs), what stayed marked in
   `SOMEDAY.md` ("Left marked by `m3-asserts-debug-zlib`").
9. Done: `REDIM _RETAIN` (`m3-redim-retain`, all 9 tasks, 2026-10-14), and `DIVERGENCES.md` D-018 found while
   closing it. What each task found is in the change's `tasks.md`, the C++ in its `design.md` D3 and "As built",
   the measured facts in `study\00` §5 (`verification\v34_*`, 30 programs).
10. Done: `$UNSTABLE:TYPEFIELDS`, the member markers, `_DYNAMIC` member arrays and whole arrays of their types in
    files (`m3-typefields`, all 19 tasks, 2026-10-15). What each task found is in the change's `tasks.md`, how it
    was built in its `design.md` ("As built"), the measured facts in `study\00` §5 (`verification\v35_*`, 78
    programs), what stayed marked in `SOMEDAY.md` ("Left marked by `m3-typefields`"). The arrays used after a
    marked declaration, the third item of the 2026-10-14 decision, are no longer named by the demand report.
11. Done: whole-array assignment (`m3-whole-array-assign`, all 13 tasks, 2026-10-15). What each task found is in
    the change's `tasks.md`, the C++ in its `design.md` D4, the measured facts in `study\00` §5
    (`verification\v36_*`, 58 programs), what stayed marked in `SOMEDAY.md` ("Left marked by
    `m3-whole-array-assign`").
12. Under way: `DECLARE LIBRARY` (user, 2026-10-15; OpenSpec change `m3-declare-library`: groups 1 to 3 done,
    the measurements `verification\v37_*` (facts in `study\00` §5), the blocks and calls in `sema`, the IR and
    the C++; the two divergences D-023 and D-024 are in its `proposal.md` and `design.md` until task 6.1).
    **Next**: group 4, the lookup of the library files and the build in the driver. After the change, the upstream report's first lines are `_SNDOPEN` (6), `_ARRAYCOPY` (4, deferred),
    `$VERSIONINFO` (4), the event handlers and switches (4 each), `_MIDISOUNDBANK` (4) and `LEN` of a whole array
    (4); the plain copy of `internal\c` waits until libqb must change.

## Numbers at the last full runs

From 2026-10-15 (task 5.3 of `m3-whole-array-assign`, release build against the reference clone, each runner run
with `--jobs 8 --build-cache target\build-cache`): tier 1 (at the close of group 4, after the last change to the
compiler; only documents changed after it), the command-line tests that build, the runner's tests, the slice
list with and without folding, the upstream and differential pass lists, the two full suites and the demand
reports, each run once after the last change to the compiler. The call-site check is from
`m3-asserts-debug-zlib` (no row of the built-in table changed since; this change adds none). The rows of the real programs, of the old
compiler and of `--pass-cache` are older (sessions 56, 40 and 54).

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | **431 of 431** (66 of them screen programs, `g12_game` the scripted game; `s119_whole_array` and `s120_whole_array_members`, new with `m3-whole-array-assign`, each passing on its first run); 431 of 431 also with `QB64RUST_NO_FOLD=1` |
| Full corpus (456 programs) | 450 pass (the 431 of the slice list, 19 `.err` programs), none wrong at run time; **none rejected with "not supported yet"**; 1 `.err` program with only marked errors (`218` `FRE`); 5 known failures (`tests\corpus\README.md`) |
| Upstream | **310 of 399** (`tests\upstream\pass.list`: 310, all passing, run by itself; 16 programs left `deferred.list` with `m3-whole-array-assign`, so 399 are counted); the full suite: 404 programs, 310 pass, 93 rejected or with only marked errors (80 at the compile, 13 `.err` programs), 1 known failure, none wrong |
| Call-site check | 1677 of 1677 programs equal (`tools\callsite\README.md`; 46 of them for `m3-asserts-debug-zlib`: `_ASSERT` 18, the data-processing functions 28) |
| Real programs (`tests\programs`) | Civil War Strategy: **no diagnostic, and it builds** (3.6 s; the old compiler 6.9 s the same day). Its C++ compared once with the old compiler's, line by line: no call differs on 4,332 lines. **Played for two turns from a keys file (`g12_game`, a copy with listed edits): 23 states equal to the old compiler's**, 1.1–1.9 s a run (`tests\programs\README.md`) |
| Differential (`tests\differential\pass.list`) | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 28 only-marked rejections (the eight `.err` programs of whole-array assignment left), 5 clean programs on no pass list (`tests\upstream\README.md`); regenerated at the close |
| Tier 1 (`cargo test`) | 311 tests pass; the 27 command-line tests that build an executable (`--ignored`, run alone) pass too (2 new with `m3-whole-array-assign`: D-004 widened, D-022) |
| The runner's own tests (`python -m unittest discover -s tools/legacy_tests -p "test_*.py"`) | 73 pass, 1 s; those of `tools\testtime` 38 (not run with this change, which does not touch it) |
| `slice.list` while working, with `--pass-cache` | 24 s when no program's executable changed, 92 s when all are run (`--jobs 8 --build-cache`, 2026-10-12) |
| Screen programs (`tests\corpus\screen`, 66) | Old compiler: 66 recorded, each from two equal runs. New compiler: 66 pass; the 12 of `m3-images-fonts-mouse` (`i01`–`i08`, `f01`, `f02`, `u01`, `u02`) each on its first run (`u01` and `u02` were recorded again after the fix of the mouse steps' wait, task 4.1) |
| Demand (`qb64rust --report unsupported`; upstream with `--include-root tests\upstream\root`) | Corpus: **nothing held back** (436 without an `.err`: 434 without errors, and `s77_exeicon` and `g12_game`, whose icons only the runner puts beside them). Upstream (348 without an `.err`): 266 without errors, 71 held back, 11 with other errors (the ten that include the old compiler's sources, and `icon_relative/test`, whose icon lies in the clone); no whole-array assignment is named (the whole arrays it names are `LEN(a())`, 4 programs); first lines: `DECLARE LIBRARY` 9, `_SNDOPEN` 6, `_ARRAYCOPY` 4 |
| Tier 2 as CI runs it, locally | about 12 min at the last full build; about 2 min with `--jobs 8 --build-cache` when little C++ changed |
| Full corpus with the old compiler (2026-10-10, 336 programs) | 331 pass, 5 known failures, `--jobs 8`; the 305 before the screen programs took about 12 min with one job, 3 min 44 s with `--jobs 8` (`tools\legacy_tests\README.md`) |

How to read them: every program of the corpus that the old compiler accepts compiles and, where it can be
compared, passes; its screen programs all pass. The game compiles and builds, and a copy of it played for two
turns shows the same 23 screens as the old compiler's build (stage 3 of `tests\programs\README.md`). Upstream
went from 50 to 114 over step 10 (64 before its member arrays) and is at 140 after group 6 of step 11 (115
after `_DEST` and `$CONSOLE`; all 19 `glut` tests with the window built-ins and `_TRUE`/`_FALSE`; four of the
nine `precomp-flags` tests), at 142 after `m3-sound`, at 148 after `m3-console-files` and at 153 after
`m3-images-fonts-mouse`, at 174 after `m3-mem`, at 186 after `m3-asserts-debug-zlib` (all of 279), at 193 of
286 after `m3-redim-retain`, at 290 of 383 after `m3-typefields` and at 310 of 399 after `m3-whole-array-assign`.
What `deferred.list` still holds needs `_ARRAYCOPY` or, `t001`, a plain scalar beside an array typed by `DIM …
AS`; outside it the report's first lines are `DECLARE LIBRARY`, `_SNDOPEN`, `$VERSIONINFO` and the event
handlers and switches.

## Known bugs

None known. (The two of 2026-10-11, a `+` after another sign and a `DIM` of several dynamic arrays, are fixed:
`tasks.md` of `m2-defxxx-common-arrays`, above its group 7; slice programs `s17_signs` and `s52_dim_list`.)

## Queued for later milestones

| When | Item |
|---|---|
| M3 | Plain copy of `..\QB64pe\internal\c` at the reference clone's commit, `16f629784e` (`DECISIONS.md` 2026-10-02 and 2026-10-09) |
| Any time | A formatter that matches `-y` (out of M2's exit criterion, `DECISIONS.md` 2026-10-09) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
