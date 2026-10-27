# Design

## Context

For the motivation see `proposal.md` ("Why"); for the behaviour, the delta specs. How the parts stand
(2026-10-26, branch `more-stuff` after `qbasic-testcases`):

- **One root does two jobs.** `driver\src\build.rs` `Roots { clone, libqb }`: `clone` (found by `find_root`:
  `--qb64pe-root`, `QB64RUST_QB64PE_ROOT`, `..\QB64pe` next to the repository; it must hold
  `internal\c\qbx.cpp`) gives the toolchain (`build_with_make` runs `internal\c\c_compiler\bin\mingw32-make.exe`
  and passes `ar.exe`, `gcc.exe`, `c++.exe`, `objcopy.exe`, `windres.exe` from the same folder), the stamps of
  the toolchain in `build_key` and `objects_stamp`, and the old compiler's folder that `DECLARE LIBRARY`
  (`driver\src\library.rs` `Places::root`) and the extra link flags (`extra_link_args`) look in. `main.rs`
  `dap` takes `lldb-dap.exe` from the same folder. A build without a clone fails even when no library is used.
- **The extension needs QB64pe for every build.** `vscode\src\buildPlan.ts` `planBuild` returns "none" when
  QB64pe is not found, before it looks at the compiler chosen; `qb64rust.ts` `buildArgs` always passes
  `--qb64pe-root`; `debug.ts` starts `qb64rust dap --qb64pe-root <QB64pe folder>`.
- **The bundle** (`vscode\scripts\stage_bundle.py`) carries `bin\qb64rust.exe`, the copy of libqb and the
  notices; no toolchain, and spec `editor/packaging` says it carries none. That stays so.
- **QB64pe's own setup** (`..\QB64pe\setup_mingw.cmd`) downloads the latest `llvm-mingw-<tag>-ucrt-x86_64.zip`
  from `github.com/mstorsjo/llvm-mingw` into `internal\c\c_compiler`; the clone's was taken on its setup and
  holds LLVM 23.1.2.

### Measured (2026-10-26)

- `llvm-mingw-20260922-ucrt-x86_64.zip` (LLVM 23.1.2): 190,725,905 bytes, SHA-256
  `e3ad77d117a4bea19a7a3b333341824d79a5a371004a10e25b8504e7b3047666` (the digest GitHub gives for the asset and
  the one computed after download). Unpacked: 9,053 files, 736 MB. **Every one of them equals the clone's
  `internal\c\c_compiler` file of the same name, byte for byte**; the clone has 5 files more, all
  `python\lib\python3.14\encodings\__pycache__\*.pyc`, written by Python when `lldb` ran. So the pinned release is
  the toolchain every recording and every passing run was made with.
- The folders of the other three targets (`aarch64-w64-mingw32` 87 MB, `armv7-w64-mingw32` 55 MB,
  `i686-w64-mingw32` 69 MB) and `lib\clang\23\lib\linux` (62 MB) are not read by an x86_64 build: 463 MB stay.
- An x86_64 subset of the clone's toolchain (408 MB) given as `--qb64pe-root` built and ran a program
  (`PRINT 1 / 3`, `SQR` and `COS` of `_FLOAT`) with the same output as the clone; the two executables have the
  same size and differ in 178 bytes (the link time stamp and offsets of the libqb copy's path, which differed by
  one character). Its `lldb` 23.1.2 stopped at `main` through `lldb-server`, and its `lldb-dap` started.
- This machine's `%SystemRoot%\System32\curl.exe` is curl 8.21.0 (Schannel) and `tar.exe` is bsdtar 3.8.9
  (libarchive), which lists the zip. Both ship with Windows since Windows 10 1803 (April 2018) and Windows Server
  2019.

## Goals / Non-Goals

**Goals:**
- A user with the extension and nothing else can build, run and debug after one question answered "Download".
- The toolchain a build uses is, byte for byte, the one the recordings were made with.
- QB64pe users and this repository's development and CI setups keep working unchanged.

**Non-Goals:**
- Choosing or updating the toolchain version at run time; a new release is a new pin in a change of its own.
- Removing a toolchain (`toolchain remove`), or more than one toolchain at once.
- Hosting a repack of our own; Linux and macOS; the compiler as its own release asset.
- Windows before 10 1803 / Server 2019: the floor `qb64rust.exe` and VS Code already set, apart from 10 before
  1803 and Server 2016, which install the toolchain by hand. Going lower is a row in `SOMEDAY.md`.
- CI's toolchain: `rust.yml` keeps QB64pe 4.7.0's release download, given as `QB64RUST_QB64PE_ROOT`.

## Decisions

### D1: The pinned upstream release, whole

