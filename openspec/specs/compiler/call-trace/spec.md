# compiler/call-trace Specification

## Purpose

A call-trace build adds a call stack to the message of a runtime error: the procedure the error is in, each
caller with the line of its call, and each pending `GOSUB`. It tells which path through the program led to the
failing line.

## Requirements

### Requirement: The call stack of a runtime error
A call-trace build SHALL add a block that starts with the line `Call stack:` to the message of an untrapped
runtime error. The block SHALL follow the error's text and come before `Continue?`. The lines before it SHALL be
those of a build without the call trace. Each row SHALL be indented by two spaces, the innermost first.

#### Scenario: An error in a SUB
- **WHEN** `main.bas` calls `outer` on line 3, `outer` calls `inner` on line 8, and line 12 in `inner` runs
  `ERROR 5`, with `QB64PE_NOPROMPT=y`
- **THEN** stderr holds `Runtime error: Line: 12 (in main module)`, `Illegal function call`, `Call stack:`,
  and the rows `SUB inner (main.bas:12)`, `SUB outer (main.bas:8)`, `Main module (main.bas:3)`

#### Scenario: An error in the main module
- **WHEN** line 4 of the main module runs `ERROR 5` with no procedure running
- **THEN** the block has the one row `Main module (main.bas:4)`

#### Scenario: The dialog
- **WHEN** the same program runs without `QB64PE_NOPROMPT`
- **THEN** the dialog's text holds the same block, and `Continue?` after it

### Requirement: What a row names
A row SHALL name its procedure as the debugger's call stack does: `Main module`, `SUB <name>` or
`FUNCTION <name>`, the name as declared. It SHALL give the file's path from the main file's folder and the line
in that file. The first row's line is the error's line. Every other row's line is the line of the call that the
row above it came from.

#### Scenario: A FUNCTION inside an expression
- **WHEN** line 5 holds `x = twice&(1) + half&(2)` and `half&` raises an error on line 20
- **THEN** the rows are `FUNCTION half& (main.bas:20)` and `Main module (main.bas:5)`; `twice&`, which has
  returned, is not listed

#### Scenario: A procedure of an included file
- **WHEN** `SUB flag_x` is in `rules/army.bm`, raises an error on line 212 of that file, and was called from
  line 88 of `rules/events.bm`
- **THEN** the rows start with `SUB flag_x (rules/army.bm:212)` and `SUB resolve (rules/events.bm:88)`

#### Scenario: A recursive procedure
- **WHEN** `SUB down` calls itself on line 9 three times and then raises an error on line 7
- **THEN** the block has `SUB down (main.bas:7)`, then three rows `SUB down (main.bas:9)`, then the main
  module's row

### Requirement: Pending GOSUBs
A `GOSUB` that has not returned SHALL have a row of its own: the procedure it is in, the word `GOSUB`, and the
file and line of the `GOSUB` statement. The row SHALL come after the row of the code that the `GOSUB` jumped to,
with the newest `GOSUB` first. A `GOSUB` that has returned is not listed.

#### Scenario: An error inside a subroutine
- **WHEN** line 3 of the main module runs `GOSUB work`, and line 10, under the label `work`, runs `ERROR 5`
- **THEN** the rows are `Main module (main.bas:10)` and `Main module, GOSUB (main.bas:3)`

#### Scenario: A call from a subroutine
- **WHEN** line 3 runs `GOSUB work`, line 10 under `work` calls `SUB fail`, and `fail` raises an error on line 20
- **THEN** the rows are `SUB fail (main.bas:20)`, `Main module (main.bas:10)` and
  `Main module, GOSUB (main.bas:3)`

#### Scenario: A GOSUB inside a procedure
- **WHEN** `SUB a` runs a `GOSUB` on line 12, and the subroutine raises an error on line 16
- **THEN** the rows are `SUB a (main.bas:16)`, `SUB a, GOSUB (main.bas:12)`, then the row of `a`'s caller

### Requirement: Code run by an event or an error handler
A SUB or a label that `ON TIMER`, `ON KEY` or `ON STRIG` runs SHALL be shown as started by an event: the row
below it SHALL read `event` in place of a file and line, since the interrupted code's line is not known. An
error raised inside an `ON ERROR` handler SHALL list the handler first, as `Error handler`, and SHALL give no
line for the procedure whose error the handler was handling.

#### Scenario: A SUB run by a timer
- **WHEN** `ON TIMER(1) tick` runs `SUB tick` while the main module loops, and `tick` raises an error on line 30
- **THEN** the rows are `SUB tick (main.bas:30)` and `Main module (event)`

#### Scenario: An error in an error handler
- **WHEN** `SUB a` raises an error that `ON ERROR GOTO fix` handles, and line 40 under `fix` raises another
- **THEN** the rows start with `Error handler (main.bas:40)` and `SUB a`, and `a`'s callers follow with their
  lines

### Requirement: Long call stacks
The block SHALL show at most 24 rows. A longer call stack SHALL show its 16 innermost rows, a row that says how
many were left out, and its 7 outermost rows. A call-trace build SHALL keep every frame whatever the depth, and
SHALL still report error 256 (out of stack space) with its call stack.

#### Scenario: Deep recursion
- **WHEN** `SUB down` is 1,000 calls deep and raises an error, so that the call stack has 1,001 rows with the
  main module's
- **THEN** the block has 16 rows, then `... 978 more ...`, then 7 rows, the last the main module's

#### Scenario: Out of stack space
- **WHEN** a `SUB` calls itself without end
- **THEN** the report of critical error 256 holds a `Call stack:` block of 24 rows

### Requirement: A handled error and a returned call leave nothing behind
The call stack SHALL hold only what is running when the error is reported. A procedure left by `EXIT SUB`,
`EXIT FUNCTION`, its end, or a `RUN` SHALL NOT be listed. An error that a handler handles SHALL print nothing.

#### Scenario: After a handled error
- **WHEN** `SUB a` raises an error, the handler runs `RESUME NEXT`, `a` returns, and `SUB b` later raises an
  untrapped error
- **THEN** the block lists `SUB b` and its callers, and not `SUB a`

#### Scenario: After RUN
- **WHEN** `SUB a` runs `RUN`, and the restarted main module raises an error on line 2
- **THEN** the block has the one row `Main module (main.bas:2)`

### Requirement: The program is otherwise the same
A call-trace build SHALL behave as the build without it, apart from the block: the same output, files, screen,
errors, `ERR`, `ERL`, `_ERRORLINE` and exit code. A procedure that runs on another thread (`SUB _GL`) SHALL NOT
disturb the call stack of the main program.

#### Scenario: A program with no error
- **WHEN** a slice program that ends without an error is built with the call trace and run
- **THEN** its output equals its recorded `.output`

#### Scenario: A SUB _GL beside the program
- **WHEN** a program with a `SUB _GL` that calls other procedures raises an error in its main module's code
- **THEN** the block lists the main program's frames only
