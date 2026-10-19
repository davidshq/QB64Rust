# Spec Delta

## MODIFIED Requirements

### Requirement: Compiler discovery
The extension SHALL locate QB64pe from the setting `qb64rust.compilerPath` if set, otherwise from `qb64pe` on
`PATH`, and the `qb64rust` binary as spec `editor/packaging` says. It SHALL show a language status item for
`qb64rust` files (in the status bar's language indicator) that says whether QB64pe was found and which compiler
builds. A build with `qb64rust` needs QB64pe too, for its toolchain.

#### Scenario: Setting wins
- **WHEN** `qb64rust.compilerPath` points to an existing `qb64pe.exe` and another `qb64pe` is on `PATH`
- **THEN** the configured path is used

#### Scenario: Not found
- **WHEN** no setting is configured and `qb64pe` is not on `PATH`
- **THEN** the language status shows the compiler as missing (error severity), and build, run, check and format commands show an error
  message with a button that opens the `qb64rust.compilerPath` setting

#### Scenario: Setting changed
- **WHEN** the user changes `qb64rust.compilerPath`
- **THEN** discovery runs again without reloading the window

#### Scenario: Status names the building compiler
- **WHEN** QB64pe and the `qb64rust` binary are both found and `qb64rust.buildWith` is `auto`
- **THEN** the language status says that `qb64rust` builds

### Requirement: Build command
The command `qb64rust.build` SHALL save the active document and compile it to an executable next to the source
file (same base name), showing compiler output in an output channel named "QB64". It SHALL build with `qb64rust`
when `qb64rust.buildWith` is `qb64rust`, or `auto` and the binary is found, using QB64pe's folder for the
toolchain and as the include root, and SHALL build with QB64pe otherwise.

#### Scenario: Successful build
- **WHEN** the user runs Build on a saved `hello.bas` without errors
- **THEN** `hello.exe` (Windows) or `hello` (other platforms) exists next to `hello.bas` and a success message is
  shown

#### Scenario: Built by qb64rust
- **WHEN** `qb64rust.buildWith` is `auto`, the `qb64rust` binary and QB64pe are found, and the user runs Build on
  a correct program
- **THEN** the output channel shows the `qb64rust` command line, and the executable is produced by it

#### Scenario: Failed build
- **WHEN** the program has a BASIC-level error
- **THEN** no run happens, the error appears in Problems at the reported file and line, and the output channel
  holds the full compiler output

#### Scenario: Failed build with qb64rust
- **WHEN** a build with `qb64rust` fails because line 3, column 5 of an included `lib.bi` has an error
- **THEN** Problems shows the error under `lib.bi` at line 3, column 5, with source `qb64rust`, and it is removed
  when that document is edited

#### Scenario: Only unsupported forms
- **WHEN** a build with `qb64rust` under `auto` fails and every error is marked "not supported yet"
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

### Requirement: Tasks
The extension SHALL provide tasks of type `qb64rust` with actions `build` and `run` for the active or a given file,
so that users can bind them in `tasks.json`. The `build` task SHALL use the compiler and the command line that
the Build command would use for that file.

#### Scenario: Build task
- **WHEN** the user runs the task "qb64rust: build" from the task list
- **THEN** the active file is compiled as by the Build command

#### Scenario: Build task with qb64rust
- **WHEN** Build would use `qb64rust` and the user runs the task "qb64rust: build"
- **THEN** the task's terminal runs `qb64rust` with the same arguments as the Build command
