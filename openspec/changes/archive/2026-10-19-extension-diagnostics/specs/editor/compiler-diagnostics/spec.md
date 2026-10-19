# Spec Delta

## MODIFIED Requirements

### Requirement: Check on save
When a `qb64rust` document is saved and `qb64rust.checkOnSave` is true (default), the extension SHALL check the
program and replace that program's diagnostics with the result. The command `qb64rust.check` SHALL do the same on
demand. While the language server runs, the check SHALL be its `qb64rust/check` request (spec
`editor/language-server`) and the old compiler SHALL NOT be run; otherwise it SHALL be the old compiler in
C++-generation-only mode (no C++ build).

#### Scenario: Error on save
- **WHEN** the language server does not run and the user saves a file whose line 2 is `x = 1 +`
- **THEN** Problems shows one error "Expected variable/value after '+'" on line 2 of that file

#### Scenario: Error on save from the new compiler
- **WHEN** the language server runs and the user saves a file with `x$ = 1` on line 2 and `GOTO nowhere` on line 3
- **THEN** Problems shows two errors with source `qb64rust`, each at its column, and none with source `qb64pe`

#### Scenario: Fixed on save
- **WHEN** the user fixes the error and saves again
- **THEN** the error disappears from Problems

#### Scenario: Disabled
- **WHEN** `qb64rust.checkOnSave` is false
- **THEN** saving runs neither the old compiler nor the language server's check

#### Scenario: Command without changes
- **WHEN** the language server runs and the user runs `qb64rust.check` on a saved document with no unsaved edits
- **THEN** the check runs and its diagnostics are shown

### Requirement: Diagnostics follow edits
Diagnostics from a check or build of the old compiler (source `qb64pe`) SHALL be cleared for a document when it
is edited, and SHALL be cleared when the document is closed. They SHALL live in their own collection beside the
language server's (source `qb64rust`, spec `editor/language-server`, whose checks follow edits by that spec);
neither collection SHALL clear or replace the other.

#### Scenario: Edit after error
- **WHEN** an error of the old compiler is shown and the user types on any line
- **THEN** the `qb64pe` diagnostics for that document are removed until the next save or check

#### Scenario: Two sources
- **WHEN** a build with the old compiler fails on a line where the language server reports a syntax error
- **THEN** Problems shows two entries on that line, one with source `qb64rust` and one with source `qb64pe`; an
  edit removes the `qb64pe` one and the `qb64rust` one follows the edit

### Requirement: Non-blocking check
A check SHALL NOT block editing. If a new check of the same file starts while one is running, the older one SHALL
be cancelled and its result discarded. A check that runs longer than `qb64rust.checkTimeoutSeconds` (default 30)
SHALL be killed when it is the old compiler's, and its request cancelled and its result discarded when it is the
language server's.

#### Scenario: Rapid saves
- **WHEN** the user saves three times within one second
- **THEN** only the result of the last check is shown
