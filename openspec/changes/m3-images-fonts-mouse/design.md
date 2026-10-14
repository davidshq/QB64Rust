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
are the table below, one line per built-in and its forms, as D2 of `m3-window-output` has.

A form that 1.1 shows to need more than a row and a slot kind is put to the user before code (task 1.3), with
the old compiler's code for it; the default is to mark that form "not supported yet" and list it in
`SOMEDAY.md`.

**As measured (task 1.1, 2026-10-14).** 225 accepted forms compiled with `qb64pe -z` outside the repository
(every argument left out in turn, every word, a variable of INTEGER, LONG, `_INTEGER64`, SINGLE, DOUBLE, `_FLOAT`,
`_UNSIGNED LONG` and a literal in one slot of each slot type, each function's result in `PRINT`, `HEX$` and an
operation, each statement after `CALL`), kept in `verification\v31_x01_accepted`; 108 rejected forms, one program
each (`v31_x02`–`x109`); the values in `v31_a`–`v31_d` (`study\00` §5, "The image, colour, font and mouse
built-ins"). **Every built-in is the plain call of its table row and template**, `_UCHARPOS` too: its array is
the existing `Slot::Block`, with the old compiler's check of its own. No form needs more than a row and a slot
kind. The slot types are passed as in `m3-window-output` D2: a float into LONG or `_UNSIGNED LONG` is `qbr(…)`,
anything into SINGLE or DOUBLE as it is. A function's optional slots given are a bit mask in a last argument (`0|1|2|4…`),
a statement's in a number; `N` stands for `NULL`.

| Built-in | Forms measured: call | Kind |
|---|---|---|
| `_RGB`, `_RGBA` | `func__rgb(r,g,b,N,0)`; with a handle `…,h,0\|1`; `_RGBA` the same with five slots; two or six arguments rejected | `Plain` function, `_UNSIGNED LONG` |
| `_RED`, `_GREEN`, `_BLUE`, `_ALPHA` | `func__red(c,N,0)`, with a handle `(c,h,0\|1)`; both slots `_UNSIGNED LONG`; bare, `()` and three arguments rejected | `Plain` function, LONG |
| `_RED32` … `_ALPHA32` | `func__red32(c)`, one `_UNSIGNED LONG` slot | `Plain` function, LONG |
| `_HSB32`, `_HSBA32` | `func__hsb32(h,s,b)`, `func__hsba32(h,s,b,a)`, DOUBLE slots | `Plain` function, `_UNSIGNED LONG` |
| `_HUE32`, `_SATURATION32`, `_BRIGHTNESS32` | `func__hue32(c)`, `func__sat32`, `func__bri32` | `Plain` function, DOUBLE |
| `_LOADIMAGE` | `func__loadimage(f,N,N,0)`; mode `…,m,N,0\|1`; `, , req$` `…,N,r,0\|2`; both `0\|1\|2`; bare, `()`, four arguments, a number name, a string mode rejected | `Plain` function, LONG; part `ImageCodec` |
| `_COPYIMAGE` | `func__copyimage(h,N,0)`, `(h,m,0\|1)`; bare and three arguments rejected | `Plain` function, LONG |
| `_SAVEIMAGE` | `sub__saveimage(f,N,N,0)`; `, h` 1; `, , fmt$` 2; both 3; bare, a number name, four arguments rejected | `Plain` statement; part `ImageCodec` |
| `_WIDTH`, `_HEIGHT`, `_PIXELSIZE` | `func__width(N,0)`, `(h,0\|1)`; `()` and two arguments rejected | `Plain` function, LONG |
| `_PUTIMAGE` | `sub__putimage(dx1,dy1,dx2,dy2,src,dst,sx1,sy1,sx2,sy2,mask)`: first pair 1, its `STEP` 2, second pair 4, its `STEP` 16, source 8, destination 32, source pairs 64 and 512 with `STEP` 256 and 1024 (`, img, 0, STEP(1, 1)-STEP(3, 3)` is 1896), `_SMOOTH` 128; every part may be left out, bare `_PUTIMAGE` is mask 0; `(1, 2)-`, three numbers in a pair, `_SMOOTH` without its comma, five commas rejected | `Plain` statement, slots DOUBLE and LONG |
| `_MAPTRIANGLE` | `sub__maptriangle(dir,sx1,sy1,sx2,sy2,sx3,sy3,src,dx1,dy1,dz1,dx2,dy2,dz2,dx3,dy3,dz3,dst,smooth,mask)`: `_SEAMLESS` 1, source 2, each z 4, 8, 16, destination 32; `_CLOCKWISE` 1 and `_ANTICLOCKWISE` 2 in the first slot; `_SMOOTH` 1, `_SMOOTHSHRUNK` 2, `_SMOOTHSTRETCHED` 3 in the last slot (no bit); the source may be left out; two points, `STEP`, a z in the source, no `TO`, `_SEAMLESS` before `_CLOCKWISE` rejected. The 3-D form is a z on a destination point (D3) | `Plain` statement, slots SINGLE and LONG |
| `_DEPTHBUFFER` | `sub__depthbuffer(w,N,0)`, `ON` 1, `OFF` 2, `LOCK` 3, `_CLEAR` 4; with a handle mask 1; bare and a number rejected | `Plain` statement |
| `_CLEARCOLOR` (statement) | `sub__clearcolor(c,N,2)`; `, h` adds 4; `_NONE` `(N,N,1)`; `_NONE, h` 5; bare `(N,N,0)` and `_NONE 4` (3) accepted | `Plain` statement, slot 1 `_UNSIGNED LONG` |
| `_CLEARCOLOR` (function) | `func__clearcolor(N,0)`, `(h,0\|1)`; two arguments rejected | `Plain` function, LONG |
| `_SETALPHA` | `sub__setalpha(a,c1,c2,h,mask)`: `c1` 1, `TO c2` 4, handle 2; `128, , h` 2; bare, `128 TO 9`, `128, TO 9`, a string rejected | `Plain` statement, `_UNSIGNED LONG` slots |
| `_BLEND`, `_DONTBLEND` | `sub__blend(N,0)`, `(h,1)`; two arguments rejected | `Plain` statement |
| `_BLEND` (function) | `func__blend(N,0)`, `(h,0\|1)` | `Plain` function, LONG |
| `_PALETTECOLOR` (statement) | `sub__palettecolor(n,c,N,0)`, `(n,c,h,1)`; one argument rejected | `Plain` statement, slot 2 `_UNSIGNED LONG` |
| `_PALETTECOLOR` (function) | `func__palettecolor(n,N,0)`, `(n,h,0\|1)`; bare and three rejected | `Plain` function, LONG |
| `_COPYPALETTE` | `sub__copypalette(N,N,0)`; source 1, destination 2; `, h` 2; three rejected | `Plain` statement |
| `_AUTODISPLAY` | `sub__autodisplay()`; with an argument "SUB does not require any arguments"; function `func__autodisplay()` | `Plain` statement and function, LONG |
| `PMAP` | `func_pmap(c,n)`, SINGLE and LONG; one or three arguments rejected | `Plain` function, SINGLE |
| `_SCREENIMAGE` | `func__screenimage(N,N,N,N,0)`, `(x1,y1,x2,y2,0\|1\|2\|4\|8)`; two arguments and `()` rejected | `Plain` function, LONG; part `ScreenImage`; never run |
| `_CLIPBOARDIMAGE` | function `func__clipboardimage()`; statement `_CLIPBOARDIMAGE = h` `sub__clipboardimage(h)`; without `=`, and the function with an argument, rejected | `Plain`, both; part `ScreenImage`; never run |
| `_PRINTIMAGE` | `sub__printimage(h)`; bare and two rejected | `Plain` statement; part `Printer`; never run |
| `_NEWIMAGE`, `_FREEIMAGE` | as before: `func__newimage(w,h,m,0\|1)`, `(w,h,N,0)`; `sub__freeimage(h,1)`, `(N,0)` | `Plain`, rows already |
| `_LOADFONT` | `func__loadfont(f,size,N,N,0)`; options 0\|1, index 0\|2, both 0\|1\|2; one or five arguments, a number name rejected | `Plain` function, LONG; part `Font` |
| `_FONT` (statement) | `sub__font(f,N,0)`, `(f,h,1)`; bare and three rejected | `Plain` statement |
| `_FONT` (function), `_FONTWIDTH`, `_FONTHEIGHT` | `func__font(N,0)`, `(h,0\|1)`; two rejected | `Plain` function, LONG |
| `_FREEFONT` | `sub__freefont(f)`; bare and two rejected | `Plain` statement |
| `_PRINTSTRING` | `sub__printstring(x,y,s,N,0)`; `STEP` 1, handle 2; no pair, no text, a number text, `-` rejected | `Plain` statement, DOUBLE slots |
| `_PRINTWIDTH` | `func__printwidth(s,N,0)`, `(s,h,0\|1)`; bare and a number rejected | `Plain` function, LONG |
| `_PRINTMODE` (statement) | `sub__printmode(w,N,0)`, `_FILLBACKGROUND` 1, `_KEEPBACKGROUND` 2, `_ONLYBACKGROUND` 3; handle mask 1; bare and a number rejected | `Plain` statement |
| `_PRINTMODE` (function) | `func__printmode(N,0)`, `(h,0\|1)` | `Plain` function, LONG |
| `_UFONTHEIGHT`, `_ULINESPACING` | `func__UFontHeight(N,0)`, `(f,0\|1)` | `Plain` function, LONG; part `Font` |
| `_UPRINTWIDTH` | `func__UPrintWidth(s,N,N,0)`; encoding 0\|1, font 0\|2 | `Plain` function, LONG; part `Font` |
| `_UPRINTSTRING` | `sub__UPrintString(x,y,s,N,N,N,N,0)`; width 1, encoding 2, font 4, image 8; `STEP`, no text, eight arguments rejected | `Plain` statement, LONG slots; part `Font` |
| `_UCHARPOS` | `func__UCharPos(s,N,N,N,0)`; the array `byte_element(…)` 0\|1 (`Slot::Block`: from the first element of a whole array, its `LBOUND`, or from the element given, to the end), encoding 0\|2, font 0\|4; "Expected LONG array-name" for anything but an array or element of LONG or `_UNSIGNED LONG` (`qb64pe.bas` 20901) | `Plain` function, LONG, with the check of its own; part `Font` |
| `_MAPUNICODE` (statement) | `sub__mapunicode(u,c)`, `u TO c`; a comma or one argument rejected | `Plain` statement |
| `_MAPUNICODE` (function) | `func__mapunicode(c)`; bare and two rejected | `Plain` function, LONG |
| `_MOUSEINPUT`, `_MOUSEHIDDEN`, `_MOUSEDISABLED` | `func__mouseinput()`, no slot; with an argument rejected; as a statement "Syntax error" | `Plain` function, LONG |
| `_MOUSEX`, `_MOUSEY`, `_MOUSEMOVEMENTX`, `_MOUSEMOVEMENTY` | `func__mousex()`, no slot | `Plain` function, DOUBLE |
| `_MOUSEBUTTON` | `func__mousebutton(n)`, one LONG slot; bare and two rejected | `Plain` function, LONG |
| `_MOUSEWHEEL` | `func__mousewheel(N,0)`, `(a,0\|1)`; two rejected | `Plain` function, DOUBLE |
| `_MOUSEMOVE` | `sub__mousemove(x,y)`, DOUBLE slots; one argument, a string rejected | `Plain` statement; never run |

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

**As measured (task 1.2, 2026-10-14).** Six programs built with `qb64pe -x` outside the repository, the defines
and the libqb variant read from the clone's `internal\temp\compilelog.txt` (the make line itself is kept nowhere
on Windows, as `m2-console-screenhide` found):

