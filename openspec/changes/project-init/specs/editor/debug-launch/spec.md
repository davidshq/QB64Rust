# Spec Delta

## MODIFIED Requirements

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
- **THEN** the program beside `game.bas` is debugged with `game.bas`'s folder as its working folder

#### Scenario: The working folder with a copy list
- **WHEN** the settings file has `copy = ["GAME.CFG"]` and a configuration for `game.bas` has no `cwd`
- **THEN** the program runs with `build` as its working folder
