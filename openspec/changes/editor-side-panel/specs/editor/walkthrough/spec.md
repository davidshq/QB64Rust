# Spec Delta

## Purpose

The first five minutes with the extension: a checklist in VS Code's own Welcome page that takes a new user from
the install to a running program and to help.

## ADDED Requirements

### Requirement: The Get Started walkthrough
The extension SHALL contribute one walkthrough, "Get Started with QB64", with these steps in order: write your
first program, run it, look a word up, try an example, find and fix a mistake, and put your program on the web
(marked optional). Each step SHALL have one button that runs a command of the extension, and SHALL be ticked
when that command has run.

#### Scenario: The first step
- **WHEN** the user clicks the button of "Write your first program"
- **THEN** "QB64: New Program" runs, and the step is ticked once it has made a program

#### Scenario: Every button has a command
- **WHEN** the extension's manifest is read
- **THEN** every command a step's button names is a command the extension contributes

### Requirement: The run step tells of the download first
The step "Run it" SHALL say, before its button is clicked, that the first run needs a C++ compiler, the size
of its download, where it comes from, and that nothing is downloaded unless the user agrees. On Linux it SHALL
say instead that the system's `g++` is used.

#### Scenario: Windows
- **WHEN** the step "Run it" is shown on Windows
- **THEN** its text names a 190 MB download from github.com/mstorsjo/llvm-mingw and says that it is asked first

### Requirement: The command that opens the walkthrough
The command "QB64: Get Started" SHALL open the walkthrough at any time. The Program view SHALL link to it while
no program has been opened in the window.

#### Scenario: Opened again
- **WHEN** the user has closed the Welcome page and runs "QB64: Get Started"
- **THEN** the walkthrough opens with the steps already done still ticked
