# Design

Measured first, as the decision asks (`DECISIONS.md` 2026-10-10). The programs are those of `tests\corpus\screen\` now (`tasks.md`, group 1); all
numbers are from 2026-10-10, the reference clone at `16f629784e`, Windows 11.

## D1. One source file on the link line

`tools\screen\screen_state.cpp` is given to a build as `-f:ExtraLinkerFlags=<path of the file>`. `qb64pe` puts the
value into `CXXLIBS_EXTRA`, the `Makefile` adds it to the end of the link command, and the C++ compiler compiles a
source file it finds there with the flags of the link (`-std=gnu++20`, the include path of libqb's headers).

- **A source file, not an object.** The file reads `img_struct` and calls `keyboard_keydown`, so it must be
  compiled against the headers of the libqb it is linked with. As a source it always is; an object built once
  would need a key of its own and could go stale. Cost, measured: a build takes 2.8 s instead of 1.9 s (three
  runs each).
- **`-f:` is a temporary setting**: `qb64pe` does not write it to `config.ini` (`qb64pe.bas`, `CASE "-f"`), so the
  clone's settings stay as they are.
- **The path must hold no blank.** `make` and the shell split the value. The runner refuses to start a screen
  program when the file's path has one, and says so.
- The file registers `atexit` from a static initialiser. It does nothing when neither variable is set (measured:
  a program built with it runs as before).

## D2. Every kind of exit writes the state

| Program ends by | Result |
|---|---|
| `SYSTEM`, `SYSTEM 7` | state written; exit code 0, 7 |
| `END`, and running off the end | state written after a key; the bottom text row holds "Press any key to continue" |
| The window closed (`WM_CLOSE`) | state written |
| A runtime error with `QB64PE_NOPROMPT=y` | state written as it was when the error happened; exit code 0 |
| `$SCREENHIDE` (no window) | state written |
| `$CONSOLE:ONLY` | state written (the eight untouched text pages; no use as a test) |

Every path ends in `exit()` on the thread that owns the window (`libqb_exit`, `GLUTEmu_ProgramExit`), and image
memory is not freed before it. Two runs of one program give the same bytes (20 upstream programs and the
screen programs, each twice).

Rule for test programs: **end with `SYSTEM`**. `END` draws its prompt into the window first, so the bottom row is
lost to the comparison.

## D3. The state file

Text, LF line ends, written in one piece at exit:

```
screen-state 1
pages 3 display 1 write 0 read 0
page 0 mode 7 graphics 320x200 bits 4 font 8 cursor 1,2 off color 15 background 0
palette FF000000 FF0000AA … 00000000*240
p 00*320
= 9
p 00*10 04*91 00*219
…
end
```

- Line 2: the number of pages, and the page displayed, written to and read from; `image` when the destination or
  source is an image that is no page. Such an image gets a record of its own after the pages (`image write`,
  `image read`, `image write read`), since a program that draws on an image and never shows it is tested by it.
- A page record: a header (mode as libqb numbers it: 0 to 13, 32, 256; `text` or `graphics`; width and height in
  cells or pixels; bits per pixel; font; text cursor column, row and on or off; colour and background, in hex
  for 32 bits), the 256 palette entries as stored (not for 32 bits), then the rows.
- A text row is two lines: `t "…"` with the characters (printable ASCII as it is; every other byte, `\` and `"`
  as `\xNN`) and `a` with the attribute bytes. A pixel row is one line `p`.
- `a`, `p` and `palette` values are hex runs: `v` or `v*n`. A row equal to the one before is not written again:
  `= k` stands for k more. So an empty 640 x 480 page is two lines, and the files stay small enough to keep in
  git and to read in a diff: a `SCREEN 12` program with a box, a circle and a line of text is 3.9 KB.
- Every valid page is written, touched or not. `SCREEN 0` has eight from the start (measured), about 200 bytes
  each when empty.

Not in the state, by decision: sound; the window (title, size, position, full screen); hardware images; the
graphics cursor, `VIEW`, `WINDOW` and `DRAW` settings (they show in what is drawn next); the glyphs of a font
(only its number). Version 1 says so in its first line; a later version can add fields.

