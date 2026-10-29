# Design

## Context

For the motivation see `proposal.md` ("Why"); for the behaviour, the delta specs. How the parts stand
(2026-10-28, branch `docs-and-cuts`):

- **The toolchain.** `crates/driver/src/toolchain.rs` finds a folder with llvm-mingw's layout (`is_toolchain`:
  `bin\c++.exe` and `bin\mingw32-make.exe`); `Found::bin(name)` takes a file name with `.exe`. `install` runs
  Windows' `curl.exe` and `tar.exe`. Callers name Windows files: `main.rs` takes `bin("lldb-dap.exe")` for `dap`
  and `bin("nm.exe")` for `library.rs`.
- **The build.** `build.rs` `build_with_make` runs `bin\mingw32-make.exe -C <libqb> OS=win BITS=64` with `AR`,
  `CC`, `CXX`, `OBJCOPY` and `WINDRES` set to `.exe` files with backslashes, links `program.exe` in the build
  folder and moves it. `write_resources` writes the icon and version resources (`windres`). `main.rs:485` names the
  default output `with_extension("exe")`. The build key and the objects stamp read the toolchain's `bin`.
- **libqb's copy** (`libqb\`) already has QB64pe's Linux path: the `Makefile`'s `OS=lnx` branch links
  `-lGL -lGLU -lX11 -lXrandr -lxcb -ldl -lrt` (and `-lasound` with audio), `-no-pie`, `-rdynamic` when unstripped.
  Of the copy's 14 edits (`tools\libqb\edits\`), one (`d048-console-eof-input-string`) has a Windows-only branch;
  the others touch portable code.
- **Libraries.** `driver\src\library.rs` `find` follows the old compiler's Windows lookup (`.lib`, `.a`, `.o`,
  `.dll`, System32); its result goes to `codegen-cpp` as `Resolved`. `codegen-cpp\src\library.rs` writes
  `LoadLibrary`, `GetProcAddress` and `typedef … (CALLBACK* DLLCALL_x)`. The old compiler's Linux forms:
  `qb64pe.bas` 4639–4930 (lookup), 5041–5110 (`DYNAMIC` lookup), 5219–5300 (link names, `dlopen`), 5473 and 5514
  (typedefs), 5892 (`dlsym`).
- **The debugger.** `crates\dap` starts the `lldb-dap` it is given, with `LLDB_USE_LLDB_SERVER=1` (`LLDB_ENV`),
  which is a fix for lldb's Windows plugin.
- **The runner** (`tools\legacy_tests\run_legacy_tests.py`, 3,186 lines) already branches on `os.name` in a dozen
  places: `press_any_key.py` on Windows, standard input from a file elsewhere. It names executables `.exe` and
  makes copies of `qb64pe.exe`.
- **Packaging.** `vscode\scripts\stage_bundle.py` puts `bin\qb64rust.exe` in the bundle; `package_zip.py` writes
  the `win-x64` zip; `release-vsix.yml` runs on `windows-latest`; `vscode\src\extension.ts:44` starts
  `bundle\bin\qb64rust.exe`; `compiler\toolchain.ts:63` tests for `c++.exe`.
- **CI** builds and tests on `windows-latest` only (`rust.yml`, `vscode-extension.yml`); `licenses.yml` and
  `repo-check.yml` already run on `ubuntu-latest`.

### Measured (2026-10-28)

- This machine has WSL 2 with Ubuntu 24.04.4 LTS: `g++` 13.3.0, `make` and `python3` present, the packages
  `libglu1-mesa-dev`, `libasound2-dev`, `libx11-dev` and `libxrandr-dev` installed; no `lldb-dap`, no `cargo`.
