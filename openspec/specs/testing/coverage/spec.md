# testing/coverage Specification

## Purpose
How much of a program its test runs execute, line by line, measured with the new compiler's coverage builds, and a
check that every line with a statement either ran in full or is listed with the reason it cannot run.

## Requirements

### Requirement: Coverage of a set of runs
`tools\coverage\coverage.py` SHALL merge the counts files of any number of runs of one program. For each line of
a source file it SHALL report one of: run (every statement that starts on the line ran at least once), partly run
(some did), not run (none did), no statement. It SHALL print the totals and the lines not run in full, with the
statements of a partly run line that did not run.

#### Scenario: A line with two statements
- **WHEN** `IF a THEN b = 1` stands on line 10 and the runs took the IF but never its THEN
- **THEN** line 10 is reported as partly run, naming the assignment as the statement that did not run

#### Scenario: Two runs merged
- **WHEN** one run covers line 10 and another covers line 11
- **THEN** the merged report has both lines as run

#### Scenario: Counts of another program
- **WHEN** a counts file names a source whose text differs from the one the report reads
- **THEN** the tool stops with a message naming the file, and reports nothing

### Requirement: The list of unreached lines
A program held to its coverage SHALL have a list of the lines its scripts do not run, one per line, as
`<line>\t<reason>` (for a partly run line, the column of each statement that does not run, after the line
number). The reason SHALL be one of a fixed set: removed by an edit of the copy, a file the program does not
ship, unreachable code, input the screen-state check cannot give, a bug of the program that ends it, or a state of
the program that no script creates (with a note naming that state).

#### Scenario: A reason outside the set
- **WHEN** a line of the list gives a reason that is not one of the set
- **THEN** the check fails, naming the line of the list

### Requirement: The coverage check
`coverage.py --check` SHALL build the program with `--coverage`, run each of its scripts as the corpus runner
runs them, merge the counts, and fail when a line with a statement is not run in full and not in the list. It
SHALL also fail when the list names a line that ran in full, so the list only shrinks. It prints what it found
either way.

#### Scenario: A new script covers a listed line
- **WHEN** a script added to the game runs a line the list gives as unreachable code
- **THEN** the check fails, naming that line as listed but run, until the line is taken out of the list

#### Scenario: A line no script reaches
- **WHEN** an edit of the game's copy adds a statement no script runs
- **THEN** the check fails, naming the line
