# Spec Delta

## ADDED Requirements

### Requirement: _DEFINE and DEFtype without their letters
`_DEFINE AS type` and `_DEFINE AS` (no letters) SHALL be a compile error with the old compiler's "_DEFINE:
Expected ... AS ..." as its note. `DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL` or `DEFSTR` with `AS type` after its
letters SHALL be a compile error, with "_DEFINE: Expected - or ," as its note after a single letter and "_DEFINE:
Expected ," after a range (measured, `verification\v24_x11`, `x19`, `v57_e05`–`e07`).

#### Scenario: No letters
- **WHEN** a program has `_DEFINE AS LONG`
- **THEN** the compile fails with an error, not marked "not supported yet" (`verification\v24_x11`)

#### Scenario: AS after the letters of DEFINT
- **WHEN** a program has `DEFINT a AS LONG`
- **THEN** the compile fails with an error, not marked "not supported yet" (`verification\v24_x19`)

#### Scenario: AS after a range
- **WHEN** a program has `DEFLNG a-c AS LONG`
- **THEN** the compile fails with an error whose note is "_DEFINE: Expected ," (`verification\v57_e05`)
