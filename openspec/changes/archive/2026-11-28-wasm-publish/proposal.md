# Proposal

## Why

A window program builds to a page that runs in a browser (W1, OpenSpec change `wasm-window`). A user still
cannot get from there to a game on the web without knowing things QB64Rust does not tell them:

- The page is written into `build`, beside the desktop executable, the saved games and the copied data files.
  That folder cannot be uploaded as it is, and the page is `<name>.html`, where hosts and itch.io want
  `index.html`.
- The page needs two HTTP headers. Nothing in the folder asks a host for them, and a host that cannot send
  them (GitHub Pages) shows a page that never starts.
- Nothing serves the folder on the user's own machine. On 2026-11-25 Civil War Strategy was run locally with
  a Python script written for the occasion.
- The Emscripten SDK is installed by hand with `emsdk`, which needs Git and Python.
- The extension has no command for any of this.

This is W2, the last step of milestone M9 before phones (`docs\ROADMAP.md`; `study\51` §6 and §8).

## What Changes

- **A web folder.** A wasm build of a window program without `-o` writes `<output>\web\`: `index.html`,
  `<name>.js`, `<name>.wasm`, `<name>.data` when the project has a `copy` list, a `_headers` file, and
  `qb64rust-sw.js`. The folder holds nothing else of the build, so it is uploaded whole.
  **BREAKING** for W1's layout: the page was `<output>\<name>.html`. A build with `-o` is unchanged.
- **One folder for every static host.** `_headers` asks Netlify and Cloudflare Pages for the two headers. On a
  host that sends none, the page registers `qb64rust-sw.js`, a service worker of our own that adds them, and
  reloads once. Where neither works, the page says what the host must do; it does not stay at "Loading...".
- **`qb64rust serve`.** It serves a web folder on `127.0.0.1` with the two headers and the right content
  types, prints the address, and with `--open` opens the browser. `--no-headers` serves as a host without
  them does.
- **`qb64rust toolchain install --target wasm`** downloads Emscripten 7.0.0 for Windows (the compiler, its
  Node and its Python), checks each archive's SHA-256 and unpacks them under the toolchain's home. A wasm
  build finds the installed SDK after `--emsdk-root` and `QB64RUST_EMSDK_ROOT`.
- **The extension** gets "QB64: Run in Browser" (build for wasm, serve, open the browser), "QB64: Install the
  Emscripten SDK", the setting `qb64rust.emsdkPath`, and an offer before the download.
- **The reference** chapter `wasm-target.md` and `docs\MANUAL.md` say how to build, try and publish a game:
  the folder, `serve`, and what to do on Netlify, Cloudflare Pages, GitHub Pages, itch.io and a server of
  one's own.

Not in this change: a one-thread build (still the open question 5 of `study\51` §10), phones (P1), a zip of
the folder for itch.io (the user compresses the folder), an SDK download on Linux, serving to other machines
of the network, and the `SOMEDAY.md` row "Each page's storage of its own".

## Capabilities

### New Capabilities
- `compiler/web-serve`: the `serve` subcommand: what it serves, on which address, with which headers, and how
  it ends.

### Modified Capabilities
- `compiler/wasm-target`: "A window program runs in a page" and "A page's files" name the web folder and
  `index.html`. Added: the web folder's contents, the page on a host without the headers, and the page's
  message when it cannot start.
- `compiler/toolchain`: "The Emscripten SDK" gains the installed SDK as a third place, and "The wasm path of
  the toolchain subcommand" an install that downloads.
- `compiler/cli`: "The output of a wasm build" and "The paths of a program" name the page of a window
  program.
- `editor/build-run`: added "Run in browser".
- `editor/packaging`: added the SDK's lookup, offer and install command in the extension.

## Impact

- `crates\driver`: `build.rs` (the page, the web folder), `main.rs` (the `serve` subcommand, `--print-paths`,
  the usage text), `toolchain.rs` (the SDK's install and lookup), a new `serve.rs`. No new dependency.
- `vscode\`: a new command and setting in `package.json`, `src\build.ts` and `src\compiler\`, the unit tests,
  `README.md`.
- `tools\wasm\browser.mjs`: a way to serve a page without the two headers, for the service worker's test.
- Documents: `docs\reference\chapters\wasm-target.md`, `docs\MANUAL.md`, `docs\QUICKSTART.md`,
  `docs\LAYOUT.md`, `DECISIONS.md`, `SOMEDAY.md` (the row `qb64rust toolchain install --target wasm` goes).
- Machine time: the SDK download is about 750 MB and is run once to measure the three checksums. The wasm
  slice (22 programs, about 90 s) covers the build; the `screen\` corpus is not run again, since `-o` builds,
  which the test runner uses, do not change.
- The user's time: four design choices to confirm (design, "For the user to decide"), and an upload to each
  host to see the folder work there, which Claude cannot do.
