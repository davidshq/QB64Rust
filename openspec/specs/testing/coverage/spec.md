# testing/coverage Specification

## Purpose
How much of a program its test runs execute, line by line, measured with the new compiler's coverage builds, and a
check that every line with a statement either ran in full or is listed with the reason it cannot run.

## Requirements

### Requirement: Coverage of a set of runs
`tools\coverage\coverage.py` SHALL merge the counts files of any number of runs of one program. For each line of
a source file it SHALL report one of: run (every statement that starts on the line ran at least once), partly run
(some did), not run (none did), no statement. It SHALL print the totals and the lines not run in full, with the
statements of a partly run line that did not run. It SHALL read the records of both versions of the counts
format, the first and the one that names each statement's built-ins.

#### Scenario: A line with two statements
- **WHEN** `IF a THEN b = 1` stands on line 10 and the runs took the IF but never its THEN
- **THEN** line 10 is reported as partly run, naming the assignment as the statement that did not run

#### Scenario: Two runs merged
- **WHEN** one run covers line 10 and another covers line 11
- **THEN** the merged report has both lines as run

#### Scenario: Counts of another program
- **WHEN** a counts file names a source whose text differs from the one the report reads
- **THEN** the tool stops with a message naming the file, and reports nothing

#### Scenario: Counts of both versions
- **WHEN** one counts file holds a record of the first version and another a record that names built-ins
- **THEN** the merged report is the one the two records give, line by line

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

### Requirement: Built-ins named in the counts
A coverage build SHALL name, for each counted statement, every built-in function and statement that the
statement calls, as written with its suffix, in the record it writes at exit. A statement that calls no built-in
SHALL name none. A built-in in an argument of another SHALL be named for the statement that holds both.

#### Scenario: Two built-ins in one statement
- **WHEN** `PRINT LEFT$(a$, LEN(b$))` is built with `--coverage` and run
- **THEN** the record's entry for that statement names `LEFT$` and `LEN`, with its count

#### Scenario: A statement that is never reached
- **WHEN** `_TITLE "x"` stands after `SYSTEM`
- **THEN** its entry names `_TITLE` with the count 0

### Requirement: Built-ins reached by the test runs
`tools\coverage\builtin_runs.py` SHALL merge the counts of runs of any programs of the test suites and report each
built-in that the records name as run (a statement naming it ran at least once) or not run, with the programs
that hold it. The test runner SHALL take `--coverage` for the corpus, verification and upstream suites.

#### Scenario: Run in one program, unreached in another
- **WHEN** `LEN` stands after `SYSTEM` in one program and runs in another
- **THEN** the report gives `LEN` as run

#### Scenario: Only in unreached code
- **WHEN** every statement that names `_TITLE` has the count 0
- **THEN** the report gives `_TITLE` as not run and names the programs that hold it

### Requirement: The list of built-ins no test runs
`tests\never_run.list` SHALL hold each built-in that no test runs, one per line as `<name>\t<reason>[ <note>]`,
with a reason from a fixed set: the desktop, the clipboard or a console the program shares; a printer; the
machine's keyboard state or bell; a person must answer; another compiler. `#` SHALL start a comment line.

#### Scenario: A reason outside the set
- **WHEN** a line of the list gives a reason that is not one of the set
- **THEN** the check fails, naming the line of the list

### Requirement: The check of the built-ins reached
`builtin_runs.py --check` SHALL fail when a built-in named in the counts is not run and not in
`tests\never_run.list`, when a listed built-in ran, and when a listed built-in, or any built-in `sema` compiles,
is named in no record. It SHALL print what it found either way. It SHALL run in the full set at a merge into
`main`.

#### Scenario: A built-in no record names
- **WHEN** a built-in `sema` compiles stands only where no statement is counted (a `CONST`)
- **THEN** the check fails, naming the built-in as named in no record

#### Scenario: A built-in reached only in unreached code
- **WHEN** a built-in's only uses in the suites stand after `SYSTEM` and it is not in the list
- **THEN** the check fails, naming the built-in and the programs that hold it

#### Scenario: A listed built-in that ran
- **WHEN** a new program runs `_TITLE` and `_TITLE` is in the list
- **THEN** the check fails, naming `_TITLE` as listed but run, until its line is taken out of the list
