# Design

## Context

For the motivation see `proposal.md` ("Why"); for the behaviour, the delta specs. How the parts stand
(2026-10-26, branch `more-stuff` after `m6-toolchain`):

- **libqb's lookup.** `driver\src\build.rs` `find_libqb`: `--libqb-root`, `QB64RUST_LIBQB_ROOT`, else
  `env!("CARGO_MANIFEST_DIR")/../../libqb`, a path compiled into the binary. A candidate counts only with
  `internal\c\qbx.cpp`. The extension always passes `--libqb-root` (`vscode\src\compiler\qb64rust.ts`), so the
  fallback has only ever served this repository's own binaries.
- **libqb's objects are built inside the copy.** `build_with_make` runs `make -C <libqb>`. Before that,
  `refresh_objects` writes `<libqb>\objects.stamp`, the first write into the folder when the objects are missing
  or stale, and returns `writing <path>: <io error>` when it fails. The extension copies libqb into its global
  storage for this reason (spec `editor/packaging`, "libqb objects outside the extension folder").
- **The bundle.** `vscode\scripts\stage_bundle.py` writes `vscode\bundle\`: `bin\qb64rust.exe`, `libqb\` (the
  tracked files, 1,233 today), `libqb.id`, `THIRD-PARTY-NOTICES.txt`, and `vscode\LICENSE` (copied from the
  repository's, packed by vsce as `extension/LICENSE.txt`). `npm run package` stages, then packs.
- **The checks.** `vscode\scripts\check_vsix.py` checks the tag against `package.json` and the `.vsix` against a
  list of required files and forbidden suffixes, then runs `tools\licenses\check.py --vsix`. That script maps
  each inventory notice to `extension/bundle/<path>` (the component's `package_prefix`) and the repository's
  `LICENSE` to `extension/LICENSE.txt`, and compares the texts.
- **The release.** `.github\workflows\release-vsix.yml` on a `vscode-v*` tag: tag check, `cargo build --release`,
  the extension's tests, `npm run package`, the `.vsix` check, `gh release create` with the `.vsix`.
- **Versions.** The workspace version is `0.1.0` and is printed nowhere. `package.json` is `0.2.0`, the released
  `vscode-v0.2.0`. Its `CHANGELOG.md` has `## 0.3.0 (unreleased)`, and `package.json` stays at 0.2.0 until the user
  tags a release.
- **Options.** `main.rs` `parse_args` returns "unknown option" for anything starting with `-` that it does not
  know, including `--version` and `--help`. `lsp`, `dap` and `toolchain` are recognised only as the first argument
  (`main`).

## Goals / Non-Goals

**Goals:**
- The zip and the `.vsix` of a release carry the same `qb64rust.exe` and the same libqb files, checked by the same
  inventory.
- A user who unpacks the zip anywhere they can write needs no option and no variable.
- Nothing changes for the extension, this repository's development binaries or CI.

**Non-Goals:**
- Putting `qb64rust` on `PATH`, an installer, a `winget` or Scoop manifest.
- Writing libqb's objects outside the unpacked folder (as the extension does). See D3.
- A commit hash or build date in `--version`.
- Linux and macOS archives (their builds do not exist yet).

## Decisions

### D1: One release, two assets, one binary

