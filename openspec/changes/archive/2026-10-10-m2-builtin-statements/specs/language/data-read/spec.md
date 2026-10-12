# Spec Delta

## Purpose

How `DATA` statements become the program's data and how `READ` and `RESTORE` use it, as measured with
`qb64pe.exe`.

## ADDED Requirements

### Requirement: The program's data
The items of every `DATA` statement of the program, in the order the old compiler collects them (main module and
procedures, included files in their place), SHALL form one list of data for the whole program. An item SHALL be
its text as the old compiler splits it: blanks around an unquoted item dropped, a quoted item kept with its blanks
and commas, an empty item kept as an empty item. `DATA` with nothing after it SHALL be one empty item. A `DATA`
statement SHALL count wherever it stands, also in a block that never runs and after the program's last executed
statement. Text after a closing quote other than blanks SHALL be a compile error.

#### Scenario: Quoted and bare items
- **WHEN** a program with `DATA 1,"two",3.5` and, later, `DATA  bare word ,9` reads five strings and prints each in
  brackets
- **THEN** it prints `[1]`, `[two]`, `[3.5]`, `[bare word]`, `[9]`

#### Scenario: DATA after the READ
- **WHEN** `READ a: PRINT a` stands before `DATA 7` in the file
- **THEN** it prints ` 7 `

#### Scenario: File order across procedures
- **WHEN** a program has `DATA m` in the main module, `DATA never` inside `IF 0 THEN … END IF`, `DATA s` after
  `SYSTEM` and `DATA p` inside a SUB, in that order in the file, and reads four strings
- **THEN** they are `m`, `never`, `s`, `p`

#### Scenario: Bare DATA
- **WHEN** a program with `DATA` and then `DATA x` reads two strings
- **THEN** the first is empty and the second is `x`

### Requirement: READ
`READ target, …` SHALL store the next item into each target in turn (a variable, an array element or a member), a
numeric target taking the item's value converted to the target's type and a string target its text, as the old
compiler's runtime does. Reading when no item is left SHALL raise error 4. An unquoted item that is no number, and
every quoted item, read into a numeric target SHALL raise error 2, and a number outside the target's range error
6; the target SHALL then be 0, every later target of that `READ` SHALL be 0 or, a string, unchanged, and the item
SHALL stay unread, so that the next `READ` meets it again. A target that is not a place SHALL be a compile error,
as SHALL the name of the FUNCTION the statement stands in. A target in parentheses and a whole array, which the
old compiler accepts, SHALL be reported "not supported yet".

#### Scenario: Numbers and strings
- **WHEN** `READ a&, s$, d#: PRINT a&; s$; d#` runs with `DATA 1,"two",3.5`
- **THEN** it prints ` 1 two 3.5 `

#### Scenario: Out of data
- **WHEN** a second `READ a` runs with `DATA 1` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 4

#### Scenario: Into an element
- **WHEN** `DIM x(2) AS INTEGER: FOR i = 0 TO 2: READ x(i): NEXT: PRINT x(0); x(1); x(2)` runs with `DATA 4,5,6`
- **THEN** it prints ` 4  5  6 `

#### Scenario: Target is not a place
- **WHEN** `READ 5` is compiled
- **THEN** it is a compile error

#### Scenario: An item that is no number stays
- **WHEN** `READ a&, b&, c&` runs with `DATA 11, zz, 33` under a handler that prints `ERR` and resumes next, and
  then `READ s$`
- **THEN** the handler prints 2 once, `a&` is 11, `b&` and `c&` are 0, and `s$` is `zz`

#### Scenario: A quoted number
- **WHEN** `READ a&` runs with `DATA "12"` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 2 and `a&` is 0, as with the old compiler

#### Scenario: Too large for the target
- **WHEN** `READ b%%` runs with `DATA 300` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 6 and `b%%` is 0

#### Scenario: The FUNCTION's own name
- **WHEN** `READ f&` is compiled inside `FUNCTION f&`
- **THEN** it is a compile error, as in the old compiler

### Requirement: RESTORE
`RESTORE` SHALL make the next `READ` start at the program's first item, and `RESTORE label` at the first item of
the first `DATA` statement after that label in the file, as the old compiler resolves it: the label MAY stand in
the main module or in any procedure, whatever body the `RESTORE` is in, and when no `DATA` follows it the next
`READ` SHALL raise error 4. A label that does not exist SHALL be a compile error, as SHALL a label name that more
than one body has. A line number as the target SHALL be reported "not supported yet".

#### Scenario: Back to the start
- **WHEN** `READ a: RESTORE: READ b: PRINT a; b` runs with `DATA 1,2`
- **THEN** it prints ` 1  1 `

#### Scenario: To a label
- **WHEN** `RESTORE two: READ s$: PRINT s$` runs with `DATA 1` and, after the label `two:`, `DATA x,9`
- **THEN** it prints `x`

#### Scenario: Unknown label
- **WHEN** `RESTORE nowhere` is compiled in a program with no label `nowhere`
- **THEN** it is a compile error

#### Scenario: A label of a procedure
- **WHEN** the main module runs `RESTORE inp: READ s$: PRINT s$` and SUB `p` holds the label `inp:` followed by
  `DATA q`
- **THEN** it prints `q`

#### Scenario: A label two bodies have
- **WHEN** `RESTORE lab` is compiled in a program whose main module and one SUB each have a label `lab:`
- **THEN** it is a compile error
