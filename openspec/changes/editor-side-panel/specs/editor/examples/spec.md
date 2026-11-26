# Spec Delta

## Purpose

Example programs inside the editor, for someone who learns by running a program and changing it. The shipped
copy of an example is never the file the user edits.

## ADDED Requirements

### Requirement: The Examples view
The Examples view SHALL list the example programs in groups: "First steps", "Graphics", "Sound and music" and
"Small games", and "One for every keyword" with the reference's examples by keyword. Each example SHALL show
its title. "First steps" SHALL start open and the other groups closed.

#### Scenario: The groups
- **WHEN** the user opens the Examples view
- **THEN** it shows the four groups and "One for every keyword", and "First steps" holds a program named "Hello"

### Requirement: An example opens as a copy
A click on an example SHALL open the user's copy of it: a file in the extension's own storage, made from the
shipped example at the first click. It opens as any file does, with nothing to save. The shipped file SHALL
never be opened for editing. A copy the user has edited SHALL be kept; a copy that was not edited SHALL follow
the shipped example. The row's menu SHALL offer "Run", which builds and runs that copy, "Open as a New Program",
which gives the shipped text to "QB64: New Program", and "Reset This Example", which makes the copy as shipped
again.

#### Scenario: A click
- **WHEN** the user clicks "Hello"
- **THEN** an editor shows `hello.bas` with the program's text and no unsaved change, and closing it asks nothing

#### Scenario: Change it and run it
- **WHEN** the user changes a line of "Hello" and clicks Run
- **THEN** the changed program is built and run with no save dialog, and no file of the extension's package
  changes

#### Scenario: An edited example opened again
- **WHEN** the user has changed and saved "Hello", closes it and clicks "Hello" again
- **THEN** the editor shows the user's text

#### Scenario: Reset
- **WHEN** the user chooses "Reset This Example" on "Hello"
- **THEN** the copy holds the shipped text again

#### Scenario: Run from the view
- **WHEN** the user chooses "Run" on "Hello"
- **THEN** the program is built and run, and no file appears in the user's workspace or documents

### Requirement: Every shipped example builds
Every example the extension ships SHALL be accepted by the compiler the same package carries, with no error.

#### Scenario: Checked at each merge
- **WHEN** the repository's tests run
- **THEN** each program under `examples\` and each example of the reference is checked by the compiler with no
  error
