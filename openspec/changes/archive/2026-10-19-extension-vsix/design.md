# Design: extension-vsix

## Context

See `proposal.md` for why. What is there now:

- `vscode\package.json` has `npm run package` (`vsce package --allow-missing-repository --skip-license`), and
  `vscode-extension.yml` uploads the resulting `qb64rust-0.1.0.vsix` as a build artifact. The package holds
  `out\src`, the grammar, the language configuration and `node_modules` (`.vscodeignore`), and no binary.
- `compiler\discovery.ts` `findCompiler` finds `qb64pe` (`qb64rust.compilerPath`, then `PATH`) and `qb64rust`
  (`qb64rust.path`, then `PATH`). `statusBar.ts` `CompilerStatus` holds the `qb64pe` discovery and `require()`
  for Build. `languageServer.ts` starts `qb64rust lsp` and gives it the include root from `qb64rust.includeRoot`
  (empty: the server takes the folder of its own binary, `crates\driver\src\main.rs` `lsp`).
- `build.ts` `Builder.build` runs `qb64pe.build` (`-c -x -w <file> -o <exe>`) through the run queue, parses with
  `parseQb64peOutput` and publishes through `Diagnostics` (collection `qb64pe`, cleared on edit and close).
  `tasks.ts` builds a `ProcessExecution` from `qb64pe.buildArgs`.
- The driver (`crates\driver\src\build.rs`) takes `--qb64pe-root` (toolchain at `internal\c\c_compiler`, and the
  folder `DECLARE LIBRARY` looks in), `--libqb-root` (a folder with `internal\c\qbx.cpp`; libqb's objects are
  built into it when missing) and `--include-root`. `QB64RUST_OBJECTS_LOCK` names a lock file for those objects.
  It prints `<file>:<line>:<column>: error: …`, `note:` and `warning:` lines and a summary (spec
  `compiler/cli`, "Diagnostics output"). The QB64pe Windows release works as the root, which CI already relies on.
