# compiler/lints Specification

## Purpose
Lint warnings point out code that compiles but is probably a mistake: a misspelled name, a routine or label
nothing uses, a local variable that hides a shared one, and code that can never run.

## Requirements

### Requirement: When lints run
The lints SHALL run after `sema` has checked the whole program, and only when it found no errors. Each lint
warning SHALL be an ordinary warning: printed only with `-w`, published by the language server, never counted in
the error summary, and never changing the exit status or the executable. A lint SHALL NOT change the C++ written.

#### Scenario: A program with an error
- **WHEN** a program has an unused label on line 2 and an undefined `GOTO` target on line 5
- **THEN** `qb64rust -w` reports the error and no lint warning

#### Scenario: Hidden without -w
- **WHEN** a program has an unused label and is compiled without `-w`
- **THEN** no warning is printed and the executable is built

### Requirement: Only the file being compiled
A warning of the five lints with a code SHALL be given only when the place it points to is in the file being
compiled, not in a file that file includes with `$INCLUDE`. The place of each lint is named in its own
requirement. QB64pe's warnings SHALL be given wherever QB64pe gives them, included files too.

#### Scenario: A toolbox include
- **WHEN** `main.bas` includes `mouse.bi`, which defines five SUBs, and calls one of them
- **THEN** no `unused-procedure` warning is given

#### Scenario: An unused SUB of the main file
- **WHEN** `main.bas` includes `mouse.bi` and defines a SUB of its own that nothing calls
- **THEN** one `unused-procedure` warning is given, at that SUB's name

### Requirement: Codes
Each lint warning SHALL carry a code: `used-once`, `unused-procedure`, `unused-label`, `hides-shared` or
`unreachable` for ours, and `unused-variable`, `empty-select`, `duplicate-const` or `debug-incompatible` for
QB64pe's. No other warning SHALL carry a code.

#### Scenario: The code of a lint
- **WHEN** a program's only lint warning is for an unused label
- **THEN** that warning's code is `unused-label`

### Requirement: Name used once
A variable that no declaration made (no `DIM`, `REDIM`, `STATIC`, `SHARED`, `COMMON`, parameter or FUNCTION
result) and whose name occurs once in the whole program SHALL get the warning ``` `<name>` is used only once: is
it misspelled? ```, at that name. A `FOR` counter and a target of `INPUT`, `LINE INPUT` or `READ` SHALL NOT get
it. With `OPTION _EXPLICIT` no variable is undeclared, so the warning never occurs.

#### Scenario: A misspelled read
- **WHEN** a program sets `total = 5` on line 1 and prints `totl` on line 2
- **THEN** the warning ``` `totl` is used only once: is it misspelled? ``` is given at line 2

#### Scenario: Two uses
- **WHEN** a variable that no `DIM` made is set on line 1 and printed on line 2
- **THEN** it gets no warning

#### Scenario: A declared variable
- **WHEN** `DIM x AS INTEGER` is on line 1 and `x` is named nowhere else
- **THEN** it gets no `used-once` warning

#### Scenario: A loop counter
- **WHEN** a program has `FOR i = 1 TO 3: PRINT "*": NEXT` and `i` occurs nowhere else
- **THEN** it gets no warning

#### Scenario: A key press thrown away
- **WHEN** a program has `INPUT "Press Enter", dummy$` and `dummy$` occurs nowhere else
- **THEN** it gets no warning

#### Scenario: A DATA item skipped
- **WHEN** a program has `READ skipped` and `skipped` occurs nowhere else
- **THEN** it gets no warning

### Requirement: Unused procedure
A SUB or FUNCTION that no statement calls or names SHALL get the warning ``SUB `<name>` is never called`` or
``FUNCTION `<name>` is never called``, at the name in its header. A `DECLARE SUB` or `DECLARE FUNCTION` line
SHALL NOT count as a call. A call from within the procedure itself SHALL count. `SUB _GL`, a routine of a
`DECLARE LIBRARY` block, and a test SUB of a `_test.bas` file SHALL NOT get it.

#### Scenario: A SUB nothing calls
- **WHEN** a program defines `SUB Spare` and declares it with `DECLARE SUB Spare ()`, and no statement calls it
- **THEN** the warning ``SUB `Spare` is never called`` is given at the name in its `SUB` line

#### Scenario: Named by an event
- **WHEN** a SUB is named only by `ON TIMER(1) Tick`
- **THEN** it gets no warning

#### Scenario: A library routine
- **WHEN** a `DECLARE LIBRARY` block declares a function the program never calls
- **THEN** it gets no warning

#### Scenario: A test SUB
- **WHEN** `math_test.bas` defines `SUB TestAdd` and nothing in it calls `TestAdd`
- **THEN** it gets no warning

### Requirement: Unused label
A named label that nothing names SHALL get the warning ``label `<name>` is never used``, at the label. A label is
named by `GOTO`, `GOSUB`, `RETURN`, `ON … GOTO`, `ON … GOSUB`, `RESTORE`, `RESUME`, `ON ERROR GOTO`, `RUN` and the
event statements, and by a line number after `THEN` or `ELSE`. A line number SHALL NOT get the warning.

#### Scenario: A label nothing names
- **WHEN** line 4 of a program is `again:` and no statement names `again`
- **THEN** the warning ``label `again` is never used`` is given at line 4

