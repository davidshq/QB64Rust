# editor/packaging Specification

## Purpose

Describes the released extension package: what the Windows `.vsix` carries (the `qb64rust` binary, the copy of
libqb and their licences), how the extension finds and prepares what it carries, and how a release is made.

## Requirements

### Requirement: Platform package contents
The released `.vsix` SHALL target `win32-x64` and SHALL hold the `qb64rust` binary built in release mode, the copy
of libqb (its `Makefile`, `internal\c` without built objects, `licenses` and `COPYING.txt`) with its `libqb.id`,
and a notice of the licences of the Rust crates linked into the binary. It SHALL NOT hold the extension's TypeScript sources, its
tests or fixtures, or a C++ toolchain.

#### Scenario: Listing the package
- **WHEN** the files of the released `.vsix` are listed
- **THEN** they include the `qb64rust` executable, the copy's `internal\c\qbx.cpp` and `COPYING.txt`, `libqb.id`,
  and the crate licence notice, and include no `.ts` file, no `.o` file and no `c_compiler` folder

### Requirement: Bundled binary lookup
The extension SHALL find the `qb64rust` binary in this order: the setting `qb64rust.path` when it is set, else the
binary carried in the extension, else `qb64rust` on `PATH`. The language server and a build SHALL use the binary
found this way.

#### Scenario: Nothing set
- **WHEN** the extension is installed from the `.vsix`, `qb64rust.path` is empty and no `qb64rust` is on `PATH`
- **THEN** the language server starts from the binary carried in the extension, and its status says so

#### Scenario: Setting wins over the bundled binary
- **WHEN** `qb64rust.path` names another existing `qb64rust` binary
- **THEN** that binary runs the language server and builds, not the bundled one

#### Scenario: Development install without a bundled binary
- **WHEN** the extension runs from its source folder, which carries no binary, and `qb64rust` is on `PATH`
- **THEN** the binary on `PATH` is used

### Requirement: libqb objects outside the extension folder
The extension SHALL NOT write into its own installation folder. Before the first build with `qb64rust`, it SHALL
copy the carried copy of libqb into its global storage, in a folder named by the extension's version and the
package's `bundle\libqb.id` (a hash of libqb's files written when the package is made, so that a package of the
same version with other files of libqb does not reuse an earlier copy), and SHALL
pass that folder to the compiler, so libqb's objects are built there. Copies left by other versions SHALL be
removed when the extension starts.

#### Scenario: First build after installing
- **WHEN** the user builds a program for the first time after installing version 0.2.0
- **THEN** a copy of libqb exists in the extension's global storage under a folder named for 0.2.0, the build's
  libqb objects are written there, and no file of the installation folder has changed

#### Scenario: Update removes the old copy
- **WHEN** version 0.2.1 starts and a copy for 0.2.0 exists in global storage
- **THEN** the 0.2.0 copy is removed

#### Scenario: Copy interrupted
- **WHEN** the copy into global storage was interrupted before it finished
- **THEN** the next build makes the copy again instead of using the partial one

### Requirement: Release from a tag
Pushing a tag `vscode-v<version>`, where `<version>` equals the version in the extension's `package.json` and the
workspace version in `Cargo.toml`, SHALL build the binary, package the `win32-x64` `.vsix` and the compiler
download of spec `compiler/download` from that one binary, check each against its contents requirement, and
attach both to a GitHub release of that tag. A tag whose version differs from either SHALL fail the release
without publishing anything, as SHALL a failed check of either package.

#### Scenario: Matching tag
- **WHEN** the tag `vscode-v0.2.0` is pushed, and `package.json` and `Cargo.toml` both say `0.2.0`
- **THEN** a release `vscode-v0.2.0` exists with `qb64rust-win32-x64-0.2.0.vsix` and
  `qb64rust-win-x64-0.2.0.zip` attached, and the `qb64rust.exe` in each is the same file

#### Scenario: Mismatched tag
- **WHEN** the tag `vscode-v0.2.1` is pushed and `package.json` says `0.2.0`
- **THEN** the workflow fails before packaging and no release is created

#### Scenario: The compiler's version differs
- **WHEN** the tag `vscode-v0.3.0` is pushed, `package.json` says `0.3.0` and `Cargo.toml` says `0.2.0`
- **THEN** the workflow fails before building, naming both versions, and no release is created

#### Scenario: The download fails its check
- **WHEN** the `.vsix` passes its check and the zip does not
- **THEN** no release is created and neither file is published

### Requirement: Toolchain lookup in the extension
The extension SHALL take the C++ toolchain from the setting `qb64rust.toolchainPath` when it is set, else from what
`qb64rust toolchain path` reports when given the folder of the QB64pe found by `qb64rust.compilerPath` or `PATH`
(spec `compiler/toolchain`, "Toolchain lookup"). It SHALL pass the folder found to every build and debug session.

#### Scenario: QB64pe user
- **WHEN** no toolchain is installed and QB64pe is found
- **THEN** builds use QB64pe's toolchain and nothing is offered

#### Scenario: Setting wins
- **WHEN** `qb64rust.toolchainPath` names a toolchain folder and the toolchain is installed too
- **THEN** builds use the folder of the setting

### Requirement: Toolchain offer
When a build, a build task or a debug session needs the toolchain and none is found, the extension SHALL ask the
user before downloading anything. The question SHALL name the source, the download size and the disk space, with
the choices "Download" and "Not Now". The extension SHALL NOT download at start, in the background, or for any
choice but "Download".

#### Scenario: User agrees
- **WHEN** the user builds with no toolchain and chooses "Download"
- **THEN** `qb64rust toolchain install` runs with a progress notification, and the build goes on when it succeeds

#### Scenario: User declines
- **WHEN** the user chooses "Not Now" or closes the question
- **THEN** nothing is downloaded, the build is cancelled with a message naming the command "QB64: Install the C++
  Toolchain" and the setting `qb64rust.toolchainPath`, and the next build asks again

#### Scenario: Install fails
- **WHEN** the download fails or its checksum differs
- **THEN** the error of `qb64rust toolchain install` is shown with a button to open the "QB64" output, and the
  build is cancelled

#### Scenario: A setting that names no toolchain
- **WHEN** `qb64rust.toolchainPath` names a folder that is not a toolchain, or the `qb64rust` binary is not found,
  and a build or a debug session needs the toolchain
- **THEN** no download is offered (an install would not change the result); the reason is shown with a button that
  opens the setting, and the language status reads "C++ toolchain not found" with that action instead of the install

#### Scenario: Install cancelled
- **WHEN** the user cancels the install's progress notification
- **THEN** the build is cancelled with a message saying nothing is installed and that what was downloaded is removed
  by the next install

#### Scenario: Start without a toolchain
- **WHEN** the extension starts and no toolchain is found
- **THEN** no question is asked and nothing is downloaded until a build or debug session needs it

### Requirement: Install command
The command "QB64: Install the C++ Toolchain" (`qb64rust.installToolchain`) SHALL ask the same question and act
the same way, on demand; when the toolchain is already installed it SHALL say so and download nothing. It SHALL be
unavailable in untrusted workspaces, as builds are.

#### Scenario: Installed already
- **WHEN** the user runs the command and the pinned toolchain is installed
- **THEN** a message names its folder and nothing is downloaded

#### Scenario: Untrusted workspace
- **WHEN** the workspace is not trusted and the user runs the command
- **THEN** a message explains that installing requires a trusted workspace and nothing runs
