# Spec Delta

## ADDED Requirements

### Requirement: Emscripten SDK lookup in the extension
The extension SHALL take the Emscripten SDK from the setting `qb64rust.emsdkPath` when it is set, else from
what `qb64rust toolchain path --target wasm` reports (spec `compiler/toolchain`, "The Emscripten SDK"). It
SHALL pass the folder found to every build for the wasm target.

#### Scenario: Setting wins
- **WHEN** `qb64rust.emsdkPath` names an SDK folder and the SDK is installed too
- **THEN** "Run in Browser" builds with the folder of the setting

#### Scenario: The installed SDK
- **WHEN** the setting is empty and the SDK is installed
- **THEN** "Run in Browser" builds with the installed SDK and nothing is offered

### Requirement: Emscripten SDK offer
When "Run in Browser" on Windows needs the SDK and none is found, the extension SHALL ask the user before
downloading anything. The question SHALL name the source, the download size and the disk space, with the
choices "Download" and "Not Now". The extension SHALL NOT download at start, in the background, or for any
choice but "Download". On Linux it SHALL offer no download and show the message of `qb64rust toolchain path
--target wasm`.

#### Scenario: User agrees
- **WHEN** the user runs "Run in Browser" with no SDK and chooses "Download"
- **THEN** `qb64rust toolchain install --target wasm` runs with a progress notification, and the build goes on
  when it succeeds

#### Scenario: User declines
- **WHEN** the user chooses "Not Now" or closes the question
- **THEN** nothing is downloaded, and a message names the command "QB64: Install the Emscripten SDK" and the
  setting `qb64rust.emsdkPath`

#### Scenario: Install fails
- **WHEN** the download fails or a checksum differs
- **THEN** the error of the install is shown with a button to open the "QB64" output, and nothing is built

#### Scenario: A setting that names no SDK
- **WHEN** `qb64rust.emsdkPath` names a folder that is not an SDK of the pinned release
- **THEN** no download is offered; the reason is shown with a button that opens the setting

#### Scenario: A desktop build
- **WHEN** the user runs Build, Run or Debug with no SDK installed
- **THEN** nothing about the SDK is asked

### Requirement: Emscripten SDK install command
The command "QB64: Install the Emscripten SDK" (`qb64rust.installEmsdk`) SHALL ask the same question and act
the same way, on demand; when the SDK is found it SHALL say so, name its folder and download nothing. It
SHALL be unavailable in untrusted workspaces, as builds are.

#### Scenario: Installed already
- **WHEN** the user runs the command and the pinned SDK is installed
- **THEN** a message names its folder and nothing is downloaded

#### Scenario: Untrusted workspace
- **WHEN** the workspace is not trusted and the user runs the command
- **THEN** a message explains that installing requires a trusted workspace and nothing runs
