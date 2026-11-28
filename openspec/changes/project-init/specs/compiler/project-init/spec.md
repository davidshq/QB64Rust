# Spec Delta

## Purpose

The `qb64rust init` subcommand, which writes a program's `qb64rust.toml` from a few answers, so that nobody has to
remember the names of the settings. It sets up a folder that already holds a program, or starts a new one in a
folder that holds none.

## ADDED Requirements

### Requirement: Init subcommand
`qb64rust init [<folder>]` SHALL write `qb64rust.toml` into the folder, which is the current folder when none is
named, and SHALL create the folder when it is missing. `init` SHALL be read as the subcommand only as the first
argument. On success it SHALL print the path of each file it wrote and exit with status 0. The usage text SHALL
name the subcommand and its options.

#### Scenario: A folder with one program
- **WHEN** a folder holds `game.bas` and no settings file, and `qb64rust init --yes` runs in it
- **THEN** the folder holds a `qb64rust.toml`, the command prints its path, and the exit status is 0

#### Scenario: A file named init
- **WHEN** `qb64rust -x init.bas` runs
- **THEN** `init.bas` is built as a program

### Requirement: Init never overwrites
When the folder already holds a `qb64rust.toml`, `init` SHALL write nothing and exit with status 1, with a message
that names the file. When a folder above holds one, `init` SHALL write its file and SHALL say that the new file is
now the only one read for this folder.

#### Scenario: A settings file is there
- **WHEN** the folder holds a `qb64rust.toml` and `qb64rust init --yes` runs
- **THEN** the exit status is 1, the message names the file, and the file is unchanged

#### Scenario: A settings file one folder up
- **WHEN** `proj/qb64rust.toml` exists and `qb64rust init proj/demo --yes` runs
- **THEN** `proj/demo/qb64rust.toml` is written, and a line says it replaces `proj/qb64rust.toml` for `proj/demo`

### Requirement: Questions
When its input and output are both a terminal and `--yes` is not given, `init` SHALL ask about the main file, the
output folder and the data files, each only when no option gave it and it applies to the folder. An empty line or
the end of the input SHALL take the default, and an answer that is not a choice SHALL be asked again. The keyword
case and the fixes SHALL be options only. With `--yes`, or without a terminal, it SHALL ask nothing.

#### Scenario: Every default
- **WHEN** the user presses Enter at each question
- **THEN** the file written is the one `qb64rust init --yes` writes

#### Scenario: Not a terminal
- **WHEN** `qb64rust init` runs with its input redirected from a file, without `--yes`
- **THEN** it asks nothing, reads nothing from its input, and writes the file of the defaults

#### Scenario: An option answers a question
- **WHEN** `qb64rust init --output build` runs in a terminal
- **THEN** it does not ask where the program goes

