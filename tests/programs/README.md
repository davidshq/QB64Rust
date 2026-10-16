# Real programs

Whole programs written by people for their own use, kept as targets between the golden corpus (small console
programs) and the old compiler's own source (`qb64pe.bas`, the M4 exit criterion). Decided 2026-10-09
(`DECISIONS.md`, `study\28` §4).

| Folder | Program | Why it is here |
|---|---|---|
| `civil-war-strategy\` | W.R. Hutsell's Civil War Strategy (MIT, `SOURCE.md` there) | A QuickBASIC 4.5 game moved to QB64pe: `SCREEN 12` drawing (`LINE`, `PAINT`, `DRAW`, `PSET`, `CIRCLE`, graphics `GET`/`PUT`, `VIEW`), `BLOAD` with `DEF SEG`, sound, files, `DEFINT`, `COMMON`, `GOSUB`, `DATA`. The kind of program most QB64 users have |

## How they are used

A real program waits for keys, seeds from the clock and plays sound, so it is not a recorded-output test as it
stands. Three stages (`study\28` §4):

1. **Tier 1** (`cargo test`, `crates\driver\tests\inputs.rs`): the front end reads every `.bas` here without a
   panic and prints its tree back byte for byte (set `programs/`), and since `m3-sound` with **no diagnostic at
   all**, the icon its `$EXEICON` names found, its IR valid and its C++ emitted
   (`real_programs_have_no_diagnostics`).
2. **Done for the build:** "compiles, and the C++ builds" (`civil_war_strategy_builds` in
   `crates\driver\tests\cli.rs`, `#[ignore]`d: it needs the clone), and small non-interactive programs cut from
   the game, one per kind of drawing, each ending by itself and checked by the screen-state dump (`study\28`
   §3.1). The cut programs are new files with their own recordings; the copies here stay untouched.
3. **Done:** a scripted game with a fixed seed (`m3-scripted-game`): `tests\corpus\screen\g12_game` is a copy
   of the game with a few listed edits, written by `tools\screen\cut_game.py` (a fixed seed, no `_FULLSCREEN`,
   an empty `SUB TICK`, and a line before each of its 14 waits for a key that announces the wait). Its keys file
   plays two turns from a new game, and 23 states of its screen are compared with the old compiler's
   (`tools\screen\README.md`, "Prompts"). The game here is still never run by a test.

## Measured (2026-10-09)

- `qb64pe.exe -x CWSTRAT.BAS` (the reference clone, `16f629784e`) builds the game in 14 s.
- `qb64rust --dump typed CWSTRAT.BAS`: 100 errors (the cap), all "not supported yet"; the first are `DEFINT` and
  `COMMON`. Without the cap (`qb64rust --report unsupported tests\programs`, 2026-10-10): 1,740 marked errors in
  38 messages; run it again to see what the game needs now.
- 48 distinct built-ins of the built-in table are used, 6 of them compiled so far (`LEN`, `INT`, `ASC`, `ABS`,
  `INSTR`, `SQR`). The most used of the rest: `LINE` 899, `COLOR` 190, `LOCATE` 145, `PAINT` 89, `RND` 73, `DRAW`
  71, `PSET` 50.

## Measured (2026-10-10, after `m2-builtin-statements`)

- Still no real error; still the cap of 100 errors, all "not supported yet".
- Compiled now, by lines of `CWSTRAT.BAS` that use them: `RND` 71, `INPUT` 28, `CLOSE` 18, `OPEN` 15,
  `_FILEEXISTS` 10, `WRITE` 8, `DATA` 7, `MID$` 4, `SWAP` 3, `TIMER` 3, `NAME` 2, and `RANDOMIZE`, `READ`,
  `RESTORE`, `EOF`, `SHELL` once each (console `INPUT` only under `$CONSOLE:ONLY`, which the game does not have).
- Needed next: `COMMON` (12 lines; `DEFINT` and arrays in procedures compile since groups 2 and 6 of
  `m2-defxxx-common-arrays`), labels named like a SUB or like `CLS`
  (step 11 of `STATUS.md`); then the screen: `LINE` 658, `COLOR` 182, `LOCATE` 144, `PAINT` 86, `DRAW` 62, `PSET`
  49, `CLS` 36, `PLAY` 32, `CIRCLE` 28, `SOUND` 28, `INKEY$` 25, `TAB` 22, `PUT` 19, `VIEW` 11, `SCREEN` 9, `GET`
  8, `DEF SEG` 8, `BLOAD` 4.

