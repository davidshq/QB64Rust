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

1. **Now, tier 1** (`cargo test`, `crates\driver\tests\inputs.rs`, set `programs/`): the front end reads every
   `.bas` here without a panic, prints its tree back byte for byte, and reports no error other than "not supported
   yet". A program the old compiler accepts must never get a real error.
2. **When features land:** "compiles, and the C++ builds", then small non-interactive programs cut from the game,
   one per kind of drawing, each ending by itself and checked by the screen-state dump (`study\28` §3.1). The cut
   programs are new files with their own recordings; the copies here stay untouched.
3. **Later:** a scripted game with a fixed seed.

## Measured (2026-10-09)

- `qb64pe.exe -x CWSTRAT.BAS` (the reference clone, `16f629784e`) builds the game in 14 s.
- `qb64rust --dump typed CWSTRAT.BAS`: 100 errors (the cap), all "not supported yet"; the first are `DEFINT` and
  `COMMON`.
- 48 distinct built-ins of the built-in table are used, 6 of them compiled so far (`LEN`, `INT`, `ASC`, `ABS`,
  `INSTR`, `SQR`). The most used of the rest: `LINE` 899, `COLOR` 190, `LOCATE` 145, `PAINT` 89, `RND` 73, `DRAW`
  71, `PSET` 50.

## Measured (2026-10-10, after `m2-builtin-statements`)

- Still no real error; still the cap of 100 errors, all "not supported yet".
- Compiled now, by lines of `CWSTRAT.BAS` that use them: `RND` 71, `INPUT` 28, `CLOSE` 18, `OPEN` 15,
  `_FILEEXISTS` 10, `WRITE` 8, `DATA` 7, `MID$` 4, `SWAP` 3, `TIMER` 3, `NAME` 2, and `RANDOMIZE`, `READ`,
  `RESTORE`, `EOF`, `SHELL` once each (console `INPUT` only under `$CONSOLE:ONLY`, which the game does not have).
- Needed next: `DEFINT` and `COMMON` (12 lines), arrays in procedures, labels named like a SUB or like `CLS`
  (step 11 of `STATUS.md`); then the screen: `LINE` 658, `COLOR` 182, `LOCATE` 144, `PAINT` 86, `DRAW` 62, `PSET`
  49, `CLS` 36, `PLAY` 32, `CIRCLE` 28, `SOUND` 28, `INKEY$` 25, `TAB` 22, `PUT` 19, `VIEW` 11, `SCREEN` 9, `GET`
  8, `DEF SEG` 8, `BLOAD` 4.

## Adding a program

The licence must allow the copy (state it in a `SOURCE.md` with the repository and commit). Copy the files byte for
byte, mark the folder `-text` in `.gitattributes`, check that `qb64pe.exe` builds it, and run `cargo test`.
