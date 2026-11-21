# Design

## Context

W0 (`wasm-console`) built `$CONSOLE:ONLY` programs for `wasm32` and ran them under Node. W0.5
(`wasm-pointer-width`) gave the target 4-byte pointers. A window program was refused with "the wasm target builds
only `$CONSOLE:ONLY` programs so far". `study\51` §5.1 to §5.6 set out how a window could run in a page. This
change builds it. It was written overnight on 2026-11-21 while the user slept, so every decision below is
Claude's and is put to the user ("Open questions").

How the parts stood (worktree of branch `wasm-window`, on `after-pr-27-real` with W0.5 applied):

- libqb's window layer is `glut-emu.cpp`, a GLUT-like layer over GLFW 3.4. Its main loop is a `while` on the
  first thread around `glfwPollEvents`. The program runs on its own thread (`QBMAIN`), and a third thread runs
  `MAIN_LOOP`. Other threads queue messages to the first thread and may wait for them.
- libqb draws with OpenGL 1: client-side vertex arrays, the matrix stack, alpha test, and textures uploaded as
  `GL_BGRA`. GLAD loads the GL functions.
- The `Makefile` edit of W0 left `glut-emu.o`, GLFW's `core.a` and `clipboard.a` out of a wasm link.

## Measured

On the Windows host, 2026-11-21, with the SDK (7.0.0) in `..\emsdk` and Chrome 1xx installed.

- **A first window program** (`SCREEN 12`, `LINE`, `CIRCLE`, `PRINT`, an `INKEY$` loop): with Emscripten's GLFW
  (`-sUSE_GLFW=3`), the GL emulation (`-sLEGACY_GL_EMULATION`) on WebGL 2 and the edits of D3 and D4, it drew
  in headless Chrome with the right colors. Before the edits the build stopped on: X11 calls in
  `WindowGetNativeHandle`, `exit` without `<cstdlib>`, `WindowLoadResourceIconNative` missing, and
  `gluBuild2DMipmaps` at the link. At run time it aborted in `glfwSetCharModsCallback`.
- **Functions of GLFW 3.4 that Emscripten's GLFW lacks or stubs**, of those `glut-emu.cpp` calls: aborting
  `glfwSetCharModsCallback`, `glfwGetKeyName`, `glfwSetWindowMonitor`; missing `glfwGetWindowTitle` (a link error
  when `_TITLE` is used); printing an error `glfwSetInputMode` for `GLFW_LOCK_KEY_MODS` and `GLFW_CURSOR_HIDDEN`,
  and `glfwSwapInterval` before a main loop exists; warning only `glfwIconifyWindow`, `glfwRestoreWindow`.
- **A hardware image** (`_COPYIMAGE(…, 33)`, `_PUTIMAGE` of it each frame under `_DISPLAYORDER _SOFTWARE,
  _HARDWARE`, `target\scratch\2026-11-21\hw\hw.bas`): drawn over the software screen at its place, in its colors,
  in headless Chrome. (The screen state holds no hardware image, so this was judged by a screenshot.)
