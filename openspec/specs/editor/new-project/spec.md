# editor/new-project Specification

## Purpose
The extension's command that starts a new QB64 program in a folder of its own, with the fixes of spec
`compiler/fixes` turned on, since a new project defaults to the fixes on and an existing program to off.

## Requirements

### Requirement: New Project command
The command "QB64: New Project" (`qb64rust.newProject`) SHALL ask for a folder, write into it `main.bas` (one
`PRINT` line) and a `qb64rust.toml` whose `[fixes]` table holds `all = true`, then open `main.bas`. It SHALL
overwrite no file: when either file exists in the chosen folder it SHALL say so and write nothing.

#### Scenario: An empty folder
- **WHEN** the user runs "QB64: New Project" and chooses an empty folder
- **THEN** the folder holds `main.bas` and `qb64rust.toml` with `all = true` under `[fixes]`, and `main.bas` is
  open in the editor

#### Scenario: A folder with a program
- **WHEN** the chosen folder already holds `main.bas`
- **THEN** a message names the file, and nothing is written

#### Scenario: Built with the fixes on
- **WHEN** the new `main.bas` is built with "QB64: Build"
- **THEN** it is built with every fix on
