# Spec Delta

## ADDED Requirements

### Requirement: The name of a fixed-length string FUNCTION elsewhere
A name written `name$n` (a `$` and a length) SHALL be a compile error where the old compiler rejects it: as the
name of a SUB ("Type symbols after a SUB name are invalid"), of a `CONST` ("Invalid constant type") and of a
`TYPE` member ("Invalid name", also `AS STRING * n`), whether a FUNCTION of that name exists or not
(`verification\v56_x19`–`x22`). The old compiler's wording SHALL be the note.

#### Scenario: A SUB of the FUNCTION's name
- **WHEN** a program has `FUNCTION fs$5 (x AS STRING)` and `SUB fs$5` (`verification\v21_x54`)
- **THEN** it is a compile error at the SUB, with the note "Type symbols after a SUB name are invalid"

#### Scenario: A CONST
- **WHEN** a program has `CONST fs$5 = "a"` and the FUNCTION `fs$5` (`verification\v21_x55`)
- **THEN** it is a compile error at the `CONST`, with the note "Invalid constant type"

#### Scenario: A TYPE member
- **WHEN** a `TYPE t` has the member `fs$5 AS STRING` (`verification\v21_x57`)
- **THEN** it is a compile error at the member, with the note "Invalid name"
