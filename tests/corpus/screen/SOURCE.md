# Source of these programs

Written for QB64Rust by the OpenSpec change `m2-screen-state-check` (2026-10-10): programs with a window, checked
by the state of the screen they leave (`tools\screen\README.md`). Each has an `.output` (empty: a program with a
window prints nothing to the console) and a `.screen`, recorded with `qb64pe.exe` 4.7.0 (`16f629784e`, default
build) using the corpus runner's `--record`; those that read the keyboard have a `.keys`.

The `d…` programs were added by the OpenSpec change `m3-window-output` (2026-10-12), recorded the same way. The
programs the new compiler passes are on `tests\corpus\slice.list` (`tests\corpus\README.md`).

## The method's own programs

| Program | Covers |
|---|---|
| `c01_text_cells` | Text cells and attributes of `SCREEN 0`, bytes the state file escapes, the text cursor left on |
| `c02_screen12_shapes` | `LINE` with `BF`, `CIRCLE`, `PALETTE`, text in a graphics mode |
| `c03_pages` | Three pages of `SCREEN 7`, `PCOPY`, the displayed page different from the written one |
| `c04_image_dest` | `_DEST` and `_SOURCE` of an image that is no page: the image gets a record of its own |
| `c05_inkey_keys` | Keys from the keys file into `INKEY$`: characters with special keys between them |
| `c06_inkey_many` | 600 characters and Enter: more than the `INKEY$` buffer holds, none lost |
| `c07_input_window` | `INPUT` and `LINE INPUT` in a window |
| `c08_end_prompt`, `c09_off_the_end` | `END` and running off the end: the prompt in the bottom row, ended by `repeat space` |
| `c10_runtime_error` | An unhandled error: the state as it was when the program stopped |
| `c11_screenhide` | `$SCREENHIDE`: no window, the pages all the same |
| `m00_width_80_50`, `m01`–`m13`, `m32_newimage_32`, `m256_newimage_256` | One program per screen mode: what a page of that mode holds |

## Drawing and text statements by form

One program per statement family, with the forms the programs above leave out (design D4 of `m3-window-output`;
the forms and their calls are in its D2). A program whose last statement would hide the others' work is split, so
that every statement leaves a mark in the recorded state.

| Program | Covers |
|---|---|
| `d01_print_window` | `PRINT` in a window: `;`, `,` (zones of 14 columns), a `PRINT` without a line end, `WRITE`, a line longer than the screen is wide, an item that does not fit the rest of its line, 27 lines that scroll |
| `d02_color_locate` | `COLOR` and `LOCATE` in `SCREEN 0` with each argument left out, a blinking colour, the cursor on, off and with a shape |
| `d03_cls_view_print` | `CLS`, `CLS , colour`, `CLS 2`; `VIEW PRINT` that scrolls, is cleared alone and is taken off |
| `d04_width` | `WIDTH` with the columns alone and with the rows alone (40 by 50) |
| `d05_color_cls_graphics` | `SCREEN 7`: the colour of `COLOR` in `LINE` and `PSET` without one, a background colour, `CLS , colour`, `CLS 1` and `CLS` in a graphics viewport, `CLS 2` in a text viewport |
| `d06_pset_line` | `PSET` and `PRESET` with `STEP` and without a colour; `LINE` with `B`, `BF`, a style, each argument left out, without its first point, `STEP` on either point, blanks around the `-`, `b` in lower case, points outside the screen |
| `d07_circle` | `CIRCLE`: the colour left out, arcs, negative angles (lines to the centre), aspects below and above 1, `STEP`, a radius of 0, a circle half outside |
| `d08_paint` | `PAINT`: a fill and a border colour, each left out, `STEP`, a tile string (a literal, a variable, an expression), a background string |
| `d09_view_window` | `VIEW` and `VIEW SCREEN` with a fill and a border, `WINDOW` and `WINDOW SCREEN`, both taken off, a `WINDOW` inside a `VIEW` |
| `d10_palette_pages` | `SCREEN` with its page arguments (mode left out too), `PCOPY`, `PALETTE` with and without arguments |
| `d11_draw` | `DRAW` with a literal, a variable and an expression: moves, a scale, an angle, a paint command |
| `d12_numeric_types` | Arguments of every numeric type in a coordinate, a colour, a radius, a row, a page and a screen mode: floats that round half to even, an `_INTEGER64` colour beyond 32 bits |
| `d13_functions` | `CSRLIN`, `POS`, `SCREEN(row, column[, 1])` and `POINT` in both forms, printed and in arithmetic; `POINT` outside the screen and under a `WINDOW` |
| `d14_errors` | The error number of 20 statements that are wrong for their screen, under a handler (a mode that does not exist, drawing in `SCREEN 0`, `LOCATE` outside the screen); two of them raise nothing |
| `d15_pacing` | `_LIMIT` in a loop, `_DELAY`, `_DISPLAY`, `SLEEP` ended by a key of the keys file |
| `d16_screen_13` | `SCREEN 13`: `CLS method, colour`, a colour above 15, `PRESET` |
| `d17_tab_window` | (`m3-get-put-bload`) `TAB`, `SPC` and `POS` in a text window: a column, the next line when the cursor is past it, a zone after `TAB`, a column past the width, `TAB` and `SPC` as functions |
| `d18_get_put` | (`m3-get-put-bload`) Graphics `GET` and `PUT` in `SCREEN 12`: no action and each action word over a background, a colour, `_CLIP` at two edges, `STEP`; the array whole and from an element, of two dimensions, of LONG, dynamic, and in a SUB a parameter, a local and a `STATIC` array; error 5 for a `PUT` that leaves the screen and a `GET` into an array that is too small |

