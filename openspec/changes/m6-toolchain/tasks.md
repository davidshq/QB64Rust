# Tasks

Standing rules: CLAUDE.md. What this change does differently: one real download and install of the toolchain on
this machine (task 3.4); the path without a clone is run by the `--ignored` tests of 2.3 and 2.4, everything else
with the clone as now. No recording with `qb64pe.exe`.

## 1. Records

- [x] 1.1 `DECISIONS.md`: a row for the user's decision of 2026-10-26 (the toolchain is the pinned upstream
  `llvm-mingw 20260922`, fetched on first need by `qb64rust toolchain install`, only after the user agrees, never
  automatically; QB64pe stays a fallback; not bundled in the `.vsix`). It names
  `openspec\changes\m6-toolchain\design.md`. Verify: the row is there.

## 2. The toolchain root in the driver

- [x] 2.1 `crates\driver\src\toolchain.rs`: the release constants (design D1), `find` with its source (design D4:
  `--toolchain-root`, `QB64RUST_TOOLCHAIN_ROOT`, a given QB64pe folder, the installed one, the default clone;
  a candidate needs `bin\c++.exe` and `bin\mingw32-make.exe`), `installed` (the marker of design D3), and
  `missing_message`. Unit tests of the order with fake folders in a temporary directory. Verify: `cargo test -p
  qb64rust-driver toolchain` passes.
- [x] 2.2 `build.rs`: `Roots { toolchain, qb64pe: Option<PathBuf>, libqb }`; `build_with_make`, `build_key` and
  `objects_stamp` read the toolchain's `bin` (the cache version in `build_key` goes up by one); `find_root` returns
  `None` when nothing was given and the default clone is missing, and an empty `QB64RUST_QB64PE_ROOT` means none
  (design D5). `main.rs`: `--toolchain-root`; `library::Places` and `extra_link_args` take the QB64pe folder
  else the include root. The unit tests in `build.rs` that make a fake clone (`two_roots`, the stamp and key
  tests) follow. Verify: tier 1 passes; `cargo test -- --ignored` builds as before with the clone.
- [x] 2.3 `qb64rust dap`: `--toolchain-root`, `lldb-dap` from `toolchain::find`, the "No toolchain found" message
  in the `initialize` error; `main.rs` usage lines. Spec `compiler/cli` "Debug adapter subcommand" scenarios as
  CLI tests, and an `--ignored` one that debugs with `--toolchain-root` set to the clone's
  `internal\c\c_compiler` and `QB64RUST_QB64PE_ROOT` empty. Verify: those tests and the `--ignored` debugger tests
  pass.
