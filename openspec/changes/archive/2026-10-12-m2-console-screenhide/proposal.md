# Proposal

## Why

The compiler builds only programs with `$CONSOLE:ONLY`: every other program is rejected with "not supported yet".
This is step 11 of `STATUS.md` "Next", the last open item of M2 (`ROADMAP.md`; `study\26` §5, accepted
2026-10-07): a program with `$CONSOLE` that sends its output to the console with `_DEST _CONSOLE` is checked by
its standard output like any other, and needs no screen-state check. 37 upstream programs have plain `$CONSOLE`;
`_DEST` (54 programs) and `$CONSOLE` (34) are the fourth and fifth most common reasons an upstream program is
rejected (`qb64rust --report unsupported`).

Measured 2026-10-12 before this proposal (`verification\v26_*`, 51 programs; the facts in `study\00` §5): the three
features alone let 2 of the 37 pass. The 19 `glut` tests also name window built-ins and `_TRUE`/`_FALSE`, and three
`precomp-flags` tests the precompiler flags in `$IF`. The user decided the wider scope on 2026-10-12 ("include
wider scope"): about 24 of the 37 are then within reach (an upper bound: each must still print what `qb64pe.exe`
prints).

## What Changes

- **`$CONSOLE` without `:ONLY`**: a program with it is compiled and built with a console and a window, as the old
  compiler builds it. A program with neither `$CONSOLE` nor `$CONSOLE:ONLY` stays "not supported yet" (its only
  output is the window; M3).
- **`$SCREENHIDE` and `$SCREENSHOW`**: the window is hidden or shown at the start, the last of the two in the
  file counting.
- **The exact forms of these metacommands**: what the old compiler rejects (`$CONSOLE:OFF`, a word, a comment or
  a colon after the metacommand, a tab before it, the metacommand after a statement and a colon) becomes an error
  of the program. Today each is hidden behind "not supported yet".
- **`_DEST`, `_SOURCE`, `_CONSOLE`, `_ECHO`**: the statements `_DEST handle`, `_SOURCE handle` and `_ECHO text$`,
  the functions `_DEST`, `_SOURCE` and `_CONSOLE`. Run and compared with the old compiler's output.
- **Console `INPUT` and `LINE INPUT` in a `$CONSOLE` program**, as they work under `$CONSOLE:ONLY`.
- **The window built-ins the upstream `glut` tests name**, compiled and never run: the statements `_SCREENHIDE`,
  `_SCREENSHOW`, `_SCREENICON`, `_TITLE`, `_ICON`, `_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR`, `_WINDOWSIZELIMIT`,
  `_FREEIMAGE`; the functions `_SCREENEXISTS`, `_SCREENICON`, `_TITLE$`, `_DESKTOPWIDTH`, `_DESKTOPHEIGHT`,
  `_SCREENX`, `_SCREENY`, `_WINDOWHANDLE`, `_WINDOWHASFOCUS`, `_NEWIMAGE`. Each comes in as a row with a
  call-site program per form (`DECISIONS.md` 2026-10-10). What they do in a window is not tested here.
- **The constants of QB64pe's auto-included file** (`_TRUE`, `_FALSE` and the others of `beforefirstline.bi`
  whose value the `CONST` evaluator can compute): known in every program, without a declaration.
- **The precompiler flags in `$IF`**: `_CONSOLE_`, `_EXPLICIT_`, `_EXPLICITARRAY_`, `_ASSERTS_`, `_DEBUG_`,
  `_SOCKETS_` get the values the old compiler gives them from the whole program.
- **The build follows the program**: the make flags and the two start-up settings the old compiler derives from
  the metacommands and from the built-ins used (console, console only, hidden window, the icon dependency).
- **Tests and lists**: more `verification\v26_x*` measurements; new `tests\corpus\slice\s62…` programs recorded
  with `qb64pe.exe`; call-site programs; front-end tests; `slice.list` and `tests\upstream\pass.list` gain what
  passes; the shrink-only lists are regenerated.

## Capabilities

### New Capabilities

- `language/console-and-window`: how a program chooses a console, a window or both (`$CONSOLE`, `$CONSOLE:ONLY`,
  `$SCREENHIDE`, `$SCREENSHOW`, their accepted and rejected forms), where its output and input go (`_DEST`,
  `_SOURCE`, `_CONSOLE`, `_ECHO`, console `INPUT` in a `$CONSOLE` program), and the window built-ins that are
  compiled without being run yet.

### Modified Capabilities

- `language/builtin-statements`: the list of supported statements grows.
- `language/builtin-functions`: the list of supported functions grows; more functions are callable without
  parentheses.
- `language/constants`: the constants of the auto-included file are known in every program.
- `compiler/pipeline`: the precompiler flags in `$IF` are evaluated; the IR carries the program's console and
  window settings and its build dependencies; the build's make flags follow them.
- `testing/compiler-tests`: the built-in coverage rule says what counts for a built-in that is compiled and never
  run.

## Impact

- Changed: `crates\sema` (`check\mod.rs`: the metacommands and the mark for a program with no console;
  `builtins.rs`: rows; the auto-included constants), `crates\syntax` (`pp.rs` and `parser\meta.rs`: the flags;
  the parser entry point takes the flags' values), `crates\builtins` and `tools\builtins\extract_builtins.py`
  (the value text of each auto-included constant in `builtins.json`), `crates\ir` (program settings, `dump`,
  `validate`), `crates\codegen-cpp` (`decl.rs`: the two globals), `crates\driver` (`build.rs`: make flags and the
  build key; the loop that settles the flags), `tests\corpus\slice.list`, `tests\upstream\pass.list`, the
  shrink-only lists; new `tests\corpus\slice\s62…`, `tests\callsite\*.bas`, `tests\frontend\*.bas`,
  `verification\v26_x*`.
- The build path changes for the first time since the first slice: a program with `$CONSOLE` and no `:ONLY`
  links another variant of libqb (with the window), which the clone's `Makefile` builds once, in its git-ignored
  folders. That first build takes minutes, locally and in the `tier2` job of `rust.yml`.
- Not in this change: any output to the window and its check (M3, `tools\screen`); `_CONSOLE ON`/`OFF`,
  `_CONSOLETITLE`, `_CONSOLEINPUT`, `_CONSOLECURSOR`, `_CONSOLEFONT`; the `_SCREENHIDE` function; `$ASSERTS`,
  `$DEBUG` and the network functions (their flags get a value, the features stay "not supported yet"); the
  auto-included constants whose value needs `CHR$` or `EXP` in a `CONST`; the three functions of
  `afterlastline.bm`; `$COLOR`; sound, images beyond `_NEWIMAGE` and `_FREEIMAGE` as never-run code, `$EMBED`,
  devices (the other nine of the 37 upstream programs).
- `STATUS.md`, `ROADMAP.md` (M2's last box), `crates\README.md`, `tests\corpus\README.md`,
  `tests\upstream\README.md`, `study\00` §5, `DECISIONS.md` (the scope decision; anything decided on the way)
  updated when done.
