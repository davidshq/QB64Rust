# Spec Delta

## ADDED Requirements

### Requirement: A statement that names no SUB
A name used as a statement that is no SUB, no keyword and no built-in statement SHALL be a compile error that is
not "not supported yet", with or without arguments, also as the target of `IF c THEN name`, where only a line
number names a line ("Syntax error" in the old compiler). `CALL` of such a name SHALL be such an error too
("Expected CALL sub-name").

#### Scenario: A label after THEN
- **WHEN** a program has `IF ASC("") = 7 THEN jumped` and a label `jumped:`
- **THEN** it is a compile error that is not "not supported yet" (`verification\v17_b_if_then_label`)

#### Scenario: A SUB that does not exist
- **WHEN** a program has `UndefinedSub` or `CALL UndefinedSub` alone on a line and defines no such SUB
- **THEN** each is a compile error that is not "not supported yet"
  (`tests\snippets\qb64fresh\error_detection__undefined_sub_call`, `verification\v55_x`)

## MODIFIED Requirements

### Requirement: Reserved names
A variable, parameter or procedure SHALL NOT have a name that, without its suffix, is a keyword or a built-in
written without a required suffix (such as `LEN`, `CLS`, `NAME`, `ERR`), when it has no suffix or `&`; nor the
name of a built-in with its required suffix (such as `LEFT$`). Such a program SHALL be rejected at compile time.
With any other suffix (`$`, `%`, `!`, `#`, `%%`, `&&`, `##`, `~` forms, `` ` ``), a variable, parameter or array of
such a name SHALL be accepted or rejected as the old compiler does, word by word and suffix by suffix, as
`verification\v52_builtin_name_suffixes` records it; where it is accepted it SHALL be an ordinary variable,
parameter or array of that type in every place a name may stand (`DIM`, `DIM SHARED`, `REDIM`, `COMMON SHARED`,
`STATIC`, `SHARED`, a parameter, a name not declared; `v52_x20`–`x37`). A procedure of such a name SHALL be
rejected with a compile error that is not "not supported yet" ("Name already in use" in the old compiler) where
`v52_x28` (`FUNCTION exp$`) and the procedures of `verification\v55_x` record it, and SHALL stay the mark "not
supported yet" for a keyword or suffix they do not record. A `CONST` of such a name SHALL be
rejected with "Name already in use" whatever its suffix, since the old compiler looks a constant's name up without
its suffix (`CONST key$`, `exp$`, `key!`; `v52_x32`, `x67`, `x68`).
A name that matches a built-in only without its required suffix (such as `left`, `chr`) SHALL be
allowed. A main-module variable SHALL NOT have the name of a procedure.

#### Scenario: Parameter named after a statement
- **WHEN** a program declares `FUNCTION greet$ (name AS STRING)`
- **THEN** compiling it fails with an error naming `name`

#### Scenario: Suffix does not free a name
- **WHEN** a program defines `FUNCTION len& (a AS LONG)`
- **THEN** compiling it fails with an error naming `len`

#### Scenario: Name free without the required suffix
- **WHEN** a program defines `SUB chr (a AS LONG)` and has a variable `left`
- **THEN** it compiles

#### Scenario: A keyword with $ as a variable
- **WHEN** a program runs `DIM section$, key$: key$ = "k": PRINT key$` (QB64pe `source\utilities\ini-manager\ini.bm`)
- **THEN** it prints `k`

#### Scenario: A built-in function's name with $ as a parameter
- **WHEN** a program has `SUB s (exp$, base$)` and calls `s "1", "2"`, and the SUB prints `exp$ + base$`
- **THEN** it prints `12`

#### Scenario: The function beside the variable
- **WHEN** a program runs `exp$ = "2": PRINT exp$; EXP(0)`
- **THEN** it prints `2 1`

#### Scenario: A FUNCTION named after a built-in with $
- **WHEN** a program defines `FUNCTION exp$ (a$)`
- **THEN** compiling it fails with an error naming `exp$` that is not "not supported yet" (`verification\v52_x28`)

#### Scenario: A CONST named after a keyword with a suffix
- **WHEN** a program has `CONST key! = 1`
- **THEN** compiling it fails with an error naming `key!`

#### Scenario: Every recorded form
- **WHEN** a form of `v52_builtin_name_suffixes` is compiled
- **THEN** it is a variable where the record says "variable", and a compile error where it says "rejected"
