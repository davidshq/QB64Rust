# Proposal

## Why

The wasm target builds console programs only, and they run under Node (W0, `wasm-console`; W0.5,
`wasm-pointer-width`). The games this project serves draw in a window. Step W1 of milestone M9 (`ROADMAP.md`,
`study\51` §5 and §8) puts that window in a browser page, so that a QB64 program with graphics, keys and the
mouse runs from a web page.

## What Changes

- **A window program builds for the browser.** With `--target wasm`, a program without `$CONSOLE:ONLY` is no
  longer rejected. The build writes `<name>.html`, `<name>.js` and `<name>.wasm`. Opening the page runs the
  program in a canvas. A `$CONSOLE:ONLY` program is built as before, for Node.
- **libqb's window layer runs in the page.** `glut-emu.cpp` is built against Emscripten's own GLFW
  (`-sUSE_GLFW=3`). The main loop is the browser's (`emscripten_set_main_loop_arg`), so the page's thread never
  blocks; the program keeps its own thread, as on the desktop. libqb's OpenGL 1 drawing runs on WebGL 2 through
  Emscripten's GL emulation, with a small shim for the texture format libqb uses (BGRA), which WebGL lacks.
  Each of these is an edit file under `tools\libqb\edits\`, inside a test for Emscripten.
- **`_FULLSCREEN` asks for the browser's full screen**, which the browser grants at the next key press or click.
- **The words a browser window cannot have are rejected** at compile time on this target, with the message of
  the console target: "`<word>` is not available on the wasm target". The list grows by what the window adds.
- **A browser test launcher.** `tools\wasm\browser.mjs` runs a built page in headless Chrome, driven over a pipe
  with no port opened, and serves its files with the two headers that threads need. It gives the test runner a
  program's output, exit status and screen state as a desktop run does. The runner uses it for a wasm program
  that has a page, so the `screen\` corpus is judged by the same `.screen` files.
- **Sound** through miniaudio's Web Audio backend, so `SOUND`, `PLAY` and the `_SND` words build for the page.
  (Planned in this change; see `tasks.md` for what is built.)

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/wasm-target`: "Console programs only" is replaced by a requirement for window programs and the
  page. "A wasm build" says which programs run under Node and which in a page. "What the target does not have is
  rejected" gains the words a window adds.
- `testing/screen-state`: a screen program built for the wasm target is run in headless Chrome and judged by the
  same `.screen` file.
- `testing/golden-corpus`: "The wasm known list" loses the reason `window`; a window program runs in a page.
- `compiler/wasm-target` also gains "A page's files": what `[build]` `copy` lists ships in the page, and what it keeps
  in the browser's storage (design D11).

## Impact

- `crates\sema\src\target.rs` (the console-only error goes; window words are added), `crates\driver\src\build.rs`
  (the page, the `--pre-js` file for a browser, the link flags of a window program).
- `libqb`: new edit files `wasm-window-*.edit`, and `wasm-makefile-flags.edit` gains the browser's link flags.
- `tools\wasm\browser.mjs` (new), `tools\legacy_tests\run_legacy_tests.py` (a page runs through it).
- Documents: the reference chapter `wasm-target.md`, `DECISIONS.md`, `DIVERGENCES-QB64PE.md` where a page
  differs, `STATUS.md`.
- Runs: Chrome must be installed where a browser test runs. The suites VM has none, so browser runs are on the
  host (user, 2026-11-21).
