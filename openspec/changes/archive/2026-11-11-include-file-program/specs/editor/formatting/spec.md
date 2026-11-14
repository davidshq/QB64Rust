# Spec Delta

## ADDED Requirements

### Requirement: Format an included file
Format Document and Format Selection SHALL work on a `qb64rust` document that is an included file (`.bi`, `.bm`),
whether or not the program that includes it is open or found. When the language server runs, the result SHALL be
the server's (spec `editor/language-server`, "Formatting an included file"). An error that is not a syntax error
SHALL NOT stop formatting and SHALL NOT show a message.

#### Scenario: Include file of a program on disk
- **WHEN** `turn.bm` is open, `CWSTRAT.BAS` beside it includes it and is not open, and the user runs Format
  Document
- **THEN** `turn.bm` is formatted, with each name in the letter case of its declaration in the program

#### Scenario: Include file with no program
- **WHEN** `lib.bm` calls a SUB that no file defines, no program includes it, and the user runs Format Document
- **THEN** `lib.bm` is indented and spaced, its keywords are in the keyword case, and no error message is shown