- QB64pe's `setup_lnx.sh` installs on Debian and Ubuntu `build-essential x11-utils mesa-common-dev
  libglu1-mesa-dev libasound2-dev libpng-dev libcurl4-openssl-dev libx11-dev libxcursor-dev libxrandr-dev
  libxinerama-dev libxi-dev`, with lists for Arch, Fedora and Void; it builds with `make OS=lnx BUILD_QB64=y`.
- **The package lists** (task 1.4, 2026-10-29): a fresh `ubuntu:24.04` container (Docker in WSL) with Rust,
  `lldb-19`, Xvfb and `python3` beside the list, started with `--cap-add=SYS_PTRACE --security-opt
  seccomp=unconfined` (Docker's default profile refuses `lldb`'s `personality` call), ran
  `xvfb-run -a cargo test -p qb64rust-driver --test cli -- --ignored` and the slice list (`--jobs 4`) with the whole
  Debian list, then with each package left out. Without `libglu1-mesa-dev`, `libpng-dev`, `libxcursor-dev`,
  `libxrandr-dev`, `libxinerama-dev` or `libxi-dev` nothing builds (85 of 86 CLI tests fail, every slice program);
  without `libasound2-dev` 6 slice programs and without `libcurl4-openssl-dev` 5 fail to compile. Without
  `x11-utils` or `mesa-common-dev` the runs equal the whole list's (84 CLI passes, the 2 failures of `lldb-19`; 469
  slice passes, `d19` the 1 failure, Xvfb's refresh rate), and so do they with both left out: the Debian list drops
  the two. `libx11-dev` stays: the other X packages bring it in, so it cannot be measured apart, and libqb reads its
  headers itself. Fedora's and Arch's lists hold neither of the two and are unchanged.

Measured while building (QB64pe's Linux build from `tools/linux/build_qb64pe.sh`, in WSL):

- **`$IF`:** `WIN` and `WINDOWS` are 0 and `LINUX` is -1 on Linux (`qb64pe.bas` 65–66 reads `_OS$`); the new
  compiler had them fixed to Windows' (`verification\v37_i_places` took its Windows branch). They now follow the
  platform the program is compiled on; the front-end tests read programs as on Windows on either host.
- **END:** libqb's Linux branch prints "Press enter to continue" where Windows' prints "Press any key to continue";
  the runner judges Linux output with Windows' words (D6), which took 236 slice programs off the failures.
- **Libraries:** `libc` loads by `DYNAMIC LIBRARY "/usr/lib/x86_64-linux-gnu/c:6"`; written whole
  (`.../libc.so.6`) it is not found (the old compiler looks for `liblibc.so.so.6`). `./v37dll` is `libv37dll.so`,
  loaded by its absolute path. A `0` to a `BYVAL` parameter of an overloaded header function is `NULL`, which gcc
  on Linux finds ambiguous (`v37_x60`), and so is `v37_i_places`'s `$IF` branch, so the old compiler's C++ fails
  on both. The `v37` family on Linux: four Linux recordings (`v37_g`, `v37_i`, `v37_x60`, and the twin `v37_l1`
  that runs where `v37_g` does on Windows) and `v58_e28` (`kernel32`); 81 pass and 3 are the known ones, as on
  Windows. The library lines of `-z` (`dlopen`, `dlsym`, the typedefs, the prototypes) are equal for 46 programs
  that both compilers take; the rest differ only by `extern "C"`, which the new compiler's `-z` writes from `nm` and
  the old compiler's build adds after `nm` too, and by `v37_x27`'s prototype of a block named `""`, which the old
  compiler writes on Windows too. One difference was the new compiler's and is fixed: an unsigned `_OFFSET`
  result is `uptrszint` in the prototype (a parameter's stays `ptrszint`).
- **gcc's messages** quote names with ‘ ’ in a UTF-8 locale, write a call's arguments after the name and name the
  function of a wrong argument only in a note; `make` writes `make:` and echoes `/usr/bin/g++`. The reader of
  C++ failures (`cpp_errors.rs`) takes them, so `v37_x04`, `x17`, `x23`, `x30`, `x31` and `x60` are errors with a
  summary on Linux too.
- **D-048 to D-053** hold on Linux (`v63`): the old compiler's `INPUT$` waits for ever there too, and the edit's
  Linux branch reads a redirected input (no text mode: a CR and a byte 26 come as they are). Four `v63` programs
  have Linux recordings: `a6` (`LINE INPUT` ends a line at a CR as at an LF), and the crashes of `b2`, `c2`, `d1`.
- **The corpus** (task 5.4): QB64pe's Linux build gives another output than the shared recording for 49 of its
  programs; the new compiler fails 42 of them on Linux and no other (the other 7 it gives as on Windows: D-001,
  D-002, and orders it already fixes). 16 programs have a Linux recording: 9 for what Linux does (`WIDTH` on the
  console, the message of an error in a window program, the file system's errors and locks, no version
  information, `$IF` flags, `/`), and 7 whose calls run in another order with `g++` on Linux (`DECISIONS.md`
  2026-10-28: left to right everywhere in a change after M7). `known.linux.list` has 26 lines: 21 `windows`
  (Civil War Strategy's copies open their data files in another case than their names, Windows' fonts, `cmd`,
  `kernel32`, a header that `g++` finds ambiguous on Linux, `s23`) and 5 `varies` (three sound programs on ALSA's
  null device, two of which crash now and then in QB64pe's build too; `_FILES$`'s order; `d19`'s
  `_DESKTOPREFRESHRATE`, 0 under Xvfb, found in task 1.4's container). Two rules of the runner
  came from it: an exit code is compared by its low 8 bits on Linux (`END -1` is 255), and `s159`'s
  `.normalize` masks a Linux path. One difference was the new compiler's and is fixed: `SWAP` of places whose
  indexes call a FUNCTION finds them left to right (D-054). The run then ends with 490 passes, 11 known failures,
  20 skipped and no failure.
- **The verification suite** (task 5.4): the new compiler fails 77 programs on Linux, each recorded then with
  QB64pe's Linux build. 62 programs have a Linux recording (`verification\README.md`, "Linux recordings", says
  what each family's show); `tests\verification\known.linux.list` has 22 lines: 17 `windows` (`v40`'s PowerShell,
  `v22_d_shell`'s `cmd`) and 5 `varies` (sound on ALSA's null device). Two rules of the runner came from it: the
  separator after `<verification>` in a log is read as Windows', and a recorded log is read without its stack
  frames, which libqb's Linux build does not write. The run then ends with 4,094 passes, 135 known failures (the
  shared list), 16 skipped and no failure; `v59_f7_getstring_main` and `v33_a09` fail only beside 8 jobs (the X
  display).
- **The upstream compile suite and qbasic** (task 5.4): the qbasic suite passes whole (143). Of the compile suite's
  pass list, `glut/screenx`, `screeny` and `screenhide_sub` read the new window's position, which is negative now
  and then under WSLg, in QB64pe's build too (on Windows they print the expected values in an `$IF WINDOWS` branch):
  `tests\upstream\known.linux.list`, 3 `varies`. The list then gives 399 passes, 2 known failures and none
  failing; `audio_sound_test/sound_test` crashed once beside 8 jobs and passed alone, the null device's crash of
  `s129`. Recordings of their own: none (the suite's expected outputs are QB64pe's files).
- **`lldb-dap`** (task 4.2): Ubuntu 24.04 has `lldb-18` (the default `lldb`), `lldb-19` and `lldb-20`, whose
  Python packages exclude each other, so one is installed at a time; each installs `lldb-dap-<n>` and `lldb-<n>`
  only (no plain `lldb` but 18's). 18 offers no data breakpoints; 19 and 20 passed the same 19 of the 26 debugger
  CLI tests before the adapter's Linux fixes. After them 20 passes all, while 19 fails 2 in a fresh container
  (task 1.4): its `lldb-dap` aborts at the first `readMemory`, so no value is shown. Both report a stop at a data breakpoint as `breakpoint`, described `watchpoint <n>` (the adapter
  names it `data breakpoint`). libqb's `error` takes its code in `edi` on Linux, not `ecx` (the adapter reads
  the first argument's register by platform). `g++` writes the column of each C++ statement into the line table
  (`steps.bas:5:16`); without columns (`-gno-column-info`) `lldb` 18 stepped otherwise, so columns stay and the
  tests compare `lldb`'s stops by line on Linux.

## Goals / Non-Goals

**Goals:**
- On Linux x64, the compiler, the extension and the debugger do what they do on Windows, from the same sources and
  the same copy of libqb.
- Every program the suites hold is judged on Linux against what QB64pe's Linux build gives, with as few
  Linux-only recordings as the differences need.
- Windows builds, recordings and runs do not change by a byte, but for the one fix the Linux runs found in the
  C++ both platforms share: `SWAP` of places whose indexes call a FUNCTION (`QB64PE-DIVERGENCES.md` D-054; one
  corpus program's C++ differs, task 8.1).

**Non-Goals:**
- A downloaded or pinned compiler on Linux (user, 2026-10-28; a `SOMEDAY.md` row); Linux distributions without
  glibc (musl, Alpine); 32-bit and ARM.
- Building for another platform than the host.
- The call-site check on Linux; it stays the Windows comparison. The Linux codegen is held by its own snapshots and
  by the Linux `qb64pe`'s `-z` output for the library programs (D3).
- Matching QB64pe's Linux build where its result depends on the machine (working rule 8).

## Decisions

### D1: The system toolchain on Linux, found on `PATH`

The user decided on 2026-10-28 that Linux uses the system's compiler (the panel's view and the alternatives, a
pinned LLVM release or both, are in `DECISIONS.md`'s row). `toolchain::find` takes a `Platform` (D3). On Linux
the lookup is `--toolchain-root`, then `QB64RUST_TOOLCHAIN_ROOT` (a folder with `bin/g++` and `bin/make`), then
`PATH`: `g++`, `make`, `objcopy`, `ar`, `gcc`, each found by a search of `PATH` in the driver (no `which`
process). `Found` gains `Source::System`, and its folder is the folder of the `g++` found. `Found::bin(name)`
becomes `Found::tool(Tool)`, with `Tool::{Cxx, Cc, Ar, Objcopy, Make, Windres, Nm, LldbDap}` mapped to file names
per platform (`c++.exe`, `mingw32-make.exe` on Windows; `g++`, `make` on Linux). On Linux, `lldb-dap` is found by
D5's rule. *Why:* QB64pe's Linux path is the system compiler, and its packages are needed whatever compiler is
used; one lookup function with a platform keeps the Windows order untouched. *Instead of:* `cfg(target_os)`
branches all through `toolchain.rs`. A `Platform` value lets the Linux rules be unit-tested on Windows with fake
folders, and the reverse.

The package hint (`toolchain::packages_message`) reads `ID` and `ID_LIKE` of `/etc/os-release`: `debian` or
`ubuntu` gives `apt-get`, `fedora`/`rhel` gives `dnf`, `arch` gives `pacman`, anything else the generic list. The
lists are QB64pe's less what no build reads, measured in WSL in a fresh container (task 1.4): a package is left
out only if a build of the slice list passes without it. `toolchain install` on Linux checks the tools and the
headers and prints either the folder or the hint. Code: `toolchain.rs` `find_in`, `system_lookup`,
`packages_message`; `main.rs` (the subcommand).

### D2: `build_with_make` per platform

On Linux: `make -C <libqb> -j3 OS=lnx BITS=64`, with `AR`, `CC`, `CXX` and `OBJCOPY` set to the tools found (no
`WINDRES`), the linked file `program` (no `.exe`) in the build folder, then moved to the output path with its mode
kept. Paths go with `/` (the `Makefile`'s Linux branch quotes with `'`). `write_resources` runs only on Windows.
What QB64pe on Linux does with `$EXEICON` and `$VERSIONINFO` (it writes no resource) is checked with the Linux
`qb64pe` and followed, warnings included. The default output name (`main.rs`) takes `Platform::host()`'s
extension. The build key gains the platform as a line, so a build cache can never give a Windows executable to a
Linux build. `toolchain_stamps` stamps the tools found by their own paths when the source is `System` (their bin
folder is `/usr/bin`, too big to stamp whole). Code: `build.rs` `build_with_make`, `build_key`,
`toolchain_stamps`, `objects_stamp`.

