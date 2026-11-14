# The sandbox family

Programs of the built-ins that act on the desktop, the clipboard or the console, run by `tools\sandbox\run.py` in
the test VM, or in Windows Sandbox with `--where sandbox` (`tools\sandbox\README.md`; spec `testing/desktop-sandbox`;
OpenSpec changes `desktop-sandbox` and `windows-vm`).
Each is the program of a built-in that `tests\never_run.list` gives the reason `sandbox`.

A program writes what it reads back to `out.txt` in its own folder, not to standard output (`_CONSOLEFONT` needs a
real console, `_SCREENPRINT` its window in front). It prints nothing that follows the sandbox's window size (the
desktop's size, the window's place before a move, which also cascades with each program of a session): a place
that depends on them is printed as a test (within 1 pixel), not as a value. `<name>.out.txt` is its recording;
`<name>.exit.txt`, when there is one, its exit code other than 0.

| Program | Shows |
|---|---|
| `clipboard_text` | `_CLIPBOARD$ =` text read back by `_CLIPBOARD$`: plain text, empty, a NUL (cut there: 1), extended characters, CR LF, and fixed-length strings of 3 and 4. It holds the cases of `verification\v49_d_clipboard` and of `tests\corpus\slice\s150_builtins_more`. On the host, Windows' clipboard history can hold the clipboard open just after the write, and libqb then reads back its own copy of all 3 bytes (`STATUS.md`, known bugs). So `v49_d_clipboard` is marked `clipboard:` and runs only in the suites VM and CI, and `s150_builtins_more` no longer uses the clipboard |
| `clipboardimage_roundtrip` | a 4 × 4 image with known pixels set with `_CLIPBOARDIMAGE =` and read back with `_CLIPBOARDIMAGE`: its size and each pixel's red, green and blue |
| `consolefont_err` | `_CONSOLEFONT` with a name and a size, then with the size alone, in a `$CONSOLE:ONLY` program: `ERR` 0 after each, and the program goes on |
| `mousemove_readback` | `_MOUSEMOVE` read back by `_MOUSEX`, `_MOUSEY` after `_MOUSEINPUT`: pixels in a graphics screen, a column and a row in `SCREEN 0` |
| `notifypopup_err` | two `_NOTIFYPOPUP`s (info, warning): `ERR` 0 after each, and the program goes on |
| `screenclick_mousebutton` | `_SCREENCLICK` on the program's own window: `_MOUSEBUTTON(1)` seen down and up through `_MOUSEINPUT`, at the place clicked within 1 pixel (libqb scales the place by `65535 / width`, so it is read a pixel short) |
| `screenmove_readback` | `_SCREENMOVE 100, 120` read back by `_SCREENX`, `_SCREENY`; `_SCREENMOVE _MIDDLE`: centered across within 1 pixel and down within 40 pixels above (the work area leaves out the taskbar) |
| `screenprint_inkey` | `_SCREENPRINT "abc" + CHR$(13)` read back by `INKEY$`: the four codes. It clicks its own window first: only a session's first window gets the keys (design.md, "Context") |
| `window_allowfullscreen` | Alt+Enter, pressed with Windows' `keybd_event` after the program clicks its own window, read back by the functions `_FULLSCREEN` and `_SMOOTH`: with no `_ALLOWFULLSCREEN`, `_STRETCH` and then `_STRETCH` with `_SMOOTH`; after `_ALLOWFULLSCREEN _SQUAREPIXELS`, `_SQUAREPIXELS`; after `_ALLOWFULLSCREEN _OFF`, no change |
| `window_fullscreen` | `_FULLSCREEN _SQUAREPIXELS`, `_FULLSCREEN _STRETCH, _SMOOTH` and `_FULLSCREEN _OFF`, read back by the functions `_FULLSCREEN` and `_SMOOTH`; `_SCALEDWIDTH` and `_SCALEDHEIGHT` in a window, and `_DESKTOPREFRESHRATE` above 0 |
| `window_hide_icon` | `_SCREENICON`, `_SCREENHIDE` and `_SCREENSHOW` read back by Windows' `IsIconic` and `IsWindowVisible`, and by the functions `_SCREENICON` and `_SCREENEXISTS`. `_SCREENSHOW` does not restore a minimized window. Each line is written as it is measured |
| `window_icon` | `_ICON` read back by the sizes of the window's big and small icons (Windows' `WM_GETICON`, `GetIconInfo`, `GetObjectA`): none at the start; one image sets both; two set the big and the small; no argument gives 32 × 32 and 16 × 16 |
| `window_size_limit` | `_WINDOWSIZELIMIT` read back by the window's answer to Windows' `WM_GETMINMAXINFO`, less its frame: both limits, none, the smallest alone, the largest alone, and in `SCREEN 0`, where the sizes are in characters (8 × 16 pixels) |
| `window_title_handle` | `_WINDOWHANDLE` checked by Windows' `IsWindow`; `_TITLE` read back as the window's text by Windows' `GetWindowTextA`; `_WINDOWHASFOCUS` after the program clicks its own window |

**Not read back:** `_TITLE$`. In QB64pe's build and in QB64Rust's, run in the test VM on 2026-11-12, `_TITLE$`
after `_TITLE "Probe title"` had the title's length, 11, but bytes that are not the title, other ones on each run.
So `window_title_handle` reads the title from Windows instead.

**A program that may need its second run:** `window_hide_icon`. Its output was the same in every run of both
compilers' builds on 2026-11-12. But in about one run in three the program did not end within the limit, after it
had minimized its window, in QB64pe's build as in QB64Rust's. The runner runs a program that timed out once more
(`tools\sandbox\README.md`), so it passes then, and its line says so.

**The recordings** are QB64pe's: `python tools\sandbox\run.py --record`, `qb64pe.exe` of the reference clone at
`16f629784e` (`main`, version 4.7.0-GLFW), its builds run in Windows Sandbox on 2026-10-30, every one exiting 0.
Each was read against what its program means to show: values read back, none of the machine's.

**Where they were made:** all seven in Windows Sandbox, on 2026-10-30. None was recorded again in the test VM.
In the VM with its 4 processors, all seven match them on 2026-11-03: the new compiler's builds in 14 full runs in
a row, and QB64pe's builds in 4. With 2 processors, `mousemove_readback` failed in about half the runs for both
compilers (`openspec\changes\archive\2026-11-03-windows-vm\design.md`, "Context", task 3.2).
`clipboard_text` was added later and recorded in the test VM on 2026-11-07, with the same `qb64pe.exe`. Its
recording matches `verification\v49_d_clipboard.out.txt` line for line, plus the 4 of the `STRING * 4` case. The
family of eight passed in the VM that day. The six `window_*` programs were added on 2026-11-12, for the
language reference, and recorded in the test VM with the same `qb64pe.exe`.