| Program | Defines of `qbx.cpp` | libqb variant, link |
|---|---|---|
| `PRINT 1` | `NO_SOCKETS NO_PRINTER NO_ICON NO_SCREENIMAGE` | `libqb_make_00000000.o`, `-mwindows`, the font stub `font.o` |
| `$CONSOLE:ONLY` | `CONSOLE_ONLY` and the four `NO_` | `00100000`, `-mconsole`, `font.o` |
| `_LOADIMAGE` | `IMAGE_CODEC` and the four `NO_` | `01000000`, `font.o` |
| `_LOADFONT` | the four `NO_` | `00000000`, FreeType's objects instead of `font.o` |
| `_SCREENIMAGE` | `IMAGE_CODEC`, `SCREENIMAGE`, three `NO_` | `01000010`, `font.o` |
| all three, `$CONSOLE:ONLY` | `IMAGE_CODEC CONSOLE_ONLY SCREENIMAGE`, three `NO_` | `01100010`, `-mconsole`, FreeType |

The old compiler sets `DEP_SCREENIMAGE=y` alone for `_SCREENIMAGE` (`qb64pe.bas` 13712); the clone's `Makefile`
then also builds the image codec into the variant (its line 339: `DEP_SCREENIMAGE` or `DEP_IMAGE_CODEC`).
`DEP_FONT=y` has no define: it links FreeType's objects in place of the font stub (`Makefile` 395). So the
mapping stands as written: `make_flags` passes the three flags in the old compiler's order and the `Makefile`
does the rest.

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

