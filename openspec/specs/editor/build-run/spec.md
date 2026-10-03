# editor/build-run Specification

## Purpose
Lets the user compile and run the current QB64 program from VS Code with the installed QB64pe compiler, as F5 /
F11 did in the old IDE.

## Requirements

### Requirement: Compiler discovery
The extension SHALL locate the compiler from the setting `qb64rust.compilerPath` if set, otherwise from `qb64pe`
on `PATH`. It SHALL show a language status item for `qb64rust` files
(in the status bar's language indicator) that says whether a compiler was found.

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

### Requirement: Build command
The command `qb64rust.build` SHALL save the active document and compile it to an executable next to the source
file (same base name), showing compiler output in an output channel named "QB64".

#### Scenario: Successful build
- **WHEN** the user runs Build on a saved `hello.bas` without errors
- **THEN** `hello.exe` (Windows) or `hello` (other platforms) exists next to `hello.bas` and a success message is
  shown

#### Scenario: Failed build
- **WHEN** the program has a BASIC-level error
- **THEN** no run happens, the error appears in Problems at the reported file and line, and the output channel
  holds the full compiler output

#### Scenario: Untitled document
- **WHEN** the active document has never been saved
- **THEN** the user is asked to save it first and the build does not start until it has a file path

### Requirement: Run commands
The command `qb64rust.run` SHALL run the existing executable for the active document in an integrated terminal
with the source folder as working directory. `qb64rust.buildAndRun` SHALL build and, only if the build succeeds,
run.

#### Scenario: Build and run
- **WHEN** the user runs Build and Run on a correct program
- **THEN** the program starts in a terminal named "QB64" after the build finishes

#### Scenario: Run without executable
- **WHEN** the user runs Run and no executable exists for the document
- **THEN** an error message offers to build first

#### Scenario: Program arguments
- **WHEN** `qb64rust.runArguments` is set to `a b`
- **THEN** the program is started with arguments `a b` (visible to it as `COMMAND$`)

### Requirement: Tasks
The extension SHALL provide tasks of type `qb64rust` with actions `build` and `run` for the active or a given file,
so that users can bind them in `tasks.json`.

#### Scenario: Build task
- **WHEN** the user runs the task "qb64rust: build" from the task list
- **THEN** the active file is compiled as by the Build command

### Requirement: Workspace trust
Build, run and their tasks SHALL be unavailable in untrusted workspaces. Checking and formatting run the
compiler only in `-z`/`-y` modes, which generate files but do not execute the program; they SHALL also be
unavailable in untrusted workspaces because the compiler path itself comes from workspace settings.

#### Scenario: Untrusted folder
- **WHEN** the workspace is not trusted and the user runs Build
- **THEN** a message explains that building requires a trusted workspace and nothing is executed