### D3: A `Platform` value from the driver to the codegen

`qb64rust_base::Platform { Windows, Linux }` with `Platform::host()` (from `cfg(target_os)`, the only place the
host is read). The driver passes it to `library::find` and puts it in `codegen_cpp::Resolved`, which already carries
what the lookup found. `codegen-cpp\src\library.rs` writes the Linux forms of `qb64pe.bas` for `Platform::Linux`:
`void *DLL_x=NULL;`, `dlopen("<name>",RTLD_LAZY)`, `typedef … (*DLLCALL_x)(…)`, `dlsym` with `dlerror()` for
error 260, and the link names (`../../<name>` or the absolute name) with `-Wl,-rpath <folder>` into the extra
libraries (`libs_extra` in `build.rs`). Sema takes no platform: nothing it checks differs by platform (the library
names it rejects are the same on both). The preprocessor does (found while building, task 3.5): `$IF WIN` and
`$IF LINUX` read the platform, which `parse_settled_on` puts in `PpFlags`; the driver passes the host, and
`parse_settled` and the driver's `frontend` read a program as on Windows, for tests that hold on both hosts. *Why:* the C++ is the only platform-dependent output of the front half,
and `Resolved` is already the bridge from the lookup to the codegen. *Instead of:* a platform in every pass
(nothing to read it); `cfg` in `codegen-cpp` (no Linux snapshot could be made on Windows). Checked by
`codegen-cpp` snapshot tests for each `DECLARE LIBRARY` form on both platforms. The library programs of
`tests\verification\` are also compared, by their `-z` fragments, with the Linux `qb64pe`'s (task 3.5).

### D4: The Linux library lookup

`library::find(kind, written, file_dir, places, platform)`: for Linux it follows `qb64pe.bas` 4639–4930 in order:
the written path (`./` alone is the source's folder, as on Windows), `/usr/lib64/<path>`, `/usr/lib/<path>`,
the bare name, each for `lib<x>.so.<ver>` and `lib<x>.so` (not `STATIC`), `lib<x>.a`, `lib<x>.o`; then the headers
`<x>.h` and `<x>.hpp`. On Linux a version after `.` is kept for `.so.<ver>`, not dropped as on Windows. The
multiarch folders (`/usr/lib/x86_64-linux-gnu`) are not searched, as QB64pe does not search them. That is a row in
`SOMEDAY.md`'s "QB64pe behaviours to review", not a divergence. `nm` (for a library without a header) is the
system's. Code: `library.rs` `find`, `find_linux`; unit tests on fake trees, which run on both platforms.

### D5: `lldb-dap` on Linux

`toolchain::lldb_dap(found, platform)`: on Windows `bin\lldb-dap.exe` of the toolchain, as now. On Linux it is
`bin/lldb-dap` of a toolchain given by option or variable, else the `lldb-dap-<n>` on `PATH` with the highest `<n>`
of at least `LLDB_DAP_MINIMUM` (Ubuntu's `lldb-20` installs `lldb-dap-20`), else `lldb-dap` on `PATH`. A versioned
one of the minimum comes before the plain name, which on Ubuntu 24.04 can be the distribution's default, 18. An
older versioned one is refused, not started: the adapter answers `initialize` with the message, which names it,
its version and the package of the minimum. Started with a warning, 19 would end the session at the first value
the Variables pane reads (the review of the staged files, 2026-10-29). Only a version given by the name is known;
the plain `lldb-dap` is not run to ask (`SOMEDAY.md`, "Other ideas"). `LLDB_ENV` is set on Windows only:
lldb on Linux debugs through `lldb-server` already. The adapter's features (data breakpoints, set next line,
evaluate) rest on `lldb-dap` requests. The oldest `lldb-dap` that passes the debugger CLI tests on Ubuntu 24.04 is
20 (18 offers no data breakpoints; 19's ends at the first `readMemory`, found in task 1.4's container; "Measured";
the user, 2026-10-29, chose 20 over reading memory through `lldb`'s `memory read` for 19). The message names that
version, the versioned package on Debian and Ubuntu (`lldb-20`) and `apt.llvm.org` (`toolchain::LLDB_DAP_MINIMUM`),
and the spec's scenario of a versioned `lldb-dap` names 20. What the adapter does for Linux's `lldb-dap` and `g++` (measured in
4.2): a stop on another line than the one a step went on from is reported at any column; `goto` selects the
program's thread first; a value is written with `memory write` where the backend offers no `writeMemory`; a stop
at a watchpoint is named `data breakpoint`; the code of libqb's `error` is read from `edi`; and a debug build for
Linux puts a procedure's end label under the line before, with a statement where every way out meets. The source paths in `<exe>.debug.json` are written with the
host's separators, as the C++ `#line` directives are; the symbol reader already compares case-sensitively off
Windows (task 4.3 checks it).

