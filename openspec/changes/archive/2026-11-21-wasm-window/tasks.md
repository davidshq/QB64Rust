# Tasks

Standing rules: CLAUDE.md. Built overnight on 2026-11-21 in the worktree `..\QB64Rust-wasm-window` (branch
`wasm-window`, on `after-pr-27-real` with W0.5 applied as a patch), while W0.5 waited to be committed in the main
folder (working rule 18). Every decision is Claude's and waits for the user's review (design, "Open questions").

## 1. The page's runtime in libqb

- [x] 1.1 Edits of `glut-emu.cpp` for Emscripten's GLFW (design D2): includes, no native handles, no resource icon,
  `glfwSetCharCallback` for characters, no key names, no swap interval, no lock-key mode, the title kept, the
  cursor hidden by CSS. Verify: a window program links and runs in headless Chrome.
- [x] 1.2 The browser's main loop and `emscripten_force_exit` (design D4; edits `wasm-window-e`, `-k`, `-l`).
  Verify: `c02_screen12_shapes` ends with status 0 and its state written by an `atexit` handler.
- [x] 1.3 The WebGL shim in `common.h` (design D3, edit `wasm-window-g-webgl-shim`). Verify: the colors of
  `SCREEN 12` are right; the `screen\` programs of 4.2 pass.
- [x] 1.4 `wasm-makefile-flags.edit`: the browser's link flags for a window program (design D1 to D4).
- [x] 1.5 libqb's messages as the page's dialogs (design D7, edits `wasm-window-p`, `-q`). Verify:
  `c10_runtime_error` passes.
- [x] 1.6 `_FULLSCREEN` as the browser's full screen (design D10, edit `wasm-window-h`). Not verified in a
  browser: headless Chrome has no full screen to show.
- [x] 1.7 Sound (design D9, edits `wasm-window-s`, `-t`). Verify: `g03_capitol` (`PLAY`) builds and passes.
  Note: what is heard was checked by nobody; the launcher's Chrome is muted.
- [x] 1.8 `copy_libqb.py --check`: 0 problems.

## 2. The compiler

- [x] 2.1 The console-only error goes (`crates\sema\src\target.rs`); `check_target` loses its `main` argument.
- [x] 2.2 The page (`crates\driver\src\build.rs`, `WASM_PAGE`, `wasm_page`) and the `--pre-js` file in a browser
  (design D5). Verify: `wasm_page_names_its_script`; the SDK test `wasm_window_page`.
- [x] 2.3 The words a page cannot do (design D8), sound for window programs only. Verify:
  `wasm_target::window_words`, `a_window_program`, `cli::target_wasm_check_only`.
- [x] 2.4 Run the ignored SDK test `cli::wasm_window_page` with the SDK. Passed in 26.5 s.

## 3. The browser launcher

- [x] 3.1 `tools\wasm\browser.mjs` and its README (design D6): pipe, interception, files, variables, standard
  input, dialogs, `--for`, `--screenshot`.
- [x] 3.2 The runner runs a program with a page through it (`run_line`, `built_files`, `BROWSER_LAUNCHER`).
- [x] 3.3 A unit test of the runner for a page, in `test_run_legacy_tests.py`
  (`test_a_window_program_runs_in_the_browser_launcher`).

## 4. The screen corpus

- [x] 4.1 `tests\corpus\known.wasm.list`: `w01_window_builtins`, `i04_maptriangle` as `target`.
- [x] 4.2 Screen programs by name on the host (design, "Measured"): 18 pass, 2 skipped as on Linux, 2 listed.
- [x] 4.3 The whole `screen\` corpus in a browser, on the host (user, 2026-11-21): 92 tests (83 programs and
  `g12_game`'s scripts) at 4 jobs, while other builds ran. 74 pass, 16 skipped (12 as on Linux, 2 as `target`,
  `f02` Windows' fonts), 1 known (`d19`). 20 failed beside the others and passed alone, which the runner allows
  for a screen program (`timing_sensitive`); run again by themselves at 4 jobs on an idle host, all but one passed.
- [x] 4.3a `i03_save_load` aborts (status 134). The launcher, with the workers' exceptions reported, gave the
  stack: `_LOADIMAGE`'s PCX decoder throws, and without exceptions a throw aborts. Fixed by design D12:
  `i03_save_load` and `v31_c_save_load` pass, and the wasm slice (22 console programs) still does.
- [x] 4.4 The verification family's 307 `window` lines, run in a browser on the host: 261 pass, 2 known, 1 skipped;
  40 rejected for the page, now `target` lines; 3 faults fixed (the launcher's output byte for byte, and D12). The
  reason `window` left the runner's set, its tests, its README and spec `testing/golden-corpus`.

## 5. Civil War Strategy

- [x] 5.1 The game's port (`..\civil-war-strategy` at `7f4fce3`, copied to scratch) checks clean with
  `-z --target wasm`, builds (`cws.js` 0.6 MB, `cws.wasm` 3.6 MB) and reaches its first turn in headless Chrome
  with its folder in the page's file system.
- [ ] 5.2 A scripted game in the browser, judged by digests as `screen\g12_game` is: needs the data files' letter
  case to match their names (as on Linux). Moved to a `SOMEDAY.md` row at the archive (user, 2026-11-21).
- [x] 5.3 This repo's frozen copy copies `cws.his` in BASIC instead of with `SHELL` (user, 2026-11-21), on the same
  line; `cut_game.py` wrote `g12_game.bas` again (one line). Verify: `-z --target wasm` and `-z` of
  `CWSTRAT.BAS` report no error.

## 6. Records

- [x] 6.1 Reference: the chapter `docs\reference\chapters\wasm-target.md`, `_windowhandle.md` and `_fullscreen.md`;
  then `reference.py render` and `--check` (400 pages, 0 problems). The delta specs are merged into
  `openspec\specs` by hand, as in W0.5, since `--check` reads the merged specs: the archive must not apply them
  again.
- [x] 6.2 `DECISIONS.md` rows of the user's answers (2026-11-21), the exception in `CLAUDE.md` working rule 6, and
  the `SOMEDAY.md` row of a WebGL 2 presentation path.
- [x] 6.2a `DIVERGENCES-QB64PE.md` rows D-074 (libqb's messages as the page's dialogs) and D-075 (`_FULLSCREEN`).
- [x] 6.3 `LAYOUT.md` row for `tools\wasm\`; `STATUS.md`.
- [x] 6.4 `just quick` in the worktree: 9 passed, 0 failed, 4 standing (2026-11-21; the first run's two clippy
  failures were fixed). The desktop slice passed on the edited libqb, and `check wasm-slice` (the 22 console
  programs under Node) passed in 92 s.

## 7. A page's files (design D11; user, 2026-11-21)

- [x] 7.1 The files of `[build]` `copy` (`output::copied_files`), in a build without `-o` (user, 2026-11-21: one
  list for the desktop and the page, in place of a `[wasm]` `files` table first built for this). Verify:
  `output::tests::copied_files_of_files_and_folders`; `target\scratch\2026-11-21\keep2\run.sh` in headless
  Chrome from a folder of the page's four files alone.
- [x] 7.2 `<name>.data` beside the page (`build::wasm_data`), removed by a build with no files. Verify:
  `wasm_data_lists_then_holds_the_files`.
- [x] 7.3 The page: the program's folder `/<name>` on IndexedDB's file system, restored, then the shipped files
  (a kept file wins unless its shipped version changed), saved every second, when hidden and at the end. Link
  flags `-lidbfs.js` and the exports. Verify by hand in headless Chrome with one profile
  (`target\scratch\2026-11-21\keep`, `keep2`): a shipped file read; a save file counted 1, 2, 3 over three runs;
  a changed shipped file kept (2, then 3 lines), and replaced by a new version (2 lines again).
- [x] 7.4 The launcher: `.data` served, `Module.qb64rustKeep = false` unless `--profile`, and the log
  `QB64RUST_BROWSER_LOG`.
- [ ] 7.5 `$SCREENHIDE` without a spinning page thread. Moved to a `SOMEDAY.md` row at the archive (user,
  2026-11-21).
