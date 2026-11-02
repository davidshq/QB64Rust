# The sandbox family

Programs of the built-ins that act on the desktop, the clipboard or the console, run in Windows Sandbox by
`tools\sandbox\run.py` (`tools\sandbox\README.md`; spec `testing/desktop-sandbox`; OpenSpec change `desktop-sandbox`).
Each is the program of a built-in that `tests\never_run.list` gives the reason `sandbox`.

A program writes what it reads back to `out.txt` in its own folder, not to standard output (`_CONSOLEFONT` needs a
real console, `_SCREENPRINT` its window in front). It prints nothing that follows the sandbox's window size (the
desktop's size, the window's place before a move, which also cascades with each program of a session): a place
that depends on them is printed as a test (within 1 pixel), not as a value. `<name>.out.txt` is its recording;
`<name>.exit.txt`, when there is one, its exit code other than 0.

| Program | Shows |
|---|---|
| `clipboardimage_roundtrip` | a 4 × 4 image with known pixels set with `_CLIPBOARDIMAGE =` and read back with `_CLIPBOARDIMAGE`: its size and each pixel's red, green and blue |
| `consolefont_err` | `_CONSOLEFONT` with a name and a size, then with the size alone, in a `$CONSOLE:ONLY` program: `ERR` 0 after each, and the program goes on |
| `mousemove_readback` | `_MOUSEMOVE` read back by `_MOUSEX`, `_MOUSEY` after `_MOUSEINPUT`: pixels in a graphics screen, a column and a row in `SCREEN 0` |
| `notifypopup_err` | two `_NOTIFYPOPUP`s (info, warning): `ERR` 0 after each, and the program goes on |
| `screenclick_mousebutton` | `_SCREENCLICK` on the program's own window: `_MOUSEBUTTON(1)` seen down and up through `_MOUSEINPUT`, at the place clicked within 1 pixel (libqb scales the place by `65535 / width`, so it is read a pixel short) |
| `screenmove_readback` | `_SCREENMOVE 100, 120` read back by `_SCREENX`, `_SCREENY`; `_SCREENMOVE _MIDDLE`: centered across within 1 pixel and down within 40 pixels above (the work area leaves out the taskbar) |
| `screenprint_inkey` | `_SCREENPRINT "abc" + CHR$(13)` read back by `INKEY$`: the four codes. It clicks its own window first: only a session's first window gets the keys (design.md, "Context") |

**The recordings** are QB64pe's: `python tools\sandbox\run.py --record`, `qb64pe.exe` of the reference clone at
`16f629784e` (`main`, version 4.7.0-GLFW), its builds run in Windows Sandbox on 2026-10-30, every one exiting 0.
Each was read against what its program means to show: values read back, none of the machine's.
