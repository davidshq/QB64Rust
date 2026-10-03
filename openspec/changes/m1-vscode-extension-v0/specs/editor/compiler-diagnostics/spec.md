# Spec Delta

## Purpose

Shows the old compiler's BASIC-level errors and warnings in the Problems view at the right file and line, quickly
enough to run on every save.

## ADDED Requirements

### Requirement: Check on save
When a `qb64rust` document is saved and `qb64rust.checkOnSave` is true (default), the extension SHALL check the
program with the compiler in C++-generation-only mode (no C++ build) and replace that program's diagnostics with
the result. The command `qb64rust.check` SHALL do the same on demand.

#### Scenario: Error on save
- **WHEN** the user saves a file whose line 2 is `x = 1 +`
- **THEN** Problems shows one error "Expected variable/value after '+'" on line 2 of that file

#### Scenario: Fixed on save
- **WHEN** the user fixes the error and saves again
- **THEN** the error disappears from Problems

#### Scenario: Disabled
- **WHEN** `qb64rust.checkOnSave` is false
- **THEN** saving does not run the compiler

### Requirement: Error location in include files
When the compiler reports an error inside an `$INCLUDE`d file, the diagnostic SHALL be attached to that file and
line, not to the main file.

#### Scenario: Error in a .bi file
- **WHEN** `main.bas` includes `inc.bi` and line 2 of `inc.bi` is `y = 1 +`
- **THEN** the error appears on line 2 of `inc.bi`

### Requirement: Warnings
Each compiler warning line of the form `<file>:<line>: warning: <message>` SHALL become a diagnostic with severity
Warning on that line. The indented detail line that follows SHALL be appended to the message.

#### Scenario: Unused variable
- **WHEN** a program declares `DIM unusedvar AS INTEGER` on line 1 and never uses it
- **THEN** Problems shows a warning "Unused variable: unusedvar% INTEGER" (detail whitespace collapsed) on line 1

### Requirement: Diagnostic range
A diagnostic SHALL cover the reported line from its first to its last non-blank character. The old compiler
reports no column; the diagnostic SHALL NOT claim one.

#### Scenario: Indented line
- **WHEN** the error line is `    x = 1 +`
- **THEN** the diagnostic range starts at the `x` and ends after the `+`

### Requirement: Unrecognised output
If the compiler exits with failure and no error or warning can be parsed from its output, the extension SHALL
report one error on line 1 of the checked file containing the output text, so that no failure is silently
dropped.

#### Scenario: Unknown failure
- **WHEN** the compiler exits with code 1 and prints only "Something unexpected"
- **THEN** Problems shows an error on line 1 whose message contains "Something unexpected"

### Requirement: Diagnostics follow edits
Diagnostics from a check or build SHALL be cleared for a document when it is edited, and SHALL be cleared when
the document is closed.

#### Scenario: Edit after error
- **WHEN** an error is shown and the user types on any line
- **THEN** the diagnostics for that document are removed until the next save or check

### Requirement: Non-blocking check
A check SHALL NOT block editing. If a new check of the same file starts while one is running, the older one SHALL
be cancelled and its result discarded. A check that runs longer than `qb64rust.checkTimeoutSeconds` (default 30)
SHALL be killed.

#### Scenario: Rapid saves
- **WHEN** the user saves three times within one second
- **THEN** only the result of the last check is shown
