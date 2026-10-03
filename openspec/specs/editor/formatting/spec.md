# editor/formatting Specification

## Purpose
Formats QB64 source the way the QB64pe IDE lays it out (indentation, spacing, keyword case) by delegating to the
compiler's formatting mode.

## Requirements

### Requirement: Format document
The extension SHALL register a document formatting provider for `qb64rust` that formats the whole document with
the compiler's `-y` mode. The current editor contents, including unsaved changes, SHALL be formatted; the file on
disk SHALL NOT be modified by formatting.

#### Scenario: Indent and keyword case
- **WHEN** the document is `for i=1 to 3` / `print   i` / `next` and the user runs Format Document
- **THEN** the document becomes `For i = 1 To 3` / `    Print i` / `Next`

#### Scenario: Unsaved edits
- **WHEN** the document has unsaved changes and the user runs Format Document
- **THEN** the formatted result is based on the unsaved text and the document stays dirty

#### Scenario: Relative includes
- **WHEN** a saved `main.bas` contains `'$INCLUDE:'inc.bi'` and `inc.bi` is in the same folder
- **THEN** formatting succeeds (includes resolve relative to the document's folder, as in a build)

#### Scenario: Temporary files are removed
- **WHEN** formatting finishes, successfully or not
- **THEN** no temporary file created for formatting remains in the document's folder

### Requirement: Format keeps line endings and encoding
The formatted text SHALL use the document's existing end-of-line sequence and SHALL be read back with the same
encoding it was written with, so that CP437 bytes in string literals survive.

#### Scenario: LF document
- **WHEN** the document uses LF line endings and the compiler writes CRLF
- **THEN** the formatted document still uses LF

#### Scenario: Extended characters
- **WHEN** a string literal contains `╔══╗` in a CP437 document
- **THEN** the same characters are present after formatting

### Requirement: Format failure leaves the document unchanged
If the compiler reports an error while formatting, the document SHALL NOT change, the error SHALL be shown as a
message, and diagnostics SHALL be updated as for a check.

#### Scenario: Syntax error
- **WHEN** the document contains `x = 1 +` and the user runs Format Document
- **THEN** the document is unchanged and an error message names line 2

### Requirement: Format on save is opt-in
Formatting SHALL run on save only through VS Code's own `editor.formatOnSave` setting; the extension SHALL NOT turn
it on by default.

#### Scenario: Default
- **WHEN** a user installs the extension and saves a file
- **THEN** the file is not reformatted
