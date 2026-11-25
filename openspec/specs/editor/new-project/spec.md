# editor/new-project Specification

## Purpose
The extension's command that starts a new QB64 program in a folder of its own, with the fixes of spec
`compiler/fixes` turned on, since a new project defaults to the fixes on and an existing program to off.

## Requirements

### Requirement: New Project command
The command "QB64: New Project" (`qb64rust.newProject`) SHALL ask for a folder and write into it:

- `main.bas` (one `PRINT` line), saved as UTF-8;
- a `qb64rust.toml` whose `[fixes]` table holds `all = true` and whose `[project]` table holds
  `encoding = "utf-8"`;
- `.vscode\settings.json`, which sets `files.encoding` to `utf8` for the `qb64rust` language.

It SHALL then open `main.bas`. It SHALL overwrite no file: when any of the three files exists in the chosen
folder it SHALL say so and write nothing.

#### Scenario: An empty folder
- **WHEN** the user runs "QB64: New Project" and chooses an empty folder
- **THEN** the folder holds `main.bas`, `qb64rust.toml` with `all = true` under `[fixes]` and
  `encoding = "utf-8"` under `[project]`, and `.vscode\settings.json`, and `main.bas` is open in the editor

#### Scenario: A folder with a program
- **WHEN** the chosen folder already holds `main.bas`
- **THEN** a message names the file, and nothing is written

#### Scenario: Built with the fixes on
- **WHEN** the new `main.bas` is built with "QB64: Build"
- **THEN** it is built with every fix on

#### Scenario: A box-drawing character in the new project
- **WHEN** the user types `PRINT "─"` in the new `main.bas`, saves it and builds it
- **THEN** the file holds the bytes 0xE2 0x94 0x80, and the program prints the one character `─`
