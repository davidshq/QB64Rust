# Proposal: extension-vsix

## Why

The third of the three changes that make the compiler usable (`DECISIONS.md` 2026-10-18, tenth review `study\35`
§3 item 1). A `.vsix` can already be packaged (`npm run package`, and CI keeps one as a build artifact), but it
holds no compiler. The language server and the check on save from `extension-diagnostics` reach only a user who
builds `qb64rust` from this repository and sets `qb64rust.path`, and Build and Run still call `qb64pe`. Someone
who installs the extension today gets QB64pe with highlighting. The new compiler is in use only when it is in
the `.vsix` and builds the program (user, 2026-10-19: "Bundle + build with it", "GitHub release asset").

## What Changes

- **A platform `.vsix` for Windows x64** (`vsce package --target win32-x64`). It holds `qb64rust.exe` (release
  build) and the copy of libqb (`libqb\`, as `libqb-copy` left it, without built objects), with the licence
  files they need: QB64pe's `COPYING.txt` and `licenses\`, and the licences of the Rust crates in the binary.
- **The bundled binary is found without a setting**: `qb64rust.path`, else the binary in the extension, else
  `qb64rust` on `PATH`. A user who sets nothing gets the language server and the check on save from the new
  compiler.
- **Build, Run and the build task go through `qb64rust`** when it is found. The QB64pe installation (from
  `qb64rust.compilerPath` or `PATH`, as now) gives the toolchain and the folder `DECLARE LIBRARY` and
  `$INCLUDE` look in (`--qb64pe-root`, `--include-root`). The extension gives the copy of libqb with
  `--libqb-root`. A failed build's errors go to Problems from `qb64rust`'s `file:line:column` output.
- **A new setting `qb64rust.buildWith`** (`auto`, `qb64rust`, `qb64pe`; default `auto`, meaning `qb64rust` when
  it is found). When a build fails only on errors marked "not supported yet", the failure message offers
  **Build with QB64pe** for that one build.
- **libqb's objects are built outside the extension's folder.** On the first build, the copy is copied into the
  extension's global storage under the extension's version, and its objects are built there. The extension's own
  folder is never written to. Copies left by older versions are removed.
- **The include root of the language server** with `qb64rust.includeRoot` empty becomes the QB64pe folder
  when one is found, so that the server and a build look includes up in the same place. Without QB64pe it is
  still the binary's folder. **BREAKING** for a user who set `qb64rust.path` and relied on that binary's folder:
  they now set `qb64rust.includeRoot`.
- **Releases**: a workflow started by a tag `vscode-v<version>` builds `qb64rust`, packages the platform
  `.vsix`, checks its contents and attaches it to a GitHub release of this repository. Claude writes the
  workflow. The user pushes the tag. The version goes to 0.2.0.
- Formatting still uses `qb64pe -y` (no formatter of the new compiler, `STATUS.md` "Queued").

## Capabilities

### New Capabilities

- `editor/packaging`: what the released `.vsix` holds, how the extension finds the `qb64rust` binary and the
  copy of libqb it carries, where libqb's objects are built, and how a release is made.

### Modified Capabilities

- `editor/build-run`: "Compiler discovery" finds both compilers, and `qb64rust.buildWith` chooses between them.
  "Build command" builds with `qb64rust` and its errors reach Problems, with the fallback to QB64pe. "Tasks"
  uses the same compiler as Build.
- `editor/language-server`: "Included files" names the include root the client gives when
  `qb64rust.includeRoot` is empty.

## Impact

- `vscode\src`: `compiler\discovery.ts` (the bundled binary), a new `compiler\qb64rust.ts` (build arguments,
  output parsing), `build.ts`, `tasks.ts`, `config.ts`, `languageServer.ts` (include root), a new module that
  prepares libqb in global storage; `package.json` (setting, version, packaging scripts), `.vscodeignore`.
- `vscode\scripts`: a staging script that puts `qb64rust.exe`, `libqb\` and the licence files into a
  git-ignored `vscode\bundle\` before packaging.
- `.github\workflows\`: `vscode-extension.yml` packages the platform `.vsix`, and a new `release-vsix.yml`
  runs on the tag.
- `crates\driver`: none expected. The driver already takes `--libqb-root`, `--qb64pe-root` and
  `--include-root`, and builds the objects in the copy it is given.
- Size: the `.vsix` grows from under 1 MB to roughly the binary (5 MB) plus libqb's sources (52 MB on disk,
  much less compressed). The first build after installing or updating builds libqb's objects (about 40 s for a
  console program, `crates\README.md`).
- Not in scope: macOS and Linux packages (CI is Windows only, `DECISIONS.md` 2026-10-03), the Marketplace or
  Open VSX, and shipping a toolchain (the QB64pe installation is still needed to build).
