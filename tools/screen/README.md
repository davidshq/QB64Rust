# The screen-state check

How a program that draws is tested: when it exits, the state of its screen is written to a file, and that file is
compared with the one recorded from the old compiler's build of the same program. It is to drawing statements what
the recorded output is to `PRINT` (OpenSpec change `m2-screen-state-check`: the measurements are in its
`design.md`; spec `testing/screen-state`).

| File | What |
|---|---|
| `screen_state.cpp` | Linked into a test program; writes the state and gives the keys |
| `state_to_png.py` | Shows a state: a graphics page as a PNG, a text page as its rows. For people; no test uses it |
| `cut_game.py` | Writes the programs cut from Civil War Strategy into `tests\corpus\screen\`: procedures of the game, or ranges of its lines, behind a few written lines, and the pictures a program loads into its `<name>.files\` folder |
| `make_images.py` | Writes the image files `tests\corpus\screen\i02_load_images` loads (an 8-bit and a 24-bit BMP, a PNG with alpha) into its `.files\` folder; standard library only, run again only when it changes (design D5 of `m3-images-fonts-mouse`) |

## How it gets into a program

`screen_state.cpp` is one C++ source file of this repo. It is put on the link line of a program:

```
qb64pe   -x prog.bas -o prog.exe -f:ExtraLinkerFlags=<repo>/tools/screen/screen_state.cpp
qb64rust -x prog.bas -o prog.exe -f:ExtraLinkerFlags=<repo>/tools/screen/screen_state.cpp
```

Both compilers pass the value to the link (`CXXLIBS_EXTRA` of the `Makefile`), where the C++ compiler compiles it
with the headers of the libqb it is linked with. Neither libqb nor the reference clone is changed. The path must
hold no blank. A build takes about 0.9 s longer.

A program built this way behaves as before unless one of two environment variables is set:

| Variable | Effect |
|---|---|
| `QB64_SCREEN_STATE=<file>` | At exit (`SYSTEM`, `END`, off the end, the window closed, a runtime error) the state is written to the file, after the states the keys file's `state` steps wrote |
| `QB64_SCREEN_KEYS=<file>` | The keys of the file are given to the program as if typed |
| `QB64_SCREEN_PROMPT_MS=<n>` | How long a `prompt` step waits (default 30000); for a test of the time limit |

The corpus runner does all of this for a program with a `<name>.screen` file (`tools\legacy_tests\README.md`).

## The state file

```
screen-state 1
pages 3 display 1 write 0 read 0
page 0 mode 7 graphics 320x200 bits 4 font 8 cursor 1,2 off color 15 background 0
palette FF000000 FF0000AA FF00AA00 … 00000000*240
p 00*320
= 9
p 00*10 04*91 00*219
…
page 1 mode 7 …
end
```

- Line 2: the number of screen pages; the page displayed, written to and read from, or `image` when the
  destination or source is an image that is no page.
- One record per valid page, touched or not (`SCREEN 0` has eight), then one for such an image (`image write`,
  `image read` or `image write read`).
- The header of a record: the mode as libqb numbers it (0 to 13, 32, 256), `text` or `graphics`, width and height
  in cells or pixels, bits per pixel, font, the text cursor (column, row, `on` or `off`), colour and background
  (hex for 32 bits).
- `palette`: the 256 entries as stored (alpha, red, green, blue); not for 32 bits. In mode 10 the first four are
  written as 0: libqb sets them at every refresh of the screen from entries 4 to 7, the mode's own palette, and
  the clock (the two phases of the mode's blink), so what they hold depends on when the program ends. Found when
  `m10_screen_10` failed under load in tier 2 (`m3-window-output`, 2026-10-12; `DECISIONS.md`).
- A text row is two lines: `t "…"` holds the characters (printable ASCII as it is; every other byte, `\` and `"`
  as `\xNN`), `a` the attribute bytes. A pixel row is one line `p`: palette indexes, or 32-bit colours.