The install takes `llvm-mingw-20260922-ucrt-x86_64.zip` from the upstream release by URL and checks the SHA-256
of "Measured". *Why:* it is the reference clone's toolchain (measured), so no recording can move; upstream
already hosts it, so nothing is redistributed by this project (GNU make in it is GPLv3: an obligation that stays
upstream's). *Instead of:* the latest release (`20261020`, LLVM 23.1.4: a toolchain change that can move float
results, `study\00` §8); a repack of our own as a release asset (smaller, 108 MB, but hosting, licence notices
for every component, and one more thing to rebuild per pin); QB64pe 4.7.0's release archive (not shown equal to
the clone). The constants (URL, file name, size, SHA-256, folder name) live in `crates\driver\src\toolchain.rs`.

### D2: `qb64rust` installs, with Windows' own `curl.exe` and `tar.exe`

`qb64rust toolchain install` runs `%SystemRoot%\System32\curl.exe -fL --retry 3` into a partial file and
`%SystemRoot%\System32\tar.exe -xf` to unpack it, by full path (Git for Windows puts GNU `tar`, which cannot read a
zip, first on many `PATH`s). The SHA-256 is computed by the `sha2` crate (RustCrypto, MIT or Apache-2.0), the
only new dependency. While `curl` runs, the driver prints `downloading <n>%` lines from the partial file's size
against the known size, then `checking`, `unpacking`, and the folder at the end; the extension shows those lines
as its progress. *Why:* one implementation for the CLI and the extension; Schannel uses Windows' certificate store
and the system proxy settings `curl` honours; no TLS stack in the binary. *Instead of:* HTTP and zip crates
(`ureq` with `rustls`, `zip`: a TLS stack and about ten crates for `cargo deny` and the notices); PowerShell's
`Invoke-WebRequest` (slow on large files in Windows PowerShell 5.1, and another process to drive); the extension
in TypeScript (the CLI would get nothing). With either executable missing, the install fails with a message that
names the release URL and `--toolchain-root` for a manual install.

### D3: Where it goes, and when a folder counts

The install root is `%LOCALAPPDATA%\qb64rust\toolchain` (`QB64RUST_TOOLCHAIN_HOME` replaces it: for the tests,
and for a user who wants it on another drive). The install takes an exclusive lock on `<root>\install.lock`; when
`<root>\llvm-mingw-20260922-ucrt-x86_64` holds the marker it reports the folder and stops (so a second install
that waited on the lock downloads nothing). Otherwise it deletes that folder if it is there without the marker
(an interrupted install), downloads to `<root>\download\<zip>`, checks the checksum, unpacks into `<root>` (the
zip's top folder is the release's name), removes the three other targets' folders and `lib\clang\23\lib\linux`,
deletes the zip, and writes `qb64rust-toolchain.txt` (release, SHA-256) into the folder **last**. A folder counts as
installed only with that marker, so an interrupted install is never used. *Why:* the CLI and the extension share
it (the extension's own storage would not be found by the CLI); the marker written last makes the install all or
nothing without renaming a folder of 9,000 just-written files, which fails with "access denied" while an
antivirus scanner holds one open. *Instead of:* unpacking aside and renaming into place (that failure). Folders of
other releases are left alone: there is none yet, and the change that moves the pin decides what becomes of the
old one. Code: `toolchain.rs` `install`, `install_from`, `installed`.

### D4: The toolchain is a root of its own

`Roots` becomes `{ toolchain, qb64pe: Option<PathBuf>, libqb }`. `toolchain` is a folder with llvm-mingw's layout
(`bin\c++.exe`, `bin\mingw32-make.exe`; QB64pe's `internal\c\c_compiler` has the same), found by
`toolchain::find` in the order of spec `compiler/toolchain`: `--toolchain-root`, `QB64RUST_TOOLCHAIN_ROOT`, an
explicitly given QB64pe folder's `internal\c\c_compiler`, the installed one, the default clone's. An explicitly
given QB64pe folder comes before the installed toolchain because a user who names a folder means it, and CI names
its 4.7.0 download so. `build_with_make` takes `toolchain.join("bin")`; `build_key` and `objects_stamp` stamp the
toolchain's `bin` (by name relative to the toolchain, size and time, as now). `find` returns the source with the
folder, which `toolchain path` prints on the second line (first line: the folder), for the extension to parse.
`dap` takes `bin\lldb-dap.exe` from `find`, and gains `--toolchain-root`. With nothing found, `find`'s error is
the message of spec "No toolchain found", built in one place (`toolchain::missing_message`).

### D5: The old compiler's folder, without QB64pe

`library::Places::root` and `extra_link_args` (and the `build_key` lines that hash the files named by the extra
link flags) take `roots.qb64pe`, else the include root. `find_root` no longer fails a build: it returns `None` when
nothing was given and the default clone is missing; a folder given explicitly that is not a QB64pe folder stays an
error, as now. An empty `QB64RUST_QB64PE_ROOT` means "no QB64pe folder", so the tests can build as a machine
without the clone does. *Why:* the include root is where the user's own files are looked for when no compiler
folder exists (`compiler/cli`, "Compiler root for included files"), and a QB64pe user keeps the old behaviour.
Code: `main.rs` (where `Places::new` and `Roots` are made), `build.rs`.

### D6: The extension asks, then runs the command

New module `vscode\src\compiler\toolchain.ts`: `findToolchain` (the setting `qb64rust.toolchainPath`, else
`qb64rust toolchain path` run with `--qb64pe-root` of the QB64pe found, when there is one) and
`offerToolchain`. The offer is a modal question (`showWarningMessage` with `modal: true`, which a build waits on;
a toast could be hidden and leave the build waiting): "Building and debugging need a C++ toolchain (llvm-mingw
20260922 with LLVM 23.1.2, from github.com/mstorsjo/llvm-mingw): a 190 MB download, about 460 MB on disk. Download
it now?" with "Download" and "Not Now" (closing the dialog is "Not Now"; its message names the install command and
`qb64rust.toolchainPath`, for a user with a toolchain folder of their own). The offer is made only when
`qb64rust toolchain path` ran and found nothing: a `qb64rust.toolchainPath` that is not a toolchain, a missing
`qb64rust` binary, or a lookup that failed for another reason (a folder given by a variable that is not one) is
shown with its reason, since an install would not change it. "Download" runs
`qb64rust toolchain install` under `withProgress` (a notification, cancellable: cancel kills the process, which
leaves a partial zip or an unmarked folder that the next install deletes, as the cancel message says), writes its
lines to the "QB64" output, and on success runs `findToolchain` again.
`planBuild` checks the compiler chosen first and asks for QB64pe only for a QB64pe build or its fallbacks; for a
`qb64rust` build it calls `findToolchain`, then `offerToolchain` when interactive (a task listing with no
toolchain lists the task and offers when it runs). `qb64rust.ts` `buildArgs` passes `--toolchain-root` and
passes `--qb64pe-root` only when QB64pe is found. `debug.ts` passes `--toolchain-root`. `statusBar.ts` adds the
toolchain to the language status, with the install command as its action when none is found. The command
`qb64rust.installToolchain` is declared in `package.json` with `enablement: isWorkspaceTrusted`; the setting
`qb64rust.toolchainPath` has `scope: machine` (a workspace cannot set the folder whose programs run).

