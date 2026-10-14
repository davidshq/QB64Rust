# Design

## Context

Motivation and scope: `proposal.md`. What the code does today (read 2026-10-14):

- A built-in that is one call of the runtime is a row: `sema\builtins.rs` `SUPPORTED` (functions, with a
  `Rule`) or `STATEMENTS` (with a `StmtRule`). Its slots come from the table entry and its template
  (`qb64rust_builtins::passing::plan`), the parser reads a statement by the same template, and the emitter
  writes the call with the `passed` mask. Templates with parenthesised pairs, `STEP`, `-` between pairs and
  words to choose from are compiled already (`LINE`, `_MOUSECURSOR`, `_WINDOWSIZELIMIT`). A name that is a
  statement and a function (`SCREEN`, `PLAY`, `_DEST`) is told apart by where it stands.
- The slot kinds are `Long`, `Int64`, `Double`, `Float`, `AnyNumeric`, `Str`, `UInt64`, `ULong`, `ColorOrTile`,
  `Block` (a whole array's bytes or from an element to its end) and `Bytes`. The table's slot types of this
  change are LONG, `_UNSIGNED LONG`, DOUBLE, SINGLE and STRING, all mapped already, and one of its own:
  `_UCHARPOS`'s second slot, "offset+size", which `qb64pe.bas` 20903 fills with the bytes of a LONG array.
- `_NEWIMAGE`, `_FREEIMAGE`, `_RGB32` (`Rule::GivenArgs(1)`), `_RGBA32`, `_MOUSESHOW`, `_MOUSEHIDE` and
  `_MOUSECURSOR` are rows already. `_NEWIMAGE`, `_FREEIMAGE` and the three mouse statements are on `NEVER_RUN`.
- A built-in whose table row names a `dependency` records a `RuntimePart` (`check\builtins.rs`
  `runtime_part`); a dependency no build provides is "not supported yet: a part of the runtime that is not
  built yet". `RuntimePart` has `Icon`, `Audio` and `Printer`; `driver\build.rs` `make_flags` writes their
  flags in the old compiler's order. The rows of this change name three more: `DEPENDENCY_IMAGE_CODEC`
  (`_LOADIMAGE`, `_SAVEIMAGE`), `DEPENDENCY_LOADFONT` (`_LOADFONT`, `_UFONTHEIGHT`, `_UPRINTWIDTH`,
  `_ULINESPACING`, `_UPRINTSTRING`, `_UCHARPOS`) and `DEPENDENCY_SCREENIMAGE` (`_SCREENIMAGE`,
  `_CLIPBOARDIMAGE`); `_PRINTIMAGE` names `DEPENDENCY_PRINTER`, which exists.
- The old compiler's make line (`qb64pe.bas` 13706–13721): `DEP_GL`, `DEP_IMAGE_CODEC`, `DEP_CONSOLE_ONLY`,
  `DEP_SOCKETS`, `DEP_PRINTER`, `DEP_ICON`, `DEP_SCREENIMAGE`, `DEP_FONT`, `DEP_DEVICEINPUT`, `DEP_ZLIB`,
  `DEP_EMBED`, `DEP_CONSOLE`, `DEP_ICON_RC`, `DEP_AUDIO_MINIAUDIO`. The clone's `Makefile` names each libqb
  variant's objects by the flags, so new variants stand beside the old ones; the first build of each builds it.
- The screen-state check (`tools\screen\screen_state.cpp`) is compiled against the headers of the libqb it is
  linked with and feeds keys from a thread of its own (`keyboard_keydown`). libqb's `mouse.h` exports
  `Mouse_QueuePositionEvent(x, y, mode)`, `Mouse_QueueButtonDownEvent(button, x, y)`,
  `Mouse_QueueButtonUpEvent(button, x, y)` and `Mouse_QueueScrollEvent(x, y, dx, dy)`, which push onto
  `mouse_event_queue`, a `RingBuffer<mouse_event, 65536, true>`, the thread-safe kind. `_MOUSEX` maps the
  event's window pixel to the screen through `environment_2d__screen_x1` and `…_x_scale`, then to a column in a
  text mode.
- `_LOADFONT` looks a name without a folder up in `%LOCALAPPDATA%\Microsoft\Windows\Fonts` and
  `%SystemRoot%\Fonts` (`parts\video\font\font.cpp` 533).
- A corpus program may have data files in `<name>.files\`, copied beside it for its run (`DECISIONS.md`
  2026-10-13, answer 3).
- `check\constants.rs` names the colour functions among the old evaluator's functions, and every function in a
  `CONST` is "not supported yet" today; this change does not touch it (proposal, "Not in this change").

Constraints: nothing copied from other projects; the reference clone and libqb are not changed, so what an image
holds is libqb's by construction and the work is the call; no divergence without a decision; a behaviour that
looks questionable is implemented QB64pe's way and listed in `SOMEDAY.md`; no font file and no image from
elsewhere goes into the repository.

## Goals / Non-Goals

**Goals:**

- Every name of the proposal compiles in each form the old compiler accepts, with its call, and is rejected where
  the old compiler rejects it.
- Each one a test can run is run by a screen program recorded with `qb64pe.exe`, on `slice.list`, compared by
  its screen state; the mouse through mouse steps of a keys file.
- The libqb parts these names need are linked exactly when the old compiler links them.
- The upstream programs that need nothing else pass and join `pass.list`.

**Non-Goals:**

- A second implementation of anything libqb draws, decodes or renders.
- `_MEM` and the helper functions of the upstream image tests (the next step).
- A built-in function in a `CONST`.
- Checking what a window shows beyond the state file (the cursor's shape, the desktop, a printed page).
- The mouse of a console, game controllers, `_DEVICES`.

## Decisions

### D1. Each built-in is a plain row unless task 1.1 shows the old compiler writes it another way

Task 1.1 compiles one program per form of each template with `qb64pe -z`, outside the repository, as task 1.1 of
`m3-window-output` did (455 forms there): every argument left out in turn, every word, a variable of INTEGER,
LONG, `_INTEGER64`, SINGLE, DOUBLE, `_FLOAT`, `_UNSIGNED LONG` and a literal in one slot of each slot type, and
for each function the type the C++ holds and the type the old compiler believes (the result in `PRINT`, in
`HEX$` and in an operation). Rejected forms are kept one per program (`verification\v31_x*`). The findings
replace "the shape expected" here with one line per form, as D2 of `m3-window-output` did.

The shape expected, from the table and `qb64pe.bas`:

- **Plain rows**, `Rule::Plain` or `StmtRule::Plain`: nearly every name. Words of templates (`_SMOOTH`,
  `_KEEPBACKGROUND`, `_CLOCKWISE`, `_SEAMLESS`, `ON`/`OFF`/`LOCK`/`_CLEAR`, `_NONE`) are compiled like `LINE`'s
  `B`/`BF`.
- **`_UCHARPOS`**: its second slot a whole LONG array passed as its bytes, the existing `Slot::Block` with a
  check of its own ("Expected LONG array-name" for any other argument, `qb64pe.bas` 20908–20917), unless 1.1
  shows the old compiler passes something else.
- **Optional arguments in the middle of a function** (`_RGB(r, g, b[, handle])`, `_RED(c[, handle])`,
  `_LOADIMAGE(f$[, [mode][, req$]])`, `_LOADFONT(f$, size[, [opt$][, index]])`): the table's optional mask, as
  `_NEWIMAGE` and `POINT` have. If a function, like `_RGB32`, has one libqb entry per number of arguments, it
  takes `Rule::GivenArgs`.
- **A statement and a function of one name**: `_CLEARCOLOR`, `_BLEND`, `_PALETTECOLOR`, `_AUTODISPLAY`, `_FONT`,
  `_PRINTMODE`, `_MAPUNICODE`, `_CLIPBOARDIMAGE`: two rows, as `SCREEN` and `PLAY`.

A form that 1.1 shows to need more than a row and a slot kind is put to the user before code (task 1.3), with
the old compiler's code for it; the default is to mark that form "not supported yet" and list it in
`SOMEDAY.md`.

*Alternative considered:* write rows straight from the table and let the call-site check find the differences.
Rejected: `m2-core-builtins` found a third of the functions special-cased, and a measured list of forms is what
the call-site programs and the `check-fail` tests are written from.

### D2. Three runtime parts, in the old compiler's order

`RuntimePart` gains `ImageCodec`, `Font` and `ScreenImage`; `of_dependency` maps `DEPENDENCY_IMAGE_CODEC`,
`DEPENDENCY_LOADFONT` and `DEPENDENCY_SCREENIMAGE` to them; `make_flags` writes `DEP_IMAGE_CODEC=y` before
`DEP_CONSOLE_ONLY=y`, and `DEP_SCREENIMAGE=y` and `DEP_FONT=y` after `DEP_ICON=y`, as `qb64pe.bas` 13707–13713
does; the IR's dump names them. Task 1.2 compares the make line of three builds of the old compiler (one per
part, one with all three and `$CONSOLE:ONLY`) in the clone's `internal\temp\compilelog.txt` with
`make_flags`, as task 1.2 of `m2-console-screenhide` did, and a unit test holds each row.

`_PRINTIMAGE` records the existing `Printer` part. A `$CONSOLE:ONLY` program may load an image
(`console_only/image`): the flags combine, nothing in the build forbids it.

### D3. What no test runs

On `NEVER_RUN`: `_SCREENIMAGE`, `_CLIPBOARDIMAGE` (both forms) and `_PRINTIMAGE` (the desktop, the clipboard, a
page), `_DEPTHBUFFER` (hardware images only), `_MOUSEMOVE` (it moves the machine's cursor), and, still,
`_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR` (the cursor's shape is in no state). `_NEWIMAGE` and `_FREEIMAGE` leave
`NEVER_RUN`: the screen programs of this change run them.

`_MAPTRIANGLE` is one row with two kinds of use: its 2-D form (a software image as the source and the
destination) runs in a screen program; its 3-D form (a destination given as `(x, y, z)` points, drawn into a
hardware image through OpenGL) is compiled and held in that program as code that is not reached. Since
`NEVER_RUN` is a list of names, its coverage is the slice program's run of the 2-D form plus call-site programs
for the 3-D forms; the 3-D form is named in `crates\README.md` as not run.

`_MOUSEHIDDEN` and `_MOUSEDISABLED` run: they read libqb's flags and need no cursor.

### D4. Mouse steps in the keys file

`screen_state.cpp` reads four more steps and takes them in order with the keys, from the same thread:

| Step | Call |
|---|---|
| `mouse <x> <y>` | `Mouse_QueuePositionEvent(x, y, GLUTEnum_MouseCursorMode::Normal)`; the position is kept |
| `press <b>` | `Mouse_QueueButtonDownEvent(b, x, y)` at the kept position, `b` 1 to 3 |
| `release <b>` | `Mouse_QueueButtonUpEvent(b, x, y)` |
| `wheel <n>` | `Mouse_QueueScrollEvent(x, y, 0, -n)`, `n` steps, negative up (the sign checked against `_MOUSEWHEEL` in task 4.1) |

The coordinates are pixels of the window's content, which `_MOUSEX` maps through libqb's scale; for the window
a test program opens, with no `_FULLSCREEN` and no resize, the scale is 1 and a pixel of the window is a pixel of
the screen (checked by the first test program of the steps). A mouse step goes in at once: the queue holds 65536
events and drops none a test can fill, so there is no waiting as for keys. Malformed steps (an unknown button,
a missing number) end the program with exit code 125 and a message, as a bad key does. `tools\screen\README.md`
documents the steps; the runner needs no change, since it hands over the keys file as it is.

A test program empties the queue (`WHILE _MOUSEINPUT: WEND`) before it announces its wait, and the keys file
puts its mouse steps after that `prompt`. A real cursor moving over the window between those moments would add
events (on the user's machine the window is shown); that risk is accepted and written in the README.

*Alternative considered:* call `GLUT_MOUSE_POSITION_FUNC` and `GLUT_MOUSE_BUTTON_FUNC`, the window's own
callbacks. Rejected: they also read the window's state (focus, the cursor mode) and convert by GLFW's scale, more
than a test needs, while the `Mouse_Queue…` entries are what those callbacks end in.

### D5. Image files for the tests are written by this project

A script `tools\screen\make_images.py` (Python, standard library only) writes the image files the screen
programs load: a small BMP of 8 and of 24 bits and a PNG with an alpha channel, each a pattern simple enough to
see in a state's picture. Its output is committed in the programs' `<name>.files\` folders and the script is
run again only when it changes. A second source of files is the programs themselves: `_SAVEIMAGE` writes a file
that `_LOADIMAGE` reads back in the same run, for the formats libqb writes (BMP, PNG, and those 1.1 finds).

*Alternative considered:* copy the upstream test images from the clone. Rejected: `tests\upstream` copies text
files only, by design, and their provenance and licence are not recorded here.

### D6. Fonts are Windows' own

The screen programs load `cour.ttf` (Courier New) by its bare name, and `arial.ttf` where a proportional font is
needed; libqb finds them in `%SystemRoot%\Fonts`. Nothing goes into the repository. Both builds run on one
machine, so they see one font file. The built-in fonts (`_FONT 8`, `14`, `16`) need no file and are tested in a
program of their own, apart from the loaded ones, so a difference between machines shows in one program only.

### D7. Upstream

Task 6.1 runs the upstream programs that name the family: those that then compile cleanly and pass join
`pass.list`; `image/svg_test` needs `.svg` files that `tests\upstream` does not copy, so if it fails for that
reason only, it goes on `tests\known_clean_not_passing.list` with that reason. Upstream image tests that need
`_MEM` stay held back by it.

### D8. Order of work

Group 1 measures (the forms, the make lines, the facts the specs' scenarios name). Groups 2 to 5 then do not
depend on one another but for the test files: 2 the colour functions and the image functions and statements, 3
the fonts, 4 the mouse with the screen-state steps, 5 the names no test runs. Group 6 is upstream, the lists and
the closing runs. Each of groups 2 to 5 ends with fmt, clippy and tier 1 green and its tests and lists in place,
so the change can stop after any group.

## Risks / Trade-offs

- **The fonts of the GitHub runner may differ from the user's machine** (another Windows version, another
  `cour.ttf`): the program with loaded fonts then fails in the `tier2` job while passing here. Mitigation: that
  program is the only one with loaded fonts; if the job fails on it, it goes on a list run locally and not by
  the job, the fallback of `m3-sound`'s design.
- **A real cursor over a test window** adds mouse events (D4). Mitigation: the queue is emptied right before the
  wait, and the steps follow at once; when a mouse program fails, its state is read first, and an event at a position no
step named is taken as such an event, not as the compiler's fault.
- **A new libqb variant takes a build** (image codecs, FreeType): the first slice run after this change builds
  three or more variants in the clone. Machine time once, then cached by the clone's `Makefile`.
- **Hardware images and `_DEPTHBUFFER` are compiled but never compared**: a wrong call there shows only in the
  call-site check, which compares the text of the call.
- **`_LOADIMAGE`'s decoders accept many formats**; this change runs three. What the other decoders do is libqb's
  and needs no compiler work, so the call is what is tested.

## Open Questions

- Whether a form of a template needs more than a row and a slot kind (D1): answered by task 1.1, put to the
  user in task 1.3 if any does.
