# Spec Delta

## Purpose

A page in the editor area that a QB64 window starts with: where to begin, what was open before, and whether the
setup is complete. It is for someone who has just installed the extension and sees an empty window.

## ADDED Requirements

### Requirement: The page QB64 Home
The extension SHALL have a page "QB64 Home", shown in one tab of the editor area, and a command "QB64: Home"
that opens it at any time. The page SHALL offer "New Program", "Open a Program", "Examples" and "Help", each
doing what the command or view of that name does. It SHALL link to the walkthrough "Get Started with QB64", to
"First steps" and to the pages for those coming from QuickBASIC 4.5 and from QB64pe.

#### Scenario: The command
- **WHEN** the user runs "QB64: Home"
- **THEN** a tab "QB64 Home" opens in the editor area

#### Scenario: The command twice
- **WHEN** the user runs "QB64: Home" while its tab is open
- **THEN** that tab is shown, and no second one opens

### Requirement: When the page opens by itself
The page SHALL open by itself when a window starts, only when all of these hold: the setting
`qb64rust.showHomeOnStartup` is on, no file is open in an editor, and the window has no folder or has a folder
that holds a `.bas`, `.bi` or `.bm` file. VS Code's own Welcome page SHALL not count as an open file.

#### Scenario: A window with no folder
- **WHEN** VS Code starts with no folder and no file
- **THEN** "QB64 Home" opens

#### Scenario: A folder with QB64 files
- **WHEN** VS Code opens a folder that holds `bounce.bas`, with no file open
- **THEN** "QB64 Home" opens

#### Scenario: A folder of another project
- **WHEN** VS Code opens a folder that holds no QB64 file
- **THEN** "QB64 Home" does not open

#### Scenario: A file is open
- **WHEN** VS Code starts and opens again the files of the last session
- **THEN** "QB64 Home" does not open

### Requirement: The checkbox that stops it
The page SHALL have a checkbox "Show this page when a QB64 window starts", which shows and changes the user's
setting `qb64rust.showHomeOnStartup`. With it cleared, the page SHALL open only by "QB64: Home".

#### Scenario: Cleared
- **WHEN** the user clears the checkbox and starts VS Code again with no folder
- **THEN** "QB64 Home" does not open, and "QB64: Home" still opens it

### Requirement: Recent programs
The page SHALL list the programs the user had open, newest first, 8 at most, each with its file name and its
folder's name. A program is the main file the Program view named. A file that no longer exists, and the user's
copy of an example, SHALL not be listed. A click SHALL open the program.

#### Scenario: A program opened before
- **WHEN** the user opened `bounce.bas` in an earlier session and opens "QB64 Home"
- **THEN** the page lists `bounce.bas` with its folder, and a click opens it

#### Scenario: An include file
- **WHEN** the user opened `turn.bm`, whose program is `CWSTRAT.BAS`
- **THEN** the page lists `CWSTRAT.BAS`, not `turn.bm`

### Requirement: The setup's state
The page SHALL say whether the QB64 compiler was found, whether the C++ compiler a build needs is installed,
and whether the Emscripten SDK for "Run in Browser" is installed, the last one marked optional. For one that is
missing and that the extension can download, it SHALL offer a link that runs the extension's install command,
with the size of the download.

#### Scenario: A fresh install on Windows
- **WHEN** the extension was just installed on Windows and no C++ toolchain is there
- **THEN** the page shows the C++ compiler as missing, with a link "Install (190 MB)"
