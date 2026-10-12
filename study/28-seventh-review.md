# 28. Seventh review: are we going astray, what else to consider, how to go faster (2026-10-09)

Asked by the user: look over the codebase and the plans from an expert panel's point of view. Are we going astray
in any major way, is there anything else to consider, can things go faster. No nits. The user then asked two
follow-up questions (what §2 means, and whether Civil War Strategy is a good graphics program) and ordered
`STATUS.md` fixed; the answers are in §2, §4 and §5.

The panel (`CLAUDE.md` rule 6, role-played inline): a pragmatic engineer, a QB64 engineer, a compiler/languages
engineer, a Rust engineer, a test engineer, and a **release engineer** (added because one finding is about which
build of the old compiler the tests are checked against).

Evidence: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md`, `DIVERGENCES.md`, `SOMEDAY.md`, `study\00`, `study\22`,
`study\27`, `crates\README.md`, the IR (`crates\ir\src\lib.rs`), the emitter (`crates\codegen-cpp`), the operator
typing (`crates\sema\src\check\ops.rs`), `rust.yml`, and the measurements stated in each section (release build of
`940a444`).

## 1. Verdict

**Not going astray.** The architecture holds, as the fourth, fifth and sixth reviews also found, and the code read
for this review agrees with its documents. The working method is what keeps code written by smaller models correct:
measure the old compiler before writing a rule, mark the unknown "not supported yet" instead of guessing,
shrink-only lists, exhaustive matches enforced by a lint, "clean programs are listed", and the differential tester.
Keep all of it. No finding about the code rises above a nit.

The findings are about the second half of the plan and about the weight of the process:

| § | Finding | Kind |
|---|---|---|
| 2 | The expected outputs come from one build of QB64pe, CI checks against another | Decision for the user; small today, grows at M3 |
| 3 | The M4 exit criterion is a graphics program; the plan treats graphics as a side item of M3 | Plan |
| 4 | Civil War Strategy is a good graphics target, with two limits | Answer to the user |
| 5 | `STATUS.md` had become a history file; reviews are more frequent than their yield | Process; `STATUS.md` fixed |
| 6 | The formatter is on M2's exit criterion and nothing later needs it | Plan |
| 7 | Smaller points: the numeric model and M6, tier-2 speed, platforms, `DECLARE LIBRARY` | Notes |

## 2. Which build of QB64pe gives the expected outputs

There is one codebase of ours, and this finding is not about it. It is about **the old compiler**, which is the
judge of every test: a test passes when the new compiler's program prints what the old compiler's program printed.
Two different builds of the old compiler are in use as that judge:

| Where | Which QB64pe | Used for |
|---|---|---|
| On the user's machine | The reference clone `..\QB64pe`, at commit `16f629784e` (2026-09-30) | Recording every expected output (`tests\corpus`, `tests\differential`, `verification\`, `baselines\`); local tier 2 links the new compiler's programs with **its** libqb |
| On GitHub (CI) | The downloaded release `v4.7.0-GLFW` (commit `26e0f6c132`, published 2026-09-26) | CI tier 2 links the new compiler's programs with **its** libqb, and compares with the outputs recorded above |

Measured for this review (GitHub's compare of the two commits): the clone is **16 commits ahead** of the release,
and those commits change libqb in three files: `internal\c\libqb.cpp`, `internal\c\libqb\src\keyboard.cpp` and
`internal\c\libqb\src\qbs_val.cpp`. The last one is `VAL`.

What it costs today: little. Two upstream programs (`basic/val_default_large_integer_decimal`,
`val_typed_large_integer_decimal`) pass locally and cannot pass in CI, which is why `STATUS.md` reports "45 of 279,
47 against the clone". Nothing else is known to differ.

Why it is worth settling anyway (release engineer):

- Every future difference between "passes here" and "fails in CI" has to be traced to this before it can be
  trusted as a real bug. Smaller models are bad at that kind of diagnosis.
- M3 copies libqb into this repo "at the pinned commit". If that is the clone's commit, the repo will carry a
  runtime that no released QB64pe has, while CI still downloads 4.7.0 for the old compiler.
- The built-in table and the verification programs describe the clone too.

**Recommendation:** make the release the one reference. The user checks the clone out at the tag `v4.7.0-GLFW`
(the clone is read-only for Claude, `CLAUDE.md` rule 3; its `origin` is the user's fork, which has no tags, so the
tag has to be fetched from `QB64-Phoenix-Edition/QB64pe`), rebuilds `qb64pe.exe`, and the corpus, differential
programs and verification outputs are recorded again (about 11 minutes for the corpus). The expected diff is small
or empty; whatever changes is exactly the set of behaviours that differ between the two builds, which is worth
knowing. The alternative, staying on the clone's commit, is workable only if CI builds QB64pe from that commit
instead of downloading the release. Moving to a newer release later is then one deliberate step: change the tag,
record again, read the diff.

## 3. Graphics are on the critical path to M4

M4 is done when `qb64pe.bas` compiles with the new compiler and the result passes the suite. The old compiler's
source is a text-mode IDE: a graphics and keyboard program. Counted for this review (names of the built-in table
found outside strings and comments, against the names in `crates\sema\src\builtins.rs`):

| Input | Distinct built-ins used | Not supported yet | Most used of those |
|---|---|---|---|
| `qb64pe.bas` with its included files | 154 | 133 | `PCOPY` 380, `SCREEN` 335, `COLOR` 321, `_PRINTSTRING` 217, `_FILEEXISTS` 153, `CVL` 123, `LOCATE` 122, `CLOSE` 95, `OPEN` 91, `GET` 80, `_PALETTECOLOR` 66, `SHELL` 60, `_KEYDOWN` 48, `_LIMIT` 44, `_RGB32` 42 |
| Corpus | 95 | 66 | `CLOSE` 46, `OPEN` 43, `INPUT` 34, `KILL` 25, `READ` 19 |
| Upstream tests | 184 | 162 | `_CAST` 105, `_DEST` 101, `_NEWIMAGE` 70, `_FREEIMAGE` 58, `_ROL`/`_ROR` 52, `_MEM` 45, `_PUTIMAGE` 37, `_RGB32` 29 |

(`PRINT`, `END`, `SYSTEM`, `ERROR`, `LINE`, `INPUT` and a few more appear in the counts because the table also
registers them as names; they are left out of the last column where the statement is already compiled.)

So the corpus, which steps 9 to 11 serve, is the smallest and least typical of the three. Steps 9 to 11 are still
the right next steps: files, `DATA`, `DEFxxx`, `REDIM` and arrays in procedures are needed by everything. But
after them the yardsticks cannot move without programs that draw, and those have no oracle yet: `study\22` §3.2
queued "a screen-state dump at exit" for M3 and nothing has been designed since.

### 3.1 A screen-state oracle that costs little

Both compilers link the same libqb. Once libqb is copied into this repo (M3), add a hook that runs at program exit
when an environment variable names a file, and writes the state a user would see: for each page in use its mode,
size, text cells with attributes or pixels, the palette, the cursor, and which page is the destination and which is
displayed. The old compiler's programs are built against the same patched copy, so the recording comes from the old
compiler's code generation and the comparison from the new one's, through the existing runner. This makes
`qbasic_testcases`, the 47 upstream programs without `$CONSOLE:ONLY`, and cut-down pieces of real programs (§4)
into tests.

Limits to state in its design (test engineer): it sees the final state only, so a program must end by itself; a
program that reads the keyboard needs scripted keys (the runner already feeds key presses for `END`); `RANDOMIZE
TIMER` and `TIMER`-paced loops need a fixed seed or a clock the test can set; sound has no oracle and is checked
only for "the call is made".

**Recommendation:** design this oracle as its own small OpenSpec change straight after step 9, before steps 10 and
11 are finished, and bring the libqb copy forward to go with it. It is the longest lead item on the path to M4 and
it needs no compiler feature to prototype (the old compiler alone can produce the first dumps).

### 3.2 Comparing the generated call, not only the output

For a built-in, the layer where a smaller model slips is the call into libqb: which function, which slot gets which
conversion, which rounding, which `passed` mask. The old compiler will show its answer for any statement (`-z`
writes its C++). A check that compiles a one-statement program with both compilers and compares the libqb call,
normalised for names and spacing, pins that layer **without running anything and without an oracle**, so it works
for drawing, sound and window built-ins today. `codegen-cpp`'s design says spelling and layout are free, so this is
a per-call comparison, never a whole-file one.

With it, most of the roughly 250 remaining plain built-ins become rows: a `Rule`, a generated one-line program per
argument type (as `difftest` generates the operator programs), and the call-site check. The by-hand
`verification\vNN` programs stay for what has semantics of its own (file modes, `INPUT` parsing, `DATA` typing).

## 4. Civil War Strategy as a graphics program

The user proposed `github.com/hutsell-games/civil-war-strategy` (MIT, maintained by the user, already in QB64pe
form). Measured for this review on its `main` branch (the source fetched to a scratch folder, nothing copied into
this repo):

- One source file, `CWSTRAT.BAS`: 5,075 lines, 160 KB, 74 procedures, no included files. Data files beside it
  (`.VGA` images of 3 KB, `.GRD`, `.DAT`, `.INI`, `.CFG`), 3.7 MB of repository in all.
- **The old compiler builds it**: `qb64pe.exe -x` at the reference clone's commit, 14 s. It needs `cws.ico` beside
  the source (`$EXEICON`).
- **The new front end reads it without a false error**: every diagnostic up to the 100-error cap is "not supported
  yet"; the first are `DEFINT` and `COMMON`.
- 48 distinct built-ins, 6 compiled today (`LEN`, `INT`, `ASC`, `ABS`, `INSTR`, `SQR`). The rest: `LINE` 899,
  `COLOR` 190, `LOCATE` 145, `PAINT` 89, `RND` 73, `DRAW` 71, `PSET` 50, `TAB` 37, `CLS` 36, `POINT` 34, `SOUND` 33,
  `PLAY` 31, `CIRCLE` 28, graphics `GET`/`PUT`, `VIEW`, `SCREEN` 12 and 0, files (`OPEN`, `CLOSE`, `WRITE`, `INPUT`,
  `EOF`), `BLOAD` with `DEF SEG`, `VARSEG` and `VARPTR`, `TIMER`, `SWAP`, `SHELL`, `CSRLIN`, `POS`, and five QB64
  names (`_FILEEXISTS`, `_FULLSCREEN`, `_ICON`, `_SMOOTH`, `_TITLE`).
- Language: `DEFINT`, `COMMON`, `DIM SHARED`, 42 `GOSUB`s, 70 `SELECT CASE`s, `DATA`/`READ`/`RESTORE`, `ON ERROR`,
  25 uses of `INKEY$`, `RANDOMIZE TIMER`.

**Verdict: yes, take it, as the real-program target between the corpus and `qb64pe.bas`.**

- It is what most QB64 users actually have: a QuickBASIC 4.5 program with `SCREEN 12` drawing, moved to QB64 with a
  handful of `_` names. No test set here represents that today; the upstream tests lean to new QB64 features and
  the corpus is console-only.
- It is a sixth of the size of `qb64pe.bas`, one file, with no `_MEM` and no 32-bit images, so it is reachable far
  earlier and gives M3 a concrete "done": the game builds and its screens match.
- Its needs line up with the plan: `DEFINT`, `COMMON`, arrays in procedures (step 11), files and `DATA` (step 9),
  then the QB 4.5 drawing statements as the first tranche of M3's graphics built-ins. It tells us which graphics
  built-ins to do first, which nothing else does.
- Licence and ownership are clean, and the user can change it, which §3.1's limits need.

Two limits:

1. **It is not a test as it stands.** It waits for keys, seeds from the clock and plays sound. Use it in three
   stages: (a) "compiles, and the C++ builds", a cheap check that moves as features land and needs no oracle;
   (b) small non-interactive programs cut from it, one per kind of drawing (the map, flags and icons loaded by
   `BLOAD`, `DRAW` strings, `PAINT` fills, the menus), each ending by itself and checked by the screen-state dump;
   (c) later, a scripted game with a fixed seed. Stage (b) is where the value is.
2. **It does not replace `qb64pe.bas`.** It uses none of `PCOPY`, `_PRINTSTRING`, `_KEYDOWN`, the mouse, `_RGB32`,
   `_NEWIMAGE` or `_PUTIMAGE`, which the IDE and the upstream tests need. It covers QB 4.5 graphics; the QB64 image
   layer still needs its own programs.

If accepted: copy the source and data files into the repo under `tests\` with a `SOURCE.md` naming the commit (as
`tests\upstream` does), keep the files byte for byte (`.gitattributes -text`), and add a `DECISIONS.md` row; the
`minimal-changes` branch is a second, plainer input (the port with the fewest QB64 names). Its licence text for the built
executable (`CWSTRAT.exe.license.txt`) is not needed, since no executable is committed.

## 5. `STATUS.md` and the weight of the process

Eight days, 66 commits, six panel reviews before this one, each re-ordering the work; the last three found nothing
structural. `STATUS.md`, which every session reads first, had grown to 386 lines, most of it a per-change history
that repeated the archived `tasks.md` files and `git log`, against its own rule of one line per step (`DECISIONS.md`
2026-10-07). For smaller models that is cost on every session and an invitation to write prose instead of code.

**Done on the user's instruction (2026-10-09):**

- `STATUS.md` now holds the current state only: where we are, open items, the next steps, the numbers of the last
  full runs, known bugs, the queue for later milestones, and a table of where each other kind of information lives.
  About 75 lines.
- Everything it held before is in `study\progress-record.md`, moved byte for byte, as a closed record. A check
  before the move found facts written nowhere else (the timing of the old compiler's progress bar, the count of
  extra mutation runs, the expression-depth measurements, the `#line` path bug, review fixes of the language
  server), so nothing was dropped.
