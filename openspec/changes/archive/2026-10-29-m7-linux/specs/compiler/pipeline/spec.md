## MODIFIED Requirements

### Requirement: Preprocessor
`$IF`, `$ELSEIF`, `$ELSE`, `$END IF` and `$LET` SHALL be evaluated in file order across included files, for the
64-bit platform the program is compiled on: `WIN` and `WINDOWS` SHALL be -1 on Windows and 0 on Linux, and `LINUX`
SHALL be -1 on Linux and 0 on Windows, as the old compiler sets them from its own platform. Code in an inactive
branch SHALL be kept in the tree but not parsed or checked; an active `$ERROR` SHALL be an error.

#### Scenario: Inactive branch
- **WHEN** a program contains `$IF LINUX THEN`, a line that is not valid BASIC, and `$END IF`, and is compiled on
  Windows
- **THEN** no error is reported for that line

#### Scenario: The platform's branch
- **WHEN** a program compiled on Linux contains `$IF WIN THEN`, a `DECLARE LIBRARY` of a header that is there,
  `$ELSE`, a `DECLARE LIBRARY "nowhere"`, and `$END IF`
- **THEN** the compile fails with "LIBRARY not found" at the `$ELSE` branch's block, as with the old compiler on
  Linux (`verification\v37_i_places`)

#### Scenario: Block header in a $IF branch, closer outside
- **WHEN** a program contains `$IF WIN THEN`, `IF x THEN`, `$END IF`, a statement and `END IF`
- **THEN** the compile fails with an error at the `$END IF` (the old compiler rejects it too, reporting the
  `END IF`)

#### Scenario: SUB inside a $IF branch
- **WHEN** a program contains `$IF WIN THEN`, a whole `SUB` … `END SUB`, and `$END IF`
- **THEN** no error is reported for the nesting

#### Scenario: Guarded self-include
- **WHEN** `a.bi` contains `$IF DONE = UNDEFINED THEN`, `$LET DONE = 1`, `'$INCLUDE:'a.bi'` and `$END IF`, and the
  main file includes `a.bi`
- **THEN** no error is reported, and `a.bi` is parsed twice, the second time with the branch inactive

#### Scenario: $LET in an included file
- **WHEN** `lib.bi` contains `$LET FAST = 1` and the main file, after including it, contains `$IF FAST = 1 THEN`
- **THEN** the branch is active
