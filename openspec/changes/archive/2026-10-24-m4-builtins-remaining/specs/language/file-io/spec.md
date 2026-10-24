# Spec Delta

## ADDED Requirements

### Requirement: FILES
`FILES` and `FILES spec$` SHALL print the listing libqb prints: the current folder, the entries `spec$` names (a
folder, a pattern, or the current folder when left out) laid out as libqb lays them out, and the free space line,
and a `spec$` that names nothing SHALL raise the old compiler's error (measured, `verification\v58`).

#### Scenario: A pattern in a known folder
- **WHEN** a program in a folder holding known files runs `FILES "*.txt"`
- **THEN** it prints the listing the old compiler's program prints, the folder's path and the free space masked
  as the verification suite masks a folder

#### Scenario: Nothing found
- **WHEN** a program runs `FILES "nosuch.*"` under a handler that prints `ERR`
- **THEN** it prints what the old compiler's program prints

#### Scenario: A statement in a console program
- **WHEN** a `$CONSOLE:ONLY` program runs `FILES "*.txt"`
- **THEN** it prints what the old compiler's program prints