- From now on: what a change did goes into its `tasks.md` and `design.md`; a measured fact into `study\00` §5; a
  decision into `DECISIONS.md`. `STATUS.md` gets a changed line, not a new paragraph.

**Recommended, not done:**

- Hold a panel review at a real unknown, not on a calendar. The next unknowns are the screen-state oracle (§3.1)
  and the libqb copy. A review that finds "nothing structural" three times running is a sign the method works and
  can be trusted between them.
- Let generators write the measurement programs where the shape repeats (§3.2). `m2-numeric-types` needed 477 lines
  of task record and several hand-written `v21_*` groups; the 59 generated differential programs found more per
  line than anything else in that change.
- `study\00` §5 is 580 lines of measured facts in one section. It is the right home, but it should be split by
  topic into files a session can read one of, before step 9 adds file I/O and `INPUT` to it.

## 6. The formatter is not on the path

`ROADMAP.md` says M2 is done when the whole language is checked without a false error **and the formatter matches
`-y`**. Nothing in M3 or M4 needs the formatter, and the extension already formats through the old compiler. It is
a well-bounded job with a perfect oracle (24 format tests plus `-y` on every corpus program), which suits a smaller
model at any time. **Recommendation:** take it out of M2's exit criterion and list it with the editor work "not
tied to a milestone", so that it cannot hold code generation back. `STATUS.md` lists it as step 12 until the user
decides.