- [x] 2.4 CLI tests (`crates\driver\tests\cli.rs`): a build with no toolchain anywhere (fails with the message,
  no network: `QB64RUST_TOOLCHAIN_HOME` an empty folder, `QB64RUST_QB64PE_ROOT` empty); `--ignored`: a build and
  run of a program with `--toolchain-root` set to the clone's `internal\c\c_compiler` and `QB64RUST_QB64PE_ROOT`
  empty, and a `DECLARE LIBRARY` header found under `--include-root` the same way (spec `compiler/cli`, "A
  library without a clone"). Verify: they pass.
- [x] 2.5 `crates\README.md` (build and run instructions): how a build finds its toolchain, in two lines. Verify:
  the text matches `toolchain::find`.

## 3. `qb64rust toolchain`

- [x] 3.1 `sha2` as a workspace dependency of `qb64rust-driver`; `cargo deny check` passes; `stage_bundle.py`'s
  notice includes its licence (the bundle staging test). Verify: both pass.
- [x] 3.2 `toolchain.rs` `install` and `install_from(url, sha256, home)` (design D2, D3): the lock, the marked
  folder reported, an unmarked folder deleted, `curl.exe` and `tar.exe` from `%SystemRoot%\System32` by full path,
  the progress lines, the checksum, the unpacking into the install root, the three folders and
  `lib\clang\23\lib\linux` removed, the zip deleted, the marker written last; no rename, nothing removed of other
  releases, no variable that changes the URL or the checksum. `main.rs`: `toolchain install`, `toolchain path`,
  the usage on any other word. Verify: the tests of 3.3 pass.
- [x] 3.3 Tests from spec `compiler/toolchain` and `compiler/cli` "Toolchain subcommand". In process, in
  `toolchain.rs`, `install_from` with a small zip of a fake toolchain written by the test and given by a
  `file:///` URL: install; already installed (nothing read from the URL: the zip deleted before the second run);
  wrong checksum (both sums in the message, nothing left); an unmarked folder deleted and replaced; two installs
  at once (two threads); a URL that fails (the URL and `--toolchain-root` named, nothing left). CLI tests:
  `toolchain path` for each source; `toolchain remove` and `toolchain.bas`. Verify: they pass, and none reaches
  the network (every URL is `file:///`).
- [x] 3.4 The real install, once, by hand: `target\release\qb64rust.exe toolchain install` with
  `QB64RUST_TOOLCHAIN_HOME` set to a folder under `target\`; record in this task the time of the download and of
  the unpacking, the folder's size, and that `toolchain path` names it. Verify: the unpacked folder's files equal
  the clone's `internal\c\c_compiler` files of the same names (a script under `target\`, not kept), less the
  folders removed.
  Done 2026-10-26: the download took 61.4 s (the first 5 % alone 21 s), the checksum 0.1 s, the unpacking with
  the removal of the four folders 6.8 s, so 68.5 s in all; it printed a `downloading <n>%` line every 5 %. The
  folder holds 6,868 files, 471,590,207 bytes (the "about 460 MB" of the offer). `toolchain path` names it with
  `installed:`, with `QB64RUST_QB64PE_ROOT` empty and also with the clone next to the repository (the installed
  toolchain comes first). Compared file by file with the clone's `internal\c\c_compiler`: all 6,867 files of the
  same names equal; only the install has the marker; only the clone has, outside the removed folders, the five
  `python\lib\python3.14\encodings\__pycache__\*.pyc` that design "Measured" names.

## 4. The extension

- [x] 4.1 `vscode\src\compiler\toolchain.ts`: `findToolchain` (design D6: `qb64rust.toolchainPath`, else
  `qb64rust toolchain path` given the QB64pe found) and `offerToolchain` (the modal question, "Download" under
  `withProgress` with cancel, "Not Now" with the message naming the command and `qb64rust.toolchainPath`);
  `package.json`: the setting (`scope: machine`), the command `qb64rust.installToolchain` (`enablement:
  isWorkspaceTrusted`), the `buildWith` description without "Both need QB64pe". Unit tests of the order and of the
  two answers and a failed install, window and process stubbed. Verify: `npm run test:unit` and `npm run lint`
  pass.
- [x] 4.2 `buildPlan.ts`: the compiler chosen first, QB64pe asked for only by a QB64pe build or its fallbacks
  ("Build with QB64pe" offered only when QB64pe is found); `qb64rust.ts` `buildArgs` with `--toolchain-root` and
  `--qb64pe-root` only when found; `debug.ts` with `--toolchain-root` and the offer before a session;
  `statusBar.ts` the toolchain in the language status. Unit tests of `planBuild` with and without QB64pe and
  without a toolchain, and of the adapter's arguments. Verify: `npm test` passes (unit, grammar, integration).
- [x] 4.3 `vscode\README.md` "Install": QB64pe is optional; the first build asks to download the toolchain (size,
  source, where it goes, `qb64rust.toolchainPath` for a folder of one's own); a `CHANGELOG.md` entry under 0.3.0
  (`package.json`'s version stays until the user tags a release). Verify: the
  README's first sentence and "Install" match the behaviour of 4.1 and 4.2.
- [x] 4.4 By hand in the development host, with `qb64rust.compilerPath` empty, no `qb64pe` on `PATH` and
  `QB64RUST_TOOLCHAIN_HOME` pointed at an empty folder under `target\`: Build asks; "Not Now" cancels with the
  message; "Download" installs with progress and the build goes on; a debug session stops at a breakpoint.
  Verify: recorded in this task, each step as seen.
  Done as a scripted run in the development host instead of by hand: `npm run test:integration:first-run`
  (`vscode\test\integration\suite\firstRun.test.ts`, run alone; `QB64RUST_TEST_QB64RUST` the release binary,
  `QB64RUST_TEST_TOOLCHAIN_HOME` an empty `target\m6\firstrun`). Only the modal question is answered by the test
  (`ToolchainSupport.ui.ask`); the commands, the install under its progress notification, the build and the debug
  session are the extension's own. Seen, 2026-10-26, 4 passing:
  1. No QB64pe found, `qb64rust toolchain path` finds none (the clone left out by an empty `QB64RUST_QB64PE_ROOT`),
     and the language status reads "C++ toolchain not installed".
  2. "QB64: Build" asks once, with the offer's text and the choices Download / Not Now; "Not Now" returns no
     executable, shows the message naming the install command and the setting, and leaves the home empty.
  3. "Download": the install ran with 21 progress lines from 0 % to 100 %, exit code 0, no error; the build went on
     and made `hello.exe`, which wrote "first run"; the toolchain is now the installed one and the status reads
     "qb64rust". Install and build together took 75.6 s.
  4. "QB64: Debug" on `debug_me.bas` stopped at the breakpoint on line 9, with the stack `SUB show` 9 and `Main module`
     4, and ran to its end; nothing was offered again.

  The first try failed at step 1: the empty `QB64RUST_QB64PE_ROOT` given to `runTests` did not reach the extension
  host, so the clone's toolchain was found. The test now sets the empty variables in the host's own process.

## 5. Lists and closing

- [x] 5.1 Specs: `openspec validate m6-toolchain --strict` passes; `LAYOUT.md` has a row for
  `crates\driver\src\toolchain.rs` and `vscode\src\compiler\toolchain.ts`. Verify: both.
- [x] 5.2 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then `--check`
  passes. Then the counts by `tools\legacy_tests\counts.py`, `STATUS.md`'s "Next" and "Waiting on the user"
  (item 1: §5.2's "usable without QB64pe" is decided), and `python tools\testtime\test_time.py`. Verify:
  `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
