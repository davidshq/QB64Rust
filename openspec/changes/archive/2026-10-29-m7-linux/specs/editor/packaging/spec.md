## MODIFIED Requirements

### Requirement: Platform package contents
The released `.vsix` files SHALL target `win32-x64` and `linux-x64`, one package each, and each SHALL hold the
`qb64rust` binary for its platform built in release mode (`bin\qb64rust.exe`, `bin/qb64rust`), the copy
of libqb (its `Makefile`, `internal\c` without built objects, `licenses` and `COPYING.txt`) with its `libqb.id`,
and a notice of the licences of the Rust crates linked into the binary. Neither SHALL hold the extension's TypeScript sources, its
tests or fixtures, or a C++ toolchain.

#### Scenario: Listing the package
- **WHEN** the files of the released `.vsix` are listed
- **THEN** they include the `qb64rust` executable, the copy's `internal\c\qbx.cpp` and `COPYING.txt`, `libqb.id`,
  and the crate licence notice, and include no `.ts` file, no `.o` file and no `c_compiler` folder

#### Scenario: The Linux package
- **WHEN** the extension from `qb64rust-linux-x64-<version>.vsix` is installed on Linux
- **THEN** its `bundle/bin/qb64rust` is an ELF executable that the language server starts, and the package holds
  no `.exe` file

### Requirement: Release from a tag
Pushing a tag `vscode-v<version>`, where `<version>` equals the version in the extension's `package.json` and the
workspace version in `Cargo.toml`, SHALL build the binary on Windows and on Linux. From each binary it SHALL package
that platform's `.vsix` and compiler download (spec `compiler/download`), check each against its contents
requirement, and attach all four to a GitHub release of that tag. A tag whose version differs from either SHALL
fail the release without publishing anything, as SHALL a failed check or build of any package.

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

#### Scenario: Four packages
- **WHEN** the tag `vscode-v0.4.0` is pushed with matching versions
- **THEN** the release holds `qb64rust-win32-x64-0.4.0.vsix`, `qb64rust-linux-x64-0.4.0.vsix`,
  `qb64rust-win-x64-0.4.0.zip` and `qb64rust-linux-x64-0.4.0.tar.gz`, and the Linux `.vsix` and tar hold the same
  `qb64rust` file

#### Scenario: The Linux build fails
- **WHEN** the Windows packages pass their checks and the Linux build fails
- **THEN** no release is created and nothing is published

### Requirement: Toolchain offer
When a build, a build task or a debug session on Windows needs the toolchain and none is found, the extension SHALL
ask the user before downloading anything. The question SHALL name the source, the download size and the disk
space, with the choices "Download" and "Not Now". The extension SHALL NOT download at start, in the background, or
for any choice but "Download". On Linux it SHALL offer no download: it SHALL show the message of `qb64rust
toolchain path`, which names the packages, with a button that copies the install command.

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

#### Scenario: Linux without g++
- **WHEN** the user builds on Ubuntu with no `g++`
- **THEN** no download is offered, the message names the `apt-get install` line, a button copies it to the
  clipboard, and the build is cancelled

### Requirement: Install command
The command "QB64: Install the C++ Toolchain" (`qb64rust.installToolchain`) SHALL ask the same question and act
the same way, on demand; when the toolchain is already installed it SHALL say so and download nothing. On Linux it
SHALL run `qb64rust toolchain install` and show what it prints: that the system toolchain is present, or the
packages to install. It SHALL be unavailable in untrusted workspaces, as builds are.

#### Scenario: Installed already
- **WHEN** the user runs the command and the pinned toolchain is installed
- **THEN** a message names its folder and nothing is downloaded

#### Scenario: Untrusted workspace
- **WHEN** the workspace is not trusted and the user runs the command
- **THEN** a message explains that installing requires a trusted workspace and nothing runs

#### Scenario: The command on Linux
- **WHEN** the user runs the command on Linux with `g++` and the packages present
- **THEN** a message says the system toolchain is used, names its folder, and nothing is downloaded
