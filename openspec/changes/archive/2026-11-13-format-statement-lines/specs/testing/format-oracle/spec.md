# Spec Delta

## ADDED Requirements

### Requirement: The rewrites in tier 1
`cargo test` SHALL format each default recording's source in process three more times: with
`OneStatementPerLine`, with `BlockIf`, and with both. These have no recording. Each output SHALL pass the checks
of spec `compiler/format`, "A rewritten program is the same program": the same typed tree and C++ as the
source, unchanged when formatted again, and nothing left to rewrite. A failure SHALL name the program, the
rewrite and the first line that differs.

#### Scenario: A rewrite changes a program
- **WHEN** a change to the formatter makes `BlockIf` write `IF a THEN x = 1 ELSE y = 2` without its `Else`
- **THEN** `cargo test` fails and names the program, `BlockIf` and the first line of the C++ that differs

#### Scenario: A rewrite misses a line
- **WHEN** a program formatted with both rewrites still has a single-line `IF` on a line that no rule leaves
  alone
- **THEN** `cargo test` fails and names the program and the line

#### Scenario: A program that is incomplete
- **WHEN** a recorded program has an error of a later stage
- **THEN** its three rewrite outputs equal its output without the rewrites, and the checks pass

### Requirement: The rewritten forms measured with the old compiler
`verification\` SHALL hold a family of pairs of programs recorded with `qb64pe.exe`: one with a packed form,
and one that is the first as the formatter writes it with a rewrite on. The two recordings of a pair SHALL be
equal. `cargo test` SHALL fail when the formatter's output for the first is not the text of the second. The
family SHALL cover the forms of the rewrite scenarios of spec `compiler/format`.

#### Scenario: A block IF after a colon
- **WHEN** one program runs `x = 1: IF a THEN b = 1: x = 2` and its pair runs the three lines `x = 1: If a Then`,
  `b = 1: x = 2`, `End If`, each for `a` zero and for `a` not zero
- **THEN** the two recordings with `qb64pe.exe` print the same values of `x` and `b`

#### Scenario: The formatter drifts from the measured form
- **WHEN** a change makes `BlockIf` write that program otherwise than its pair's text
- **THEN** `cargo test` fails and names the pair and the first line that differs

#### Scenario: Run with the new compiler
- **WHEN** the family is run with `qb64rust`
- **THEN** each program's output equals its recording
