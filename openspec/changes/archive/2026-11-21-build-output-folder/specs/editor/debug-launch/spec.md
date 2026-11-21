## MODIFIED Requirements

### Requirement: Debug build before launch
A configuration whose `program` is a `.bas` file SHALL be saved and built with `qb64rust --debug` by the same
path, compiler roots and output channel as the Build command, its errors shown in Problems; the session SHALL
start only when the build succeeds, with the built executable as the adapter's `program`. A `program` that is an
executable SHALL be launched as it is, without a build.

#### Scenario: Launch a source file
- **WHEN** the user starts debugging `hello.bas`, which has a breakpoint on line 3
- **THEN** the "QB64" output shows a `qb64rust` command line with `--debug`, `build/hello.exe` and
  `build/hello.exe.debug.json` exist in `hello.bas`'s folder, and the program stops on line 3

#### Scenario: Build fails
- **WHEN** the user starts debugging a program with an error on line 2
- **THEN** no debug session starts, and the error is in Problems at line 2

#### Scenario: Launch an executable
- **WHEN** a configuration's `program` names an executable built earlier with `--debug`
- **THEN** no build runs and the session starts with that executable

### Requirement: Launch defaults
An attribute left out SHALL take these values: `program` the active editor's file (for an included file, its
program's main file: spec `editor/build-run`, "Included file"), `cwd` the folder the program runs in (`run_in` of
`qb64rust --print-paths`, spec `compiler/cli`) for a `.bas` program and the executable's folder for an executable,
`console` `integratedTerminal`, `noPrompt` the setting `qb64rust.noPrompt`, `args` the setting
`qb64rust.runArguments` split as a shell would, `stopOnEntry` false.

#### Scenario: Run arguments
- **WHEN** `qb64rust.runArguments` is `a "b c"` and a configuration has no `args`
- **THEN** the program receives the two arguments `a` and `b c`

#### Scenario: Console input
- **WHEN** a `$CONSOLE:ONLY` program that runs `INPUT x` is debugged with the defaults
- **THEN** its prompt is in an integrated terminal, and what the user types there is read by the program

#### Scenario: The working folder without a copy list
- **WHEN** `game.bas` has no settings file and a configuration for it has no `cwd`
- **THEN** `build/game.exe` is debugged with `game.bas`'s folder as its working folder

#### Scenario: The working folder with a copy list
- **WHEN** the settings file has `copy = ["GAME.CFG"]` and a configuration for `game.bas` has no `cwd`
- **THEN** the program runs with `build` as its working folder