### Requirement: The answers and their options
The answers SHALL be: the main file (`--main <file>`, default the first `.bas` file by name); the output folder
(`--output <folder>`, `.` for the source's folder, the default; the question offers it and `build`); the data
files (`--copy <path>`, repeatable, and `--no-copy`); the keyword case (`--keyword-case camel|upper|lower`); the
fixes (`--fixes on|off`). A value that is not allowed SHALL be an error with status 1, and nothing SHALL be
written.

#### Scenario: Several programs
- **WHEN** the folder holds `game.bas` and `editor.bas`, and the user answers `game.bas`
- **THEN** the file has `main = "game.bas"` under `[project]`

#### Scenario: One program
- **WHEN** the folder holds only `game.bas`
- **THEN** the main file is not asked, and `main` is a commented line

#### Scenario: A build folder with data
- **WHEN** `qb64rust init --yes --output build --copy GAME.CFG --copy maps` runs
- **THEN** the file has `output = "build"` and `copy = ["GAME.CFG", "maps"]` under `[build]`

#### Scenario: A copy list without an output folder
- **WHEN** `qb64rust init --yes --copy GAME.CFG` runs
- **THEN** the file has `output = "build"` and `copy = ["GAME.CFG"]`, as the compiler builds a `copy` list into
  `build`

#### Scenario: A bad value
- **WHEN** `qb64rust init --yes --keyword-case title` runs
- **THEN** the exit status is 1, the message lists `camel`, `upper` and `lower`, and no file is written

#### Scenario: A data file that is not there
- **WHEN** `qb64rust init --yes --copy NONE.CFG` runs and the folder has no `NONE.CFG`
- **THEN** the exit status is 1, the message names `NONE.CFG`, and no file is written

### Requirement: A file a build would refuse
Answers whose settings file a build would refuse SHALL be an error with status 1, with the message the build
would give, and nothing SHALL be written.

#### Scenario: A file a build would refuse
- **WHEN** `qb64rust init --yes --output build --copy ../shared` runs, and `../shared` is a folder
- **THEN** the exit status is 1, the message says that a `copy` entry must be in the settings file's folder, and
  no file is written

### Requirement: The data files offered
The data files offered SHALL be the folder's own entries, except: a name that starts with `.`, `qb64rust.toml`,
`qb64pe-log.txt`, a `.bas`, `.bi` or `.bm` file, the output folder (`build` when none is given), and a program of
the folder with its `.license.txt`, `.debug.json` and `.qb64rust`. With an output folder and neither `--copy` nor
`--no-copy`, every file offered SHALL be the default: one left out breaks the program.

#### Scenario: What is offered
- **WHEN** the folder holds `game.bas`, `game.exe`, `turn.bm`, `GAME.CFG`, `maps`, `.git` and `build`
- **THEN** the data files offered are `GAME.CFG` and `maps`

#### Scenario: The default with a build folder
- **WHEN** `qb64rust init --yes --output build` runs in that folder
- **THEN** the file has `copy = ["GAME.CFG", "maps"]`

#### Scenario: No data files
- **WHEN** `qb64rust init --yes --output build --no-copy` runs
- **THEN** the file has `output = "build"` and no `copy` list

### Requirement: A program that exists and a new project
In a folder with a `.bas` file the fixes SHALL default to off and no encoding SHALL be set, so that the program
behaves as before. In a folder with none, `init` SHALL start a new project: it SHALL also write `main.bas` (one
`PRINT` line, UTF-8) and `.vscode/settings.json` (which sets `files.encoding` to `utf8` for the `qb64rust`
language), the fixes SHALL default to on, and the file SHALL have `encoding = "utf-8"` under `[project]`. If
either of those two files exists, nothing SHALL be written.

#### Scenario: An existing program
- **WHEN** `qb64rust init --yes` runs in a folder with `game.bas`
- **THEN** the file sets no fix and no `encoding`, and `game.bas` builds to the same program as before

#### Scenario: An empty folder
- **WHEN** `qb64rust init --yes` runs in an empty folder
- **THEN** the folder holds `main.bas`, `.vscode/settings.json` and a `qb64rust.toml` with `all = true` under
  `[fixes]` and `encoding = "utf-8"` under `[project]`, the same settings "QB64: New Project" writes

#### Scenario: The fixes turned on for an existing program
- **WHEN** `qb64rust init --yes --fixes on` runs in a folder with `game.bas`
- **THEN** the file has `all = true` under `[fixes]`

### Requirement: The file written
The file SHALL hold each setting an answer changed from its default, and every other setting of `[build]`,
`[format]` and `[project]` as a commented line with its default and a few words on what it does. For `[fixes]` and
`[lints]` it SHALL hold the `all` line and say where their names are listed. The compiler SHALL read the file
without an error, and SHALL still do so when any one commented setting is uncommented.

#### Scenario: Every default
- **WHEN** `qb64rust init --yes` runs in a folder with one `.bas` file
- **THEN** the file has no setting that is not a comment, and a build of the program is the same with it and
  with `--no-settings`

#### Scenario: A setting found later
- **WHEN** the user removes the `#` from the line `# indent-size = 4`
- **THEN** the next build reads the file without an error

### Requirement: The ignore file
When the folder is in a git work tree (an entry `.git` in it or a folder above), `init` SHALL print the line to
add to `.gitignore`: `/<output>/` for an output folder, else `/<program>` for each program built beside its
source. It SHALL change no `.gitignore` and run no `git` command.

#### Scenario: A build folder in a repository
- **WHEN** `qb64rust init --yes --output build` runs in a folder of a git repository
- **THEN** it prints the line `/build/`, and no `.gitignore` is written or changed

#### Scenario: Beside the source in a repository
- **WHEN** `qb64rust init --yes` runs on Windows in a folder of a git repository with `game.bas`
- **THEN** it prints the line `/game.exe`

#### Scenario: Not a repository
- **WHEN** the same command runs in a folder that is in no git work tree
- **THEN** it prints no line about `.gitignore`

### Requirement: The plan of a folder
`qb64rust init [<folder>] --plan` SHALL print one line of JSON and write nothing: `folder`, `settings` (the
folder's settings file, or null), `above` (a settings file above, or null), `new_project`, `programs` (its `.bas`
files), `data_files` (the data files offered), `git`, `main` (the default main file, or null) and `fixes` (the
default of the fixes). It SHALL take `--output`, since the data files offered depend on it. It is how the extension
learns what to ask.

#### Scenario: A folder with a program
- **WHEN** `qb64rust init --plan` runs in a folder with `game.bas` and `GAME.CFG`
- **THEN** the line names `game.bas` as the only program and `GAME.CFG` as a data file, and no file is written

#### Scenario: A settings file is there
- **WHEN** the folder holds a `qb64rust.toml`
- **THEN** the line says so, and the exit status is 0