### D7: Tests

- The install, in process (unit tests in `toolchain.rs`, no network): `install` is `install_from(url, sha256,
  home)` with the pinned constants, and the tests call `install_from` with a small zip of a fake toolchain they
  write, given by a `file:///` URL (`curl` reads it): install; already installed; a wrong checksum; an unmarked
  folder left by an interrupted install; two installs at once (two threads); a URL that fails. The released binary
  has no way to take another URL or checksum.
- CLI tests (`crates\driver\tests\cli.rs`, no network): the lookup order through `qb64rust toolchain path` with
  fake toolchain folders (an empty `bin\c++.exe` and `bin\mingw32-make.exe` are enough to be found) and
  `QB64RUST_TOOLCHAIN_HOME`; a build with no toolchain anywhere; `toolchain remove` and `toolchain.bas`.
- The `--ignored` CLI tests that build: one builds and runs a program with `--toolchain-root` set to the clone's
  `internal\c\c_compiler` and `QB64RUST_QB64PE_ROOT` empty, one finds a `DECLARE LIBRARY` header under the include
  root the same way, and one runs `dap` the same way; the rest keep the clone. These are the no-clone path; the
  slice list is not run again without the clone, as the toolchain's bytes are the clone's (measured).
- One real install, by hand, recorded in `tasks.md`: `qb64rust toolchain install` on this machine.
- The extension's unit tests: `findToolchain`'s order; the offer's two answers and a failed install, with the
  window and the process stubbed as the existing tests stub them; `planBuild` without QB64pe.

## Risks / Trade-offs

- [The release disappears upstream] → llvm-mingw has kept every release since 2018; if it goes, the constant
  points at a copy attached to a release of this repository (same bytes, same SHA-256), a one-line change.
- [GitHub unreachable, a proxy that `curl` does not see, or no `curl.exe`/`tar.exe`] → the message names the
  URL and `--toolchain-root` / `qb64rust.toolchainPath`; a zip fetched by hand and unpacked anywhere works.
- [Antivirus scanning slows unpacking 9,000 files] → measured once in the install task; the progress says
  "unpacking" so it does not look hung.
- [A user with QB64pe and an installed toolchain gets the installed one] → the same bytes (measured); a named
  `qb64rust.compilerPath` QB64pe is still the include root and the libraries' folder.
- [Changing `Roots` touches every build path] → the full closing set runs with the clone as now; the `--ignored`
  tests of D7 run the path without it.
- [A second pin leaves the first one's 460 MB on disk] → none exists yet; the change that moves the pin removes or
  keeps it, deliberately.

## Migration Plan

Nothing to migrate: QB64pe users are found as before (D4 order), and the setting `qb64rust.compilerPath` keeps
its meaning. Rollback is reverting the change; an installed toolchain folder left behind is inert.