### D6: The runner on Linux, and recordings per platform

`run_legacy_tests.py` gets `--platform` (default: the host). It is used only to choose the files a program is
judged by, never the compiler line. The executable's name drops `.exe` off Windows. A program is judged
against `<name>.linux.output` (`.linux.err`, `.linux.out.txt`) when the platform is Linux and the file exists,
else the shared recording. Each suite gets a `known.linux.list` (same format and reasons as `known.list`, plus
`windows`: the program needs Windows, such as `kernel32` or the Windows console API; it is skipped, not run). The
runner's check of the recordings fails on a Linux file equal to the shared one. `--record --platform linux` takes
`--qb-root` as the folder of a Linux `qb64pe` and writes or removes Linux files only (spec). The copies of
`qb64pe.exe` for parallel recording (`--copies-dir`) are made for the Linux `qb64pe` too, with symlinks for the
junction.

The Linux `qb64pe` comes from `tools/linux/build_qb64pe.sh <scratch>`. It runs `git clone --no-checkout` of the
reference clone into the scratch folder, checks out the commit of `libqb\SOURCE.md`, runs `make OS=lnx BITS=64
BUILD_QB64=y -j3`, and prints the folder. It reads the clone only (working rule 2). *Instead of:* QB64pe's Linux
release archive: it is not shown equal to the reference commit, as the Windows release was not (`m6-toolchain`
D1).

