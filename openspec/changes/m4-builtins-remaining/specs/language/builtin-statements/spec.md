# Spec Delta

## ADDED Requirements

### Requirement: DATE$ and TIME$ as statements
`DATE$ = text$` and `TIME$ = text$` SHALL compile with the old compiler's libqb calls, `sub_date` and
`sub_time`, which change nothing in libqb: the system's date and time stay as they were, and `DATE$` and `TIME$`
read them. A value that is not a string SHALL be the old compiler's error (measured, `verification\v58`).

#### Scenario: Set and read back
- **WHEN** a program runs `d$ = DATE$: DATE$ = "01-02-2003": PRINT DATE$ = d$` (and the same for `TIME$` with the
  seconds dropped)
- **THEN** it prints what the old compiler's program prints

#### Scenario: A number
- **WHEN** a program has `DATE$ = 5`
- **THEN** the compile fails with an error whose note is the old compiler's message