## 7. Smaller points

- **The numeric model is language, not ABI (compiler engineer).** The IR is described as ABI-neutral, and for
  storage, calls and errors it is. But every value carries a held type, which is the C type the old compiler's
  expression has, and results depend on it (`SOMEDAY.md` "QB64pe behaviours to review" is a list of such results).
  That is correct. M6 ("the new compiler owns the ABI") should say that the numeric rules are fixed by the
  numeric-semantics spec and the differential programs, and that only the representation of strings, errors and
  threads is free to change. Otherwise a later session will "clean up" arithmetic that programs can observe.
- **Tier 2 can probably run two to four times faster (test engineer).** A program takes about 2 s, nearly all of it
  the C++ compile of `qbx.cpp` with libqb's headers. A precompiled header for those headers is one measurement
  away. CI also runs the lists one program at a time although the runner has `--jobs`.
- **Every recording is a Windows recording.** The console comma zones already differ by platform by decision
  (D-011). Before macOS or Linux CI, the expected-output files need a way to hold a per-platform variant.
- **`DECLARE LIBRARY` and `SUB _GL` are the reason the back end is C++**, and nothing has touched them. A small
  measured slice of `DECLARE LIBRARY` early in M3 would confirm the fragment model carries them (`regsf.txt`, the
  header includes) before the model is relied on further.

