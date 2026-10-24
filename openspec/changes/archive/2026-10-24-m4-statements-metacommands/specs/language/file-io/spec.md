# Spec Delta

## ADDED Requirements

### Requirement: WRITE with a semicolon
A `;` between the items of `WRITE` or `WRITE #n`, or after its last item, SHALL be a compile error with the old
compiler's "Invalid expression" as its note, and a `;` after its file number in place of the `,` one with
"Expected # ... ," (measured, `verification\v22_x57`, `x58`, `v57_e03`, `e04`).

#### Scenario: Between two items of a file
- **WHEN** a program has `WRITE #1, "w"; "semicolon"`
- **THEN** the compile fails with an error at the `;`, not marked "not supported yet" (`verification\v22_x57`)

#### Scenario: Between two items of the console
- **WHEN** a program has `WRITE 1; 2`
- **THEN** the compile fails with an error at the `;` (`verification\v22_x58`)

#### Scenario: After the file number
- **WHEN** a program has `WRITE #1; "a"`
- **THEN** the compile fails with an error at the `;`, with "Expected # ... ," as its note (`verification\v57_e03`)
