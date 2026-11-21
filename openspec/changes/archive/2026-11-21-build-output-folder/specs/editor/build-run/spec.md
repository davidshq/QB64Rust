## MODIFIED Requirements

### Requirement: Build command
The command `qb64rust.build` SHALL save the active document and compile it to an executable (same base name),
showing compiler output in an output channel named "QB64". A build with `qb64rust` SHALL pass no `-o`, so the
program goes into the output folder (spec `compiler/cli`, "The output folder"); a build with QB64pe SHALL write
it next to the source. It SHALL build with `qb64rust`
when `qb64rust.buildWith` is `qb64rust`, or `auto` and the binary is found, with the toolchain found (or offered,
when there is none), QB64pe's folder as the include root when QB64pe is found, and SHALL build with QB64pe
otherwise.

#### Scenario: Successful build
- **WHEN** the user runs Build with `qb64rust` on a saved `hello.bas` without errors
- **THEN** `build/hello.exe` (Windows) or `build/hello` (other platforms) exists in `hello.bas`'s folder and a
  success message is shown

#### Scenario: Build with a copy list
- **WHEN** `qb64rust.toml` beside `game.bas` has `[build]` and `copy = ["GAME.CFG"]`, and the user runs Build
  on `game.bas` with `qb64rust`
- **THEN** `build/game.exe` and `build/GAME.CFG` exist

#### Scenario: Built by QB64pe
- **WHEN** the user builds `game.bas` with QB64pe
- **THEN** `game.exe` is written next to `game.bas`, since QB64pe does not read the settings file

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

### Requirement: Run commands
The command `qb64rust.run` SHALL run the existing executable for the active document in an integrated terminal.
With `qb64rust` the executable and the working directory SHALL be `exe` and `run_in` of `qb64rust --print-paths`
(spec `compiler/cli`, "The paths of a program"), unless the executable next to the source is newer, as after
"Build with QB64pe"; then it SHALL run that one in the source folder. With QB64pe they SHALL be the executable next
to the source and the source folder. `qb64rust.buildAndRun` SHALL build and, only if the build succeeds, run.

#### Scenario: Build and run
- **WHEN** the user runs Build and Run on a correct program
- **THEN** the program starts in a terminal named "QB64" after the build finishes

#### Scenario: Run in the source folder
- **WHEN** `game.bas` has no settings file, `build/game.exe` exists, and the user runs Run on `game.bas`
- **THEN** `build/game.exe` starts with `game.bas`'s folder as its working folder, so it finds the data files
  beside `game.bas`

#### Scenario: Run in the output folder
- **WHEN** the settings file has `copy = ["GAME.CFG"]`, `build/game.exe` exists, and the user runs Run on
  `game.bas`
- **THEN** `build/game.exe` starts with `build` as its working folder, so a file it writes by name alone stays in
  `build`

#### Scenario: Run after a build with QB64pe
- **WHEN** `build/game.exe` exists from a build with `qb64rust`, the user then accepts "Build with QB64pe", and
  runs Run on `game.bas`
- **THEN** the newer `game.exe` next to the source starts, in the source folder

#### Scenario: Run without executable
- **WHEN** the user runs Run and no executable exists for the document
- **THEN** an error message offers to build first

#### Scenario: Program arguments
- **WHEN** `qb64rust.runArguments` is set to `a b`
- **THEN** the program is started with arguments `a b` (visible to it as `COMMAND$`)

### Requirement: Included file
When the language server names another file as the program of the document that Build, Run, Build and Run, Debug
or a task without `file` acts on (spec `editor/language-server`, "Program notification"), the action SHALL act on
that program's main file instead. Build SHALL save the document first, then the main file. When the server names
no program, or does not run, the action SHALL act on the document itself.

#### Scenario: Build from an include file
- **WHEN** `turn.bm` is the active editor and the server names `CWSTRAT.BAS` as its program
- **THEN** Build saves `turn.bm`, then builds `CWSTRAT.BAS` to `build/CWSTRAT.exe` with `qb64rust` (to
  `CWSTRAT.exe` beside it with QB64pe), and the build task does the same

#### Scenario: Debug from an include file
- **WHEN** `turn.bm` is the active editor, its program is `CWSTRAT.BAS`, and the user presses F5 with no
  `launch.json`
- **THEN** `CWSTRAT.BAS` is built with `--debug` and debugged, with `CWSTRAT.BAS`'s folder as `cwd` (its program has no `copy` list)

#### Scenario: No program found
- **WHEN** `lib.bm` is the active editor and the server names no program for it
- **THEN** Build builds `lib.bm` itself, as before
