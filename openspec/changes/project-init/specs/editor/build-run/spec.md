# Spec Delta

## MODIFIED Requirements

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
- **WHEN** `game.bas` has no settings file, `game.exe` exists beside it, and the user runs Run on `game.bas`
- **THEN** `game.exe` starts with `game.bas`'s folder as its working folder, so it finds the data files beside
  `game.bas`

#### Scenario: Run in the output folder
- **WHEN** the settings file has `copy = ["GAME.CFG"]`, `build/game.exe` exists, and the user runs Run on
  `game.bas`
- **THEN** `build/game.exe` starts with `build` as its working folder, so a file it writes by name alone stays in
  `build`

#### Scenario: Run after a build with QB64pe
- **WHEN** the settings file has `copy = ["GAME.CFG"]`, `build/game.exe` exists from a build with `qb64rust`, the
  user then accepts "Build with QB64pe", and runs Run on `game.bas`
- **THEN** the newer `game.exe` next to the source starts, in the source folder

#### Scenario: Run without executable
- **WHEN** the user runs Run and no executable exists for the document
- **THEN** an error message offers to build first

#### Scenario: Program arguments
- **WHEN** `qb64rust.runArguments` is set to `a b`
- **THEN** the program is started with arguments `a b` (visible to it as `COMMAND$`)

#### Scenario: A console window of its own
- **WHEN** `qb64rust.run.terminal` is `external` and the user runs Run
- **THEN** the program starts in a window of its own: on Windows through `cmd /c start`, on Linux in the terminal
  `terminal.external.linuxExec` names (`x-terminal-emulator` when it is empty), with `-e`

#### Scenario: The runtime's log
- **WHEN** `qb64rust.run.logLevel` is `information`, `qb64rust.run.logHandlers` is `["file"]`, and the user runs Run
  or the run task
- **THEN** the program starts with `QB64PE_LOG_LEVEL=information`, `QB64PE_LOG_SCOPES=qb64`,
  `QB64PE_LOG_HANDLERS=file` and `QB64PE_LOG_FILE_PATH=qb64pe-log.txt`, as the QB64pe IDE sets them; with the level
  `none` the extension adds none of the four
