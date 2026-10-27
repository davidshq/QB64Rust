# Spec Delta

## MODIFIED Requirements

### Requirement: Compiler discovery
The extension SHALL locate QB64pe from the setting `qb64rust.compilerPath` if set, otherwise from `qb64pe` on
`PATH`, the `qb64rust` binary as spec `editor/packaging` says, and the C++ toolchain as spec `editor/packaging`
("Toolchain offer") says. It SHALL show a language status item for `qb64rust` files (in the status bar's language
indicator) that says which compiler builds and whether its needs are met: for `qb64rust`, the binary and the
toolchain; for QB64pe, QB64pe. QB64pe is needed only for a build by QB64pe and for its fallbacks.

#### Scenario: Setting wins
- **WHEN** `qb64rust.compilerPath` points to an existing `qb64pe.exe` and another `qb64pe` is on `PATH`
- **THEN** the configured path is used

#### Scenario: Not found
- **WHEN** `qb64rust.buildWith` is `qb64pe`, no setting is configured and `qb64pe` is not on `PATH`
- **THEN** the language status shows the compiler as missing (error severity), and build and run commands show an
  error message with a button that opens the `qb64rust.compilerPath` setting

#### Scenario: No QB64pe, qb64rust builds
- **WHEN** QB64pe is not found, the `qb64rust` binary is found, `qb64rust.buildWith` is `auto`, and the toolchain
  is found
- **THEN** the language status says that `qb64rust` builds, at information severity, and Build works

#### Scenario: No toolchain yet
- **WHEN** QB64pe is not found, the `qb64rust` binary is found, and no toolchain is found
- **THEN** the language status says the C++ toolchain is not installed, with an action that offers it

#### Scenario: Setting changed
- **WHEN** the user changes `qb64rust.compilerPath` or `qb64rust.toolchainPath`
- **THEN** discovery runs again without reloading the window

#### Scenario: Status names the building compiler
- **WHEN** QB64pe and the `qb64rust` binary are both found and `qb64rust.buildWith` is `auto`
- **THEN** the language status says that `qb64rust` builds

### Requirement: Build command
The command `qb64rust.build` SHALL save the active document and compile it to an executable next to the source
file (same base name), showing compiler output in an output channel named "QB64". It SHALL build with `qb64rust`
when `qb64rust.buildWith` is `qb64rust`, or `auto` and the binary is found, with the toolchain found (or offered,
when there is none), QB64pe's folder as the include root when QB64pe is found, and SHALL build with QB64pe
otherwise.

#### Scenario: Successful build
- **WHEN** the user runs Build on a saved `hello.bas` without errors
- **THEN** `hello.exe` (Windows) or `hello` (other platforms) exists next to `hello.bas` and a success message is
  shown

#### Scenario: Built by qb64rust
- **WHEN** `qb64rust.buildWith` is `auto`, the `qb64rust` binary and a toolchain are found, and the user runs
  Build on a correct program
- **THEN** the output channel shows the `qb64rust` command line, and the executable is produced by it

#### Scenario: Built by qb64rust without QB64pe
- **WHEN** QB64pe is not found, the toolchain is installed, and the user runs Build on a correct program
- **THEN** the executable is produced by `qb64rust` with the installed toolchain

#### Scenario: Build offers the toolchain
- **WHEN** a build by `qb64rust` finds no toolchain
- **THEN** the toolchain is offered (spec `editor/packaging`, "Toolchain offer"); the build goes on only after it
  is installed, and is cancelled with a message when the user declines

#### Scenario: Failed build
- **WHEN** the program has a BASIC-level error
- **THEN** no run happens, the error appears in Problems at the reported file and line, and the output channel
  holds the full compiler output

#### Scenario: Failed build with qb64rust
- **WHEN** a build with `qb64rust` fails because line 3, column 5 of an included `lib.bi` has an error
- **THEN** Problems shows the error under `lib.bi` at line 3, column 5, with source `qb64rust`, and it is removed
  when that document is edited

#### Scenario: Only unsupported forms
- **WHEN** a build with `qb64rust` under `auto` fails, every error is marked "not supported yet", and QB64pe is
  found
- **THEN** the failure message offers "Build with QB64pe", which builds that program once with QB64pe without
  changing `qb64rust.buildWith`

#### Scenario: qb64pe chosen
- **WHEN** `qb64rust.buildWith` is `qb64pe`
- **THEN** Build calls QB64pe as before, whether or not the `qb64rust` binary is found

#### Scenario: qb64rust chosen but missing
- **WHEN** `qb64rust.buildWith` is `qb64rust` and no `qb64rust` binary is found
- **THEN** Build shows an error that names `qb64rust.path` and does not fall back to QB64pe

#### Scenario: Untitled document
- **WHEN** the active document has never been saved
- **THEN** the user is asked to save it first and the build does not start until it has a file path
