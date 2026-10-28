# Spec Delta

## ADDED Requirements

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
