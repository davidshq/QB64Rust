# Spec Delta

## ADDED Requirements

### Requirement: Lints table of the settings file
The `[lints]` table of the program's `qb64rust.toml` (the file of spec `compiler/fixes`) SHALL hold `all =
true|false` and `<code> = true|false` for each of the nine codes of "Codes". A single code's setting SHALL
override `all`. A lint that is off SHALL give no warning: not with `-w`, and not in the language server. With no
table, every lint SHALL be on. A warning without a code SHALL NOT be affected.

#### Scenario: One lint off
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[lints]` and `unused-label = false`, and `p.bas` has an unused
  label and an unused SUB
- **THEN** `qb64rust -w p.bas` prints the `unused-procedure` warning and no `unused-label` warning

#### Scenario: All off but one
- **WHEN** the table holds `all = false` and `used-once = true`
- **THEN** only `used-once` warnings are given

#### Scenario: No table
- **WHEN** `qb64rust.toml` holds only a `[fixes]` table
- **THEN** the warnings are those of a program with no `qb64rust.toml`

#### Scenario: A warning without a code
- **WHEN** the table holds `all = false` and line 2 of the program is `CONST c = 2 ^ 3 ^ 2`
- **THEN** `qb64rust -w` still prints that line's warning

### Requirement: Command-line lints
`--lints on|off` SHALL set every lint and `--lint <code>=on|off` a single lint, either one any number of times.
The command line SHALL override the file: `--lints` replaces the file's whole set, then each `--lint` overrides
its lint. An unknown code or value SHALL be an error with exit status 1 that names the known codes.

#### Scenario: On again for one run
- **WHEN** `qb64rust.toml` holds `[lints]` and `all = false`, and `qb64rust -w --lints on p.bas` is run
- **THEN** every lint warning of `p.bas` is printed

#### Scenario: One lint off
- **WHEN** `qb64rust -w --lint unreachable=off p.bas` is run on a program with code after a `GOTO`
- **THEN** no `unreachable` warning is printed

#### Scenario: An unknown code
- **WHEN** `qb64rust --lint unused-labl=off p.bas` is run
- **THEN** the exit status is 1 and the message names the nine codes

### Requirement: A bad lints table
An unknown code in `[lints]` or a value that is not `true` or `false` SHALL be an error that names the file, the
line and, for a code, the known codes. The program SHALL NOT be built.

#### Scenario: A misspelled code
- **WHEN** `qb64rust.toml` holds `[lints]` and `unused-labl = false`
- **THEN** the compile fails with an error naming `qb64rust.toml`, its line 2 and the nine codes, and exit status 1

### Requirement: The warning report counts every lint
`qb64rust --report warnings` SHALL count the warnings of every lint, whatever `[lints]` says: it measures how
often each lint fires.

#### Scenario: A lint turned off
- **WHEN** `qb64rust.toml` beside `p.bas` turns `unused-label` off and `p.bas` has an unused label
- **THEN** `qb64rust --report warnings p.bas` counts one `unused-label` warning
