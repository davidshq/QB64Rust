# Spec Delta

## ADDED Requirements

### Requirement: Built-in statements in the IR
The IR SHALL represent a built-in statement by the built-in's identity, never by a libqb name: one slot per
argument in table order, each a value, a place or absent, and the words chosen where the statement's form offers a
choice. File output and input, `READ` and console input SHALL be operations of their own that list their items or
targets in order. Each SHALL be marked as possibly raising, and the IR SHALL state where each checks for a pending
error (file output and input before each item or target, `READ` nowhere, as measured) so that the emitter decides
none of it.

#### Scenario: A statement call lowered
- **WHEN** `OPEN f$ FOR OUTPUT AS #1` is lowered
- **THEN** the IR names the built-in `OPEN` with the name value, the chosen mode word, the file number value and
  the absent slots, and no `sub_open`, mode number or `passed` mask appears before the C++ output

#### Scenario: A place argument
- **WHEN** `SWAP x(i), y` is lowered
- **THEN** the IR names two places, and no temporary or address-of appears before the C++ output

#### Scenario: Items checked one by one
- **WHEN** `PRINT #1, "a"; CHR$(-1); "b"` is lowered
- **THEN** the IR lists three items for file 1 under the rule that a raising item skips the rest of the statement,
  as for `PRINT`

### Requirement: Program data in the IR
The IR SHALL hold the program's `DATA` items as one ordered list of byte strings, each marked quoted or not, with
the position in that list of each label that a `RESTORE` names. `READ` SHALL name its target places and `RESTORE`
a position, with the label that gave it; the encoding of the list in the executable SHALL be the emitter's.

#### Scenario: Data lowered
- **WHEN** a program with `DATA 1,"two"` and, after the label `lab:`, `DATA 3` and `RESTORE lab` is lowered
- **THEN** the IR's data list has three items, the second quoted, the restore names position 2 and the label
  `LAB`, and no `data_offset` name or separator byte appears before the C++ output

## MODIFIED Requirements

### Requirement: The whole language parses
Every statement and expression form the old compiler accepts SHALL parse into a node of its own kind, without a
parser diagnostic. A construct that the later stages do not compile yet SHALL be reported "not supported yet" by
`sema` at that node, not by the parser.

#### Scenario: Statement not compiled yet
- **WHEN** a program contains `DO: LSET a$ = b$: LOOP UNTIL a > 0`
- **THEN** the tree has a `DoBlock` holding an `LsetStmt`, and the only diagnostic is one "not supported yet" error
  at `LSET`

#### Scenario: Built-in statement read by its template
- **WHEN** a program contains `LINE (0, 0)-(9, 9), , BF`
- **THEN** it parses into one `BuiltinStmt` with `BF` as a word, not as a variable

#### Scenario: Member access after an index
- **WHEN** a program contains `PRINT a(1).b`
- **THEN** it parses without a diagnostic, and the item is a member access on `a(1)`
