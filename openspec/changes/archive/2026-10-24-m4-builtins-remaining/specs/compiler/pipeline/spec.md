# Spec Delta

## MODIFIED Requirements

### Requirement: The whole language parses
Every statement and expression form the old compiler accepts SHALL parse into a node of its own kind, without a
parser diagnostic. A construct that the later stages do not compile yet SHALL be reported "not supported yet" by
`sema` at that node, not by the parser.

#### Scenario: Statement not compiled yet
- **WHEN** a program contains `DO: FOR r.m(1) = 1 TO 2: NEXT: LOOP UNTIL a > 0`, with `r` of a `TYPE` that has the
  member array `m` (`verification\v56_f3`, a form step 3 of M4 left marked)
- **THEN** the tree has a `DoBlock` holding the `FOR` block, and the only diagnostic is one "not supported yet"
  error at the `FOR` variable

#### Scenario: Built-in statement read by its template
- **WHEN** a program contains `LINE (0, 0)-(9, 9), , BF`
- **THEN** it parses into one `BuiltinStmt` with `BF` as a word, not as a variable

#### Scenario: Member access after an index
- **WHEN** a program contains `PRINT a(1).b`
- **THEN** it parses without a diagnostic, and the item is a member access on `a(1)`

### Requirement: Metacommands the old compiler does not know
A `$` line outside a comment that is no metacommand of the old compiler SHALL be a compile error at that line,
with the old compiler's "Syntax error" as its note, in the main file and in an included one (measured,
`verification\v57_m01`–`m08`). `$NOPREFIX` SHALL be a compile error with the old compiler's own message as its
note. Every metacommand the old compiler knows SHALL be built, `$ERRORLOCATION` (`language/error-handling`) and
`$USELIBRARY` (`language/external-libraries`) among them; none is "not supported yet".

#### Scenario: Other lines that are no metacommand
- **WHEN** a line is `$STATIC`, `$nosuch`, `$FOO:BAR`, `$CONSOLE:OFF`, a `$` alone, or `$INCLUDE: 'x.bi'`
- **THEN** the compile fails with an error at that line, with "Syntax error" as its note (`verification\v57_m01`–
  `m05`, `m08`)

#### Scenario: Unknown name
- **WHEN** an active line of a program is `$NOSUCHTHING`
- **THEN** the compile fails with an error at that line, not marked "not supported yet" (`verification\v16_m6`)

#### Scenario: Unknown name in an inactive branch
- **WHEN** `$NOSUCHTHING` stands inside `$IF 0 THEN` … `$END IF`
- **THEN** no diagnostic is given for it

#### Scenario: $DYNAMIC without a comment
- **WHEN** a program's line is `$DYNAMIC`
- **THEN** the compile fails with an error at that line (`verification\v24_x38`)

#### Scenario: A name close to $MIDISOUNDFONT
- **WHEN** a line is `$MIDISOUNDFONT`, `$MIDISOUNDFONT :DEFAULT`, `$MIDISOUNDFONTS:DEFAULT` or `$MIDISOUND:DEFAULT`
- **THEN** the compile fails with an error at that line (`verification\v39_x74`, `x79`–`x81`)

#### Scenario: $NOPREFIX
- **WHEN** a program has `$NOPREFIX`
- **THEN** the compile fails with an error whose note is the old compiler's message that `$NOPREFIX` is deprecated
  (`verification\v43_x32`)

#### Scenario: A metacommand not built yet
- **WHEN** a program has `$ERRORLOCATION:ON`, or `$USELIBRARY:'test/a'` with that library under the compiler root
- **THEN** it compiles, with no "not supported yet": no metacommand of the old compiler is left unbuilt

#### Scenario: A form of $ERRORLOCATION it does not know
- **WHEN** a line is `$ERRORLOCATION`, `$ERRORLOCATION:MAYBE`, `$ERRORLOCATION ON`, `$ERRORLOCATION: ON` or
  `$ERRORLOCATION:ON ' a comment` (the old compiler compares the whole line)
- **THEN** the compile fails with an error at that line, with "Syntax error" as its note
  (`verification\v58_e14`–`e18`); `$errorlocation:on` in lower case compiles