## 8. Questions for the user

1. **The reference build (§2):** move the reference clone to the tag `v4.7.0-GLFW` and record again, or keep the
   clone's commit and have CI build it?
2. **Civil War Strategy (§4):** copy it into `tests\` as the real-program graphics target, in the three stages
   described?
3. **The screen-state oracle (§3.1):** design it, with the libqb copy, straight after step 9?
4. **The formatter (§6):** take it out of M2's exit criterion?
5. **The call-site check (§3.2):** adopt it as the way the remaining plain built-ins come in?

Until they are answered the order of work in `STATUS.md` stands unchanged.

## 9. The user's answers (2026-10-09) and what was done

| Question | Answer | Done |
|---|---|---|
| 1. The reference build | **The local clone is the reference**, and CI should use it too; CI is not a concern for now, work is checked locally. (This is §2's alternative, not its recommendation.) | `DECISIONS.md`. `rust.yml` job `tier2` checks QB64pe's sources out at `16f629784e` and takes only the C++ toolchain from the 4.7.0 release download; **written, not yet seen running**. The two upstream programs that need the later `VAL` were run against the clone (both pass) and moved from `tests\known_clean_not_passing.list` to `tests\upstream\pass.list`: 47 of 279. M3's libqb copy is taken at this commit |
| 2. Civil War Strategy | **Yes**, copy it or take parts, as recommended | Copied whole (source, data, images, icon; 23 files, each checked against its git blob hash) to `tests\programs\civil-war-strategy\` at commit `b02d218811`; `tests\programs\README.md`. Whole, because stage (a) and the cut programs both start from the complete file. Tier 1 has a new input set `programs/` (`crates\driver\tests\inputs.rs`): the game goes through the front end with no panic, an exact round trip, no parse gap and no real error. `ALTMAP.BAT` was left out (a DOS batch file the program does not use) |
| 3. The screen-state oracle | The user asked what it is, then: **yes**; instead of Civil War Strategy or in addition, as the panel thinks best | Explained below. **In addition** (panel): the check is the method and the game is a program to apply it to, so neither replaces the other. `DECISIONS.md`; step 9a in `STATUS.md` |
| 4. The formatter | **Yes**, move it later | `DECISIONS.md`; `ROADMAP.md` lists it under "Not tied to a milestone yet" and M2's "done when" no longer names it |
| 5. The call-site check | The user asked for the recommendation | Below: yes, tried first inside step 9; waits for a yes |

### What the screen-state oracle is (question 3)

"Oracle" is the test word for whatever tells you the right answer. Today the right answer for a program is the
text it printed when the old compiler built it: the runner saves that text, and the new compiler's build of the
same program must print the same. That works only for programs that print to the console.

A program that draws prints nothing. Civil War Strategy puts a map on the screen and stops; there is no text to
compare. So today there is no way to tell whether the new compiler built a drawing program correctly, short of a
person looking at it.

The proposal is to make the screen itself the saved answer. A few lines are added to the runtime library (libqb,
which both compilers' programs link): when the program ends, and only when a test asks for it, write to a file what
is on the screen. In a text mode that is each character cell with its colours; in a graphics mode the colour of
every pixel; plus the palette and the cursor position. The old compiler's build of a program writes that file once
and it is kept as the expected result. The new compiler's build writes it again and the two files must be equal.
It is the same compare-with-a-recording test as today, with a picture of the screen in place of printed text.

What it needs: libqb in this repo (so the lines can be added, which is M3's first item anyway), programs that end
by themselves, and for programs like the game either cut-down pieces that draw one thing and stop, or scripted key
presses and a fixed random seed.

**Recommendation, unchanged:** design it right after step 9, together with the libqb copy. Nothing on the way to
compiling a real graphics program can be checked without it, and the first recordings need only the old compiler.

### The call-site check (question 5)

**Recommendation: yes, with a trial first.** It is the cheapest way to bring in the roughly 250 remaining built-ins
correctly, and it is the only check available for drawing, sound and window built-ins until the screen-state file
exists. But the old compiler's C++ may turn out harder to compare than expected (temporaries, cleanup calls,
statement wrappers around the call). So: build it inside step 9 for the first file I/O statements, where the
recorded-output tests also exist and the two checks can be compared. If the normalised call is stable there, it
becomes the standard way a plain built-in comes in (a table row, a generated one-line program per argument type,
the call-site check). If it is not, drop it at the cost of a day and keep measuring by hand.

**Accepted by the user the same day** ("do as you recommend"): the trial is part of step 9 (`DECISIONS.md`,
`STATUS.md`).

## 10. Does the old compiler's IDE cover the rest of the graphics built-ins? (user, 2026-10-09)

Asked after §9: with Civil War Strategy covering QB 4.5 drawing, does `qb64pe.bas` cover the rest of the screen
and graphics built-ins? **No.** Counted as in §3 (names of the built-in table outside strings and comments; the
table has 404 distinct names):

| Input | Distinct built-ins | Not used by the IDE or the game |
|---|---|---|
| `qb64pe.bas` with its includes (the IDE) | 154 | |
| Civil War Strategy | 48, 17 of them not in the IDE | |
| The IDE and the game together | 171 | |
| Upstream tests | 184 | 71: `_CAST` 105, `_ROL`/`_ROR` 52, `_MEM` 45, `_PUTIMAGE` 37, `_MEMFREE` 20, `_MIN`/`_MAX`, `_MEMGET`, `WINDOW`, `PMAP`, the `_SND…` family, `_LOADIMAGE`, `_MEMIMAGE`, `_SOURCE`, the mouse cursor, the `_U…` font functions, colour functions |
| `qbasic_testcases` (in the clone, never copied) | 132 | 52: `OUT` 466, `WAIT` 217, `SLEEP` 212, `POKE` 116, `PALETTE` 114, `BSAVE` 69, `INP` 55, `PEEK` 41, `PRESET`, `WINDOW`, `PMAP`, `_BLEND`/`_DONTBLEND`, `_GLRENDER`, `_SND…` |
| Used by no program we have | about 116 | dialogs, the clipboard, `_MAPTRIANGLE`, `_SAVEIMAGE`, `_COPYIMAGE`, `_SETALPHA`, `_SNDRAW…`, game controllers (`_AXIS`, `_BUTTON…`, `STICK`, `STRIG`), `_DEFLATE`/`_INFLATE`, `_MD5`, the lock-key functions and more |

What each covers:

- **The IDE is a text-mode program in a window.** It uses `SCREEN`, `COLOR`, `LOCATE`, `PCOPY`, `_PRINTSTRING`,
  the palette, fonts, the keyboard and the mouse, files and `SHELL`. It draws almost nothing: of `LINE`, `CIRCLE`,
  `PAINT`, `DRAW`, `PSET`, `POINT` it uses at most `LINE` (17 uses of the word, `LINE INPUT` among them).
- **The game covers QB 4.5 drawing** (`CIRCLE`, `DRAW`, `PAINT`, `PSET`, `POINT`, `BLOAD`, `PLAY`, `SOUND`).
- **Neither covers** the QB64 image layer (`_PUTIMAGE`, `_LOADIMAGE`, `_SOURCE`, blending, alpha), `_MEM`, the
  `_SND…` sound family, coordinate mapping (`WINDOW`, `PMAP`), the emulated DOS hardware that old programs lean on
  (`OUT`, `INP`, `PEEK`, `POKE`, `WAIT`), or OpenGL. The upstream tests and `qbasic_testcases` cover most of these.
- **About 116 names are used by nothing in hand.** For those the only check is a generated one-line program per
  built-in with the call-site check of §3.2, which is one more reason it was accepted.

Two consequences for the plan:

1. **The screen-state check is needed whatever the IDE covers**, because the tests that do cover the image layer
   (upstream's 47 programs without `$CONSOLE:ONLY`, `qbasic_testcases`) draw and print nothing.
2. **The M4 exit criterion does not test the IDE's screen code** (test engineer). "`qb64pe.bas` compiles and the
   result passes the suite" runs the compiled compiler from the command line (`-x`), so the 133 built-ins it needs
   must compile, but its `PCOPY`, `_PRINTSTRING`, keyboard and mouse code is never run by that test. Compiling is
   proven; behaving is not. The call-site check covers those calls at no extra cost (the old compiler's C++ for
   `qb64pe.bas` is at hand, `internal\source`); a scripted IDE session with a screen dump would cover them fully and
   is a later option, not a requirement.
