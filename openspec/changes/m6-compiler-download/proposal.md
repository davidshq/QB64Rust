# Proposal

## Why

The compiler reaches users only inside the VS Code extension's `.vsix`. Someone who works from a command line,
another editor or a build script has no way to get `qb64rust` short of building this repository. The user decided
on 2026-10-26 that the compiler is released as a download of its own (`DECISIONS.md`; `study\38` §5.2 row 1).
`m6-toolchain` made that possible: the C++ toolchain is fetched by `qb64rust toolchain install`, and QB64pe is no
longer needed.

A release binary today cannot be used on its own. Without `--libqb-root` or `QB64RUST_LIBQB_ROOT`, it looks for
libqb in the repository it was built from (`build.rs` `find_libqb`), a path that exists only on the build machine.

## What Changes

- **A compiler download on each release.** The `vscode-v<version>` release gets a second asset beside the `.vsix`:
  `qb64rust-win-x64-<version>.zip`, from the same build. It holds one folder with `qb64rust.exe`, the copy of
  libqb (tracked files only, as in the `.vsix`), the repository's licence, the notice of the Rust crates linked
  into the binary, and a short manual. The release workflow checks the zip's contents and licence notices before
  it publishes anything, as it checks the `.vsix`.
- **The driver finds libqb beside its executable.** The lookup becomes `--libqb-root`, `QB64RUST_LIBQB_ROOT`,
  `libqb` in the executable's folder, then the repository the binary was built from. When a build has to write
  libqb's objects into a folder it cannot write to (for example, the zip unpacked under `Program Files`), it fails
  with a message that names the folder and `--libqb-root`. Today the failure is a bare "writing …: access denied".
- **`qb64rust --version` and `qb64rust --help`.** `--version` prints `qb64rust <version>` and `--help` prints the
  usage, both with exit status 0. Today both are "unknown option".
- **One version number.** The workspace version (`Cargo.toml`, now 0.1.0) is raised to the extension's
  (`package.json`, 0.2.0) and kept equal to it: the release fails before building when the two differ.
- **A manual**, `MANUAL.md`, shipped in the zip: what the download is, unpacking it, `qb64rust toolchain install`,
  compiling and running a program, the options a user needs, the exit statuses, where it differs from QB64pe
  (`QB64PE-DIVERGENCES.md`), and how to report a problem.

Not in this change: the Marketplace listing (still the user's open question), Linux and macOS builds, an
installer or `PATH` setup, a compiler-only release tag.

## Capabilities

### New Capabilities
- `compiler/download`: the standalone compiler download: its name, its layout and contents, the licence notices
  it carries, the manual, and that it works unpacked anywhere the user can write.

### Modified Capabilities
- `compiler/cli`: "Locating the reference runtime" (`libqb` beside the executable; a libqb folder that cannot be
  written); a new requirement for `--version` and `--help`.
- `editor/packaging`: "Release from a tag" (the release also attaches the compiler download, and the workspace
  version must equal the extension's).

## Impact

- `crates\driver`: `build.rs` (`find_libqb` gains the executable's folder; `refresh_objects` names the folder
  and `--libqb-root` when it cannot write there), `main.rs` (`--version`, `--help`, the usage line).
- `Cargo.toml` (workspace version 0.2.0) and `Cargo.lock`.
- `vscode\scripts`: a new `package_zip.py` that writes and checks the zip from the staged bundle, with its tests;
  `check_vsix.py --tag` also compares the workspace version.
- `tools\licenses\check.py`: checks the notices of a zip as it checks those of a `.vsix`.
- `.github\workflows\release-vsix.yml`: makes, checks and attaches the zip; the notes say what the zip is.
- New `MANUAL.md`; `LAYOUT.md` rows; a `vscode\CHANGELOG.md` line under 0.3.0; `DECISIONS.md` (the user's choice
  of one release and one version number).
- Tests: unit tests in `build.rs`, CLI tests (`crates\driver\tests\cli.rs`), the Python tests of the scripts, and
  one real zip built, unpacked under `target\` and used by hand.
