# Tasks

Standing rules: CLAUDE.md. What this change does differently: its Linux runs happen in WSL's Ubuntu 24.04, on a
copy of the working tree made by `tools/linux/sync.sh` (design D7), never on `/mnt/c`; packages that need `sudo` are
installed by the user; Windows recordings are not touched (Linux results go into `*.linux.*` sidecars only).

## 1. Records and the Linux machine

- [x] 1.1 `DECISIONS.md`: a row for the system toolchain on Linux (user, 2026-10-28, with the panel's view and the
  two options not chosen), and one for M7 as one change, naming this change's `design.md`. `SOMEDAY.md`: a pinned
  or downloaded Linux compiler (other ideas), and the multiarch library folders that QB64pe does not search (QB64pe
  behaviours to review). Verify: the rows are there.
- [x] 1.2 `tools/linux/sync.sh` (design D7): copies `git ls-files -co --exclude-standard` into `~/qb64rust` with
  `rsync --checksum --delete` and prints the count. Verify: run from WSL, then `diff -r` of a sample folder against
  `/mnt/c` shows no difference, and a file deleted on Windows is gone after the next sync.
- [x] 1.3 WSL set-up, recorded in `crates/README.md` ("Building on Linux"): `rustup` (stable, the workspace's
  `rust-toolchain` if any), and the packages and `lldb` installed by the user. Verify: `cargo build --release` in
  `~/qb64rust` succeeds.
- [x] 1.4 The package lists of design D1: start from QB64pe's Debian list, and leave a package out only if
  `cargo test -p qb64rust-driver -- --ignored` and the slice list still build in a fresh `ubuntu:24.04` container
  (Docker, in WSL) without it. Fedora's and Arch's lists are taken from `setup_lnx.sh` less the same packages.
  Verify: the measured lists and the container commands are in the design's "Measured". Note (2026-10-29): the
  Debian list drops `x11-utils` and `mesa-common-dev`; Fedora's and Arch's hold neither. Found on the way: `lldb-dap`
  19 aborts at the first `readMemory` (the minimum is 20 now, `DECISIONS.md`), and `screen/d19_blink_display` reads
  a refresh rate of 0 under Xvfb (`known.linux.list`, `varies`).
- [x] 1.5 `tools/linux/build_qb64pe.sh <scratch>` (design D6): a clone of the reference clone at `libqb\SOURCE.md`'s
  commit, built with `make OS=lnx BITS=64 BUILD_QB64=y`. Verify: `<scratch>/qb64pe -x -q -m hello.bas` builds and
  runs a program in WSL, and `git -C ../QB64pe status --porcelain` is empty before and after.

## 2. The platform and the toolchain

- [x] 2.1 `qb64rust_base::Platform` with `host()` (design D3); the default output name in `main.rs` by its
  extension. Verify: unit tests for both platforms' names; on Linux `qb64rust -x game.bas` writes `game`, executable
  (CLI test, `cfg(unix)`).
- [x] 2.2 `toolchain.rs` (design D1): `find` takes a `Platform`; the Linux lookup (option, variable, `PATH`);
  `Source::System`; `Found::tool(Tool)` in place of `bin(name)` at every caller (`main.rs` `dap` and `nm`,
  `build.rs`); the `toolchain path` line `system`. Verify: unit tests of the Linux order on fake folders with
  a fake `PATH`, running on Windows too; `cargo test -p qb64rust-driver` on both platforms.
- [x] 2.3 `packages_message` from `/etc/os-release` (design D1), used by "No toolchain found", `toolchain install` on
  Linux, and a C++ build that fails on a missing header of the packages (matched in `cpp_errors.rs`). Verify: unit
  tests with `os-release` texts of Ubuntu, Debian, Fedora, Arch and an unknown one; a CLI test with an empty `PATH`
  on Linux checks the exit status 1 and the `apt-get` line; the scenarios of spec `compiler/toolchain`.

## 3. Building on Linux

- [x] 3.1 `build_with_make` per platform (design D2): `make OS=lnx`, the tools found, `program` without `.exe`, the
  mode kept on the move, no `write_resources` on Linux; the platform line in `build_key`; the stamps of a `System`
  toolchain. Verify: in WSL a program builds and runs, a second build with `QB64RUST_BUILD_CACHE` is a hit, and the
  `--ignored` CLI tests that build pass on Linux (those that need `qb64pe.exe` are `cfg(windows)`).
- [x] 3.2 `$EXEICON`, `$VERSIONINFO` and the other Windows-only metacommands on Linux: what the Linux `qb64pe`
  (task 1.5) does with each is followed, warnings included. Verify: a CLI test per metacommand on Linux, its
  expected text taken from the Linux `qb64pe`.
- [x] 3.3 The Linux library lookup (design D4): `library::find` with a platform; `find_linux` in the old compiler's
  order. The multiarch row of task 1.1 is linked from its doc comment. Verify: unit tests on fake trees for each
  scenario of spec `language/external-libraries`, "Finding a library", running on both platforms.
