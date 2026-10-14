# Proposal

## Why

This is step 6 of M3's order of work (user, `DECISIONS.md` 2026-10-14, last row): the image built-ins that run,
the colour functions, fonts and the mouse. It is the last open feature item of M3 in `ROADMAP.md`. Every
corpus program the old compiler accepts compiles now, so no corpus program waits on these names. Upstream, 20
programs name them; the demand report (`qb64rust --report unsupported`, 2026-10-14) counts `_LOADIMAGE` in 15,
`_WIDTH` in 15, `_HEIGHT` in 15, `_RED32` in 14, `_PUTIMAGE` in 13, `_PIXELSIZE`, `_PALETTECOLOR` and `_SAVEIMAGE`
in 12 each. In none of them is an image built-in the only blocker. Ten share a helper that compares images
through `_MEMIMAGE`, `MKL$`, `_INFLATE$` and `_BASE64DECODE$`, and `_MEM` is the next step, not this one (answer
1 of that row). After this change about four upstream programs pass (`image/test`, `console_only/image`,
`colors/test` and perhaps `image/svg_test`), and the step is tested mainly by programs of its own.

Read before this proposal (the table `tools\builtins\builtins.json`, `qb64pe.bas`, libqb's sources at the
reference clone's commit):

- **About 60 names are new**, nearly all one table row each: a statement or function with slots and a template,
  some names both. `_NEWIMAGE`, `_FREEIMAGE`, `_RGB32`, `_RGBA32`, `_MOUSESHOW`, `_MOUSEHIDE` and `_MOUSECURSOR`
  are compiled already.
- **`qb64pe.bas` special-cases one of them**: the second argument of `_UCHARPOS` must be a LONG array, passed as
  its bytes (`qb64pe.bas` 20903). The others are written from their rows; task 1.1 measures each.
- **Three libqb parts are new to the build**: `DEP_IMAGE_CODEC=y` (`_LOADIMAGE`, `_SAVEIMAGE`), `DEP_FONT=y`
  (`_LOADFONT` and the `_U…` font functions) and `DEP_SCREENIMAGE=y` (`_SCREENIMAGE`, `_CLIPBOARDIMAGE`). Each
  is a flag of the old compiler's make line (`qb64pe.bas` 13707, 13712, 13713).
- **The mouse can run under test**: libqb exports `Mouse_QueuePositionEvent`, `Mouse_QueueButtonDownEvent`,
  `Mouse_QueueButtonUpEvent` and `Mouse_QueueScrollEvent`, which push onto the queue `_MOUSEINPUT` reads
  (`libqb\src\mouse.cpp`). Its ring buffer is the thread-safe kind, so the key thread of the screen-state check
  can push events.
- **A font needs no file in the repository**: `_LOADFONT` looks a bare name up in the user's and the system's
  font folders (`parts\video\font\font.cpp` 533), so `_LOADFONT("cour.ttf", 16)` loads Windows' Courier New on
  any Windows machine, the GitHub runner included.

Four things were the user's to decide (2026-10-14, `DECISIONS.md`): `_MEM` stays out; the mouse is tested
through the screen-state check; tests load Windows' own fonts; the whole family comes in, and a name whose
result depends on the machine is compiled and never run.

## What Changes

- **Images**: `_LOADIMAGE`, `_COPYIMAGE`, `_SAVEIMAGE`, `_PUTIMAGE` (every form of its template), `_WIDTH`,
  `_HEIGHT`, `_PIXELSIZE`, `_CLEARCOLOR` (statement and function), `_SETALPHA`, `_BLEND`, `_DONTBLEND`,
  `_BLEND` (function), `_PALETTECOLOR` (statement and function), `_COPYPALETTE`, `_AUTODISPLAY` (statement and
  function), `PMAP`, the 2-D form of `_MAPTRIANGLE`. `_NEWIMAGE` and `_FREEIMAGE` now run in tests.
- **Colours**: `_RGB`, `_RGBA`, `_RED`, `_GREEN`, `_BLUE`, `_ALPHA`, `_RED32`, `_GREEN32`, `_BLUE32`,
  `_ALPHA32`, `_HSB32`, `_HSBA32`, `_HUE32`, `_SATURATION32`, `_BRIGHTNESS32`.
- **Fonts and text in graphics**: `_LOADFONT`, `_FONT` (statement and function), `_FREEFONT`, `_FONTWIDTH`,
  `_FONTHEIGHT`, `_PRINTSTRING`, `_PRINTWIDTH`, `_PRINTMODE` (statement and function), `_UFONTHEIGHT`,
  `_UPRINTWIDTH`, `_ULINESPACING`, `_UPRINTSTRING`, `_UCHARPOS` (its LONG array argument), `_MAPUNICODE`
  (statement and function).
- **The mouse**: `_MOUSEINPUT`, `_MOUSEX`, `_MOUSEY`, `_MOUSEBUTTON`, `_MOUSEWHEEL`, `_MOUSEMOVEMENTX`,
  `_MOUSEMOVEMENTY`, `_MOUSEHIDDEN`, `_MOUSEDISABLED`, `_MOUSEMOVE`.
- **Compiled and never run** (each compared by the call-site check and held in a slice program as code that is
  not reached): `_SCREENIMAGE` and `_CLIPBOARDIMAGE` (they read the desktop and the clipboard), `_PRINTIMAGE`
  (it prints a page), the 3-D form of `_MAPTRIANGLE` and `_DEPTHBUFFER` (hardware images, drawn by OpenGL
  outside the state the check reads), `_MOUSEMOVE` (it moves the machine's cursor), and the mouse statements
  compiled before (`_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR`).
- **The build** links libqb's image codecs, fonts and screen capture when a program names a built-in that needs
  them, with the old compiler's make flags in its order.
- **The screen-state check** gains mouse steps in the keys file: a position, a button pressed and released, the
  wheel. `screen_state.cpp` hands each to libqb's queue.
- **Tests and lists**: `verification\v31_*` (the measured forms, accepted and rejected); a call-site program per
  form and argument type; `typed` and `ir` front-end tests; screen programs recorded with `qb64pe.exe` on
  `tests\corpus\slice.list`, some with image files in `<name>.files\` and keys files with mouse steps; the
  upstream programs that pass on `tests\upstream\pass.list`; the shrink-only lists regenerated.

## Capabilities

### New Capabilities

- `language/images`: image handles and what is drawn between them: loading, copying, saving and freeing an
  image, `_PUTIMAGE` and `_MAPTRIANGLE`, the size and depth of an image, transparency and blending, an image's
  palette, `_AUTODISPLAY`, `PMAP`, and the colour functions that build and take apart a colour of an image or a
  32-bit colour. Every behaviour is the old compiler's program's.
- `language/fonts`: fonts and text drawn in graphics: loading and freeing a font, choosing one for an image, the
  width and height of a font and of a text, `_PRINTSTRING`, `_UPRINTSTRING` and the other Unicode font functions,
  `_PRINTMODE`, `_MAPUNICODE`.
- `language/mouse`: reading the mouse in a program with a window: the queue `_MOUSEINPUT` reads, the position,
  the buttons, the wheel, the movement, whether the cursor is hidden or disabled; `_MOUSEMOVE`.

### Modified Capabilities

- `language/builtin-functions`: the new functions join the list of supported functions.
- `language/builtin-statements`: the new statements join the list of supported statements.
- `testing/screen-state`: mouse steps in the keys file; every built-in of the three new capabilities used by a
  screen program, as for `language/screen-output`.
- `testing/compiler-tests`: the built-ins that no test runs gain the names that read the desktop, the clipboard
  or OpenGL, print a page or move the cursor.

## Impact

- Changed: `crates\sema` (`builtins.rs`: rows, `NEVER_RUN`; `check\builtins.rs`: `_UCHARPOS`'s array, any slot
  kind task 1.1 shows; `lib.rs`: three `RuntimePart`s), `crates\ir` (the runtime parts, `dump.rs`),
  `crates\codegen-cpp` only where a measured form is not a plain call, `crates\driver` (`build.rs`: make flags),
  `crates\syntax` only where a measured form does not parse, `tools\screen\screen_state.cpp` and its README
  (mouse steps), `tools\legacy_tests` only if a new kind of data file needs it; new `tests\callsite\*.bas`,
  `tests\frontend\*.bas`, `tests\corpus\slice\*` and `tests\corpus\screen\*`, `verification\v31_*`;
  `tests\corpus\slice.list`, `tests\upstream\pass.list`, the shrink-only lists.
- No change to libqb or the reference clone. No font or third-party image goes into the repository: the image
  files of the tests are written by this project (by a program of the old compiler or by a script).
- Not in this change: `_MEM` and `_MEMIMAGE`, `MKL$`, `_INFLATE$`, `_BASE64DECODE$` (the next step); a built-in
  function in a `CONST` (the colour functions there need the old compiler's constant evaluator for functions,
  which the maths functions need too, `const/math_funcs`); the mouse of a console (`_MOUSEX` when the source is
  the console page); `_RESIZE…`, `_SCALEDWIDTH`, `_SCALEDHEIGHT` and the other window functions; `_KEYHIT` and
  the other keyboard functions; `_DEVICES` and game controllers; hardware images beyond compiling their forms.
- `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `tools\screen\README.md`, `study\00`
  §5, `SOMEDAY.md` (what stays marked; questionable behaviours), `DIVERGENCES-QB45.md` where needed, and
  `CLAUDE.md`'s layout table updated when done.
