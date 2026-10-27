# editor/debug-launch Specification

## Purpose

How the VS Code extension starts a debug session of a QB64 program: the debugger it contributes, the debug build
before launch, how it starts the debug adapter `qb64rust dap` (spec `editor/debug-adapter`), and its defaults.

## Requirements

### Requirement: Debugger contribution
The extension SHALL contribute a debugger of type `qb64rust` with a `launch` request whose attributes are
`program`, `args`, `cwd`, `stopOnEntry`, `console` (`internalConsole`, `integratedTerminal`, `externalTerminal`)
and `noPrompt`, and SHALL allow breakpoints in `qb64rust` documents. It SHALL offer an initial configuration and
a `launch.json` snippet that debug the active file.

#### Scenario: Breakpoint in a .bas file
- **WHEN** the extension is installed and the user clicks the gutter of a line of a `.bas` file
- **THEN** a breakpoint is set on that line

#### Scenario: Adding a configuration
- **WHEN** the user creates a `launch.json` for a folder with a `.bas` file and picks "QB64"
- **THEN** the file holds a configuration of type `qb64rust`, request `launch`, whose `program` is `${file}`

### Requirement: Debug build before launch
A configuration whose `program` is a `.bas` file SHALL be saved and built with `qb64rust --debug` by the same
path, compiler roots and output channel as the Build command, its errors shown in Problems; the session SHALL
start only when the build succeeds, with the built executable as the adapter's `program`. A `program` that is an
executable SHALL be launched as it is, without a build.

#### Scenario: Launch a source file
- **WHEN** the user starts debugging `hello.bas`, which has a breakpoint on line 3
- **THEN** the "QB64" output shows a `qb64rust` command line with `--debug`, `hello.exe` and
  `hello.exe.debug.json` exist next to `hello.bas`, and the program stops on line 3

#### Scenario: Build fails
- **WHEN** the user starts debugging a program with an error on line 2
- **THEN** no debug session starts, and the error is in Problems at line 2

#### Scenario: Launch an executable
- **WHEN** a configuration's `program` names an executable built earlier with `--debug`
- **THEN** no build runs and the session starts with that executable

### Requirement: Only qb64rust builds are debugged
When the build for a `.bas` file would not be by `qb64rust` (`qb64rust.buildWith` is `qb64pe`, or the `qb64rust`
binary is not found), the launch SHALL be refused with a message that says debugging needs `qb64rust` and names
the setting to change; it SHALL NOT build with QB64pe instead.

#### Scenario: buildWith is qb64pe
- **WHEN** `qb64rust.buildWith` is `qb64pe` and the user starts debugging a `.bas` file
- **THEN** an error message names `qb64rust.buildWith`, nothing is built and no session starts

#### Scenario: Only unsupported forms
- **WHEN** the debug build fails and every error is marked "not supported yet"
- **THEN** the failure is shown without the offer "Build with QB64pe", and no session starts

### Requirement: The adapter
The extension SHALL start the debug adapter as `qb64rust dap --toolchain-root <folder>`, from the `qb64rust`
binary the language server and builds use (spec `editor/packaging`), with the toolchain folder a build uses. When
no toolchain is found, the launch SHALL offer it as a build does (spec `editor/packaging`, "Toolchain offer") and
start no session until it is installed.

#### Scenario: Bundled binary
- **WHEN** `qb64rust.path` is empty and the extension carries its binary
- **THEN** the debug session's adapter is the carried binary run with `dap` and the toolchain folder

#### Scenario: Toolchain without lldb-dap
- **WHEN** the toolchain folder has no `bin\lldb-dap.exe`
- **THEN** the session ends with the adapter's error naming that path, shown to the user

#### Scenario: QB64pe without lldb-dap
- **WHEN** the toolchain used is the QB64pe folder's and it has no `internal\c\c_compiler\bin\lldb-dap.exe`
- **THEN** the session ends with the adapter's error naming that path, shown to the user

#### Scenario: Debugging without QB64pe
- **WHEN** QB64pe is not found, the toolchain is installed, and the user starts debugging `hello.bas` with a
  breakpoint on line 3
- **THEN** the program is built with `--debug` and stops on line 3

### Requirement: Launch defaults
An attribute left out SHALL take these values: `program` the active editor's file, `cwd` the program's folder,
`console` `integratedTerminal`, `noPrompt` the setting `qb64rust.noPrompt`, `args` the setting
`qb64rust.runArguments` split as a shell would, `stopOnEntry` false.

#### Scenario: Run arguments
- **WHEN** `qb64rust.runArguments` is `a "b c"` and a configuration has no `args`
- **THEN** the program receives the two arguments `a` and `b c`

#### Scenario: Console input
- **WHEN** a `$CONSOLE:ONLY` program that runs `INPUT x` is debugged with the defaults
- **THEN** its prompt is in an integrated terminal, and what the user types there is read by the program

### Requirement: Debugging without a configuration
Starting debugging (F5) with no `launch.json`, or with one that has no configuration, while a `qb64rust` file is
active SHALL debug that file with the defaults. The command `qb64rust.debug` ("QB64: Debug") SHALL do the same for
the active or a given `.bas` file, and SHALL appear in the editor's run menu beside Build and Run.

#### Scenario: F5 on a .bas file
- **WHEN** the folder has no `launch.json`, `hello.bas` is the active editor, and the user presses F5
- **THEN** `hello.bas` is built with `--debug` and debugged

#### Scenario: The Debug command
- **WHEN** the user runs "QB64: Debug" from the editor's run menu on `hello.bas`
- **THEN** a debug session of `hello.bas` starts as with F5

#### Scenario: Another file active
- **WHEN** there is no `launch.json` and the active editor is not a `qb64rust` file
- **THEN** the extension offers no configuration of its own, and VS Code's choice of debugger is unchanged

### Requirement: Workspace trust for debugging
Debugging SHALL be unavailable in an untrusted workspace: a launch SHALL be refused with a message, and nothing
SHALL be built or run.

#### Scenario: Untrusted folder
- **WHEN** the workspace is not trusted and the user starts debugging a `.bas` file
- **THEN** a message says debugging requires a trusted workspace, and no build or session starts
