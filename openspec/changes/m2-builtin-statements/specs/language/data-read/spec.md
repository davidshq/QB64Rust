# Spec Delta

## Purpose

How `DATA` statements become the program's data and how `READ` and `RESTORE` use it, as measured with
`qb64pe.exe`.

## ADDED Requirements

### Requirement: The program's data
The items of every `DATA` statement of the program, in the order the old compiler collects them (main module and
procedures, included files in their place), SHALL form one list of data for the whole program. An item SHALL be
its text as the old compiler splits it: blanks around an unquoted item dropped, a quoted item kept with its blanks
and commas, an empty item kept as an empty item.

#### Scenario: Quoted and bare items
- **WHEN** a program with `DATA 1,"two",3.5` and, later, `DATA  bare word ,9` reads five strings and prints each in
  brackets
- **THEN** it prints `[1]`, `[two]`, `[3.5]`, `[bare word]`, `[9]`

#### Scenario: DATA after the READ
- **WHEN** `READ a: PRINT a` stands before `DATA 7` in the file
- **THEN** it prints ` 7 `

### Requirement: READ
`READ target, …` SHALL store the next item into each target in turn (a variable, an array element or a member), a
numeric target taking the item's value converted to the target's type and a string target its text, as the old
compiler's runtime does. Reading when no item is left SHALL raise error 4. What an item that is no number gives a
numeric target SHALL be as measured.

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

### Requirement: RESTORE
`RESTORE` SHALL make the next `READ` start at the program's first item, and `RESTORE label` at the first item of
the first `DATA` statement after that label, as the old compiler resolves it. A label that does not exist SHALL be
a compile error.

#### Scenario: Back to the start
- **WHEN** `READ a: RESTORE: READ b: PRINT a; b` runs with `DATA 1,2`
- **THEN** it prints ` 1  1 `

#### Scenario: To a label
- **WHEN** `RESTORE two: READ s$: PRINT s$` runs with `DATA 1` and, after the label `two:`, `DATA x,9`
- **THEN** it prints `x`

#### Scenario: Unknown label
- **WHEN** `RESTORE nowhere` is compiled in a program with no label `nowhere`
- **THEN** it is a compile error
