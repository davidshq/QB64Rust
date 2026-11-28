# Spec Delta

## Purpose

The extension's command that sets up a folder's `qb64rust.toml` by asking the questions of `qb64rust init` as pick
lists. The compiler writes the file, so the extension never writes a settings file of its own for a program that
exists.

## ADDED Requirements

### Requirement: Set Up Project command
The command "QB64: Set Up Project" (`qb64rust.setUpProject`) SHALL take the folder of the active `qb64rust` file,
or ask for a folder when there is none. It SHALL ask what `qb64rust init --plan` says is to be asked (spec
`compiler/project-init`), each as a pick list with the default first, then have `qb64rust init` write the file
with those answers, and open `qb64rust.toml`. Leaving any list without a choice SHALL write nothing.

#### Scenario: A program beside its data
- **WHEN** `game.bas` is open, its folder holds `GAME.CFG`, and the user picks the `build` folder and keeps the
  data files ticked
- **THEN** the folder's `qb64rust.toml` has `output = "build"` and `copy = ["GAME.CFG"]`, and it is open in the
  editor

#### Scenario: Every default
- **WHEN** the user accepts the first choice of each list
- **THEN** the file is the one `qb64rust init --yes` writes

#### Scenario: Canceled
- **WHEN** the user presses Escape at the second list
- **THEN** no file is written

### Requirement: Folders the command does not set up
When the folder already has a `qb64rust.toml`, the command SHALL open it and say that the folder is set up. When
the folder has no `.bas` file, it SHALL run "QB64: New Project" for that folder (spec `editor/new-project`). In an
untrusted workspace it SHALL be unavailable, since it runs the compiler. With a `qb64rust` that has no `init`, it
SHALL say that a newer `qb64rust` is needed and write nothing.

#### Scenario: A settings file is there
- **WHEN** the folder of the active file has a `qb64rust.toml`
- **THEN** that file opens, a message says the folder is already set up, and nothing is written

#### Scenario: An empty folder
- **WHEN** the user picks an empty folder
- **THEN** the folder gets what "QB64: New Project" writes

#### Scenario: An older compiler
- **WHEN** `qb64rust.path` names a `qb64rust` without the `init` subcommand
- **THEN** a message says a newer `qb64rust` is needed, and nothing is written