Window programs need an X display: in WSL the WSLg display, in CI `xvfb-run -a`. Sound programs run with ALSA's
null device where there is no card (`ALSA_CONFIG_PATH` pointing at a file in `tools/linux/` that maps `default`
to `null`). What fails for want of a device goes to `known.linux.list` as `varies` with the device named. Code:
`run_legacy_tests.py` (`expected_file`, `exe_name`, `load_known`), its tests in `test_run_legacy_tests.py`.

### D7: Working on Linux from this machine

Development stays on Windows; the Linux runs happen in WSL's Ubuntu 24.04. The working tree must not be built in
place from Linux: libqb's objects are built inside `libqb\`, and a Linux build would overwrite the Windows
objects (and the reverse), and `/mnt/c` is slow for 9,000 files. `tools/linux/sync.sh` copies the files of
`git ls-files -co --exclude-standard` (the working tree, uncommitted edits included, nothing staged by it) into
`~/qb64rust` in WSL with `rsync --checksum --delete`. `CARGO_TARGET_DIR` stays inside that folder. Rust comes from
`rustup` in WSL; the packages are installed by the user (`sudo`). *Instead of:* a second clone kept in step by
git (it needs commits, and only the user commits); building in `/mnt/c` with a per-platform objects folder (a
change to libqb's `Makefile`, which is copied, not edited by hand).

### D8: The extension on Linux

`stage_bundle.py --target linux-x64 --binary <qb64rust>` stages `bin/qb64rust` and the notice of the crates for
`x86_64-unknown-linux-gnu` (`cargo tree --target`). `vsce package --target linux-x64`. `extension.ts` names the
bundled binary by `process.platform`; `compiler\toolchain.ts` tests a toolchain folder per platform (`bin/g++` and
`bin/make`) and, on Linux, never offers a download: the message of `qb64rust toolchain path` is shown with
"Copy Install Command" (the command line the hint names) and "Open Output". `installToolchain` runs `qb64rust
toolchain install` and shows its lines. Running a built program (`runQueue.ts`) already starts its own process
group off Windows; a console program runs in the integrated terminal as on Windows. Code: those files and their
unit tests, which take the platform as an input as `discovery.ts` does.

### D9: The Linux download

`package_zip.py` becomes `package_download.py --platform win-x64|linux-x64`. For Linux it writes a `.tar.gz` with
Python's `tarfile`, the binary at mode 0755 and everything else 0644, owner and group 0, and times taken from the
bundle so the archive is the same for the same bundle. Its `--check` holds the archive to the spec. The licence
check (`tools\licenses\check.py`) reads a tar as it reads a zip. `MANUAL.md` gains a Linux section with the
package lines of D1, generated into the manual from the same table `toolchain.rs` uses, through a check that
compares them (`--check` fails if the manual's lines differ), so the two cannot drift.

### D10: CI

- `rust.yml`: a job `linux` on `ubuntu-24.04`: apt install of D1's Debian list plus `lldb` and `xvfb`, then fmt,
  clippy, `cargo test --workspace --no-fail-fast`, the CLI tests that build (`--ignored`), and the slice list
  under `xvfb-run`, all with the system toolchain. It needs no QB64pe: on Linux nothing reads the clone but the
  `--ignored` tests that compare with `qb64pe`, which are skipped there (`cfg(windows)`).
- `vscode-extension.yml`: a job `linux` that builds `qb64rust`, runs the unit tests and the integration tests
  under `xvfb-run`.
- `release-vsix.yml`: a `linux` job builds and checks the Linux `.vsix` and tar and uploads them as artifacts. The
  publishing step waits for both jobs and creates the release only when all four packages pass.
- `tier2` stays manual; it gains a matrix entry for Linux (the full corpus, upstream and verification suites).

Not a gate (working rule 6): the jobs run on push as the Windows ones do; nobody waits on them.

### D11: The Linux runs in the test cadence

A change's close runs the Linux set (in WSL, after `sync.sh`: tier 1 and the slice list on Linux) only when the
change touches what the release build is made from (a file under `crates\`, `Cargo.toml`, `Cargo.lock`), the
runner, libqb's copy or its edits, `tools\linux\`, or a Linux sidecar or list. The merge set into `main` always runs
it, with the full suites on Linux. `tools\close\close.py` gets a `linux` family that `--plan` prints as WSL commands
and that `--check` requires under those conditions. *Why:* a Linux fault shows only on Linux, and any crate can
emit C++ that `g++` takes otherwise (`SWAP`'s order, the end of a procedure in a debug build). The first rule named
only the platform files (`toolchain.rs`, `build.rs`, `library.rs`, `codegen-cpp\src\library.rs`, `dap`); it fell
behind within this change and was widened at the review of its staged files (`DECISIONS.md` 2026-10-29). This is a new rule of the cadence and goes into `DECISIONS.md` and
`tools\close\README.md` in task 7.4.

### D12: Linux recordings and working rule 8

The Linux sidecars hold QB64pe's Linux results, which come from glibc where they come from the C runtime (the
`printf` text of floats, `strtod`). glibc rounds correctly, so these are well-defined values and need no
`QB64PE-DIVERGENCES.md` row. Where QB64pe's Linux build gives a defect the Windows build does not (a crash, a hang),
the fix already made for Windows applies on Linux as well, and its row gains "Linux too". Where it gives a defect
of its own, the defect is kept or fixed by the rule of `DECISIONS.md` 2026-10-08 (a crash fixed, a result kept),
with a row.

## Risks / Trade-offs

- [libqb's Linux path has bugs the Windows path does not] → the Linux `qb64pe` builds on the same libqb, so its
  recordings show them. A crash gets a fix and a row (D12); the copy's edits get their Linux branch where they
  need one (`d048-console-eof-input-string`).
- [The slice list's window and sound programs need devices CI lacks] → `xvfb-run` and ALSA's null device (D6);
  what still needs a real device is `varies` in `known.linux.list`.
- [Many Linux sidecars] → only programs whose Linux result differs get one, and the check removes one that
  equals the shared recording. The count is in the change's closing note, and a family with many sidecars of
  one kind (`printf` text) is looked at for a `.normalize` rule instead.
- [Ubuntu's `lldb-dap` 18 lacks a request the adapter uses] → D5's measurement. The fallback is `apt.llvm.org`'s
  packages, named in the message.
- [WSL is not a real Linux desktop] → it is a real Ubuntu kernel and userland with WSLg's display; the CI job on
  `ubuntu-24.04` is the second machine. A desktop check (GNOME, a terminal outside VS Code) is a task by hand.
- [Distributions other than Ubuntu] → the hint covers Fedora and Arch, but only Ubuntu is tested. The manual says
  so.
- [Platform code reaches every build path] → the Windows closing set runs in full, and the Windows recordings
  cannot move (D6 reads Linux files on Linux only).

## Migration Plan

Nothing to migrate on Windows. Linux is new: the release after this change has two more assets. Rollback is
reverting the change; the Linux sidecars and lists are inert on Windows.

## Open Questions

- How many Linux sidecars the suites need. It changes no spec or task, only the size of task 5.4's recording.
