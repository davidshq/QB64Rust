# Reviews of upstream's new commits

QB64pe's commits after the reference `16f629784e`, read at each merge into `main` (`CLAUDE.md` working rule 7,
`DECISIONS.md` 2026-11-12). Each commit gets one result:

- **Same here:** this project already does what the commit does.
- **Wanted:** a `SOMEDAY.md` row names the commit. Taking it is a `DIVERGENCES-QB64PE.md` row, since the parity
  target stays the reference.
- **Not relevant:** the commit changes something this project does not use (QB64pe's IDE, its own compiler's
  internals, its build files and test scripts), or a platform it does not build for.

`STATUS.md` names the last commit reviewed. The newest review is first.

## 2026-11-13: `16f629784e` to `ce6e54aee` (87 commits)

Read for the merge of branch `after-pr26`, from `..\QB64pe-latest` pulled that day.

### The runtime (libqb) and the language

| Commit | What it does | Result |
|---|---|---|
| `9fdb51be7` | `_ICON` waits until the window's thread has set the icon | Wanted: `SOMEDAY.md`, the `_ICON` row |
| `acf8ade9c`, `1ad362bd0`, `c57ea8e0b`, `75915083e`, `7e4f0439a`, `021c0fdb8`, `65aaf24fb` | Window sizing and scaling by the display scale, GLFW's screen coordinates, `_SCALEDWIDTH` and `_SCALEDHEIGHT` as the framebuffer's size, fractional `_MOUSEMOVE`, a mouse fix (pull request 786). `75915083e` also reformats `array-copy.cpp` with no change of behavior | Wanted: `SOMEDAY.md`, the display-scale row |
| `108086a78` | `_DROPPEDFILE$` copies the name instead of pointing at a list it then clears | Wanted: `SOMEDAY.md`, the `_DROPPEDFILE$` row |
| `01f775bd9` | Fixes the mouse on macOS (also reshapes `mouse.cpp` and `glut-emu.cpp`) | Not relevant: macOS is postponed (`DECISIONS.md` 2026-11-07) |
| `fae85f8e1` | A `_BIT` parameter is declared once in the C++ | Same here: D-057 |
| `c0dfe26ef` | `TIMER(accuracy)` finer than 1 ms | Wanted: `SOMEDAY.md`, the `TIMER` row |
| `50c371e13` | Thread safety in the audio engine, libxmp-lite and QOA updated, a `_FULLSCREEN` fix (pull request 800) | Wanted: the audio part is D-062's row in `SOMEDAY.md` (one line of it taken), the `_FULLSCREEN` part its own row |
| `1f843237a`, `4376e0e46`, `1c3f129fd` | Mouse hover and Cmd+key in `_KEYHIT` on macOS | Not relevant: macOS is postponed |
| `6e5b92657` | Backgrounds 8 to 15 in `SCREEN 0` with `_BLINK OFF` | Wanted: `SOMEDAY.md`, the `SCREEN 0` row |
| `8288757ca`, `56171415c` | `_SEAMLESS` in `CIRCLE` and `LINE` | Wanted: `SOMEDAY.md`, "Deferred QB64pe features" |
| `996668229` | `_SNDCLOSE` frees the `_MEMSOUND` lock on the program's thread | Wanted: `SOMEDAY.md`, the `_SNDCLOSE` row |
| `04993e54e` | `_FILELEN`, `_GETATTR`, `_FILEDATETIME$` | Wanted: `SOMEDAY.md`, "Deferred QB64pe features" |
| `37641e597` | `$INCLUDEONCE` compares file names without case on Windows and macOS | Same here: our compiler keys included files by canonical path. A Windows program that includes `once.bm` and then `ONCE.bm` prints the file's line once here and twice with the reference (checked 2026-11-13). That difference from the reference is D-065 (`verification\v70_i3_includeonce_name_case`) |
| `55a71ac61` | `_TITLE$` copies the title instead of pointing at a string that is gone | Wanted: `SOMEDAY.md`, the `_TITLE$` row (P2) |

### QB64pe's compiler, IDE, build and tests

| Commit | What it does | Result |
|---|---|---|
| `14c3c43cd` | The tests that read example.com | Not relevant: `http/read_example` is out of scope (`tests\upstream\README.md`) |
| `21676c5ff`, `630ab292e` | The IDE's export | Not relevant |
| `91106accc` | `sound_test.bas` made `$CONSOLE:ONLY` | Not relevant: our copy in `tests\upstream\` is the reference's, and passes |
| `585fa66e7`, `b0c7ce291` | QB64pe's own compiler: s-buffer handles widened to `LONG`, for programs of about 8,000 procedures; its readme | Not relevant: our compiler has no s-buffers |
| `8386112d4`, `eea2512b4`, `bb154f498`, `263214724`, `ef722a6f0` | The IDE: Cmd shortcuts on macOS, recent files, inline syntax hints, the window's size limit | Not relevant |
| `fdc7cd2d0`, `8fe4b4ef6`, `8684f1b5c` | QB64pe's test script: `.lnx.err` and other per-platform expected files | Not relevant |
| `b35490541` | QB64pe's compiler splits a program's C++ into several units and builds them incrementally; `qbx.cpp` becomes `qbx.cpp` and `qbx.h` | Not relevant: our compiler writes its own C++. Build time is the precompiled-header row of `SOMEDAY.md` |
| `6512534dd` | The `declare_library_dynamic` test's DLL uses `__stdcall`, which matters for 32-bit Windows only | Not relevant: no 32-bit build |
| `af0d63c0c`, `77fccfbaf`, `be2c1fc27` | QB64pe's `Makefile` and `dist_tests.sh` | Not relevant |

### Merges and source updates

Not relevant: none holds a change of its own. Each merge brings in commits listed above, and each "Automatic
update of ./internal/source" regenerates QB64pe's compiler from `source/`.

- Merges: `c783f9452`, `ec5374eca`, `f7c9ec7ae`, `1a4df6378`, `e6bcbd4a3`, `fcfb223b3`, `b2afee430`, `a693349b6`,
  `09db91d5e`, `d8892f9e3`, `d7a7a6366`, `074ea1170`, `253932b8a`, `9d0d559b1`, `8c9d0b8b7`, `e4b5af065`,
  `a18abf9bb`, `7331f88f0`, `46e0e269a`, `a748e8bb1`, `c1b77d9ce`, `77917c9e4`, `73327fa04`, `0afd909a0`,
  `1060e4565`, `3d329acd5`, `1c02fa71c`, `322a88627`, `ce6e54aee`.
- Source updates: `8d8e9e475`, `1ca61b420`, `442faf039`, `deed8f11e`, `81b44769b`, `e6b0783e1`, `8070eebd2`,
  `ae2964b2b`, `819359deb`, `cf13c687a`, `e81de41ac`, `e96c36c21`, `7eab93b05`, `2dbd658cf`, `dd66c0c58`,
  `0db9c9b96`.
