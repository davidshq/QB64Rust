# Spec Delta

## MODIFIED Requirements

### Requirement: Language configuration
The extension SHALL configure `'` as the line comment, `()` as brackets, `"` as an auto-closing pair, and folding
for `SUB`…`END SUB`, `FUNCTION`…`END FUNCTION` and `TYPE`…`END TYPE` blocks. These static folding markers SHALL
stay in place when the language server runs (spec `editor/language-server`), which adds the other blocks; the
outline view and go to definition come from the server only.

#### Scenario: Toggle comment
- **WHEN** the user runs Toggle Line Comment on `PRINT 1`
- **THEN** the line becomes `' PRINT 1`, or `'PRINT 1` per the editor's comment style

#### Scenario: Fold a SUB
- **WHEN** a file contains `SUB Foo` … `END SUB`
- **THEN** the editor offers a folding range covering the block, with or without the language server

#### Scenario: Fold a DO loop
- **WHEN** a file contains `DO` … `LOOP` and the language server is running
- **THEN** the editor offers a folding range covering the loop