- [x] 3.4 The Linux C++ of `DECLARE LIBRARY` (design D3): `Resolved` carries the platform; `dlopen`, `dlsym`, the
  typedefs without `CALLBACK`, the link names and `-Wl,-rpath`. Verify: `codegen-cpp` snapshot tests of each form
  for both platforms; on Windows the corpus's `-z` output of every library program equals the commit before.
- [x] 3.5 The `DECLARE LIBRARY` programs on Linux: the library programs of `verification\` (`v37*` and the others
  that name a library) get Linux twins where theirs are Windows files (a `.so` built by a `.files` script, `libc`
  by `DYNAMIC`). Their `-z` fragments are compared with the Linux `qb64pe`'s, and they are run. Verify: the
  fragments are equal but for the lines `callsite.py`'s normalisation names; the programs pass on Linux; the
  scenarios of "DLLs at the start of the program" are among them.

## 4. The debugger on Linux

- [x] 4.1 `toolchain::lldb_dap` (design D5) and `LLDB_ENV` on Windows only. Verify: unit tests of the lookup
  (`lldb-dap`, `lldb-dap-17` and `lldb-dap-18` on a fake `PATH`, none); the scenarios of spec
  `editor/debug-adapter` on Linux.
- [x] 4.2 The debugger CLI tests (`crates/driver/tests`, the `dap` ones) in WSL with Ubuntu's `lldb-dap-18`. If one
  fails for a request 18 lacks, measure the oldest `lldb-dap` that passes (`apt.llvm.org`), put that version in the
  message and in the spec's scenario. Verify: the debugger CLI tests pass on Linux, and the version measured is in
  the design's "Measured".
- [x] 4.3 Source paths in `<exe>.debug.json` and the symbol reader on Linux (case-sensitive, `/`). Verify: a debug
  session of a program with an `$INCLUDE` in a subfolder stops at a breakpoint in the included file, in WSL.

## 5. The runner on Linux

- [x] 5.1 `run_legacy_tests.py` (design D6): `--platform`, executable names, the Linux sidecars, `known.linux.list`
  with the reason `windows` (skipped), the check that fails on a sidecar equal to its shared recording. Verify: new
  tests in `test_run_legacy_tests.py` for each scenario of spec `testing/golden-corpus` that needs no compiler,
  running on Windows; the Windows corpus run is unchanged (the pass cache's slice list gives the same summary).
- [x] 5.2 `--record --platform linux` with the Linux `qb64pe` (spec "Recording on Linux"), and the copies for
  `--jobs` with symlinks. Verify: runner tests for "Same result as Windows" (no sidecar written, an old one
  removed); one corpus program recorded in WSL.
- [x] 5.3 Devices on Linux: the ALSA null configuration in `tools/linux/` and the display (WSLg, `xvfb-run`).
  Verify: a sound program and a window program of the slice list run in WSL and under `xvfb-run`.
- [x] 5.4 The recordings on Linux, suite by suite (the slice list, the corpus, upstream, compile, verification),
  each recorded with the Linux `qb64pe` and then run with the new compiler. A difference that is the new
  compiler's is fixed. One that is QB64pe's Linux build's becomes a sidecar. What cannot run becomes a line in
  `known.linux.list`. Each family with sidecars gets a sentence in `verification/README.md`. Verify: each suite's
  run in WSL ends with no failure outside `known.linux.list`, and the counts of sidecars and lines are in the
  design's "Measured".
- [x] 5.5 The copy's edits on Linux: `d048-console-eof-input-string`'s Linux branch, and each edit's `v63` program run
  on Linux (D-048 to D-053 hold there, or their row says where they differ). Verify: `copy_libqb.py --check`
  passes, the `v63` programs pass in WSL, and the rows of `QB64PE-DIVERGENCES.md` say "Linux too" or name the
  difference.

## 6. Packaging for Linux

- [x] 6.1 `stage_bundle.py --target linux-x64` and the crate notice for `x86_64-unknown-linux-gnu` (design D8).
  Verify: in WSL the staged bundle holds `bin/qb64rust` and no `.exe`, and `tools/licenses/check.py` passes on it.
- [x] 6.2 `package_zip.py` becomes `package_download.py` with `--platform` (design D9); the tar's modes, owners
  and times; `--check` for the tar; `check.py` reads a tar. `release-vsix.yml` and `LAYOUT.md` name the new script.
  Verify: its tests (two packages of one bundle are equal byte for byte; the scenarios of spec
  `compiler/download`); the tar unpacked in a fresh WSL folder builds and runs `hello.bas` ("First use on Linux").
 - [x] 6.3 The extension (design D8): the binary's name per platform, `toolchain.ts` per platform with no offer
  on Linux, "Copy Install Command", `installToolchain` on Linux. Verify: unit tests with the platform as input
  for each scenario of spec `editor/packaging` that names Linux; `npm test` on Windows and in WSL. Note: in WSL
  (Node 24, VS Code under `xvfb-run`, `libnss3` installed) 127 unit and 82 integration tests pass at both versions.
  Found there and fixed: a system toolchain is given no `--toolchain-root` (its folder is `/usr/bin`), QB64pe's
  Linux build colors its output, and tests that wrote Windows paths or `.exe` names.
- [x] 6.4 The Linux `.vsix` (`vsce package --target linux-x64`), installed in VS Code in WSL (Remote - WSL) or
  on the WSLg desktop: the integration tests under `xvfb-run`, then by hand a build, Build and Run, a console
  program, and a debug session stopping at a breakpoint. Verify: the integration tests pass; the session by hand is
  recorded in this task's note. Note (2026-10-29): packaged in WSL (9.2 MB, `bundle/bin/qb64rust` mode 755 in the
  zip), unpacked with its modes and loaded as the extension in VS Code 1.144.0 under `xvfb-run`, the language
  server and debugger being the bundle's binary: 82 integration tests pass. The session by hand was driven by those
  tests rather than a person: a build, Build and Run in terminal `QB64`, the build task, and the Debug command and F5
  stopping at a breakpoint in a SUB with its parameter read by name.
- [x] 6.5 `MANUAL.md`'s Linux section and `vscode/README.md`'s "Install" for Linux, with the package lines
  checked against `toolchain.rs`'s table (design D9). Verify: the check passes, and fails when one line is changed.

## 7. CI and the cadence

- [ ] 7.1 `rust.yml`: the `linux` job of design D10. Verify: `gh workflow run` on the branch; its result read
  when it comes (not waited on).
- [ ] 7.2 `vscode-extension.yml`: the `linux` job. Verify: as 7.1.
- [ ] 7.3 `release-vsix.yml`: the `linux` job and the publishing step that waits for both. Verify: a run started
  by hand without a tag (`workflow_dispatch`, packaging and checks only, nothing published) produces the four
  artifacts.
- [x] 7.4 The Linux family of `tools/close/close.py` (design D11), its conditions, its line in
  `tools/close/README.md`, "The test cadence", and a `DECISIONS.md` row. Verify: `close.py`'s own tests for a change
  that touches `build.rs` (Linux family required) and one that touches only documents (not required).

## 8. Closing

- [x] 8.1 On Windows, the corpus's C++ from `-z` equals that of the commit before this change. Verify: a script
  under `target\` compares the two, and no program differs. Note: 592 compared; one differs on purpose,
  `slice\s61_member_arrays`, whose `SWAP` of places with FUNCTION calls now finds them left to right
  (`QB64PE-DIVERGENCES.md` D-054, found by task 5.4); the `v37_l*` Linux twins are new (rejected on Windows).
- [x] 8.2 `openspec validate m7-linux --strict` passes. `LAYOUT.md` has rows for `tools/linux/` and the
  `known.linux.list` files. `crates/README.md`'s build instructions cover Linux. `vscode/CHANGELOG.md`'s unreleased
  section has a line on Linux. Verify: all four.
- [x] 8.3 By hand on a Linux desktop session (WSLg): the extension from the Linux `.vsix`, a window program
  (`SCREEN 12`, `LINE`, `INKEY$`), a sound program and a program with `DECLARE DYNAMIC LIBRARY "libc"`. Verify:
  each works; the note is in this task. Note (2026-10-29): built with the `.vsix`'s binary and libqb, run on the
  WSLg desktop. The window program draws as written (a capture of its window was looked at), reads the pixels
  back with `POINT`, and takes keys sent through XTest by `INKEY$`. The sound program (`SOUND`, `PLAY`, `_SNDRAW`)
  was heard on the host; WSLg plays only through PulseAudio, so WSL needs `libpulse0` (without it ALSA finds no
  card and the program runs silent; a desktop distribution has it). `getpid`, `strlen` and `atoi` from
  `"/usr/lib/x86_64-linux-gnu/c:6"` return their values.
- [x] 8.4 The closing runs: `python tools\close\close.py --plan`, each command run by its own (the Linux family in
  WSL after `sync.sh`), then `--check` passes. Then `STATUS.md`'s "Next" (M7 built; M8 next). Verify: `--check`
  passes.
- [x] 8.5 The review of the staged files (2026-10-29, the pragmatic engineer's recommendations, user: "do prag
  recs", twice): `close.py`'s `LINUX_FILES` follows `BUILT_FROM` (`DECISIONS.md` 2026-10-29), its `--plan` and
  design D11 say so; `release-vsix.yml` publishes only from a pushed tag; `lldb_dap` takes a versioned `lldb-dap`
  of the minimum before the plain one and refuses an older versioned one, naming it (D5, spec
  `editor/debug-adapter`); `STATUS.md`'s "Next" on M7; a `SOMEDAY.md` row for a plain `lldb-dap`'s version.
  Verify: `test_close.py`, `cargo test -p qb64rust-dap`, the driver's `toolchain` unit tests.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
- At the merge into `main`: the full set on Windows and the Linux full suites (design D11), `ROADMAP.md`'s M7
  ticked and moved to `ROADMAP-COMPLETED.md`, the counts by `tools\legacy_tests\counts.py`.
