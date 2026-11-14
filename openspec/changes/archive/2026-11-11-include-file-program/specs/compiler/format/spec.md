# Spec Delta

## MODIFIED Requirements

### Requirement: Program with an error
When the parser reports an error (a syntax error, or an included file that is not found), `-y` SHALL print the
diagnostics as a compile prints them, exit with status 1, and write no output file. An error of a later stage (a
SUB that is not defined, a type error, a bad `qb64rust.toml`) SHALL NOT stop formatting: `-y` SHALL print the
diagnostics, write the output file and exit with status 0. A warning SHALL NOT stop formatting.

#### Scenario: Syntax error
- **WHEN** a program whose second line is `x = 1 +` is formatted
- **THEN** the exit status is 1, the error names line 2, and no output file exists

#### Scenario: A SUB that is not defined
- **WHEN** a file whose only line is `call   missing` is formatted
- **THEN** the error `CALL missing`: no SUB of this name is printed, the exit status is 0, and the output file
  reads `Call missing`

## ADDED Requirements

### Requirement: Names in an incomplete program
A program is incomplete when a later stage reports an error, or when its main file's name does not end in
`.bas`. In an incomplete program a name SHALL take the spelling of its definition only when a statement declares
it: a `SUB`, `FUNCTION` or `DECLARE` line, `DIM`, `REDIM`, `COMMON`, `SHARED`, `STATIC`, `CONST`, a parameter, a
`TYPE` block, or a label. A variable that no statement declares SHALL be written as read at each use. Built-ins
and keywords SHALL be written as in a complete program.

#### Scenario: A variable declared elsewhere
- **WHEN** `lib.bm` holds `SUB a`, `atkfac = 1`, `ATKFAC = 2`, `END SUB` and is formatted alone
- **THEN** the two lines read `atkfac = 1` and `ATKFAC = 2`

#### Scenario: A name the file declares
- **WHEN** `lib.bm` holds `SUB Bump`, `DIM Total AS LONG`, `TOTAL = 1`, `END SUB`, `SUB b`, `BUMP`, `CALL other`,
  `END SUB` and is formatted alone
- **THEN** the lines read `total` as `Total` and `BUMP` as `Bump`, and `Call other` keeps `other` as read

#### Scenario: Alone and inside its program
- **WHEN** an include file is formatted alone, and then as a file of the program that includes it
- **THEN** the two results are equal but for the letter case of names, and every name whose case the first
  changed has the same case in the second

#### Scenario: A complete program is not affected
- **WHEN** `prog.bas` without errors holds `total = 1` and then `TOTAL = 2`, with no declaration of `total`
- **THEN** both lines read `total`, as `qb64pe -y` writes them

### Requirement: Lines of an incomplete program keep their tokens
In an incomplete program, a line whose layout would read as other tokens than the line, letter case and blanks
aside, SHALL be indented and otherwise kept as typed but for letter case. When that also reads as other tokens,
the line SHALL be written as read.

#### Scenario: A statement the check rejects is not changed
- **WHEN** `prog.bas` holds `kill ("a", "b")`, which the check rejects
- **THEN** the line reads `Kill ("a", "b")`, not `Kill ("a"), "b"`
