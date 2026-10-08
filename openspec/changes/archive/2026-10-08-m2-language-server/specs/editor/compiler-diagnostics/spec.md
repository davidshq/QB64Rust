# Spec Delta

## MODIFIED Requirements

### Requirement: Diagnostics follow edits
Diagnostics from a check or build (source `qb64pe`) SHALL be cleared for a document when it is edited, and SHALL
be cleared when the document is closed. They SHALL live in their own collection beside the language server's
(source `qb64rust`, spec `editor/language-server`); neither collection SHALL clear or replace the other.

#### Scenario: Edit after error
- **WHEN** an error is shown and the user types on any line
- **THEN** the `qb64pe` diagnostics for that document are removed until the next save or check

#### Scenario: Two sources
- **WHEN** a saved file has a syntax error reported by both the language server and the old compiler
- **THEN** Problems shows two entries on that line, one with source `qb64rust` and one with source `qb64pe`; an
  edit removes the `qb64pe` one and the `qb64rust` one follows the edit
