# Spec Delta

## ADDED Requirements

### Requirement: Check request
The server SHALL answer the request `qb64rust/check`, whose parameter names a document by `uri`, by running the
compiler's whole front end over that document's program and publishing what it finds. The program is read as
the live parse reads it (open documents first, then the disk, in their encodings). The answer SHALL be the
numbers of errors and warnings found, `{ "errors": n, "warnings": n }`.

#### Scenario: Type error on check
- **WHEN** an open document's line 2 is `x$ = 1` and the client sends `qb64rust/check` for it
- **THEN** the answer is `{ "errors": 1, "warnings": 0 }` and an error on line 2 is published, which the live
  parse alone does not report

#### Scenario: Not an open document
- **WHEN** `qb64rust/check` names a uri that is not open
- **THEN** the server answers with the `InvalidParams` error and publishes nothing

### Requirement: Check diagnostics
A check SHALL publish, under the uri of the file each is in, every error and warning the CLI reports for the same
files with `-w`: the parser's, `sema`'s (with "not supported yet" in the message where the CLI marks it) and the
missing files of `$EXEICON` and `$EMBED`. Each SHALL have the CLI's message, line and column, the old compiler's
wording as the second line where the CLI gives it a note, source `qb64rust`, and severity error or warning.

#### Scenario: Several errors in one run
- **WHEN** a program has an undefined `GOTO` label on line 3 and `x$ = 1` on line 7 and is checked
- **THEN** both errors are published, each at the column of the CLI's report

#### Scenario: Same as the CLI
- **WHEN** a program that the CLI rejects is checked
- **THEN** the published diagnostics are those `qb64rust -w` prints for it, file, line, column and message alike

#### Scenario: Warning
- **WHEN** a checked program has `$DEBUG` on line 1
- **THEN** a diagnostic with severity warning is published on line 1

#### Scenario: Missing icon
- **WHEN** a checked program has `$EXEICON:'./game.ico'` and no `game.ico` lies beside it
- **THEN** an error on the metacommand's line names `./game.ico`

#### Scenario: Error in an included file
- **WHEN** `main.bas` includes `lib.bi`, line 4 of `lib.bi` is `y$ = 2`, and `main.bas` is checked
- **THEN** the error is published under `lib.bi`'s uri at line 4

### Requirement: Check result until the next parse
The diagnostics of a check SHALL stand until the next parse of the same program after a change, which SHALL
publish the parser's diagnostics alone again for the files it covers. A newer check of the same program SHALL
replace an older one; an older check that ends later SHALL publish nothing and SHALL be answered with the
`ContentModified` error.

#### Scenario: Edit after a check
- **WHEN** a check has published a type error and the user then types in the document
- **THEN** after the parse that follows, only the parser's diagnostics are published for it

#### Scenario: Two checks in a row
- **WHEN** two `qb64rust/check` requests for one program arrive before the first has ended
- **THEN** only the second one's diagnostics are published

## MODIFIED Requirements

### Requirement: Syntax diagnostics
For every open document the server SHALL publish the parser's diagnostics of the program the document is, after
every change, under the uri of the file each diagnostic is in, with severity error and source `qb64rust`. It SHALL
NOT publish anything from the later stages (no "not supported yet" of `sema`, no type errors) after a change; the
later stages run only for a check (requirement "Check request"). A diagnostic the
parser itself marks "not supported yet" (a syntax error at a BASIC word, an expression nested too deep) is
published with the marker in its message. An error with the old compiler's wording (requirement "Diagnostics
output" of `compiler/cli`) SHALL have it as a second line of its message, `QB64pe: <wording>`.

#### Scenario: Error appears and goes
- **WHEN** the user types `x = 1 +` on a line and then completes it to `x = 1 + 2`
- **THEN** a diagnostic appears on that line after the first edit and is gone after the second, without a save

#### Scenario: Unsupported statement is no diagnostic
- **WHEN** an open document contains a statement `sema` rejects and the parser accepts, and no check has run
- **THEN** the server publishes no diagnostic for that line

#### Scenario: Error in an included file
- **WHEN** the open main file includes `lib.bi` and `lib.bi` has a syntax error on line 3
- **THEN** the diagnostic is published under `lib.bi`'s uri at line 3, and none under the main file for it

#### Scenario: The old compiler's wording
- **WHEN** a parser error the table holds is published
- **THEN** its message is the error's, a line end, and `QB64pe: ` with the old compiler's wording