## Measured (2026-10-10, `m2-screen-state-check`)

- Stage 2 has its first cut programs: `tests\corpus\screen\g01`–`g06` (flags, the cannon, the end screen, the two
  ships, the message box), written by `tools\screen\cut_game.py` from the game's own procedures and recorded with
  the old compiler. The new compiler compiles none of them yet (`DEFINT`, then the drawing statements).
- Not cut yet: anything that loads a `*.VGA` picture with `BLOAD`. The corpus runner copies only the `.bas` into
  a program's folder; data files beside a program come first.

## Building and starting the game

Build it outside the repository, from a copy of its folder (the game writes its save files beside itself):

```
xcopy /e /i tests\programs\civil-war-strategy %TEMP%\cws
target\release\qb64rust.exe -x %TEMP%\cws\CWSTRAT.BAS -o %TEMP%\cws\CWSTRAT.exe
%TEMP%\cws\CWSTRAT.exe
```

`..\QB64pe\qb64pe.exe -x <copy>\CWSTRAT.BAS -o <copy>\CWSTRAT.exe` builds the old compiler's from a second copy,
to look and listen side by side. No test starts the game (`DECISIONS.md` 2026-10-13).

## Adding a program

The licence must allow the copy (state it in a `SOURCE.md` with the repository and commit). Copy the files byte for
byte, mark the folder `-text` in `.gitattributes`, check that `qb64pe.exe` builds it, and run `cargo test`.

## Measured (2026-10-11, after group 7 of `m2-defxxx-common-arrays`)

- Still no real error, and from here on none is hidden: the game's first marked declaration (`COMMON`) is
  gone, so the follow-on rule no longer drops real errors after it. `DEFINT`, the arrays in its procedures and
  its 12 `COMMON SHARED` lines compile (47 arrays listed there and given their bounds by the `DIM SHARED` lines
  below).
- `qb64rust --report unsupported tests\programs`: 1,709 marked errors in 33 messages. Needed next: labels named
  like a SUB or like `CLS` (17 marks), `DEF SEG` (8), `TAB` (22); then the screen (M3): `LINE` 899, `COLOR` 190,
  `LOCATE` 145, `PAINT` 89, `DRAW` 71, `POINT` 34, `SOUND` 33, `PLAY` 31, `PSET` 31, `CIRCLE` 28, `CLS` 26,
  `INKEY$` 25, `PUT` 19, `VIEW` 11, `SCREEN` 9, `GET` 8, `BLOAD` 4.

## Measured (2026-10-12, after `m3-window-output`)

- Still no real error. `qb64rust --report unsupported tests\programs`: **135 marked errors in 15 messages**
  (1,709 in 33 before): the game's window, its text-screen and drawing statements, `INKEY$`, `POINT` and its ten
  lines that start `CLS :` compile.
- What it still needs, in order of uses: `SOUND` 33, `PLAY` 31, `TAB` 22, graphics `PUT` 19, `DEF SEG` 8, graphics
  `GET` 8, labels named like a SUB (`cannon`, `ships`, `topbar`: 7 marks), `BLOAD` 4, and `BEEP`, `_FULLSCREEN`
  and the metacommand `$EXEICON` once each. That is the second step proposed for M3 (`STATUS.md`, "Next").
- Of the six programs cut from it, four pass with the new compiler and are on `tests\corpus\slice.list` (`g01`
  flags, `g02` cannon, `g04` ironclad, `g05` schooner); `g03` waits on `PLAY`, `g06` on graphics `GET` and `PUT`.

## Measured (2026-10-13, after `m3-get-put-bload`)

- Still no real error. `qb64rust --report unsupported tests\programs`: **66 marked errors in 4 messages** (135 in
  15 before): `SOUND` 33, `PLAY` 31, `BEEP` 1 and the metacommand `$EXEICON` 1. Everything else the game has
  compiles: its 22 `TAB` lines, graphics `PUT` (19) and `GET` (8), `DEF SEG` (8) and `BLOAD` (4) with `VARSEG`
  and `VARPTR`, `_FULLSCREEN`, `DATE$`, and its lines that are a SUB's name and a colon.
