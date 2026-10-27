# Proposal

## Why

A build or a debug session with `qb64rust` needs a C++ toolchain and `lldb-dap`, and today the only place it can
take them from is a QB64pe installation (`internal\c\c_compiler`). A user without QB64pe can install the extension
but cannot build, run or debug anything (`study\38` §2, §5.2; `study\37` §5.2). The user decided on 2026-10-26 to
remove that dependency: the toolchain is fetched from its upstream on first need, **only after the user agrees**,
never installed automatically.

## What Changes

- `qb64rust` gets a toolchain of its own: the pinned upstream release `llvm-mingw 20260922` (LLVM 23.1.2, UCRT,
  x86_64 host), downloaded from `github.com/mstorsjo/llvm-mingw`, checked against its SHA-256 and unpacked into a
  per-user folder. It is byte for byte the toolchain of the reference clone (measured, design "Measured"), so every
  recording stays valid.
- New subcommand `qb64rust toolchain install` (downloads, checks, unpacks; the command itself is the user's
  consent) and `qb64rust toolchain path` (says which toolchain a build would use, or that there is none).
- A build and `qb64rust dap` find the toolchain in this order: `--toolchain-root` / `QB64RUST_TOOLCHAIN_ROOT`;
  a QB64pe folder given with `--qb64pe-root` / `QB64RUST_QB64PE_ROOT`; the installed toolchain; the default clone
  (`..\QB64pe` next to the repository). With none, the build fails with a message that names
  `qb64rust toolchain install` and the size of the download. Nothing is downloaded without that command.
- A QB64pe folder is no longer needed to build or debug. Where the old compiler looks in its own folder (the files
  of `DECLARE LIBRARY`, the relative words of the extra link flags), `qb64rust` uses the QB64pe folder when there is
  one, else the include root.
- The extension: a build, Build and Run, a build task or a debug session that finds no toolchain **asks** the user
  (download about 190 MB from GitHub, about 460 MB on disk; Download / Not Now). Only "Download" runs
  `qb64rust toolchain install`, with progress; "Not Now" names the install command and the setting. A command
  "QB64: Install the C++ Toolchain" does the same on demand. A setting `qb64rust.toolchainPath` names a toolchain
  folder instead.
- **BREAKING (extension behaviour):** a `qb64rust` build no longer requires `qb64rust.compilerPath`; QB64pe is
  needed only when `qb64rust.buildWith` resolves to QB64pe or for its fallbacks ("Build with QB64pe", formatting by
  `qb64pe -y`). The language status reports the toolchain as well as the compilers.

Not in this change: the compiler as its own release asset, the Marketplace listing, Linux and macOS, the CI jobs'
own toolchain (they keep QB64pe 4.7.0's release download).

## Capabilities

### New Capabilities
- `compiler/toolchain`: which C++ toolchain a build and the debug adapter use, where it is looked for, and its
  installation by `qb64rust toolchain install` (pinned release, checksum, unpacking, failure cases).

### Modified Capabilities
- `compiler/cli`: "Locating the reference runtime" (the clone is no longer required to build; `--toolchain-root`);
  "Debug adapter subcommand" (takes `--toolchain-root` too); the `toolchain` subcommand is not a file name.
- `editor/build-run`: "Compiler discovery" and "Build command" (a `qb64rust` build needs no QB64pe; the
  toolchain is found or offered).
- `editor/debug-launch`: "The adapter" (started with the toolchain found, not a QB64pe folder).
- `editor/debug-adapter`: "Adapter start and backend" (`lldb-dap` from the toolchain found).
- `editor/packaging`: a new requirement for the toolchain offer (asked, never automatic; where it is stored);
  "Platform package contents" stays without a toolchain.

## Impact

- `crates\driver`: `build.rs` (`Roots` gains the toolchain; `build_with_make`, `build_key`, `objects_stamp`
  read it), a new module `toolchain.rs` (lookup and install), `main.rs` (options, the subcommand, `dap`'s backend),
  `library.rs` (the folder `DECLARE LIBRARY` looks in without a clone). New dependency `sha2` (MIT or Apache-2.0).
  The download and unpacking run Windows' own `curl.exe` and `tar.exe`.
- `crates\dap`: none beyond the path it is given.
- `vscode\src`: `buildPlan.ts`, `debug.ts`, `statusBar.ts`, `config.ts`, `extension.ts`, a new
  `compiler/toolchain.ts`; `package.json` (setting, command); `README.md` ("Install").
- Specs as listed; `DECISIONS.md` (the user's decision of 2026-10-26).
- CLI tests (`crates\driver\tests`) and the extension's unit and integration tests; the closing set as usual.