- **The whole `screen\` corpus in a browser** (on the host, user's ruling; 92 tests at 4 jobs while other builds
  ran): 74 pass, 16 skipped as listed, 1 known failure (`d19`). 20 failed beside the others and passed alone, which
  the runner allows for a screen program. Run again on an idle host at 4 jobs, all passed but `i03_save_load`,
  whose abort was a C++ exception (D12); with D12 it passes.
- **The verification family's 307 window programs in a browser** (on the host, 4 jobs, about 45 minutes): 261 pass,
  2 known failures, 1 skipped. Of the 43 failures, 40 are rejected for the page (`SUB _GL` 35, `_GLRENDER` 4,
  `$DEBUG` 1) and are listed as `target`; `v26_e_end` and `v26_e2_fall_off` lost their last line's missing line
  end (the launcher now passes output on byte for byte), and `v31_c_save_load` was D12's abort. All three pass now.
- **A rebuild of libqb's wasm objects** for a window program takes about 25 s on the host (34 objects). One
  build took 37 minutes; it was not seen again and its cause was not found.
- **The browser launcher** (D6). A page served with the two headers is cross-origin isolated
  (`self.crossOriginIsolated` is true) and WebGL 2 runs on SwiftShader. A screen program takes 2.5 s from start
  to exit; through the runner, 9 to 30 s each, most of it the build.
- **Screen programs of `tests\corpus\screen`**, run one by one or a few at a time on the host (not the whole
  corpus: working rule 6). Passing with the recorded `.screen` and output: `c01`, `c02`, `c03`, `c05`
  (`INKEY$` keys), `c07` (`INPUT` in a window), `c08`, `c09`, `c10` (an unhandled error), `c11` (`$SCREENHIDE`),
  `c15` (resize), `d08`, `d11`, `d18`, `f01`, `g03_capitol` (`PLAY`), `i02` (loaded images), `k01` (`ON KEY`),
  `u01` (the mouse queue). Skipped as on Linux: `g07`, `g09` (data files opened in another letter case).
  Rejected at compile time by D8: `w01_window_builtins` (`_WINDOWHANDLE`), `i04_maptriangle` (`_SCREENIMAGE`).

## Goals / Non-Goals

**Goals:**

- A window program builds for the browser from the same C++ fragments and libqb sources, and runs in a page.
- The `screen\` corpus is judged in a browser by the same `.screen` files as on the desktop.
- What a page cannot do is refused at compile time, never built as a routine that does nothing.
- The desktop builds compile the same libqb code: every edit is inside a test for Emscripten.

**Non-Goals:**

- A one-thread build for hosts that cannot send the two headers (`study\51` §5.1; decided after W1).
- Serving, uploading, the SDK's download for users, the editor's "Run in browser": W2.
- Phones: P1.
- Rewriting libqb's drawing for WebGL (D3 keeps the emulation).

## Decisions

### D1: A window program runs in a page, a console program under Node

The program's settings decide: without `$CONSOLE:ONLY` it gets the browser's link flags and a page; with it,
Node's flags as in W0. A `$CONSOLE` program (a window and a console) is a window program; what it prints to the
console goes to the page's `Module.print`, which the page sends to the browser's console.

*Instead of:* a `--web` option. The program already says whether it has a window, and a console program in a page
would need a terminal in the page, which is W2's if anyone asks.

Code: `crates\sema\src\target.rs` (the console-only error is gone); `crates\driver\src\build.rs` (the page).

### D2: Emscripten's own GLFW, not the `contrib.glfw3` port

`-sUSE_GLFW=3` selects the GLFW that ships inside the SDK (`src\lib\libglfw.js`). The port that `study\51` §5.2
named (`--use-port=contrib.glfw3`) is fetched from the network at the first build, which the suites VM has
not. The SDK's GLFW lacks some of GLFW 3.4 (see "Measured"); each place `glut-emu.cpp` calls one is an edit.

### D3: libqb's OpenGL 1 drawing through Emscripten's GL emulation

`-sLEGACY_GL_EMULATION -sMAX_WEBGL_VERSION=2`. The emulation turns libqb's fixed-function calls into WebGL 2.
What it lacks is shimmed in `common.h` under `QB64_EMSCRIPTEN` (edit `wasm-window-g-webgl-shim`): an upload in
`GL_BGRA` is converted to `GL_RGBA`, and `gluBuild2DMipmaps` (only a fallback) uploads level 0. Earlier GL errors
are cleared before an upload, since libqb reads `glGetError` after it and WebGL raises errors for desktop-only
texture settings.

*Instead of:* the rewrite of the presentation path to one WebGL 2 shader that `study\51` §5.2 recommended. The
emulation worked with one shim of about 30 lines, where the rewrite is about 1,500 lines of `libqb.cpp`. *Risk:*
the SDK calls its emulation "a collection of limited workarounds". A hardware image under `_DISPLAYORDER` drew
right ("Measured"). A fault found later is fixed in the shim or, if it cannot be, the rewrite comes back as its
own change.

### D4: The browser's main loop

`GLUTEmu::MainLoop` under Emscripten calls `emscripten_set_main_loop_arg` with one pass of the desktop loop, and
returns to the browser (edit `wasm-window-e-glut-main-loop`). The browser paces the frames, so the pass refreshes
the window directly instead of `GLUT_IDLE_FUNC`, whose `Sleep` paces them on the desktop. The program's thread and
`MAIN_LOOP` are Web Workers; the pool has 8, made before `main` (`-sPTHREAD_POOL_SIZE=8`).

The end of a program: the main loop keeps Emscripten's runtime alive, so `exit()` would halt without ending it
(no `atexit` handler, no exit status for the page). The program's exit is `emscripten_force_exit` (edits
`wasm-window-k` and `-l`).

### D5: The page

The driver writes `<name>.html` beside `<name>.js`: a canvas with the id `canvas`, a status line while loading,
focus to the canvas. Its script keeps a `Module` set before it (the launcher's, D6). A build as `$CONSOLE:ONLY`
removes a page left by an earlier build. The `--pre-js` file finds the `.wasm` file beside the script in a page
and in a thread's worker, as it does under Node.

### D6: The browser test launcher

`tools\wasm\browser.mjs` runs a page in headless Chrome as the runner runs a program: it is started with the
page, prints the program's output, and exits with its exit status.

- Chrome is driven over `--remote-debugging-pipe`. No port is opened, so Windows asks nothing (working rule 6).
- The page's files are served at `http://localhost/` through DevTools' request interception, with
  `Cross-Origin-Opener-Policy` and `Cross-Origin-Embedder-Policy`. Nothing listens.