- `libqb\` is 52 MB on disk, 1,233 tracked files. Built objects (`libqb_make_*.o` and the like) are ignored by
  git but sit in the working tree.

## Goals / Non-Goals

**Goals:**

- One install and one setting (`qb64rust.compilerPath`, or QB64pe on `PATH`) give the new compiler for live
  errors, check on save, Build and Run.
- The packaged files are exactly what the spec lists, and the release checks this.
- No behaviour changes for a user who sets `qb64rust.buildWith` to `qb64pe`, apart from the include root.

**Non-Goals:**

- Formatting through the new compiler. There is none yet.
- Shipping a toolchain. The QB64pe installation already holds one, and shipping mingw would add its licences and
  size for no gain.
- Packages for other platforms, and the Marketplace or Open VSX.
- Mapping C++ build failures back to BASIC lines. As now, they point to the output channel.

## Decisions

### D1. A platform `.vsix` staged from a git-ignored `vscode\bundle\`

`vscode\scripts\stage_bundle.py` (Python, as the repo's other tools) writes `vscode\bundle\`:
`bin\qb64rust.exe` from `target\release`, `libqb\` from the repo's `libqb\` (the tracked files only, read from
`git ls-files libqb`, so the working tree's built objects never ship), and `THIRD-PARTY-NOTICES.txt` (D7). It
also copies the repository's `LICENSE` to `vscode\LICENSE`, where `vsce` looks for it.
`npm run package` becomes `npm run stage && vsce package --target win32-x64`, writing
`qb64rust-win32-x64-<version>.vsix`. `.vscodeignore` keeps `bundle\**`, and `.gitignore` ignores
`vscode\bundle\` and `vscode\LICENSE`. The script takes `--binary <path>` so CI can pass its own build, and fails if the binary or
`libqb\internal\c\qbx.cpp` is missing.

**Alternative:** keep a universal `.vsix` and download the binary on first start. Rejected because it needs a
network fetch, a place to host binaries and checksums, and it fails offline. A platform package is what `vsce` and
VS Code support for native binaries.
**Alternative:** copy from the working tree with exclusions. Rejected because `git ls-files` is the same list
`copy_libqb.py --check` holds to the commit, so nothing built or stray can ship.

### D2. Binary lookup: setting, bundle, `PATH`

`findCompiler` gets an optional `bundled?: string` in `DiscoveryInput`, tried after the setting and before
`PATH`, with `source: "bundle"`. `languageServer.ts` and the builder pass
`context.asAbsolutePath("bundle/bin/qb64rust.exe")`. The bundle comes before `PATH` so that an installed
extension uses the binary it was tested with, not an older one a user built once and left on `PATH`. A
development host has no `bundle\` (it is git-ignored and staged only for packaging), so it falls through to
`PATH` or `QB64RUST_TEST_QB64RUST`, as the tests use now. The language status names the source
(`qb64rust (bundled)`).

### D3. libqb prepared in global storage, under a lock

New module `vscode\src\compiler\libqbStore.ts`, function `prepareLibqb(context): Promise<string>`. It copies
`bundle\libqb` to `<globalStorage>\libqb-<version>\` and returns that path. `<version>` is the extension's
version followed by `bundle\libqb.id`, a hash of libqb's tracked files that staging writes, so that a package of
the same version with other files of libqb (a local package after an edit of the copy) does not reuse the copy and
objects of the earlier files. The copy goes to a sibling temporary
folder and is renamed into place when complete, so an interrupted copy is never used (the spec scenario "Copy
interrupted"). At activation, folders `libqb-*` of other versions are removed (best effort: a folder in use by
another window's build is left for next time). This version's temporary folders are removed only when older than
an hour: a younger one may be another window's copy under way, which removing would make fail. A build sets
`QB64RUST_OBJECTS_LOCK` to `<globalStorage>\libqb-<version>.lock`, so two windows that build at once do not write the same objects at the
same time. The driver already takes that lock (`build.rs` `lock_objects`).

Without a bundle (a development host with `qb64rust.path` set), `prepareLibqb` returns undefined and no
`--libqb-root` is passed. The driver then uses its own default, the repo's `libqb\`, as it does now.

**Alternative:** let the driver build objects in the extension folder. Rejected because the installation folder
belongs to VS Code: it is replaced on update and may be checked against the package's signature.
**Alternative:** a cache folder chosen by the driver (`%LOCALAPPDATA%`). Rejected because the extension already has
a per-user folder that VS Code cleans up on uninstall, and the driver should stay told rather than guessing.

### D4. The build with `qb64rust`: `compiler\qb64rust.ts`

A new module beside `compiler\qb64pe.ts`, with the same shape: `buildArgs(file, roots)` and `build(queue, binary,
file, roots, onOutput)`. `roots` holds `qb64pe` (the QB64pe folder, from the `qb64pe` discovery), `libqb`
(from D3) and `include` (D6). The command line is `-x -w <file> -o <exe> --qb64pe-root <qb64pe> [--libqb-root
<libqb>] --include-root <include>`. `-c` is QB64pe's console switch and the driver does not take it. The
executable path is `qb64pe.executableFor`, shared. The output is parsed by a new pure `parseQb64rustOutput.ts`:
the `file:line:column: error|warning:` lines become messages with a column (a `note:` line is kept with the error
before it, as the message's second line). It also reports whether every error was marked "not supported yet", for
D5. The summary line and the build folder's path are not messages. A file name relative to the source's folder
is resolved against it.

`CompilerMessage` gets an optional `column`. `Diagnostics.publish` gets a `source` argument. A `qb64rust` build's
messages go into a second collection the `Diagnostics` class owns, `qb64rust-build`, with source `qb64rust`,
cleared on edit and close like the `qb64pe` one. The language client's collection is never written by the
extension. One collection per producer is the rule `extension-diagnostics` set, and the check on save does not
replace a build's errors.

### D5. `qb64rust.buildWith` and the fallback

`config.ts` gets `buildWith(): "auto" | "qb64rust" | "qb64pe"`. `Builder.build` picks the compiler. `auto` uses
`qb64rust` when it is found, `qb64rust` requires it (error naming `qb64rust.path`), and `qb64pe` uses QB64pe.
QB64pe is required in every case. With `qb64rust` it gives the toolchain, so `CompilerStatus.require()` runs
first as now. When a `qb64rust` build under `auto` fails and the parser says every error is marked, the error
message carries the button "Build with QB64pe", which reruns that build through `qb64pe.build`. Under an explicit
`qb64rust` no button is offered, because the user chose that compiler. `CompilerStatus` keeps the text `qb64pe`
(the item is about finding QB64pe, and the server's own item already names `qb64rust`) and ends its detail line
with `builds with qb64rust` or `builds with qb64pe`. It refreshes on a change of `buildWith` and `compilerPath`,
and when the language server has looked for the binary (`LanguageServer.restart` calls `CompilerStatus.update`).
Tests answer the button through `Builder.showError`, which they replace, and read `Builder.lastCommand`: VS
Code gives each extension its own API object, so a test cannot stub `vscode.window` for the extension.

**Alternative:** fall back to QB64pe automatically. Rejected because it hides that the new compiler failed, and a
program would build with a different compiler from one save to the next.

### D6. One include root for the server and the build

`config.ts` `effectiveIncludeRoot(qb64peFolder, binaryFolder)` returns `qb64rust.includeRoot` (resolved against
the workspace folder as now), else the QB64pe folder, else the binary's folder. `languageServer.ts` passes it in
the initialization options and restarts on a change of `compilerPath` too. `qb64rust.ts` passes it as
`--include-root`. QB64pe looks includes up in its own folder after the source's, so a program written for QB64pe
finds the same files. The bundled binary's own folder holds nothing to include. The spec in `editor/language-server`
names the order.

### D7. Licence notice of the binary

`stage_bundle.py` runs `cargo metadata --format-version 1 --locked`. It walks the resolved dependency graph of
`qb64rust-driver` (normal dependencies, the target `x86_64-pc-windows-msvc`, as `cargo tree -e normal` would).
For each package it writes its name, version, licence expression and the text of its `LICENSE*`/`COPYING*` files
from its manifest folder into `THIRD-PARTY-NOTICES.txt`. A package with no licence file fails the script, which
names it, so a new dependency cannot ship without its text. The notice also points to `libqb\COPYING.txt` and
`libqb\licenses\`. The extension's own `LICENSE` (MIT, copied from the repo root by the script) replaces
`--skip-license`. The repository had no `LICENSE` and its crates stated none; the user chose MIT for the whole
repository while this was built (`DECISIONS.md` 2026-10-19), so a root `LICENSE` and `license = "MIT"` on the
workspace's crates came with this change. One crate, `astro-float-num`, ships no licence file; its MIT text, taken
from its repository, is `vscode\scripts\licenses\astro-float-num.txt`, which the script uses for a crate without
one. `cargo tree -e normal --target x86_64-pc-windows-msvc` gives the list (it evaluates target-specific
dependencies, which `cargo metadata`'s graph leaves as `cfg` expressions); `cargo metadata` gives each crate's
folder. Only the workspace's own crates, whose source `cargo tree` prints as a local folder, are left out; a
git dependency is listed like a registry crate.

### D8. The release workflow

`.github\workflows\release-vsix.yml`, on `push: tags: ['vscode-v*']`, Windows. It compares the tag's version
with `vscode\package.json` (fails first if they differ), then runs `cargo build --release --locked -p
qb64rust-driver`, `npm ci`, the unit tests, `npm run package` with `--binary`, and the content check. It then
runs `gh release create <tag> <vsix> --title … --notes-file …`, with `contents: write` permission. The content
check is `vscode\scripts\check_vsix.py`: it opens the `.vsix` (a zip) and holds it to the spec's list (required
files present, no `.ts`, `.o` or `c_compiler`). `vscode-extension.yml` runs the same check on its package. The
integration tests stay in `vscode-extension.yml`. The release does not repeat them because a tag is pushed on a
commit that CI has already run (CI is not a gate, working rule 9; the user pushes the tag).

The release notes come from a section of `vscode\CHANGELOG.md` (new, kept by hand per release), which `vsce`
also shows.

### D9. Version 0.2.0

`package.json` goes to `0.2.0`, the first release with the new compiler. The publisher id `qb64rust` stays, since
no marketplace is involved.

## Where the code lives

| Decision | Place |
|---|---|
| D1 | `vscode\scripts\stage_bundle.py`, `vscode\package.json` (`package`), `vscode\.vscodeignore`, `.gitignore` |
| D2 | `vscode\src\compiler\discovery.ts` (`findCompiler`, `DiscoveryInput.bundled`), `languageServer.ts`, `build.ts` |
| D3 | `vscode\src\compiler\libqbStore.ts` (`prepareLibqb`, `removeOtherVersions`), `extension.ts` |
| D4 | `vscode\src\compiler\qb64rust.ts`, `vscode\src\compiler\parseQb64rustOutput.ts`, `diagnostics.ts` |
| D5 | `vscode\src\config.ts` (`buildWith`), `build.ts` (`Builder.build`), `statusBar.ts`, `package.json` (setting) |
| D6 | `vscode\src\config.ts` (`effectiveIncludeRoot`), `languageServer.ts`, `compiler\qb64rust.ts` |
| D7 | `vscode\scripts\stage_bundle.py` (notice), `vscode\LICENSE` (staged) |
| D8 | `.github\workflows\release-vsix.yml`, `vscode\scripts\check_vsix.py`, `vscode-extension.yml`, `vscode\CHANGELOG.md` |
| Tasks | `vscode\src\tasks.ts`: `provideTasks`/`resolveTask` become async, prepare libqb and take the build's command line from the builder's choice; when `qb64rust` would build but cannot (not found, libqb not prepared), the task prints why and fails rather than running `qb64pe` |

## Risks / Trade-offs

- [The first build after an install or update takes about 40 s more, while libqb's objects are built] → The
  status item shows "building libqb (first build)" while `make` builds them. The README says so.
- [A QB64pe version other than 4.7.0 gives a different toolchain] → The copy of libqb is compiled by that
  toolchain. The release is tested against the 4.7.0 release as CI does. The README names 4.7.0 as the tested
  version, and a build that fails in C++ points to the output channel as now.
- [The `.vsix` is large (sources of libqb)] → It is measured at packaging and the number goes in
  `vscode\DEVELOPMENT.md`. libqb's `parts` that a Windows build never compiles stay, because the `Makefile`
  names them and filtering would be a second copy rule to maintain.
- [Two windows build at once into the same storage] → the objects' lock (D3).
- [The include root changes for users of `qb64rust.path`] → `CHANGELOG.md` and the setting's description say it.
  It is the root a build uses, so the server and the build now agree.
- [A dependency without a licence file blocks packaging] → This is intended (D7). The script names the crate.

## Migration Plan

Install the `.vsix` over 0.1.0. Settings carry over, and `qb64rust.buildWith` defaults to `auto`. To go back,
set `qb64rust.buildWith` to `qb64pe`, or install 0.1.0. The tag is pushed by the user after the change is merged.
