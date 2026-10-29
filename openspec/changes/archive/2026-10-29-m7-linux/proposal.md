# Proposal

## Why

The project's purpose is a QB64 that anyone can install on Windows, Linux or macOS and use without QB64pe
(`DECISIONS.md` 2026-10-26). Today everything after the front end is Windows only: the driver builds with
llvm-mingw's `mingw32-make.exe` and `OS=win`, the executable is `.exe`, `DECLARE LIBRARY` looks for `.dll` and
`.lib` files, the debugger looks for `lldb-dap.exe`, the extension and the compiler download are `win32-x64`, and CI
builds only on `windows-latest`. Linux comes first (M7, `DECISIONS.md` 2026-10-27): GitHub has a runner for it, and
libqb's Linux path is the one QB64pe exercises most outside Windows. This change is the whole of M7 (user,
2026-10-28).

## What Changes

- **The C++ toolchain on Linux is the system's** (user, 2026-10-28, against the roadmap's "fetched as on
  Windows"): `g++`, `make` and `objcopy` from `PATH` (or `--toolchain-root` / `QB64RUST_TOOLCHAIN_ROOT`, a folder
  with `bin/g++`), the distribution's development packages for OpenGL, GLU, X11, Xrandr, xcb and ALSA, and
  `lldb-dap` from `PATH` (the newest `lldb-dap-<n>` of version 20 or later, else `lldb-dap`; an older
  `lldb-dap-<n>` is named in the error, not started). Nothing is downloaded on Linux.
  `qb64rust toolchain install` on Linux prints the packages to install for the distribution it detects (Debian and
  Ubuntu, Fedora, Arch, as QB64pe's `setup_lnx.sh` lists them) and exits 1. `toolchain path` reports `system`.
  A missing compiler or a missing header names the packages too.
- **The driver builds and links on Linux**: `make OS=lnx BITS=64` against the same copy of libqb. The default output
  is the source name with no extension, as `qb64pe` writes on Linux. The build cache, the objects lock and the stamps
  read the system toolchain's programs.
- **The C++ written for the platform**: what the old compiler writes differently with `os$ = "LNX"` is written the
  same way. That covers `DECLARE DYNAMIC LIBRARY` through `dlopen` and `dlsym` with plain function pointers, the
  link names of `.so` and `.a` files with their `-Wl,-rpath`, and absolute library names. The platform is the
  host's.
- **Shared libraries in `DECLARE LIBRARY`** (moved in from `SOMEDAY.md`): `lib<name>.so.<version>`,
  `lib<name>.so`, `lib<name>.a` and `lib<name>.o`, by the written path, then under `/usr/lib64/` and `/usr/lib/`,
  then by the bare name, in the old compiler's order. A header `.h` or `.hpp` is found as on Windows.
- **The debugger on Linux**: `qb64rust dap` starts the `lldb-dap` it finds and reads a debug build's DWARF
  information as on Windows. The extension's debug launch works unchanged.
- **The test runner runs on Linux**, with recordings judged per platform: where QB64pe's own Linux build prints
  something else (the console's comma zones, D-011; the C runtime's `printf` text of floats), a sidecar
  `<name>.linux.output` (or `.linux.err`) holds the Linux result. It is recorded from `qb64pe` built on Linux from
  the reference commit, and the runner reads it in place of `<name>.output` on Linux. A `known.linux.list` per
  suite holds what cannot pass there.
- **The extension packaged for Linux**: a `linux-x64` `.vsix` with `bin/qb64rust` and the copy of libqb. Its
  toolchain offer on Linux names the packages to install and offers no download. The compiler download for Linux
  is `qb64rust-linux-x64-<version>.tar.gz`. A tag releases all four packages.
- **CI on Linux**: a job on `ubuntu-24.04` installs the packages and runs tier 1, the slice list and the extension's
  tests. A job of the release workflow builds the Linux packages.
- **BREAKING (specs only)**: the Windows-only wording of the toolchain, download, packaging and library specs is
  scoped to Windows. Nothing changes on Windows.

Not in this change: macOS (M8); 32-bit or ARM Linux; a pinned or downloaded compiler for Linux (a `SOMEDAY.md`
row); the multiarch library folders (`/usr/lib/x86_64-linux-gnu`) that the old compiler does not search, which
are kept as QB64pe does with a row in "QB64pe behaviours to review"; the Marketplace listing; the call-site check
on Linux (it stays a Windows comparison).

## Capabilities

### New Capabilities
- `compiler/platforms`: the platforms the compiler targets (Windows x64, Linux x64), how the target is chosen, and
  the conventions that follow from it: the executable's name, path separators in what the compiler writes, and
  `os$`-dependent C++. Also the rule that a program's behaviour on Linux is QB64pe's on Linux.

### Modified Capabilities
- `compiler/toolchain`: "Toolchain lookup", "No toolchain found" and "Toolchain install" become per platform;
  Linux uses the system toolchain and installs nothing.
- `compiler/cli`: "qb64pe-compatible compile" (the default output name has no extension on Linux).
- `compiler/download`: "Download contents", "Usable where it is unpacked" and "The manual" (the Linux archive).
- `editor/packaging`: "Platform package contents", "Release from a tag", "Toolchain offer" and "Install command"
  (the `linux-x64` package; no download on Linux).
- `editor/debug-adapter`: "Adapter start and backend" (`lldb-dap` from `PATH` on Linux).
- `language/external-libraries`: "Finding a library" and "DLLs at the start of the program" (shared objects,
  `dlopen`).
- `testing/golden-corpus`: a new requirement "Recordings per platform" for the runner (sidecars, the Linux known
  lists, recording from the old compiler built on Linux).

## Impact

- `crates/driver`: `toolchain.rs` (the system lookup, the package hint, `install` on Linux), `build.rs`
  (`build_with_make` per platform, the stamps, `program.mk`), `main.rs` (default output name, `dap`'s backend,
  `nm`), `library.rs` (Linux lookup).
- `crates/base`: a `Platform` value. `crates/codegen-cpp`: it arrives with the resolved libraries, and
  `codegen-cpp/src/library.rs` writes `dlopen`/`dlsym` and the Linux typedefs; `-Wl,-rpath` goes to the link line.
- `crates/dap`: the `lldb-dap` name, `LLDB_USE_LLDB_SERVER` on Windows only.
- `tools/legacy_tests/run_legacy_tests.py` (platform sidecars, known lists, the Linux `qb64pe` for `--record`),
  `tools/close/` (the Linux runs are not in the Windows close), `tools/libqb/copy_libqb.py` (unchanged copy;
  each edit's Linux branch checked).
- `vscode/`: `scripts/stage_bundle.py`, `scripts/package_zip.py` (tar.gz), `src/compiler/toolchain.ts`,
  `src/extension.ts` (binary name), `package.json` scripts for the target.
- `.github/workflows/`: `rust.yml` (a Linux job), `vscode-extension.yml` (a Linux job), `release-vsix.yml` (Linux
  packages).
- Recorded outputs: new `*.linux.output` sidecars, and `known.linux.list` per suite.
- Documents: `DECISIONS.md` (the system toolchain, 2026-10-28), `QB64PE-DIVERGENCES.md` (any Linux-only row),
  `SOMEDAY.md` (pinned Linux compiler, multiarch folders), `LAYOUT.md`, `crates/README.md` (Linux build
  instructions), `vscode/README.md`, `MANUAL.md`.
