# Spec Delta

## ADDED Requirements

### Requirement: $EXEICON
A line `$EXEICON:'name'` SHALL give the program's executable the icon in the file `name`, and SHALL be, where it
stands, the statement `_ICON` with no argument, as in the old compiler. The name stands between the first two
single quotes of the line; what follows the second SHALL be dropped. A name that starts with `./` or `.\` SHALL
be looked for in the folder of the main source file. A line in a `$IF` branch that is not taken SHALL NOT count.

#### Scenario: The game's line
- **WHEN** a program with `$EXEICON: './cws.ico'` is compiled and `cws.ico` lies beside it
- **THEN** there is no diagnostic, the line compiles to the call the old compiler writes for it, and the
  executable carries the icon

#### Scenario: Reached in a console program
- **WHEN** a program with `$CONSOLE:ONLY` runs `PRINT "before"`, the line `$EXEICON:'./icon.ico'` and
  `PRINT "after"`
- **THEN** it prints both lines and ends, as with the old compiler

#### Scenario: What follows the name
- **WHEN** a program has `$EXEICON:'./icon.ico' and more`, `$EXEICON:'./icon.ico' ' the icon` or
  `$EXEICON:'./icon.ico': PRINT "same line"`
- **THEN** each compiles as `$EXEICON:'./icon.ico'` does, and the `PRINT` of the third is not part of the
  program, as with the old compiler

#### Scenario: Case, blanks and backslashes
- **WHEN** a program has `   $exeicon:'  ./icon.ico  '` or a name written `.\icon.ico`
- **THEN** each compiles as `$EXEICON:'./icon.ico'` does

#### Scenario: Inside a SUB and after the last statement
- **WHEN** the line stands inside a SUB that is never called, or after `SYSTEM` as the last line
- **THEN** the program compiles and its executable carries the icon

#### Scenario: In a comment
- **WHEN** a program has `'$EXEICON:'./missing.ico'` or `REM $EXEICON:'./missing.ico'`
- **THEN** the line is a comment: there is no diagnostic, no statement and no icon, as with the old compiler

#### Scenario: With an _ICON statement
- **WHEN** a program has `$EXEICON:'./icon.ico'` and a line `_ICON`
- **THEN** it compiles, each line to the call the old compiler writes for it

#### Scenario: A branch that is not taken
- **WHEN** `$EXEICON:'./missing.ico'` stands between `$IF 0 THEN` and `$END IF`
- **THEN** there is no diagnostic and the executable carries no icon of the program's

### Requirement: Errors of $EXEICON
The compiler SHALL report an error that is not "not supported yet" for a `$EXEICON` line without a name between
single quotes, for a second `$EXEICON` line of a program (whatever it names), and for a name whose file is not
there, each at the line, as the old compiler does. A relative name that does not start with `./` or `.\` SHALL be
reported "not supported yet". The line rules of every metacommand SHALL hold for it.

#### Scenario: No name
- **WHEN** a program has `$EXEICON`, `$EXEICON:''`, `$EXEICON:"./icon.ico"` or `$EXEICON:'./icon.ico`
- **THEN** the compiler reports that it expected `$EXEICON:'filename'`

#### Scenario: Twice
- **WHEN** a program has `$EXEICON:'./icon.ico'` on line 4 and a `$EXEICON` line further down
- **THEN** the compiler reports at the second line that `$EXEICON` is already defined in line 4

#### Scenario: A missing file
- **WHEN** a program has `$EXEICON:'./missing.ico'` and no such file lies beside it
- **THEN** the compiler reports that the file is not found, naming it as it is written in the program

#### Scenario: A relative name without ./
- **WHEN** a program has `$EXEICON:'icons/icon.ico'` or `$EXEICON:'/icons/icon.ico'`
- **THEN** the compiler reports "not supported yet" at the line (the old compiler looks in its own folder, or on
  its own drive)

#### Scenario: After a colon, and a tab before it
- **WHEN** a program has `l = 1: $EXEICON:'./icon.ico'` or a tab before `$EXEICON`
- **THEN** the compiler reports an error, as for any metacommand
