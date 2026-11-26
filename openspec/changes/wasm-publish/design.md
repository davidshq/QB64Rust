# Design

## Context

See `proposal.md` for the motivation. What stands today (OpenSpec change `wasm-window`, read in the code on
2026-11-25):

- `crates\driver\src\build.rs` writes `<name>.html` (`WASM_PAGE`, `wasm_page`), `<name>.js`, `<name>.wasm` and,
  without `-o`, `<name>.data` (`wasm_data`) into the output folder. For Civil War Strategy that folder also
  holds `cws.exe`, `cws1.sav`, `CWS.CFG` and the copied `graphics` and `scenarios`.
- `crates\driver\src\toolchain.rs` finds the SDK by `--emsdk-root` or `QB64RUST_EMSDK_ROOT` only
  (`find_emsdk_in`). `install_inner` downloads llvm-mingw with `curl.exe`, checks one SHA-256, unpacks with
  `tar.exe`, and writes a marker last, under a lock.
- The test runner builds every wasm program with `-o` and looks for `<name>.html`
  (`tools\legacy_tests\run_legacy_tests.py` lines 349 and 395). `tools\wasm\browser.mjs` serves the page
  through DevTools' request interception, with the two headers, and opens no port.
- The extension's toolchain rules are in `vscode\src\compiler\toolchain.ts` (free of VS Code) and
  `vscode\src\toolchainSupport.ts`.

Every decision below is Claude's (2026-11-25). Four of them are put to the user at the end.

## Goals / Non-Goals

**Goals:**
- One folder that a user uploads unchanged to any static host.
- A page that either runs or says what is missing.
- Trying a game locally with one command and no other tool.
- A wasm build on a Windows machine that has neither Git, Python nor Node.

**Non-Goals:**
- A one-thread build. It is still question 5 of `study\51` §10.
- Serving to a phone on the network. A page from another machine's plain HTTP address is not a secure
  context, so it gets no threads; that needs HTTPS and belongs to P1.
- A zip of the web folder. itch.io wants one; the user compresses the folder.
- The SDK download on Linux. A wasm build on a Linux host is not tested (`SOMEDAY.md`).
- Trimming the SDK. It is unpacked whole.

## Decisions

### D1. The page of a build without `-o` goes into `<output>\web` as `index.html`

A build with `-o` keeps today's three files beside each other, so the test runner, `browser.mjs` and every
recorded wasm run are untouched. A build without `-o` is the one a user makes, and it writes the web folder.

- `index.html`, since a host serves it for the folder's address and itch.io requires that name ("The ZIP file
  must contain a file called index.html", [itch.io HTML5 docs](https://itch.io/docs/creators/html5)).
- A folder of its own, since `build` holds the desktop build and what the game wrote.
- `<name>.js`, `<name>.wasm` and `<name>.data` keep the program's name, so the page's storage folder
  (`/<name>`, `wasm_page`) and saved games of W1 pages are unchanged.
- The files of `[build]` `copy` are packed into `<name>.data` only. They are not copied into `web`.
- The build removes a `<name>.data` it no longer writes and touches no file it did not write.

Code: `crates\driver\src\build.rs`, where the page is written (about line 1464), with a `web_folder` helper
beside `default_paths` in `main.rs`; `--print-paths` gains `page` there.

*Not chosen:* `build\<name>.html` as now, with a `qb64rust publish` command that copies the files out. It
adds a command and a second copy that goes stale.

### D2. `_headers` and a service worker of our own, in every web folder