`d15_pacing` empties the key buffer before its `SLEEP`: the keys file gives the next key only when the buffer is
empty, and a key ends `SLEEP` only when it comes while `SLEEP` waits. Its `SLEEP 2` is the longest it can take
(when the key comes between the two), not what it takes.

## Cut from Civil War Strategy

`g01`–`g11` are written by `tools\screen\cut_game.py`: a few lines of setup and calls, then procedures of
`tests\programs\civil-war-strategy\CWSTRAT.BAS` (MIT, the user's own; `SOURCE.md` there) copied byte for byte. Do
not edit them by hand: change the script and run it again.

The game's map and its reports read the whole state of a game and cannot be cut as procedures. Of those, a range
of the game's lines stands among the written ones, byte for byte too (the script names each range by its first
and last line and stops when the first line has changed). The pictures a program loads are copies of the game's
`*.VGA` files in its `<name>.files\` folder, made by the script; the corpus runner puts them beside the program
when it runs (`tests\corpus\README.md`).

| Program | Procedures | Covers |
|---|---|---|
| `g01_flags` | `flags` | `LINE` with `B` and `BF`, `PSET`, `SELECT CASE`, text in `SCREEN 12` |
| `g02_cannon` | `cannon`, `clrrite` | `DRAW` strings, `POINT(0)` and `POINT(1)`, `CIRCLE` with arcs and an aspect, `PAINT` with a border, `VIEW`, `CLS 1` |
| `g03_capitol` | `capitol`, `flags` | The game's end screen without its music: `CIRCLE` arc, `DRAW` with a scale and in loops, `GOTO`, `LOCATE` and `PRINT` over a filled box |
| `g04_ironclad` | `ironclad` | `LINE -(x, y)` chains, `PAINT` with and without a border colour |
| `g05_schooner` | `schooner` | `LINE` with the colour left out, `PAINT` without a colour |
| `g06_message_box` | `image2`, `mxw`, `TICK` | Graphics `GET` and `PUT`, `VIEW`, `CLS 1`; `TICK` waits for a key from the keys file |
| `g07_mountains` | lines 139–141 (the main module's `DEF SEG`, `BLOAD`, `DEF SEG`) and 4145–4160 (the start of `SUB usa`); `MTN.VGA` | `BLOAD` into an array in conventional memory, `VARSEG` and `VARPTR`, `PUT` with `PSET` eight times, `_FILEEXISTS` |
| `g08_icon` | lines 169–175 (the main module); `CWSICON.VGA` | `BLOAD`, `PUT`, and a `GET` out of the picture just drawn (the capital marker), which a written line then puts in a row |
| `g09_faces_forts` | lines 1008–1010 (`SUB battle`) and 4817–4819 (`SUB menu`), in a written loop; `FACE1`–`FACE5.VGA`, `FORT0`–`FORT2.VGA` | `BLOAD` with the file's name in a string, eight pictures one after the other into the same two arrays |
| `g10_snapshot` | `snapshot` | `GET` and `PUT` through a SUB on a shared array: the screen under a place taken and put back |
| `g11_city_report` | lines 3553–3571 (`SUB report` from its label `cityrep` on), over values written in the program | `TAB` in `PRINT` in `SCREEN 12`, after `LOCATE` and with numbers |

Each state was looked at as a picture before it was trusted (`tools\screen\state_to_png.py`). Not cut: anything
with sound (`g03_capitol` runs without its music and still names `PLAY`).
