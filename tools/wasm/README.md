# The browser launcher

`browser.mjs` runs a window program built for the wasm target in headless Chrome, the way a test runs a desktop
program (OpenSpec change `wasm-window`, design D6). The test runner uses it for every wasm program whose build
wrote a page (`<name>.html`): `run_legacy_tests.py --platform wasm`.

```text
node tools\wasm\browser.mjs <name>.html [--for <seconds>] [--screenshot <file.png>] [--profile <folder>]
                            [--chrome <chrome>] [--no-headers]
node tools\wasm\browser.mjs --find-chrome
```

Run it with the Node of the Emscripten SDK (`..\emsdk\node\<version>\node.exe`) or any Node 22 or later. It uses
Node's standard library only.

## What it does

- **Chrome** is started headless with `--remote-debugging-pipe`: DevTools talks over the process's pipes, so no
  port is opened and Windows asks nothing. It is found by `--chrome`, `QB64RUST_CHROME`, or where an install puts
  Chrome or Edge (`find_chrome.mjs`). On Windows that is `Google\Chrome\Application\chrome.exe` under
  `PROGRAMFILES`, `PROGRAMFILES(X86)` and `LOCALAPPDATA` (an install for one user), then Edge's `msedge.exe`. On
  Linux it is `google-chrome-stable`, `google-chrome`, `chromium-browser` or `chromium` on `PATH`. The Linux
  lookup has not been run on Linux, only tested with a made-up `PATH` (`test_find_chrome.py`). Each run has a
  profile of its own in the temporary folder, removed at the end.
- **The page's files** (`<name>.html`, `.js`, `.wasm`) are served at `http://localhost/` through DevTools' request
  interception, with `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`.
  A page needs both to use threads. Nothing listens on the port.
- **The program's folder.** Every other file of the page's folder, with its subfolders, is written into the
  page's file system, in the program's current folder, before the program starts.
- **A web folder** (OpenSpec change `wasm-publish`): when the page is `index.html`, every file of its folder is
  served, as a host serves them, and none is written into the page's file system.
- **`--no-headers`** serves without the two headers, as a host that cannot send them. A web folder's page then
  registers its service worker and reloads once. The requests of the whole browser are intercepted in this mode,
  not only the page's, so that the worker's own requests are served too.
- **A page that never starts.** When `--for` runs out, the launcher prints the page's status line as `the page
  says: ...`: the reason a web folder's page gives when it cannot start, or `Loading...`.
- **Variables.** Each `QB64_*`, `QB64PE_*` and `QB64RUST_*` variable of this process is the program's (but the
  tools' own: `QB64RUST_CHROME`, `QB64RUST_EMSDK_ROOT`, `QB64RUST_LIBQB_WASM_ROOT`,
  `QB64RUST_ALLOW_NETWORK`, `QB64RUST_BROWSER_LOG`). One whose value is an absolute path names a file of the
  page instead, written from the host's file first if it exists. When the program ends, the page's file is written to the host's if it
  changed. So `tools\screen`'s keys and screen state (`QB64_SCREEN_KEYS`, `QB64_SCREEN_STATE`), the prompt
  setting (`QB64PE_NOPROMPT`) and the trace work as on the desktop.
- **Standard input** is this process's, read to its end, when it is not a terminal.
- **Output.** The program's standard output and error are printed. The two warnings Emscripten's GL emulation
  prints at every start are not.
- **Dialogs** of the page (libqb's messages: an unhandled error's "Continue?") are answered No, an alert closed,
  as nobody answers a desktop program's dialog in a test.
- **The end.** The process exits with the program's exit status, 134 if it aborted, or 124 when `--for` ran out.
  `--screenshot` saves the page as a PNG at the end.
- **What the page keeps.** A page keeps what its program writes in the browser's storage (design D11). Here it
  keeps nothing, so each run starts as a desktop run in a fresh folder. With `--profile <folder>` the launcher
  uses that Chrome profile, keeps it, and lets the page keep files: two runs with one profile show what a player
  sees at a second visit.
- **A log.** With `QB64RUST_BROWSER_LOG=<file>`, each run appends its page, its exit status and the launcher's own
  messages to the file, one line each with the time. A test runner that runs a failed program again overwrites
  that program's output; the log keeps the first run's.
- **The page's data file** (`<name>.data`, the files of `[build]` `copy`) is served beside the page.

## Where it may run

On the host, also for a large run such as the whole `screen\` corpus (`CLAUDE.md` working rule 6; user,
2026-11-21): it opens no window and no port. The suites VM has no Chrome, so a wasm run there builds each window
program and skips it, with the detail `BROWSER_SKIP` of `run_legacy_tests.py`. The runner asks
`browser.mjs --find-chrome` once a run whether a Chrome is found.
