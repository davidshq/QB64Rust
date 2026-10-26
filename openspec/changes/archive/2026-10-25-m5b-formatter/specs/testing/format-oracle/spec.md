# Spec Delta

## Purpose

The old compiler's `-y` output, recorded for real programs, and the checks that hold the new compiler's formatter
to it: equal output, the same program, idempotence, and QB64pe's own format tests.

## ADDED Requirements

### Requirement: Programs recorded
`tests\format\` SHALL hold the old compiler's `-y` output for every program that the old compiler accepts in these
sets: the golden corpus (`tests\corpus\`), the upstream copy (`tests\upstream\`), the snippets
(`tests\snippets\`) and Civil War Strategy (`tests\programs\civil-war-strategy\`). Each output SHALL be kept under
the set's name and the program's path within it. A program the old compiler rejects SHALL have no output.

#### Scenario: Corpus program
- **WHEN** `tests\corpus\slice\s01_literals.bas` is accepted by the old compiler
- **THEN** `tests\format\corpus\slice\s01_literals.bas` holds its `-y` output

#### Scenario: Rejected program
- **WHEN** a corpus program has an `.err` recording
- **THEN** `tests\format\` holds nothing for it

### Requirement: Recording with the old compiler
A suite of `run_legacy_tests.py` run with `qb64pe` SHALL record each output with `-y` and QB64pe's default layout
settings given explicitly as `-f:` settings, so that QB64pe's `config.ini` plays no part. It SHALL run in the
program's own folder so included files resolve, and leave an output alone when it differs only by CRLF against LF.

#### Scenario: Recording unchanged
- **WHEN** outputs that are current are recorded again with `qb64pe`
- **THEN** no file of `tests\format\` changes

### Requirement: Comparing another compiler
The same suite run with any other compiler SHALL compare and SHALL NOT write into `tests\format\`. A program SHALL
pass when the compiler exits with 0 and its output equals the recording with line endings taken as LF, or when
the program takes a `DIVERGENCES.md` row that a sidecar names, as the other suites allow.

#### Scenario: Output differs
- **WHEN** `qb64rust -y` writes `Print  x` where `Print x` is recorded
- **THEN** the program fails, and the result shows the first line that differs

### Requirement: Tier 1 checks
`cargo test` SHALL format every recorded program in process, with the default settings, and SHALL fail when an
output differs from its recording. For each program it SHALL also check that the formatted text gives the same
typed tree as the input, and that formatting the output again changes nothing. A program known to differ SHALL be
named with its reason in a shrink-only list.

#### Scenario: A rule regresses
- **WHEN** a change to the formatter alters the spacing after `Then` in one program
- **THEN** `cargo test` fails and names that program and line

#### Scenario: Shrink-only list
- **WHEN** a program on the list of known differences comes to equal its recording
- **THEN** `cargo test` fails until the program is taken off the list

### Requirement: QB64pe's format tests
`run_legacy_tests.py --suite format` run with `qb64rust` SHALL pass all of QB64pe's `format_tests` variants, every
combination of settings that their `.flagmap` files name.

#### Scenario: All variants
- **WHEN** `--suite format --qb64 target\release\qb64rust.exe` runs
- **THEN** every variant passes, as with the old compiler (24 of 24)