**As measured (task 1.2, 2026-10-14)**, with a first version of the steps (task 4.1) and a program of the old
compiler that prints every event to its console: in `SCREEN 12` a step's pixel is the screen's pixel (`mouse 40
30` gives `_MOUSEX` 40, `_MOUSEY` 30); in `SCREEN 0` it is a column and row (40, 30 is 6, 2; 100, 50 is 13, 4).
**A `mouse` step is two events** (libqb pushes the move, then a copy with the movement cleared), a `press` or
`release` one, a `wheel` two (the steps, then 0). `press 1` gives `_MOUSEBUTTON(1)` -1, `press 2`
`_MOUSEBUTTON(2)` -1. `wheel 1` gives `_MOUSEWHEEL` 1 and `wheel -2` -2, so the sign above is right. **A real
cursor over the shown window does reach the queue**: before its first `prompt` the program read 30 events of
the user's mouse, and the movement of the first `mouse` step (`_MOUSEMOVEMENTX`) is counted from the real
cursor's last position (28, -384 in that run). Between two `mouse` steps the movement is the steps' own (40, 30
then 45, 33 gives 5, 3). So a test program prints no movement of a first step, and empties the queue right before
its `prompt`.

**The queue has one writer by design** (`ring-buffer.h`: "single-producer single-consumer", and the variant that
overwrites when full is "NOT safe for general concurrent use"); `Mouse_QueuePositionEvent` also sets
`last_gui_pushed` and `g_lastRawMouse…` with no lock. The steps' thread is a second writer beside the window's
callbacks, so a real event at the moment of a step can lose or overwrite one of the two. The window's position
and scroll callbacks drop events while the window has no focus or the cursor is outside it (`mouse.cpp` 658–661,
646–649), and a real click needs the cursor over the window, so the race needs the real cursor inside a focused
test window: the risk below, not a second one.

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
- **A real cursor over a test window** adds mouse events (D4), and an event at the moment of a step can lose or
  overwrite one (the queue has one writer by design). Mitigation: the queue is emptied right before the wait, and
  the steps follow at once; the cursor kept off the window (or another window in focus) while mouse programs run
  takes both away; when a mouse program fails, its state is read first, and an event at a position no step named,
  or a step's event missing, is taken as such an event, not as the compiler's fault.
- **A new libqb variant takes a build** (image codecs, FreeType): the first slice run after this change builds
  three or more variants in the clone. Machine time once, then cached by the clone's `Makefile`.
- **Hardware images and `_DEPTHBUFFER` are compiled but never compared**: a wrong call there shows only in the
  call-site check, which compares the text of the call.
- **`_LOADIMAGE`'s decoders accept many formats**; this change runs three. What the other decoders do is libqb's
  and needs no compiler work, so the call is what is tested.

## Open Questions

- Whether a form of a template needs more than a row and a slot kind (D1): answered by task 1.1, put to the
  user in task 1.3 if any does.
