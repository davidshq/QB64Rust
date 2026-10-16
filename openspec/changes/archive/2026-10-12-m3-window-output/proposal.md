# Proposal

## Why

M2 is complete and M3 is next (`STATUS.md`, `ROADMAP.md`); no order of work is set for it. The compiler still
rejects every program whose output goes to the window: a program with neither `$CONSOLE` nor `$CONSOLE:ONLY` gets
one "not supported yet" mark, and no statement that writes to the screen is compiled. That holds back all 29
screen programs of `tests\corpus\screen` (recorded with the old compiler since `m2-screen-state-check`, none
compiled by the new one) and Civil War Strategy, the real-program target of M3 (`LINE` 899 uses, `COLOR` 190,
`LOCATE` 145).

Measured 2026-10-12 with `qb64rust --report unsupported tests\corpus\screen`: the 29 programs wait on the
program-level mark (29), then `SCREEN` (23 programs), `LINE` (20), `PSET` (17), `COLOR` (6), `LOCATE` (6), `CIRCLE`
(5), `PAINT` (3), `CLS` (3), `INKEY$` (3), `DRAW` (2), `VIEW` (2), `_LIMIT` (2), and `POINT`, `PALETTE`, `PCOPY`,
`WIDTH`, `PLAY`, graphics `GET` and `PUT` (1 each). All but `PLAY`, `GET` and `PUT` are rows of the built-in table
with a template, the kind of built-in that comes in as a row with call-site programs (`DECISIONS.md` 2026-10-10).

This change is the first step of M3. The order proposed for the milestone, for the user to accept or change
(task 1.5; nothing after this change depends on it):

1. **This change**: a program with a window as its output, the text-screen statements and the first drawing
   statements, checked by the screen state.
2. Graphics `GET`/`PUT`, `DEF SEG` with `BLOAD`, data files beside a corpus program, `TAB`/`SPC`, labels named like
   a SUB: what Civil War Strategy needs to build; more programs cut from it.
3. Sound (`PLAY`, `SOUND`, `BEEP`: compiled, and what of them can be checked).
4. The console programs of the corpus that still wait (`PRINT USING`, random and binary files).
5. The plain copy of `internal\c`, when libqb must change (`DECISIONS.md` 2026-10-10).

## What Changes

- **A program without `$CONSOLE` compiles and builds**: a window and no console, as the old compiler builds it.
  The mark "programs without `$CONSOLE` or `$CONSOLE:ONLY`" goes. `PRINT`, `WRITE`, `INPUT`, `LINE INPUT`, `END`
  and a runtime error in such a program are the calls they are today; what they do in the window is checked by
  screen programs.
- **Text-screen statements**, each a row: `SCREEN`, `CLS`, `COLOR`, `LOCATE`, `WIDTH`, `VIEW PRINT`.
- **Drawing statements**, each a row: `PSET`, `PRESET`, `LINE`, `CIRCLE`, `PAINT`, `DRAW`, `VIEW`, `WINDOW`,
  `PALETTE`, `PCOPY`.
- **Pacing statements**: `_LIMIT`, `_DELAY`, `SLEEP`, `_DISPLAY`.
- **Functions**: `INKEY$`, `CSRLIN`, `POS`, `POINT`, `SCREEN`.
- **Every form of each template** the old compiler accepts, with the call the old compiler writes for it; a form
  it rejects is an error of the program. Where the old compiler writes a statement by hand in a way no existing
  rule covers, the statement gets a rule of its own or stays marked, said in the design after task 1.1.
- **Screen programs are run with the new compiler**: the programs of `tests\corpus\screen` that pass join
  `tests\corpus\slice.list` and are checked in tier 2 by their screen state; new screen programs are recorded
  with `qb64pe.exe` for the built-ins the 29 do not use.
- **Tests and lists**: `verification\v27_*` measurements; call-site programs for every form; front-end tests; the
  shrink-only lists regenerated; `tests\known_clean_not_passing.list` gains the programs that now compile and
  have no recorded screen (with the reason).

## Capabilities

### New Capabilities

- `language/screen-output`: what a program writes to its window and reads from it: text output in a program
  without a console, the screen modes, the text-screen and drawing statements, the functions that read the screen
  and the keyboard, and the pacing statements, each as the old compiler's program behaves, checked by the state of
  the screen.

### Modified Capabilities

- `language/console-and-window`: a program with neither `$CONSOLE` nor `$CONSOLE:ONLY` is compiled (a window, no
  console) instead of being reported "not supported yet".
- `language/builtin-statements`: the list of supported statements grows.
- `language/builtin-functions`: the list of supported functions grows; `CSRLIN` and `POS` are no longer named as
  unsupported.
- `testing/screen-state`: the new compiler's screen programs are part of tier 2.

## Impact

- Changed: `crates\sema` (`check\mod.rs`: the mark; `builtins.rs`: rows, slot types `SINGLE` and `_UNSIGNED LONG`
  for statements, rules where task 1.1 finds one is needed; `check\builtins.rs`), `crates\codegen-cpp`
  (`builtins.rs`: the conversions of the new slot types), `crates\driver` (`build.rs`: the flags of a program
  without a console, if they differ), `tests\corpus\slice.list`, `tests\known_clean_not_passing.list`, the
  shrink-only lists; new `tests\callsite\*.bas`, `tests\frontend\*.bas`, `tests\corpus\screen\*`,
  `verification\v27_*`.
- Tier 2 gets programs that open a window (about 0.3 s each, shown on the desktop unless hidden) and runs longer;
  the `tier2` job of `rust.yml` runs them on GitHub's Windows runner for the first time.
- No change to libqb, the reference clone or `tools\screen\screen_state.cpp` is planned; the libqb copy is not part
  of this change. (One change to `screen_state.cpp` was made after all, decided by the user on 2026-10-12: the
  four palette entries of `SCREEN 10` that follow the clock are left out of the state, `DECISIONS.md`.)
- Not in this change: graphics `GET` and `PUT`, `PALETTE USING`, `DEF SEG`, `BLOAD`/`BSAVE`, `PEEK`/`POKE`,
  `PLAY`, `SOUND`, `BEEP`, `PRINT USING`, `TAB`, `SPC`, `LPRINT`, `_PRINTSTRING`, the image built-ins beyond the
  two compiled already (`_LOADIMAGE`, `_PUTIMAGE`, `_RGB32`, …), fonts, the mouse, `_KEYHIT` and `_KEYDOWN`,
  `ON KEY`/`ON TIMER`, the `_CONSOLE` statement; Civil War Strategy as a whole.
- `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `tests\programs\README.md`,
  `study\00` §5, `SOMEDAY.md` (what stays marked) and `DECISIONS.md` (the order of M3; anything decided on the
  way) updated when done.
