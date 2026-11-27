# Spec Delta

## ADDED Requirements

### Requirement: UTF-8 text in a file that declares nothing
A file SHALL get one `undeclared-utf8` warning when all of these hold:

- the file has no `'$ENCODING` line, and no `encoding` key of a settings file covers it (spec
  `compiler/source-encoding`, "A file's encoding");
- the file's bytes are valid UTF-8;
- a string literal or a `DATA` item of the file holds a byte above 127.

The warning SHALL be at the first such string or item. It SHALL say that the file looks like UTF-8 and is read
as code page 437, give the number of bytes the first such character becomes in the program, and name both ways
to declare the file. It SHALL be given for an included file too.

#### Scenario: A new file from an editor
- **WHEN** `p.bas` is saved as UTF-8, declares nothing, and line 2 is `PRINT "─"`
- **THEN** `qb64rust -w p.bas` gives one `undeclared-utf8` warning at line 2, column 7, which says that `─` is
  3 characters in the program and names `'$ENCODING:UTF8` and `encoding` in `qb64rust.toml`

#### Scenario: Only in a comment
- **WHEN** a file that declares nothing holds `' café` in UTF-8 and no byte above 127 in any string
- **THEN** no `undeclared-utf8` warning is given

#### Scenario: A code page 437 file
- **WHEN** a file that declares nothing holds the single byte 0xC4 in a string
- **THEN** no `undeclared-utf8` warning is given, since the file is not valid UTF-8

#### Scenario: Declared code page 437
- **WHEN** a file holds `'$ENCODING:CP437` and UTF-8 bytes in a string, for `_UPRINTSTRING` with encoding 8
- **THEN** no `undeclared-utf8` warning is given

## MODIFIED Requirements

### Requirement: Only the file being compiled
A warning of `used-once`, `unused-procedure`, `unused-label`, `hides-shared` or `unreachable` SHALL be given
only when the place it points to is in the file being compiled, not in a file that file includes with
`$INCLUDE`. The place of each lint is named in its own requirement. QB64pe's warnings SHALL be given wherever
QB64pe gives them, included files too. `undeclared-utf8` SHALL be given for any file of the program.

#### Scenario: A toolbox include
- **WHEN** `main.bas` includes `mouse.bi`, which defines five SUBs, and calls one of them
- **THEN** no `unused-procedure` warning is given

#### Scenario: An unused SUB of the main file
- **WHEN** `main.bas` includes `mouse.bi` and defines a SUB of its own that nothing calls
- **THEN** one `unused-procedure` warning is given, at that SUB's name

### Requirement: Codes
Each lint warning SHALL carry a code: `used-once`, `unused-procedure`, `unused-label`, `hides-shared`,
`unreachable` or `undeclared-utf8` for ours, and `unused-variable`, `empty-select`, `duplicate-const` or
`debug-incompatible` for QB64pe's. No other warning SHALL carry a code.

#### Scenario: The code of a lint
- **WHEN** a program's only lint warning is for an unused label
- **THEN** that warning's code is `unused-label`

### Requirement: Lints table of the settings file
The `[lints]` table of the program's `qb64rust.toml` (the file of spec `compiler/fixes`) SHALL hold `all =
true|false` and `<code> = true|false` for each of the ten codes of "Codes". A single code's setting SHALL
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
- **THEN** the exit status is 1 and the message names the ten codes

### Requirement: A bad lints table
An unknown code in `[lints]` or a value that is not `true` or `false` SHALL be an error that names the file, the
line and, for a code, the known codes. The program SHALL NOT be built.

#### Scenario: A misspelled code
- **WHEN** `qb64rust.toml` holds `[lints]` and `unused-labl = false`
- **THEN** the compile fails with an error naming `qb64rust.toml`, its line 2 and the ten codes, and exit status 1