- `topbar:` in its main module is a **call** of `SUB topbar` (no jump names the word); `ships:` and `cannon:` in
  `SUB navy` are **labels** (`GOTO ships`, `GOTO cannon`) beside `CALL ships` and `CALL cannon`. That is the old
  compiler's rule (`study\00` §5).
- `mtn`, `graphic` and `graft`, the arrays its pictures are loaded into, lie in conventional memory; `mtn` is
  listed in `COMMON` and so a dynamic array to the old compiler.
- Eleven programs are cut from it now (`tests\corpus\screen\g01`–`g11`, `tools\screen\cut_game.py`); ten pass
  with the new compiler and are on `tests\corpus\slice.list`, among them the ones that load its `*.VGA` pictures
  (the mountains, the icon, the faces and the forts) and its city report. `g03_capitol` waits on `PLAY`.
- What it needs next is sound, step 3 of M3's order; then stage 2's "compiles, and the C++ builds" can be tried
  on the whole game.

## Measured (2026-10-13, after `m3-sound`)

- `qb64rust --report unsupported tests\programs`: 1 program, 1 without errors. The last four messages are gone
  (`SOUND` 33, `PLAY` 31, `BEEP` 1, `$EXEICON` 1); `-w` shows no warning either.
- **The game builds.** `qb64rust -x CWSTRAT.BAS` on its first try: 3.6 s with libqb's objects built (the front
  end and the C++ of the program; about 7 s as the test, which copies the sources first), against 6.9 s for
  `qb64pe.exe -x` on the same machine the same day (14 s on 2026-10-09). The executable holds the first image
  of `cws.ico`.
- **Its C++ compared with the old compiler's, line by line, once** (a script outside the repository on the
  normaliser of `tools\callsite`; `qb64pe -z` 5.3 s, `qb64rust -z` 0.4 s): of the 4,332 source lines either
  compiler writes C++ for, 1,619 are equal token for token, 1,475 have equal libqb and procedure calls and
  differ in the statement's frame or an explicit cast, 1,159 differ only in parentheses, and 79 differ
  otherwise. None of the 79 is a call with another routine or argument: 48 are the numbering of a temporary
  on a line that has two (`pass1` against `pass2` in a one-line `IF`), 17 are declarations the two compilers
  write in different places (`DIM` of a static array, the copy of a string argument), 11 are the old
  compiler's `CHAIN` data for `COMMON` lines, which the new compiler does not write (`CHAIN` is not compiled),
  and the rest are one each: the end of the main module after the last `RETURN`, `"Raze \077"` for `"Raze ?"`
  (the same bytes), a clean-up after `ON ERROR GOTO 0`. What this comparison does not see: arithmetic between
  the calls on the 2,634 lines that are not equal token for token (the slice and differential programs cover
  it).
- **Started once by hand**, each build from a copy of the folder outside the repository, left ten seconds and
  closed: neither ended by itself, each had one window titled "Civil War Strategy Game" and no error box. Not
  compared: what the screens show and what is heard (stage 3).
- All eleven programs cut from it pass now (`g03_capitol` joined `tests\corpus\slice.list`: its `PLAY` lines
  are compiled and not reached).
- What comes next is stage 3, a scripted game with a fixed seed whose screens are compared.

## Measured (2026-10-14, `m3-scripted-game`)

- **Stage 3 is done, and the new compiler's build passes on its first run.** `g12_game` (the copy, with its data
  files and a `CWS.CFG` with no sound in `g12_game.files\`) plays as the Union from March to July 1861: recruit,
  two moves, a ship, a railroad move, three reports, Fortify, the Utility menu, a save, END TURN twice (the
  computer's moves, a battle each turn, the month's events), Files, Quit, Yes. 23 states, 2.5 MB, recorded from
  two equal runs of `qb64pe.exe`'s build; a run takes 1.1–1.9 s.
- What the script does not reach, and why the game as it stands cannot be scripted: the change's `tasks.md`
  (task 4.1) and `study\00` §5 ("Civil War Strategy played from a keys file"); a longer script is in
  `SOMEDAY.md`.
