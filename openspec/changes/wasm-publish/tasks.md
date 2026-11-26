# Tasks

Standing rules: CLAUDE.md. Different here: the socket test of `qb64rust serve` runs on the host, since it
listens on `127.0.0.1` only (the user's answer 4, design "Decided by the user"). The SDK download of task 4.3
is about 750 MB and runs once.

## 1. The web folder

- [x] 1.1 `build.rs`, `compile.rs` and `main.rs` (design D1): a window program built without `-o` writes
  `index.html`, `<name>.js`, `<name>.wasm` and `<name>.data` into `<output>\web`, removes a `<name>.data` it no
  longer writes, and prints the path of `index.html`. A build with `-o` is unchanged. Verified: `wasm_web_folder`
  and `wasm_window_page` in `crates\driver\tests\it\cli.rs` (the scenarios "A drawing", "A named output",
  "Beside a desktop build", "The copy list removed", "A file of the user's", "A window program"). The tests are
  in `cli.rs`, where the other wasm builds are, not in `wasm_target.rs`.
- [x] 1.2 `--print-paths --target wasm` prints `page` (spec `compiler/cli`, "The paths of a program"). Verified:
  `print_paths` in `cli.rs`, with the host's line unchanged.
- [x] 1.3 `_headers` (design D2, `WASM_HEADERS`). Verified: `wasm_web_folder_files` in `build.rs`, and the file
  in the web folder of 1.1's test.
- [x] 1.4 One window program run through its web folder: `node tools\wasm\browser.mjs build\web\index.html`.
  Verified by hand on 2026-11-25 with `SCREEN 12: LINE (10, 10)-(100, 50), 4, BF: SYSTEM`, the program of the
  spec's scenario, not one of `tests\wasm\slice.list`: exit status 0, and its screen state equal byte for byte to
  its `-o` build's. `browser.mjs` serves every file of a folder whose page is `index.html`
  (`tools\wasm\README.md`). Found on the way and fixed: the launcher made its folder for the files that
  variables name only when one such file existed, so `QB64_SCREEN_STATE` alone wrote nothing.

## 2. The page on any host

- [x] 2.1 `qb64rust-sw.js` (`WASM_WORKER`) and the page's first script (design D2, `WASM_START_WEB`): register
  only when not isolated, reload once, a `sessionStorage` flag. A `-o` build's page has no such script.
  Verified: `wasm_page_of_a_web_folder` and `wasm_page_names_its_script` in `build.rs`.
- [x] 2.2 The page's message when it cannot start, for `file:` and for a page still not isolated after the
  reload (spec `compiler/wasm-target`, "A page that cannot start says why"). Verified: `wasm_web_folder` removes
  the worker and reads "the page says: This game cannot start here." from `browser.mjs`, which now prints the
  page's status line when `--for` runs out. The `file:` message is checked as text only (`build.rs`); no test
  opens the page from the disk.
- [x] 2.3 `browser.mjs --no-headers` (design D6). Verified: `wasm_web_folder` runs the page with the headers
  and without them, both to exit status 0; without them and without the worker it cannot start (2.2), so the
  worker is what starts it. DevTools' interception serves the worker's requests when it is enabled for the
  whole browser. Not checked by a test: that no worker is registered on a host that sends the headers; the
  page's script decides it (`if (window.crossOriginIsolated)`, 2.1).

## 3. `qb64rust serve`

- [x] 3.1 `crates\driver\src\serve.rs` (design D3): the request handler, the path rules, the content types
  and the headers, with `--no-headers`. Verified: `serves_only_the_folder` and `headers_and_content_types`,
  without a socket, among them `/../qb64rust.toml`, `/%2e%2e/qb64rust.toml` and `/..%5c..%5cqb64rust.toml`.
- [x] 3.2 The subcommand in `main.rs`: the folder, a `.bas` file or nothing; `--port`, the ports 8064 to 8083,
  `--open`, `--no-headers`; the usage text. Verified: `serve_refuses_what_it_cannot_serve` ("Not a folder",
  "Not built yet", "Usage", "An unknown option"), which opens no socket, and
  `serve_answers_on_the_loopback_address` ("The default port", "A taken port", "A named port that is taken",
  a real request, a path outside the folder). Not verified: `--open`, which would open the user's browser.
- [x] 3.3 The socket test runs on the host after the user's answer 4, so no run in the suites VM is needed.
  Verified: it passed on the host on 2026-11-25.
- [x] 3.4 `docs\MANUAL.md`: a section "A game in a browser" with the install, the build, `serve` and its
  options. Verified: `vscode\scripts\test_package_download.py` passes (14 tests), which checks every option the
  manual names against the usage text.

## 4. The SDK download

- [x] 4.1 The download step of `install_inner` is the function `fetch`, which the SDK's install shares. As
  built, `install_inner` itself is not a loop over archives: the SDK's steps differ enough (three archives, a
  part moved to its place, two files written) that it has a function of its own, `install_emsdk_from`.
  Verified: the existing `toolchain_install_*` tests pass unchanged.
