# Spec Delta

## ADDED Requirements

### Requirement: Demand report
`qb64rust --report unsupported <file or folder>... [--include-root <dir>]` SHALL run the front end over every
program given (a file as given; of a folder every `.bas` file below it, in any letter case) and print what the set
still needs: a summary line with the number of programs, of those without errors, of those with only "not
supported yet" errors and of those with other errors; then one line per distinct "not supported yet" message with
the number of programs it occurs in, the number of programs in which it is the only message and no other error
occurs, and the number of times it occurs, ordered by those three numbers, largest first. The error cap of a
compile run SHALL NOT apply. A program with an `.err` file beside it SHALL be left out and counted in a line of its
own. A program with an error that is not marked SHALL be named after the table. Nothing SHALL be built. The exit
status SHALL be 0 when every input was read, whatever errors the programs hold, and 1 for an input that is neither
a file nor a folder or for any other word after `--report`. Without `--report`, more than one input file SHALL
stay an error. A program the inputs reach more than once, under whatever spelling of its path, SHALL be counted
once.

#### Scenario: A folder of programs
- **WHEN** a folder holds a program without errors, a program with one `LPRINT`, a program in a subfolder with 105
  of them, and a program with an `.err` file beside it, and `LPRINT` is not compiled yet
- **THEN** the report says 3 programs, 1 without errors, 2 with only "not supported yet" errors, 1 left out, and
  one row for `LPRINT` with 2 programs, 2 alone and 106 uses

#### Scenario: A program with another error
- **WHEN** the one program given has an error that is not marked
- **THEN** the summary counts it under "other errors", it is named after the summary, and the exit status is 0

#### Scenario: Unknown report
- **WHEN** `qb64rust --report typed p.bas` is run
- **THEN** the exit status is 1 with the message that the report is unknown
