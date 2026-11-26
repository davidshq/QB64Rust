# Spec Delta

## Purpose

The command that starts a new program as one file, ready to run, for someone who wants to type BASIC and not to
set up a project. The user chooses the folder and the name each time, in a save dialog, as VS Code asks for any
new file.

## ADDED Requirements

### Requirement: New Program command
The command "QB64: New Program" (`qb64rust.newProgram`) SHALL show a save dialog for a `.bas` file, write a
short program that runs to the file the user names, and open it. It SHALL write that one file and nothing else:
no folder, no `qb64rust.toml` and no setting. When the user cancels the dialog, it SHALL write nothing. The
command SHALL work in an untrusted workspace.

#### Scenario: A name is given
- **WHEN** the user runs the command and saves as `bounce.bas` in the folder `games`
- **THEN** `games\bounce.bas` exists with a program that prints a line, it is open in the editor, and `games`
  holds no other new file

#### Scenario: Cancel
- **WHEN** the user closes the dialog without saving
- **THEN** no file and no folder is made, and no setting changes

#### Scenario: A name without an extension
- **WHEN** the user types `bounce` as the name
- **THEN** the file written is `bounce.bas`

### Requirement: Where the save dialog opens
The dialog SHALL open in the workspace folder when one is open; with several, in the folder of the active file.
With none, it SHALL open in the folder where this command last wrote a program, when that folder still exists,
and otherwise in the user's documents folder. The name filled in SHALL be the first of `program1.bas`,
`program2.bas` and so on that the folder does not hold.

#### Scenario: A folder is open
- **WHEN** the workspace folder is `games`, which holds `program1.bas`
- **THEN** the dialog opens in `games` with the name `program2.bas`

#### Scenario: No folder, first use
- **WHEN** no folder is open and the command has never written a program
- **THEN** the dialog opens in the user's documents folder

#### Scenario: No folder, a later use
- **WHEN** no folder is open and the command last wrote `D:\basic\bounce.bas`
- **THEN** the dialog opens in `D:\basic`

### Requirement: An existing file is replaced only when the user says so
When the user names a file that exists, the command SHALL replace it only after the save dialog's own question
has been answered with yes.

#### Scenario: The user says no
- **WHEN** the user names `bounce.bas`, which exists, and answers no to the dialog's question
- **THEN** `bounce.bas` is unchanged

### Requirement: A new program follows its folder's settings
The new file SHALL be saved in the encoding its folder's programs use: UTF-8 when the nearest `qb64rust.toml`
declares it, else the QB64 language's encoding in VS Code. The command SHALL NOT turn the fixes on: a program in
a folder without a `qb64rust.toml` builds as an existing program does.

#### Scenario: Inside a new project
- **WHEN** the folder holds the `qb64rust.toml` that "QB64: New Project" wrote
- **THEN** the new file is saved as UTF-8 and builds with every fix on

#### Scenario: A plain folder
- **WHEN** the folder has no `qb64rust.toml`
- **THEN** the new file is saved as code page 437, and no `qb64rust.toml` is written

### Requirement: A new program from given text
When an example hands its text to the command, the command SHALL write that text in place of the short program
and fill in the example's name.

#### Scenario: From an example
- **WHEN** the user chooses "Open as a new program" on the example of `_CLAMP`
- **THEN** the dialog's name starts as `clamp.bas`, and the file the user saves holds the example's text