Per mode, measured (`m00`–`m256`): modes 1, 2, 7 to 13 are one page of 1 byte per pixel with `bits` 2, 1, 4, 4,
4, 2, 1, 4, 8; mode 0 and `WIDTH 80, 50` are text with eight pages; `_NEWIMAGE(…, 32)` is one page of 32 bits
without a palette; `_NEWIMAGE(…, 256)` is 8 bits with one.

## D4. Keys from inside the program

The decision named a helper that posts keys to the program's window (`PostMessage`), to be measured. Measured
(`verification\v23_post_keys.py` on `c05_inkey_keys` and `c07_input_window`): it works for `INKEY$`, `INPUT`, `LINE INPUT` and `END`'s wait,
without the window in the foreground, but with these limits:

| Found | Cause |
|---|---|
| More than 15 keys posted at once: the rest are lost | libqb keeps the BIOS key buffer (16 slots); the helper cannot see when the program has read it, so it can only pace by time |
| The first character after any special key is lost | `GLUT_KEYBOARD_BUTTON_FUNC` sets "ignore the next character" for a key that makes no character; a spare character has to be posted after each one |
| Enter, Esc, Tab and Backspace cannot be posted as characters | GLFW drops `WM_CHAR` below 32 |
| A letter posted as a key press is lost unless a pause follows | `TranslateMessage` puts its `WM_CHAR` behind the messages already posted |
| Keys are dropped when the window believes it has no focus | both callbacks return early; `WM_SETFOCUS` has to be posted too |

Each has a workaround, and the first only by timing. **Instead the keys are given from inside the program**, by
the same file: `keyboard_keydown` and `keyboard_keyup` are libqb functions other files can call
(`libqb\include\keyboard.h`), below the window, the focus rule and GLFW. A thread started by the static
initialiser reads the keys file and gives the runtime one key at a time, **each only when the `INKEY$` buffer is
empty** (`cmem[0x41a] == cmem[0x41c]`), so no key is ever dropped and nothing depends on timing. Measured: the 14
keys that failed in different ways when posted all arrive; 600 characters followed by Enter arrive in order,
three runs with equal state; `INPUT` and `LINE INPUT` in a window take them. No helper process, no window
handle, and the same for both compilers. Still no change to libqb, which is what the decision cared about.

The keys file, one step per line (`#` starts a comment):

```
text Dave
key enter
wait 200
repeat space
```

`text` is printable ASCII; `key` is one of `enter esc tab backspace space up down left right home end pageup
pagedown insert delete f1`…`f12`; `wait` is milliseconds; `repeat`, last line only, presses its key every 100 ms
until the program exits (for `END`, which empties the buffer and then waits for a new key). A line that is none
of these, or a file that cannot be read, ends the program at once with exit code 125 and a message on stderr, so
a typing mistake is a failed test and not a different recording.

Limits, stated: a key is pressed and released at once, so `_KEYDOWN` and `INP(&H60)` polling never see it held;
no Shift, Ctrl or Alt combinations; a program that never reads `INKEY$` (it reads `_KEYHIT` only) gets the first
key and no more. None of the first inputs needs them (Civil War Strategy reads `INKEY$` 25 times and nothing
else).

## D5. The runner

A corpus program is a screen program when `<name>.screen` exists beside it (empty before its first recording).
Everything else stays as it is: it still has its `.output` (what it wrote to stdout and stderr; nothing for a
program with a window, also after a runtime error, measured), `.stdin`, `.normalize`, `.noprompt`.

- Compile: `-f:ExtraLinkerFlags=<repo>/tools/screen/screen_state.cpp` is added, with either compiler.
- Run: `QB64_SCREEN_STATE` names a file in the results folder; `<name>.keys`, when it exists, is copied there and
  named by `QB64_SCREEN_KEYS`.
