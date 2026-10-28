# Spec Delta

## MODIFIED Requirements

### Requirement: Named fixes
Each fix SHALL have a fixed kebab-case name. The names in this release SHALL be `resume-after-block`,
`dim-empty-error`, `static-local-dynamic`, `whole-array-value`, `resume-in-procedure`, `run-in-procedure`,
`checking-on-service` and `bit-number-check`. A fix SHALL change nothing while it is off: the program SHALL compile
and run as with no switch at all.

#### Scenario: Off is QB64pe's
- **WHEN** a program of the verification suite is built with every fix off
- **THEN** its output is its `qb64pe.exe` recording, as before this change

### Requirement: Listing the fixes
`qb64rust --list-fixes` SHALL print one line per fix, its name and what it changes, and exit with status 0.

#### Scenario: The list
- **WHEN** `qb64rust --list-fixes` is run
- **THEN** it prints eight lines, the first beginning `resume-after-block` and the last `bit-number-check`, and
  exits with status 0