- `_headers` is three lines: `/*`, then the two headers indented. Netlify reads it from the publish folder
  ([Netlify docs, custom headers](https://docs.netlify.com/manage/routing/headers/)) and Cloudflare Pages from
  the output folder ([Cloudflare Pages docs, headers](https://developers.cloudflare.com/pages/configuration/headers/)).
  Other hosts serve it as an unused file.
- `qb64rust-sw.js` is a service worker written here, about 30 lines. It answers each request of its folder
  with the network's response plus `Cross-Origin-Opener-Policy: same-origin`, `Cross-Origin-Embedder-Policy:
  require-corp` and `Cross-Origin-Resource-Policy: cross-origin`. The page's first script registers it only
  when `window.crossOriginIsolated` is false, waits until it controls the page, and reloads once. A
  `sessionStorage` flag stops a second reload.
- This is the method of `coi-serviceworker` (MIT,
  [gzuidhof/coi-serviceworker](https://github.com/gzuidhof/coi-serviceworker)), which reloads the page on the
  first visit so that the worker takes control. None of its code is copied: the worker is short, ours to
  change, and it adds no third-party file to every game a user publishes.
- When the page is still not isolated (no service workers, as in Firefox's private windows; not a secure
  context; a frame of a site that is not isolated; `file:`), it replaces "Loading..." with a message.

Code: two constants beside `WASM_PAGE` in `build.rs` (`WASM_HEADERS`, `WASM_WORKER`), and a script block of
`WASM_PAGE` that a `-o` build leaves out (`wasm_page` gets a flag).

*Not chosen:* shipping `coi-serviceworker.js` (others' code in each game; `study\52` §4 has not decided
whether others' code may be shipped); host files only for a host named in `qb64rust.toml` (the user must then
know the host before the first build).

### D3. `qb64rust serve` is a small server in the driver, on `127.0.0.1` only

- `crates\driver\src\serve.rs`, new: the standard library's `TcpListener`, a thread per connection, `GET` and
  `HEAD` only, `Connection: close`. No new dependency.
- The request is handled by a function from a request line and a folder to a response (status, headers,
  file), which is unit-tested without a socket. The path is split at `/`, percent-decoded, and refused when a
  part is `..`, empty in the middle, or holds `\` or `:`.
- It binds `127.0.0.1` and nothing else. A program that listens on an address of the network makes Windows
  Defender Firewall ask the user (`CLAUDE.md` working rule 6). That a loopback address does not ask could not
  be confirmed from a source (a web search on 2026-11-25 found only advice pages). The Python server that
  showed Civil War Strategy on 2026-11-25 listened on `127.0.0.1`; the user saw whether Windows asked.
- The default port is fixed at 8064, because a browser keeps saved games per address with its port
  (IndexedDB is per origin). Another port each run would lose them.
- `--open` starts the browser with the system's opener (`cmd /c start` on Windows, `xdg-open` on Linux).
- `--no-headers` is for seeing the service worker's path, and for its test.

### D4. `qb64rust toolchain install --target wasm` fetches Emscripten's own three archives

The SDK's manifest (`emsdk_manifest.json` of `..\emsdk` at `708dcbc`) names what `emsdk install 7.0.0` fetches
on Windows:

| Part | Archive | Unpacked to |
|---|---|---|
| The compiler | `https://storage.googleapis.com/webassembly/emscripten-releases-builds/win/6afbd83493d647051772d491124187d4e9a0cda9/wasm-binaries.zip` | `upstream\` |
| Node 24.19.0 | `node-v24.19.0-win-x64.zip` of the same site's `deps` folder | `node\24.19.0_64bit\` |
| Python 3.13.3 | `python-3.13.3-0-win-amd64.zip` of the same folder | `python\3.13.3_64bit\` |

`qb64rust` downloads the three itself, checks a pinned SHA-256 of each, unpacks them into the layout above
under `<home>\emsdk-7.0.0\`, and writes `.emscripten` with the five lines `emsdk activate` writes (as in
`..\emsdk\.emscripten`). The marker file is written last, so `find_emsdk_in` sees a whole SDK or none.

- Code: `install_inner` in `toolchain.rs` becomes a function over a list of archives, each with a URL, a
  SHA-256, a size and a target folder. llvm-mingw is a list of one. `find_emsdk_in` gets the home as a third
  place, with `EmsdkSource::Installed`.
- The three checksums are measured from one download while building this change, and recorded with the date.
  The manifest gives no checksum for them.
- The exact folder inside each archive (whether the zip holds `install\` or the files at its top) is read
  from the archives in that same task.
- The message before the download gives the sizes: about 750 MB down and 2.2 GB on disk (measured for
  `wasm-console`, `SOMEDAY.md`). The task measures them again.

*Not chosen:* downloading `emsdk` and running `emsdk install`. It needs a Python on the machine before the
SDK's own Python exists, and Git or a second download for `emsdk` itself.

### D5. The extension builds, serves in a terminal, and opens the address

- `vscode\src\compiler\emsdk.ts`, new and free of VS Code, as `toolchain.ts` is: the lookup, the offer's text
  and the rules. `vscode\src\toolchainSupport.ts` gets the window parts.
- "Run in Browser" asks `qb64rust --print-paths --target wasm` for `page`. After a build it serves the page's
  folder only if `index.html` is newer than the build's start; otherwise the program was `$CONSOLE:ONLY`.
- The server runs in a terminal named "QB64 Web". A second run disposes of that terminal first and passes the
  same port, so the address stays the same.
- The extension reads the address from `qb64rust serve`'s first line and opens it with
  `vscode.env.openExternal`. It does not pass `--open`, so that VS Code's remote windows open the browser on
  the user's side.

### D6. Tests, and where each runs

| What | Test | Where |
|---|---|---|
| The web folder's files, `_headers`, the `.data` removed | `crates\driver\tests\it\wasm_target.rs` | host, needs the SDK |
| The request handler, the path rules, the content types | unit tests in `serve.rs`, no socket | host |
| The port rules and one real request | `serve_answers_on_the_loopback_address` in `crates\driver\tests\it\cli.rs` | host (the user's answer 4) |
| The page through `index.html` with the headers | `browser.mjs index.html`, in `wasm_web_folder` of `cli.rs` | host, needs the SDK and Chrome |
| The service worker's reload, and the page's message without the worker | `browser.mjs index.html --no-headers`, in the same test | host, needs the SDK and Chrome |
| The SDK install's steps | unit tests with `file:///` archives, as llvm-mingw's | host |
| A build with the installed SDK and no Python on `PATH` | once by hand, with `PATH` cut down | host |
| The extension's rules | unit tests of `emsdk.ts` | host |

`browser.mjs --no-headers` serves the same responses without the two headers. As built: DevTools' interception
sees a service worker's own requests when it is enabled for the whole browser, not for the page's session, so
`--no-headers` enables it that way. A run with the headers keeps the page's session, as before. The web folder's
tests are in `cli.rs`, beside the other wasm builds, not in `wasm_target.rs`, which only checks programs.

## Risks / Trade-offs

- [A host changes its `_headers` rules] → The page's own check does not depend on them: without isolation it
  falls back to the worker, and then to the message.
- [The service worker fails in a browser we cannot run: Safari, Firefox] → Only Chrome is tested here. The
  reference says so, and the user's uploads (task 7.2) are the check.
- [itch.io serves the game in a frame of another site] → The worker cannot isolate a frame whose parent is
  not isolated. itch.io needs its "SharedArrayBuffer support" setting; the reference names it, and the page
  shows its message when the setting is off.
- [Emscripten removes an archive from its storage] → The install fails with the `emsdk` commands to use by
  hand, as now. The release is pinned, so nothing else changes.
- [`emcc.exe` of the installed SDK does not find the SDK's Python] → Measured in task 4.4 on a `PATH` without
  Python; if it needs `EMSDK_PYTHON`, the driver sets it where it sets `EM_CONFIG` (`build.rs` line 1396).
- [W1's page moves] → A build into the web folder removes the `build\<name>.html`, `.js`, `.wasm` and `.data`
  that an earlier build left, so no stale page stays beside the new folder.

## Decided by the user

The user answered the four questions below on 2026-11-25: "1. Yes 2. Yes 3. Yes 4. No". So the page moves, the
service worker of our own is built, `qb64rust` downloads the three archives, and Windows asked nothing about the
Python server on `127.0.0.1`. After answer 4 the socket test of `serve` runs on the host, and D6's rows that
name the suites VM no longer hold. One doubt stays with answer 4: Python may already have had a firewall rule on
this machine, which a newly built `qb64rust.exe` has not.

The questions as they were put. The panel is the five roles of `CLAUDE.md` working rule 4 and the web platform engineer of `study\51` §9,
since hosting is the subject.

1. **The page moves to `build\web\index.html` (D1).** Panel: all for it. Pragmatic engineer: a folder that is
   uploaded whole is the point of W2. Test engineer: `-o` builds do not change, so no recorded run changes.
   QB64 engineer: a user will look for `game.html`; the build prints the page's path.
2. **A service worker of our own in every web folder (D2).** Panel: for it, 5 to 1. Web platform engineer: it
   makes GitHub Pages work with no step by the user. Rust engineer and compiler engineer: two constants.
   Pragmatic engineer against: one more thing that can go wrong in a browser we cannot test; this role would ship
   only `_headers` and the message, and add the worker when someone asks for GitHub Pages.
   **Recommendation:** build it.
3. **`qb64rust` downloads Emscripten's three archives itself, about 750 MB, pinned by checksums we measure
   (D4).** Panel: all for it over running `emsdk`. Test engineer: the checksums are ours, not Emscripten's,
   so the first download is trusted as it comes.
4. **Did Windows ask anything when the Python server started on 2026-11-25?** If not, the socket test of
   `serve` and the worker test may run on the host (D6).
