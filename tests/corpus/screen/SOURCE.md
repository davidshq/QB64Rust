# Source of these programs

Written for QB64Rust by the OpenSpec change `m2-screen-state-check` (2026-10-10): programs with a window, checked
by the state of the screen they leave (`tools\screen\README.md`). Each has an `.output` (empty: a program with a
window prints nothing to the console) and a `.screen`, recorded with `qb64pe.exe` 4.7.0 (`16f629784e`, default
build) using the corpus runner's `--record`; those that read the keyboard have a `.keys`.

The new compiler builds console programs only so far, so none of these is on `slice.list`.

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

## Cut from Civil War Strategy

`g01`–`g06` are written by `tools\screen\cut_game.py`: a few lines of setup and calls, then procedures of
`tests\programs\civil-war-strategy\CWSTRAT.BAS` (MIT, the user's own; `SOURCE.md` there) copied byte for byte. Do
not edit them by hand: change the script and run it again.

| Program | Procedures | Covers |
|---|---|---|
| `g01_flags` | `flags` | `LINE` with `B` and `BF`, `PSET`, `SELECT CASE`, text in `SCREEN 12` |
| `g02_cannon` | `cannon`, `clrrite` | `DRAW` strings, `POINT(0)` and `POINT(1)`, `CIRCLE` with arcs and an aspect, `PAINT` with a border, `VIEW`, `CLS 1` |
| `g03_capitol` | `capitol`, `flags` | The game's end screen without its music: `CIRCLE` arc, `DRAW` with a scale and in loops, `GOTO`, `LOCATE` and `PRINT` over a filled box |
| `g04_ironclad` | `ironclad` | `LINE -(x, y)` chains, `PAINT` with and without a border colour |
| `g05_schooner` | `schooner` | `LINE` with the colour left out, `PAINT` without a colour |
| `g06_message_box` | `image2`, `mxw`, `TICK` | Graphics `GET` and `PUT`, `VIEW`, `CLS 1`; `TICK` waits for a key from the keys file |

Not cut yet: the pictures the game loads with `BLOAD` (`*.VGA`). The runner copies only the `.bas` into a
program's folder, so a program that reads a data file needs that first.