- `--record`: both runs must write a state and the two must be equal; then `.screen` is written, with `.output`.
- A normal run: the state must equal `.screen` (CRLF as LF, like `.output`). No state written is a failure of
  its own ("screen"), so a build that lost the file cannot pass.
- `.screen` with `.err` or `.norun` is an error in the test, like two expected files today.
- `.gitattributes`: `tests/corpus/**/*.screen -text` and `*.keys -text`, as for `.output` and `.stdin`.

The window shows for the 0.3 s a program runs. On a desktop in use that is a flicker and, if the user types at
that instant, a key in the test program; the runs are short and the keys go in below the window, so this is
accepted for now. A hidden desktop for the test programs would remove it and is noted in `SOMEDAY.md`.

The rules above are tested in `test_run_legacy_tests.py` with the stand-in for `run_proc` (it writes the state
file the environment names): equal and different state, no state, `--record` with equal and differing states,
`.keys` passed on, `.screen` with `.err`.

## D6. `qb64rust -f:ExtraLinkerFlags=`

The driver takes the setting as `qb64pe` does and passes it to `make` as `CXXLIBS_EXTRA`. The build key gets the
value and the contents of each word of it that names a file, so a changed state file is a new build.

## D7. The new compiler's side, measured by hand

The new compiler builds console programs only (`DEP_CONSOLE_ONLY=y`) and compiles no drawing statement, so none of
the screen programs is on `slice.list`. That the mechanism holds for its builds was measured once: a
`$CONSOLE:ONLY` program written by `qb64rust -z` and built by the `make` line of `build.rs` with
`CXXLIBS_EXTRA=<the file>` writes the same state, byte for byte, as the old compiler's build of it. The first
program that makes the comparison mean something comes with M3's first drawing statement.

## D8. First recordings

`STATUS.md` named upstream's 47 programs without `$CONSOLE:ONLY` as the first inputs. Measured: 27 of them have an
`.err` (they test a compile error and never run); 10 are `$CONSOLE` programs that print to the console and hide
the window, whose check is their output (step 11); the other 10 (`icon`, `versioninfo`, `opengl`, …) leave an
empty or one-line text screen. All 20 that run were recorded twice with equal states, which shows the method
holds, but **none is worth a recording**. They are left out.

Recorded instead, in `tests\corpus\screen\`:

1. The measurement programs, as tests of the method itself: text cells and attributes, each screen mode, pages
   and `PCOPY`, an image as destination, keys into `INKEY$`, `INPUT` in a window, `END` with a repeated key, a
   runtime error, an exit code.
2. Small drawing programs cut from Civil War Strategy (`tests\programs\README.md`, stage 2), one per kind of
   drawing, each ending with `SYSTEM`: `g01`–`g06` (the flags, the cannon, the end screen, the two ships, the
   message box). `tools\screen\cut_game.py` writes them: a few lines of setup and calls, then the game's
   procedures copied byte for byte, so what is drawn is the game's own code. Their states turned into pictures
   (`tools\screen\state_to_png.py`) show the game's screens. Not cut: what loads a `*.VGA` picture with `BLOAD`,
   since the runner copies only the `.bas` into a program's folder (`SOMEDAY.md`).

**`qbasic_testcases` are not taken.** `STATUS.md` named them third. Found: 143 programs by several authors
(`pete`, `thebob`, `qb45com`, …) with no licence stated, and a README saying they lack files they need to run;
the rule for a program taken into this repo (`tests\programs\README.md`: the licence must allow the copy) keeps
them out. As inputs they are also poor: 134 read the keyboard and 46 seed from the clock. A
trial of the 9 that read no key, built in a scratch copy: 6 end by themselves with equal states in two runs, one
differs between runs (dice), two never end. Running them from the clone, with only the recordings here, stays
possible and is noted in `SOMEDAY.md`.

## D9. Tools for people

`tools\screen\state_to_png.py` turns a state into a PNG (a graphics page) or prints its rows (a text page), with
the standard library only. No test uses it. It is there because a failed comparison of two 20 KB run-length files
tells a person little, and a picture tells at once what is wrong.