The `vscode-v<version>` release attaches the zip beside the `.vsix`. Both are made in the same job from the one
`target\release\qb64rust.exe` and the one staged `vscode\bundle\`, so they cannot differ. *Why:* the user's choice
(2026-10-26). It keeps one tag, one workflow and one changelog, and a bug report's version means one binary.
*Instead of:* a `qb64rust-v*` tag and workflow of its own (two release schedules and two version numbers for one
compiler). Code: `.github\workflows\release-vsix.yml`.

### D2: The layout mirrors QB64pe's: the executable at the top

```
qb64rust-<version>\
  qb64rust.exe
  libqb\            (tracked files only; Makefile, internal\c, licenses, COPYING.txt, SOURCE.md)
  LICENSE.txt       (the repository's LICENSE)
  THIRD-PARTY-NOTICES.txt
  MANUAL.md
```

*Why:* `qb64rust.exe` at the top makes the folder the compiler root for included files (spec `compiler/cli`), as
`qb64pe.exe`'s folder is for QB64pe, so a user's `$INCLUDE` libraries go beside it. `libqb` beside the executable
is then the one extra place `find_libqb` needs to look. A top folder named by version keeps two unpacked releases
apart. *Instead of:* the bundle's own layout (`bin\qb64rust.exe` with `libqb\` one level up). That would make
`bin\` the include root and need a lookup of `..\libqb`, which would then also match the extension's bundle, where
the extension already passes the option. `libqb.id` is left out: only the extension uses it.

### D3: libqb beside the executable, and a clear failure when it cannot be written

`find_libqb(option)` becomes a thin wrapper over `find_libqb_in(option, env, exe_dir, repo)`, which the unit tests
call with folders they make. The order is `--libqb-root`, `QB64RUST_LIBQB_ROOT`, `<exe dir>\libqb`, the
repository's `libqb`, each counting only with `internal\c\qbx.cpp`, as now. `exe_dir` is
`std::env::current_exe()`'s parent; when that fails, the step is skipped. The "not found" message names the
places looked in. In this repository, `target\release\` and `target\debug\` hold no `libqb`, so development
binaries keep finding the repository's copy, and the CLI tests are unchanged.

`refresh_objects`'s write of `objects.stamp` maps a failure to: `cannot write into the copy of libqb at <dir>
(libqb's objects are built there): <io error>; unpack qb64rust where you can write, or give a copy in such a
folder with --libqb-root <dir>`. Only that first write gets the message; the later writes are by `make`, after it
has succeeded. *Why:* the objects stay where they are built today, so no new state, cache key or clean-up exists
for the standalone case. The usual place to unpack a zip (Downloads, Desktop, a folder under the profile) can be
written to. The one common case that cannot, `Program Files`, gets a message that says what to do. *Instead of:*
building objects in `%LOCALAPPDATA%\qb64rust\libqb\<id>` as the extension does (a copy step, an id, and the old
copies to remove, for a case the message covers); copying libqb there on first use (the same). Code: `build.rs`
`find_libqb`, `find_libqb_in`, `refresh_objects`.

### D4: `--version`, `--help` in `main`

Handled in `main` before `parse_args`, like `lsp`: the exact word as the only argument. `--version` prints
`qb64rust {CARGO_PKG_VERSION}`, and `--help` / `-h` prints `USAGE` on stdout, each with exit status 0. Either word
with another argument prints `qb64rust: \`--version\` takes no other argument` and the usage, with exit status 1.
`USAGE` gains the line `qb64rust --version | --help`. *Why:* the two things a person types first at an unknown
command, and the version a bug report needs. Code: `main.rs` `main`, `USAGE`.

### D5: One version number, checked at the tag

The workspace version goes to `0.2.0` (equal to `package.json`). `check_vsix.py --tag` also reads the workspace
version from `Cargo.toml` (`tomllib`, `[workspace.package] version`) and fails with both numbers when it differs
from `package.json`'s. That step already runs first in the workflow, before anything is built. Raising the
version for a release is then two edits, `package.json` and `Cargo.toml` (and `Cargo.lock` by `cargo`), named in
the header comment of `release-vsix.yml`, the only place the release procedure is written. *Why:* the `--version` a user reports must
name the release they downloaded. *Instead of:* `--version` printing the extension's version through a build
script reading `package.json` (the compiler crate reading the editor's manifest); a test in tier 1 that keeps the
two equal at every commit (`package.json` is meant to lag until the user tags, see Context). Code:
`vscode\scripts\check_vsix.py` `check_tag`.

### D6: `package_zip.py` writes and checks the zip

New `vscode\scripts\package_zip.py`, run after `npm run package` (the bundle is staged then):
`package_zip.py --out <dir>` writes `qb64rust-win-x64-<version>.zip` (version from `package.json`) from
`vscode\bundle\` and the repository (`LICENSE`, `MANUAL.md`), in the layout of D2, with `zipfile` and
`ZIP_DEFLATED`, entries sorted and timestamped from the files. `package_zip.py --check <zip>` checks the top
folder name against the file name, the required files of spec `compiler/download`, the forbidden suffixes and
folders (`.o`, `.a`, `c_compiler/`, `bundle/`, `.ts`), and runs `tools\licenses\check.py --zip <zip>`. Its exit
status and messages follow `check_vsix.py`'s (`package_zip: …`). *Why:* the bundle already holds exactly the
files the zip needs, staged and tested. Reading it means the two packages share their file list by construction.
*Instead of:* a second staging from the repository (two lists of what libqb ships); folding the zip into
`check_vsix.py` (the zip is not the extension). Code: `vscode\scripts\package_zip.py`, its tests in
`vscode\scripts\test_package_zip.py` (a fake bundle in a temporary folder, as `test_stage_bundle.py` does).

### D7: The licence check learns the zip

`tools\licenses\check.py` `check(root, files, components, package=None, layout=VSIX)` takes a layout: the
package path of the repository's `LICENSE` and a function from a component's `package_prefix` path to the
package's path. `VSIX` is today's mapping (`extension/LICENSE.txt`, the prefix as given). `zip_layout(top)` maps
`LICENSE` to `<top>/LICENSE.txt` and replaces `extension/bundle/` with `<top>/`. `--zip <file>` takes the top
folder from the zip's single top entry. `inventory.toml` is unchanged. *Why:* one inventory and one comparison for
both packages. A notice added to the inventory is then required in both. Code: `tools\licenses\check.py`, its
tests in `tools\licenses\test_check.py`.

### D8: The release workflow

After "Check the package", a step runs `python scripts/package_zip.py --out $env:RUNNER_TEMP\zip` and
`python scripts/package_zip.py --check <zip>`, and records the file. "Release" passes both files to one
`gh release create`, so a failed zip check publishes nothing. The title becomes `QB64Rust $tag`. The notes keep
the changelog section, and `check_vsix.py --notes-out` appends one fixed paragraph: the `.vsix` is the extension;
the zip is the compiler alone for a command line, see its `MANUAL.md`. *Why:* one `gh release create` call is all
or nothing; two would leave a release with one asset when the second failed. Code:
`.github\workflows\release-vsix.yml`, `check_vsix.py`.

### D9: The manual

`MANUAL.md` at the repository's root, written for a QB64 programmer at a command prompt, about two pages:
requirements (Windows 10 1803 or later, x64); unpack, then `qb64rust toolchain install` (size, where it goes,
`QB64RUST_TOOLCHAIN_HOME`, `--toolchain-root` for a folder of one's own); `qb64rust -x prog.bas -o prog.exe`, then
running it; `-w`, `-z`, `-y … -o`, `--include-root`, `--debug`, `-f:` settings, `toolchain path`, `--version`;
the exit statuses (0, 1, 3); the folder must be writable (D3); differences from QB64pe 4.7.0 (`DIVERGENCES.md` on
GitHub); issues on GitHub. A Python test (`test_package_zip.py`) checks that every `--option` and subcommand the
manual names in code spans appears in `USAGE` (read from `main.rs`), the scenario of spec `compiler/download`
"An option in the manual". *Why at the root:* it documents the command line as a whole. `crates\README.md` stays
the developer's build-and-test text (the config rule for `crates\README.md`). Code: `MANUAL.md`.

### D10: Tests

- Unit tests in `build.rs` for `find_libqb_in`: the option wins over a `libqb` beside the executable; the
  variable wins over it; beside the executable wins over the repository's; a folder without `qbx.cpp` is passed
  over; none found names the places. For the unwritable copy: `objects.stamp` made a folder, so its write fails,
  gives the message with the folder and `--libqb-root`.
- CLI tests (`crates\driver\tests\cli.rs`): the three scenarios of "Version and help", with the version taken from
  `env!("CARGO_PKG_VERSION")`.
- Python: `test_package_zip.py` (layout, required files, a missing notice, a forbidden `.o`, the top folder that
  does not match the name, the manual's options against `USAGE`); `test_check.py` (the zip layout); a
  `test_check_vsix.py` case for a `Cargo.toml` version that differs.
- By hand, once, recorded in `tasks.md`: `npm run package`, `package_zip.py --out target\m6-zip` and `--check`;
  unpack the zip under `target\m6-zip\unpacked`; from a folder outside the repository, with `QB64RUST_LIBQB_ROOT`
  and `QB64RUST_QB64PE_ROOT` empty and the clone's toolchain given by `QB64RUST_TOOLCHAIN_ROOT` (the same bytes as
  the pinned release, `m6-toolchain` design "Measured"), build and run a `hello.bas`. Check that the objects were
  built in the unpacked `libqb`, and that `--version` prints `qb64rust 0.2.0`. A real release is not run: the
  user pushes the next tag.

## Risks / Trade-offs

- [A user unpacks under `Program Files`] → the message of D3 says why and what to do. The manual says so first.
- [Antivirus or SmartScreen flags an unsigned `qb64rust.exe` downloaded from the web] → the same as the `.vsix`'s
  binary today. Signing is out of scope, and a row goes in `SOMEDAY.md` if the user wants it.
- [Raising the workspace version changes `Cargo.lock` and every crate's `CARGO_PKG_VERSION`] → its one reader is
  the language server's `serverInfo` (`crates\lsp\src\server.rs`), which then reports the release's version. No
  test pins `0.1.0`. The build cache key does not include it.
- [The manual drifts from the usage] → the option test of D9. Wording drift is caught at review only.
- [`current_exe` resolves a symlink or a path through a junction] → the folder it gives is still the executable's
  real folder, where `libqb` was unpacked.

## Migration Plan

Nothing to migrate: the new lookup step comes after the option and the variable, and before the repository's copy,
which no unpacked binary can see. Rollback is reverting the change. A release already made keeps its assets.