- The program's folder is written into the page's file system, and the variables a test sets (`QB64_*`,
  `QB64PE_*`, `QB64RUST_*`) are the program's. A variable that names a host file names a page file instead, filled
  from the host's and copied back at exit if it changed. So `QB64_SCREEN_KEYS`, `QB64_SCREEN_STATE`,
  `QB64PE_NOPROMPT` and the trace work as on the desktop, and `tools\screen\screen_state.cpp` is unchanged.
- A dialog of the page (libqb's messages, D7) is answered No, as nobody answers a desktop program's dialog in a
  test. The two warnings Emscripten's GL emulation prints at every start are dropped.
- Chrome is found by `--chrome`, `QB64RUST_CHROME`, or where an install puts Chrome or Edge.

The runner (`run_legacy_tests.py --platform wasm`) runs a program whose build wrote a page through the launcher.

*Instead of:* Playwright, which `study\51` §5.6 named. It is an npm package with a browser download of its own.
The launcher needs neither: it is one file on Node's standard library.

### D7: libqb's messages are the page's dialogs

`gui_alert` (an unhandled error's "Continue?", an alert) uses the page's `alert` and `confirm`, run on the
page's thread (edit `wasm-window-p-gui-alert`). tinyfiledialogs has no browser backend: it fell back to a console
prompt and crashed.

### D8: What a page cannot do is rejected

Added to `WASM_UNAVAILABLE` (`crates\sema\src\target.rs`), for a window program:
`_WINDOWHANDLE` (a page has no window of the system; returning 0 would make a program take a wrong branch, as
`w01_window_builtins` did), `_SCREENCLICK`, `_SCREENPRINT`, the dialogs (`_MESSAGEBOX`, `_INPUTBOX$`,
`_OPENFILEDIALOG$`, `_SAVEFILEDIALOG$`, `_SELECTFOLDERDIALOG$`, `_COLORCHOOSERDIALOG`, `_NOTIFYPOPUP`), and
`SUB _GL` with `_GLRENDER` (`study\51` §5.5). The dialogs could map to the browser's later. A `$CONSOLE:ONLY`
program keeps them, as W0 built it for Node: it never shows a window, so `SUB _GL` never runs. Rejecting them
there too would have refused 39 verification programs that pass under Node, among them `v59_a5_console_only` of
the wasm slice list.

Sound is no longer "not yet" in a window program (D9). It stays "not yet" in a `$CONSOLE:ONLY` one: Node has no
Web Audio.

### D9: Sound through miniaudio's Web Audio backend

The audio part builds for wasm with two edits. On a Windows host the part took the VST instrument player, which is
Windows code; it is left out when `WASM=y` (edit `wasm-window-s-audio-vsti`). miniaudio's backend keeps its
devices on the page's `window` and makes an `AudioContext`, which exist only on the page's thread, while libqb
starts sound from the program's thread; so its `EM_ASM` blocks run on the main thread (edit
`wasm-window-t-audio-main-thread`). A browser starts audio only after a click or touch (miniaudio resumes then);
the launcher's Chrome allows it at once.

### D10: Full screen

`_FULLSCREEN` asks for the browser's full screen of the canvas (edit `wasm-window-h-glut-fullscreen`). Asked
outside a key press or a click, the browser grants it at the next one.

### D11: A page's files, and what the program keeps

The user's ruling (2026-11-21, open question 4): a game names the files its page ships in `qb64rust.toml`, and the
page keeps what the program writes in the browser's storage. The list is `[build]` `copy`, the data files a build
without `-o` copies into its output folder (OpenSpec change `build-output-folder`; user, 2026-11-21, "yes, lets do
A", which replaced a `[wasm]` `files` table of its own).

- **What ships**: the files of `[build]` `copy` (`output::copied_files`): a file entry, and every file below a
  folder entry, each at its path in the output folder. As the copy, only a build without `-o`; with `-o` the page
  ships nothing.
- **`<name>.data`** beside the page: a first line `QB64RUST-DATA 1`, a line of JSON with each file's path, size and
  FNV-1a hash, then the files' bytes (`build::wasm_data`). A build with no files removes an earlier one.
- **The program's folder in the page** is `/<name>`. The page mounts IndexedDB's file system there (`-lidbfs.js`),
  restores what was kept, then writes each shipped file. A shipped file that was kept stays as the program left
  it, unless the page now ships another version of it than when it was kept (the hashes of the last shipped
  versions are in the page's `localStorage`). So a high-score file keeps its scores, and a new version of a map
  replaces the old one.
- **Saving**: every second, when the page is hidden, and when the program ends (`FS.syncfs`), one save at a time.
- **Tests**: the launcher sets `Module.qb64rustKeep = false`, so nothing is kept and each run starts empty, as a
  desktop run in a fresh folder. With `--profile <folder>` it keeps a Chrome profile and lets the page keep
  files, which is how keeping was tested.

*Instead of:* Emscripten's `--preload-file`, whose loader writes the files before the program's own folder could be
restored, so a kept file could not win over a shipped one; and `$EMBED`, which holds one file inside the program
and is not a folder the program writes into.

### D12: C++ exceptions

The wasm target builds libqb and the program with WebAssembly's own exceptions (`-fwasm-exceptions`, compiled and
linked; edit `wasm-makefile-flags`), on both of its kinds, Node and the page. Emscripten leaves exceptions out by
default, and then a `throw` aborts. libqb relies on them: `_LOADIMAGE` tries its decoders in turn, and the PCX
decoder throws on a file that is not one, which libqb catches (found by `screen\i03_save_load`, whose stack read
`PCXImage::Stream::Read` → `__cxa_throw` → `abort`). The objects' stamp of the wasm tree names it, so a tree built
before is built again.

*Instead of:* Emscripten's exceptions in JavaScript (`-fexceptions`), which are slower and larger; every browser
the target runs in, and Node 24, has WebAssembly's.

## Risks / Trade-offs

- The GL emulation (D3) may fault on what was not tried: `_MAPTRIANGLE` to a hardware image with depth
  (`w02_depthbuffer`), `_DISPLAYORDER _GLRENDER`.
- `$SCREENHIDE` until `_SCREENSHOW`: the page's thread waits for the program's, which Emscripten allows by
  spinning, and the tab does not answer meanwhile. It passed `c11_screenhide`, which never shows the window.
- Keys the browser keeps (Ctrl+W, F5, Alt+Tab) never reach the program (`study\51` §5.4).
- A browser starts audio only after a click or touch: a game whose first input is a key press plays no sound
  until then.
- A file the program writes is saved at most a second later, and when it ends; a tab closed in that second may
  lose the last change (D11).
- Two pages of programs of the same name on one site share the program's folder in the browser's storage (D11).

## Open Questions

None. The user answered the four set aside overnight on 2026-11-21 (`DECISIONS.md`):

1. Where the whole `screen\` corpus and the window programs of the verification family run in a browser: on the
   host (working rule 6's exception).
2. D3 stays: the GL emulation. A WebGL 2 rewrite of the presentation path is a `SOMEDAY.md` row, with what speaks
   for and against it.
3. This repo's frozen copy of Civil War Strategy copies `cws.his` in BASIC instead of with `SHELL`, on the same
   line (`tests\programs\civil-war-strategy\SOURCE.md`).
4. A game's files and what it keeps: D11.
