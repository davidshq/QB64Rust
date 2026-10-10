# testing/call-site-check Specification

## Purpose
Checks, without running a program, that the new compiler calls the runtime library for a built-in as the old
compiler does: the same entry point with the same arguments, conversions and optional-argument mask.

## Requirements

### Requirement: Comparing the calls of one statement
The call-site check SHALL take a program whose last statement before `END` is the statement under test, have each
compiler write its C++ without building it, extract from each the runtime calls of that statement, normalise them
(variable names, spacing, redundant parentheses, and the spellings the design lists as free) and report whether
the two lists of calls are equal. It SHALL never compare whole files.

#### Scenario: Same call
- **WHEN** the check runs on a program ending in `KILL f$`
- **THEN** it reports the calls equal, both being one call of the same entry point with the string variable

#### Scenario: Different conversion
- **WHEN** the new compiler passes an argument without the rounding call the old compiler writes for it
- **THEN** the check fails and prints both normalised calls

#### Scenario: Statement the new compiler rejects
- **WHEN** the new compiler reports a diagnostic for the program
- **THEN** the check reports that program as not compared, with the diagnostic, and does not count it as equal

### Requirement: The check leaves nothing behind
The check SHALL write only into a scratch folder outside the repository and into the reference clone's git-ignored
build folder, and its reports SHALL hold no local path.

#### Scenario: Clean trees
- **WHEN** the check has run over its programs
- **THEN** `git status --porcelain` prints the same as before in this repository and in the reference clone

### Requirement: A recorded verdict
The trial SHALL end with a verdict recorded in `DECISIONS.md`: for the file I/O statements, how many statements
were compared, how many were equal after normalisation, and which differences needed a normalisation rule. With
the verdict "kept", the check SHALL run over its programs in tier 2; with "dropped", the tool SHALL be removed.

#### Scenario: Verdict written
- **WHEN** the change is ready to archive
- **THEN** `DECISIONS.md` has a dated row with the counts and the verdict, and this capability's main spec says
  which of the two states holds