- [x] 4.2 The SDK's install with `file:///` archives made by the tests, the marker written last, `.emscripten`
  written, and `find_emsdk_from` with the installed place. Verified: `emsdk_install_then_installed_already`,
  `emsdk_install_wrong_checksum`, `emsdk_install_that_does_not_finish` (offline, interrupted, two at once),
  `emsdk_install_of_what_is_no_sdk` and `emsdk_lookup_finds_the_installed_one_last` in `toolchain.rs`;
  `toolchain_wasm_forms` in `cli.rs` ("Found installed", "Already installed", and on Linux "Install is by hand
  for now"). The Linux scenarios were not run: no test ran on Linux.
- [x] 4.3 The three archives downloaded once on 2026-11-25: 748,527,107, 37,304,352 and 29,736,374 bytes, 815
  MB together, with their SHA-256 in `emsdk_archives`. The compiler's archive holds one folder `install`,
  Node's one folder `node-v24.19.0-win-x64`, and Python's has its files at the top. Unpacked they take 2,186
  MB. Found by the first real install: the compiler's archive names its release `7.0.0-git`, and `emsdk
  install` writes `"7.0.0"` over it; the install does the same now. Verified: `qb64rust toolchain install
  --target wasm` into a `QB64RUST_TOOLCHAIN_HOME` of a scratch folder; see the closing message for its result.
- [x] 4.4 A window program built with an SDK assembled from the three archives, on a `PATH` of
  the two Windows system folders alone. It failed first: `emcc.exe` starts the `python.exe` of `PATH`.
  The driver now gives the SDK's own Python as `EMSDK_PYTHON` (`Emsdk::python`, `build.rs` beside `EM_CONFIG`).
  Verified: the build then succeeds in 8.6 s and the page ends with status 0 in headless Chrome, run by the
  SDK's Node; `emsdk_install_then_installed_already` checks that the installed SDK's Python is found. The
  program was `SCREEN 12: LINE (10, 10)-(100, 50), 4, BF: SYSTEM`, not one of `tests\wasm\slice.list`. No unit
  test reads the command's environment: the command is built inside `build::build`.
- [x] 4.5 Four `DECISIONS.md` rows of 2026-11-25: the web folder, the worker of our own, the SDK's download,
  and a listener on `127.0.0.1` on the host. The `SOMEDAY.md` row "`qb64rust toolchain install --target wasm`"
  is removed.

## 5. The extension

- [x] 5.1 `vscode\src\compiler\emsdk.ts`: the lookup (`qb64rust.emsdkPath`, checked by the compiler with
  `--emsdk-root`; else `toolchain path --target wasm`), the offer's text with the sizes of 4.3, and the rules.
  Verified: 16 unit tests in `vscode\test\unit\emsdk.test.ts` for the scenarios of "Emscripten SDK lookup in
  the extension", "Emscripten SDK offer" and "Emscripten SDK install command".
- [x] 5.2 `package.json`: the commands `qb64rust.runInBrowser` and `qb64rust.installEmsdk`, the setting
  `qb64rust.emsdkPath`, both commands unavailable in an untrusted workspace. Verified: the manifest test in
  `emsdk.test.ts`.
- [x] 5.3 "Run in Browser" (design D5), in `vscode\src\browser.ts`: build with `--target wasm`, check `page`,
  the terminal "QB64 Web", the address opened. Verified: 5 unit tests of the plan (the build's arguments, the
  page's path, a page that was not built, the server's arguments with the port of the run before, the address);
  `npm run test:unit` (184 passing) and `npm run lint`. **Not verified: `browser.ts` itself never ran.** No
  test starts VS Code for it, and `npm run test:integration`, which opens VS Code's window on the user's
  desktop, was not run. The command in VS Code is the user's to try (task 7.2).
- [x] 5.4 `vscode\README.md`: "Run in Browser", the install command and the setting's row. The suite has no
  test of the README's commands; read against `package.json`.

## 6. The reference and the guides

- [x] 6.1 `docs\reference\chapters\wasm-target.md`: "Building and running" and "Window programs in a page"
  name the web folder, `index.html`, `serve` and the installed SDK. A new section "Putting a game on the web":
  the folder's files, `serve`, the service worker, and a row each for Netlify and Cloudflare Pages, GitHub
  Pages, itch.io and a server of one's own. It says that only headless Chrome was tried and no upload was
  made. Verified: `reference.py render` and `reference.py --check` (401 pages, 0 problems).
- [x] 6.2 `docs\QUICKSTART.md`: "Play a game in a browser" under "Next steps". `docs\LAYOUT.md`: rows for
  `serve.rs` and `emsdk.ts`. Verified: `tools\repo_check`, in task 7.3's run.

## 7. Integration

- [x] 7.1 Civil War Strategy (`..\civil-war-strategy`, branch `more-changes`) built with the installed SDK in
  35 s into `build\web`: the six files, `cws.data` among them, and the page and script that W1 left in `build`
  removed. Verified: `browser.mjs build\web\index.html --for 20 --screenshot` shows the game's opening menu
  with its two flags, which come from `cws.data`. **Not as planned:** the game was not played to its first turn,
  since no script of keys for the port exists (`SOMEDAY.md`, "A scripted Civil War Strategy game in the
  browser"), and `qb64rust serve cws.bas --open` was not run, since it opens the user's browser.
- [x] 7.2 The list of what only the user can check is in the closing message of 2026-11-25: "Run in Browser"
  in VS Code, `serve --open`, an upload to each host that matters, and Firefox and Safari.
- [x] 7.3 `check wasm-slice`: 22 of 22 pass (20 verification, 2 corpus; 118.8 s). `just quick`: `fmt`, `lists`,
  `nextest`, `clippy`, `clippy-linux`, `build`, `reference`, `slice` and `doc` pass; `tools`, `tools-check`,
  `libqb-copy` and `v37-libs` stood from earlier runs. `tools\repo_check`: no finding in this change's files; it
  reports a control byte in `tests\frontend\encoding_cp437_ir.bas` and `verification\v74_f1_twin_cp437.bas`,
  which this change did not touch. The real install of 4.3 ended with status 0 in 4 min 42 s; `toolchain path
  --target wasm` named it as installed; a build with it on a `PATH` without Python or Node succeeded; a second
  install said "installed already".
- [x] 7.4 `STATUS.md` "Next": W2 built, and what waits on the user.

## Workflow follow-up

- Sync the delta specs and archive the change once the user has reviewed it.
- `ROADMAP.md` and the rest of `STATUS.md` are updated at the merge into `main`.