- `a`, `p` and `palette` are hex runs: `v` or `v*n` (`n` in decimal). `= k` means the row before, k more times.
- A file may hold several states, one after the other, each from `screen-state 1` to `end`: those of the keys
  file's `state` steps, then the one at exit. A run that a `prompt` ended has a last line `stopped <keys> line
  <n>: <why>`.

Not in the state: sound (checked by what a program can observe of it, `tests\corpus\slice\s74_sound_queue`,
`s75_sound_wait` and `s76_sound_errors`; what is heard is checked by no test); the window (title, size, full
screen); hardware images; the graphics cursor and the
`VIEW`, `WINDOW` and `DRAW` settings; the glyphs of a font.

## The keys file

One step per line; `#` starts a comment.

| Step | Meaning |
|---|---|
| `text <characters>` | Each printable ASCII character |
| `key <name>` | `enter esc tab backspace space up down left right home end pageup pagedown insert delete f1` … `f12` |
| `wait <ms>` | A pause |
| `prompt` | Wait until the program announces its next wait for a key (below) |
| `state` | Write the state of the screen now, before the state at exit |
| `repeat <name>` | Last line only: the key every 100 ms until the program exits, for `END`'s "Press any key to continue" |

A key is given only when the program's `INKEY$` buffer is empty, so none is lost however many there are (the
buffer holds 15) and nothing depends on timing. A line that is none of these, or a file that cannot be read, ends
the program with exit code 125 and a message on stderr.

**Prompts.** A program that also reads `INKEY$` only to look for a key (a pause that a key ends, a tune that a key
stops) would eat a key the file gives too early. Such a program announces each real wait by counting in byte
`&H4F0` of conventional memory, which libqb does not use, right before it reads:

```
DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
```

The k-th `prompt` of the keys file waits for the k-th announcement; the program then has drawn everything it draws
before that read, so a `state` right after a `prompt` is the screen the player sees. Put a `prompt` before **every**
key of such a program: a menu that announces before each read announces again after every arrow key. A `prompt`
that waits more than 30 s ends the program with exit code 124; one that finds the program has announced more
waits than that (a key was given without a `prompt`) ends it with exit code 125. Either way the state then is
written, with the reason as the file's last line (libqb points stderr at the console the program was started
from, so a pipe does not see it). `tests\corpus\screen\c12_prompts` shows the steps; `g12_game` plays Civil War
Strategy with them.

Limits: a key is pressed and released at once (`_KEYDOWN` never sees it held); no Shift, Ctrl or Alt combinations;
a program that never reads `INKEY$`, `INPUT` or `LINE INPUT` gets the first key only.

## Writing a screen program

- End with `SYSTEM`. `END`, and running off the end, print a prompt into the bottom row first and wait for a key.
- No `RANDOMIZE TIMER`, no loop paced by `TIMER`, nothing else that differs between two runs: `--record` runs the
  program twice and refuses to record states that differ.
- The window shows while the program runs (about 0.3 s).
- `SLEEP` and the keys file: a key ends `SLEEP` only when it comes while `SLEEP` waits, and the next key of the
  file is given only when the key buffer is empty. Empty the buffer right before (`DO: LOOP UNTIL INKEY$ = ""`),
  end the keys file with `repeat <name>`, and give `SLEEP` a time, which is then the longest it can take
  (`tests\corpus\screen\d15_pacing`). A `SLEEP` with no time and a key already in the buffer never ends.

## Looking at a state

```
python tools\screen\state_to_png.py tests\corpus\screen\g03_capitol.screen -o capitol.png
python tools\screen\state_to_png.py tests\corpus\screen\c07_input_window.screen        # a text page: prints its rows
python tools\screen\state_to_png.py tests\corpus\screen\c12_prompts.screen --state 1   # the first of several states
python tools\screen\state_to_png.py tests\corpus\screen\c12_prompts.screen --all -o c12.png   # c12-1.png, c12-2.png, …
```