#### Scenario: A DATA label
- **WHEN** a label is named only by `RESTORE`
- **THEN** it gets no warning

#### Scenario: A numbered program
- **WHEN** every line of a program has a line number and only some are jumped to
- **THEN** no `unused-label` warning is given

### Requirement: Local hiding a shared variable
A `DIM` or `REDIM` in a SUB or FUNCTION that makes a new local variable with the name and type of a main-module
`DIM SHARED` variable SHALL get the warning ``this `<name>` is a new local variable, which hides the `DIM SHARED`
one of the main module``, at the name in the `DIM` or `REDIM`.

#### Scenario: A local DIM
- **WHEN** the main module has `DIM SHARED score AS LONG` and a SUB has `DIM score AS LONG`
- **THEN** the warning is given at `score` in the SUB's `DIM`

#### Scenario: SHARED in the procedure
- **WHEN** a SUB has `SHARED score` to use a main-module variable
- **THEN** it gets no `hides-shared` warning

### Requirement: Unreachable code
A statement that follows `GOTO`, `END`, `SYSTEM`, `STOP`, `EXIT SUB`, `EXIT FUNCTION`, `EXIT DO`, `EXIT FOR`,
`EXIT WHILE`, `RETURN`, `RESUME`, `RUN` or `RUN label` in the same statement list, with no label or line number
between, SHALL get the warning `this code is never run: no label follows the <statement> on line <n>`. It SHALL be
given once, at the first such statement. A statement that runs nothing, such as `DATA` or `DECLARE`, SHALL NOT
start it.

#### Scenario: After a GOTO
- **WHEN** line 3 is `GOTO done`, lines 4 and 5 print, and line 6 is `done:`
- **THEN** one warning is given, at line 4, naming the `GOTO` on line 3

#### Scenario: Labeled code after END
- **WHEN** line 5 is `END` and line 6 is `handler:`, a target of `ON ERROR GOTO`
- **THEN** no `unreachable` warning is given

#### Scenario: Inside a single-line IF
- **WHEN** a line is `IF x THEN GOTO 10` and the next line has no label
- **THEN** no `unreachable` warning is given

#### Scenario: DATA after END
- **WHEN** `END` is followed by `DATA` lines and nothing else
- **THEN** no `unreachable` warning is given

### Requirement: Code that may run after all
A `$IF`, `$ELSEIF`, `$ELSE` or `$END IF` line between a leaving statement and the next statement SHALL end the
stretch of `unreachable` as a label does: the code after it may run on another platform or with other flags.
`RUN file$` SHALL NOT start a stretch: when the file cannot be run it raises an error, and a handler can go on
after it.

#### Scenario: After a platform's branch
- **WHEN** `$IF WIN THEN` is followed by `PRINT` and `SYSTEM`, then `$END IF` and a `PRINT` for other platforms
- **THEN** no `unreachable` warning is given

#### Scenario: After RUN with a file name
- **WHEN** `RUN "next.bas"` is followed by `PRINT "could not start"`
- **THEN** no `unreachable` warning is given

### Requirement: Names used in a skipped branch
A name that occurs as a word in a `$IF` branch the compile skips SHALL NOT get `used-once`, `unused-procedure` or
`unused-label`: that branch may use it on another platform or with other flags.

#### Scenario: A SUB called on another platform
- **WHEN** `$IF WIN THEN` calls `WinSetup`, its `$ELSE` branch calls `LinuxSetup`, and both SUBs are defined
- **THEN** neither SUB gets `unused-procedure`, on any platform

### Requirement: QB64pe's warnings
The compiler SHALL give the warnings of `qb64pe -w` that it did not give before: "Unused variable", "Empty SELECT
CASE block", "Duplicate constant definition", and "Feature incompatible with $DEBUG mode" if `qb64pe -w` gives it
outside the IDE. Each SHALL have QB64pe's header as its message and QB64pe's detail on the indented second line.
It SHALL be given for the programs and at the lines where `qb64pe -w` gives it in `verification\v68_*`.

#### Scenario: An unused variable
- **WHEN** line 1 of a program is `DIM unusedvar AS INTEGER` and the name occurs nowhere else
- **THEN** the warning on line 1 is `Unused variable`, and its second line names `unusedvar%` and `INTEGER`

#### Scenario: An empty SELECT CASE
- **WHEN** a `SELECT CASE` on line 3 is followed directly by its `END SELECT`
- **THEN** the warning is `Empty SELECT CASE block`, at the line where `qb64pe -w` gives it

#### Scenario: A repeated constant
- **WHEN** `CONST a = 5` is on line 1 and again on line 2
- **THEN** line 2 gets `Duplicate constant definition`, with the detail `a = 5` as QB64pe writes it

#### Scenario: In an included file
- **WHEN** `main.bas` includes `lib.bi`, which has an unused variable
- **THEN** the warning is given at its line in `lib.bi`, as `qb64pe -w` gives it

### Requirement: One warning per variable
A variable SHALL get at most one of `used-once` and "Unused variable". Where QB64pe's rule reports it, the
warning SHALL be "Unused variable". `used-once` SHALL be given only to the implicit variables named once that
QB64pe's rule does not report.

#### Scenario: Both rules apply
- **WHEN** an implicit variable is named once, and `qb64pe -w` reports it as unused
- **THEN** it gets "Unused variable" only
