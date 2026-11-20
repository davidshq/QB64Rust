# Proposal

## Why

The user decided on 2026-11-19 that QB64Rust gets a web target, built from the same C++ and the same libqb with
Emscripten, for `wasm32` (`DECISIONS.md`; milestone M9 in `ROADMAP.md`; the study is `study\51`). Nothing of it
has been built or measured. This change is the first step, W0: console programs, built for WebAssembly and run
under Node. It needs no browser, no window and no sound, so it is small. It answers the question the later
steps rest on: how close is libqb under Emscripten to libqb on the desktop? The answer is a number, the share
of the `$CONSOLE:ONLY` programs of `tests\verification` that give their recorded output, and a list that
says why each of the others does not.

## What Changes

- **A build can name its target.** `qb64rust -x --target wasm prog.bas` writes `prog.js` and `prog.wasm`
  beside the source, and `node prog.js` runs the program. Without `--target`, a build is for the machine it
  runs on, as now. This is the first build for another platform than the host; the platforms spec said there
  was none.
- **The wasm target takes `$CONSOLE:ONLY` programs only, for now.** Any other program is rejected at compile
  time with a message that says so. Windows, sound and files in a browser are the next steps (W1).
- **What the target does not have is an error at compile time, never a stub that does nothing.** In this
  change: `SHELL` and its forms, `RUN` of a file, `CHAIN`, the network words (`_OPENHOST`, `_OPENCLIENT`,
  `_OPENCONNECTION`), `DECLARE DYNAMIC LIBRARY`, and a `DECLARE LIBRARY` that names a prebuilt library.
- **`_OFFSET` and `_MEM` are rejected on this target until the next step.** A pointer is 4 bytes in `wasm32`,
  and the user accepted a 4-byte `_OFFSET` there. The front end still assumes 8 bytes everywhere. Until W0.5
  makes the width a property of the target, a program that uses these types would be compiled wrongly, so it
  is refused.
- **The Emscripten SDK is a second toolchain.** The driver finds it from `--emsdk-root`, else
  `QB64RUST_EMSDK_ROOT`. It accepts one pinned release and refuses any other, naming both. In this change the
  SDK is installed by hand with Emscripten's own `emsdk`; a download by `qb64rust toolchain install` waits for
  W2, when users need it.
- **libqb builds for `wasm32`** through edit files (`tools\libqb\edits\wasm-*.edit`): a branch for Emscripten
  in the `Makefile` and in `libqb-common.h`, and whatever else the first builds show. Each edit is inside a
  test for Emscripten, so the Windows and Linux builds compile the same code as before.
- **The wasm build has its own libqb objects**, in a copy of libqb's sources kept apart from `libqb\`. Without
  that, a wasm build would overwrite the Windows objects, and each switch of target would build libqb again.
- **`$IF WASM` and `_OS$`.** On this target `$IF WASM` is true, `WIN`, `LINUX` and `MAC` are false, `32BIT` is
  true, and `_OS$` is `[WASM][32BIT]`.
- **The test runner gets `--platform wasm`.** It builds with `--target wasm`, runs under Node, and judges
  against the recording used on Linux, because libqb takes its Unix paths under Emscripten. There is no QB64pe
  build for WebAssembly, so there is nothing to record from. A `known.wasm.list` per suite gives each program
  that cannot pass a reason from a fixed set.
- **The measurement.** A short list of about 20 programs runs on the host by name. The whole family, the
  `$CONSOLE:ONLY` programs of `tests\verification`, runs in the suites VM through a new `check` step. Its
  result and the sorted list of differences go into the design's "Measured" section.

Not in this change: a window, graphics, sound or files in a browser (W1); the 4-byte `_OFFSET` in `sema` and
the atomic thread flags (W0.5); `qb64rust serve`, the SDK download and the extension (W2); phones; a
one-thread build (`study\51` §5.1, not decided); a CI job; the Linux host as a place to build for wasm.

## Capabilities

### New Capabilities

- `compiler/wasm-target`: what the wasm target builds, which programs it takes, what it rejects and with
  which message, the pointer width, `$IF WASM` and `_OS$`, and whose behavior a wasm program is held to.

### Modified Capabilities

- `compiler/platforms`: "Supported platforms" says a build targets the platform `qb64rust` runs on and that
  there is no cross-compilation. It gains the exception: a build with `--target wasm`.
- `compiler/cli`: a new requirement for `--target <name>`, its values, its default and the output's names.
- `compiler/toolchain`: a new requirement for finding the Emscripten SDK and for the pinned release.
- `testing/golden-corpus`: new requirements for the runner's `--platform wasm` and for `known.wasm.list`.
- `testing/check-workflow`: a new requirement for the `wasm-verification` step and where it runs.

## Impact

- **Code:** `crates\base` (`Platform::Wasm`), `crates\syntax` (`$IF` flags), `crates\sema` (the target's
  rejections), `crates\codegen-cpp` (only if a fragment differs on the target; none is expected),
  `crates\driver` (`--target`, the SDK lookup, the build with `emmake`, the wasm copy of libqb).
- **libqb:** new edit files, all inside tests for Emscripten. `libqb\SOURCE.md` lists them.
- **Tools:** `tools\legacy_tests\run_legacy_tests.py` (`--platform wasm`), `tools\check` (one step),
  `tools\winvm` (the SDK in the suites VM).
- **Tests and lists:** `tests\wasm\slice.list`, `tests\verification\known.wasm.list`.
- **Dependencies:** the Emscripten SDK, 2.2 GB on disk (measured, design "Measured"), outside the repo,
  in `..\emsdk`. Node comes with the SDK; the suites VM also has Node 24.22.3 (`DECISIONS.md` 2026-11-17).
- **Run time:** the whole family is about 4,150 builds with `emcc`. How long one takes is not known; task 1
  measures it before the first large run is started, and the run's length is said before it starts
  (`CLAUDE.md` working rule 4).
- **Documents:** `DIVERGENCES-QB64PE.md` rows for what differs by design on the target (`_FLOAT`, the pointer
  width), a page for `--target` in the reference's chapters, `CLAUDE.md` ("Where things live": `..\emsdk`),
  `LAYOUT.md`.
- **Windows and Linux builds do not change.** Their C++ is the same byte for byte, and their recordings are
  not touched.
