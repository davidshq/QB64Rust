# Spec Delta

## Purpose

The extension's QB64 side panel: one place in VS Code that shows which program runs, runs it, says whether it is
right, and lists its procedures.

## ADDED Requirements

### Requirement: The QB64 view container
The extension SHALL add one view container named "QB64" to the activity bar. It SHALL hold four views in this
order: "Program", "SUBs & FUNCTIONs", "Examples" and "Help". The container SHALL be there in every window,
with or without a QB64 file open. "SUBs & FUNCTIONs" is shown only for a program with an include file (below).
(It is written with `&`: VS Code shows a view's name in title case, which made "and" read "And".)

#### Scenario: A window with no file
- **WHEN** VS Code opens with no folder and no file
- **THEN** the activity bar has a QB64 icon, and a click on it shows "Program", "Examples" and "Help"

### Requirement: The program named in the Program view
The Program view SHALL name the program that Run would build for the active editor: the main file of the
document's program as the language server names it, else the active `.bas` file. With no QB64 file active it
SHALL keep the last program named. With none yet it SHALL say so and offer "New Program" and "Open a Program".

#### Scenario: An include file is active
- **WHEN** `turn.bm` is active and the language server names `CWSTRAT.BAS` as its program
- **THEN** the Program view shows `CWSTRAT.BAS`

#### Scenario: Another kind of file becomes active
- **WHEN** `bounce.bas` was active and the user moves to a `.md` file
- **THEN** the Program view still shows `bounce.bas`, and Run builds `bounce.bas`

#### Scenario: Nothing yet
- **WHEN** no QB64 file has been active in this window
- **THEN** the view says no program is open and offers "New Program" and "Open a Program"

### Requirement: The buttons of the Program view
The Program view SHALL have a Run button, larger than the others, and the buttons Debug, Run in Browser and
Build. Each SHALL run the extension's command of that name on the program the view names: Run is "QB64: Build
and Run". In an untrusted workspace the four buttons SHALL be disabled, with a line that says why.

#### Scenario: Run
- **WHEN** the view names `bounce.bas` and the user clicks Run
- **THEN** `bounce.bas` is saved, built and run, as "QB64: Build and Run" does

#### Scenario: An untrusted workspace
- **WHEN** the workspace is not trusted
- **THEN** the four buttons are disabled and the view says that running needs a trusted workspace

### Requirement: The status line
The Program view SHALL show one status line for its program. While a compiler run goes on, it SHALL say what
runs. Otherwise, when a file of the program has an error in Problems, it SHALL show the first one: the main
file's before an include file's, then by line. Otherwise it SHALL show "OK". An error SHALL be shown with its
line, its message, a link that goes to the line, and a link to help when the message's statement has a page;
the link SHALL name the page ("Help on IF"). Below the status line, in every state, the view SHALL have a
"Search Help" link, which runs "QB64: Search Help".

#### Scenario: No error
- **WHEN** the program has no error in Problems and nothing runs
- **THEN** the line reads "OK"

#### Scenario: An error
- **WHEN** line 10 of `bounce.bas` has the only error
- **THEN** the line shows "Line 10", the error's message and a "Go to line 10" link, and the link puts the cursor
  on line 10

#### Scenario: An error in a closed include file
- **WHEN** the program is `cws.bas`, which includes 47 files, and the only error is on line 2 of
  `ui/menu.bm`, which is not open in the editor
- **THEN** the line shows the error of `menu.bm`, line 2, and the status bar reads "1 error: line 2"

#### Scenario: Errors in two files
- **WHEN** `bounce.bas` has an error on line 30 and its include file `rules.bm` one on line 2
- **THEN** the line shows the error of `bounce.bas`

#### Scenario: A warning only
- **WHEN** the program has warnings and no error
- **THEN** the line reads "OK" with the number of warnings

### Requirement: The first build says what it does
While a build also builds the runtime library, or while the C++ toolchain downloads, the status line SHALL say
so in plain words, and that it happens only once, with a moving bar. For the runtime library it SHALL also say
about how long it takes.

#### Scenario: The first build after an install
- **WHEN** the first build after installing the extension builds the runtime library
- **THEN** the status line says that the QB64 runtime is being built, that it can take about a minute and that
  this happens only the first time, and a bar moves under the text

### Requirement: The SUBs and FUNCTIONs view
The "SUBs & FUNCTIONs" view SHALL be shown only while the Program view's program has an include file: for a
program of one file, VS Code's Outline lists the same procedures. It SHALL list every SUB and FUNCTION of the
program, from every file of it, each with what its header has after the name (`DrawBall (x, y)`) and its
file's name. Its first row SHALL be the main program. A click on a row SHALL open the file
at that procedure. The view's title SHALL have a button that changes the order between the order in the program
and A to Z. The list SHALL follow the text as it is edited.

#### Scenario: A procedure in an include file
- **WHEN** `bounce.bas` includes `rules.bm`, which holds `FUNCTION Speed!`
- **THEN** the view lists `Speed!` with `rules.bm`, and a click opens `rules.bm` at the FUNCTION line

#### Scenario: A new SUB
- **WHEN** the user types a new `SUB DrawBall` into `bounce.bas`
- **THEN** the view lists `DrawBall` without a save

#### Scenario: A program of one file
- **WHEN** the program is `hello.bas`, which includes no file
- **THEN** the side panel does not show the "SUBs & FUNCTIONs" view

#### Scenario: No language server
- **WHEN** the language server does not run
- **THEN** the view is not shown, since the files of the program are not known

### Requirement: The status bar item
The extension SHALL show two items at the left of the status bar while a QB64 file is active: "Run", which runs
"QB64: Build and Run", and the Program view's status in short: "OK", or the number of errors with the first
one's line. A click on the second SHALL open the QB64 side panel.

#### Scenario: An error
- **WHEN** the program's first error is on line 10
- **THEN** the status bar reads "1 error: line 10", and a click opens the QB64 side panel

#### Scenario: Another language
- **WHEN** a `.md` file is active
- **THEN** the two items are hidden
