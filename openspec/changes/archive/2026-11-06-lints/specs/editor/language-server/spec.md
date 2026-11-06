# Spec Delta

## MODIFIED Requirements

### Requirement: Check diagnostics
A check SHALL publish, under the uri of the file each is in, every error and warning the CLI reports for the same
files with `-w`: the parser's, `sema`'s (with "not supported yet" in the message where the CLI marks it) and the
missing files of `$EXEICON` and `$EMBED`. Each SHALL have the CLI's message, line and column, the old compiler's
wording as the second line where the CLI gives it a note, source `qb64rust`, and severity error or warning. A lint
warning SHALL also have its code (spec `compiler/lints`, "Codes").

#### Scenario: Several errors in one run
- **WHEN** a program has an undefined `GOTO` label on line 3 and `x$ = 1` on line 7 and is checked
- **THEN** both errors are published, each at the column of the CLI's report

#### Scenario: Same as the CLI
- **WHEN** a program that the CLI rejects is checked
- **THEN** the published diagnostics are those `qb64rust -w` prints for it, file, line, column and message alike

#### Scenario: Warning
- **WHEN** a checked program has `$DEBUG` on line 1
- **THEN** a diagnostic with severity warning is published on line 1

#### Scenario: Lint code
- **WHEN** a checked program has a label on line 4 that nothing names
- **THEN** a warning with code `unused-label` is published on line 4

#### Scenario: Missing icon
- **WHEN** a checked program has `$EXEICON:'./game.ico'` and no `game.ico` lies beside it
- **THEN** an error on the metacommand's line names `./game.ico`

#### Scenario: Error in an included file
- **WHEN** `main.bas` includes `lib.bi`, line 4 of `lib.bi` is `y$ = 2`, and `main.bas` is checked
- **THEN** the error is published under `lib.bi`'s uri at line 4
