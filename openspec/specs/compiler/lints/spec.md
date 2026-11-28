# compiler/lints Specification

## Purpose
Lint warnings point out code that compiles but is probably a mistake: a misspelled name, a routine or label
nothing uses, a local variable that hides a shared one or is never set, a SINGLE that holds a fraction it cannot
hold exactly, and code that can never run.

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
A warning of `used-once`, `unused-procedure`, `unused-label`, `hides-shared` or `unreachable` SHALL be given
only when the place it points to is in the file being compiled, not in a file that file includes with
`$INCLUDE`. The place of each lint is named in its own requirement. QB64pe's warnings SHALL be given wherever
QB64pe gives them, included files too. `undeclared-utf8`, `pds-function`, `never-set` and `single-fraction`
SHALL be given for any file of the program.

#### Scenario: A toolbox include
- **WHEN** `main.bas` includes `mouse.bi`, which defines five SUBs, and calls one of them
- **THEN** no `unused-procedure` warning is given

#### Scenario: An unused SUB of the main file
- **WHEN** `main.bas` includes `mouse.bi` and defines a SUB of its own that nothing calls
- **THEN** one `unused-procedure` warning is given, at that SUB's name

### Requirement: Codes
Each lint warning SHALL carry a code: `used-once`, `unused-procedure`, `unused-label`, `hides-shared`,
`unreachable`, `undeclared-utf8`, `pds-function`, `never-set` or `single-fraction` for ours, and `unused-variable`, `empty-select`, `duplicate-const` or
`debug-incompatible` for QB64pe's. No other warning SHALL carry a code.

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

### Requirement: A BASIC PDS 7.1 function read as a variable
BASIC PDS 7.1 has functions that a program can call with no argument: `CURDIR$`, `DIR$`, `STACK` and
`SAVEPOINT`. QB64pe has none of them, so it builds each bare name as a variable that is never set, with no
message. Each name of such a variable SHALL get the warning ``` `<name>` is a BASIC PDS 7.1 function that QB64
does not have: here it is a variable that is never set, so it is always <value> ```, where `<value>` is `""` when
the variable is a string and `0` when it is a number, by its suffix or its `DEF` type. It SHALL be given when
the name is written as one of the four, in any letter case, and the variable is a scalar that no declaration
made and that no statement sets. A statement sets a variable when it assigns it, when it copies a whole record
into it, when it is a target of `INPUT`, `LINE INPUT`, `READ`, `FOR`, `LSET`, `RSET`, `FIELD` or `_MEMGET`, when
a built-in statement stores into it or exchanges it (`SWAP`, `MID$`, `GET` of a file, not `PUT`), or when it is
passed by reference to a SUB or FUNCTION that sets that parameter, or to a `DECLARE LIBRARY` routine. A
parameter passed on SHALL be followed. A variable whose memory the program takes, with `VARPTR`, `VARSEG`,
`SADD`, `VARPTR$`, `_MEM`, `_MEMELEMENT`, `_OFFSET` or `CALL ABSOLUTE`, SHALL count as set. A variable that gets
this warning SHALL NOT get `used-once`.

#### Scenario: A port's current folder
- **WHEN** line 1 of a program is `here$ = CURDIR$` and line 2 is `PRINT curdir$`
- **THEN** both lines get the warning, naming `CURDIR$` and `""`

#### Scenario: A variable of the program's own
- **WHEN** a program sets `stack = 3` and prints `stack`
- **THEN** it gets no `pds-function` warning

#### Scenario: Set by a SUB
- **WHEN** a program calls `SetIt DIR$`, whose parameter `s$` the SUB assigns, and prints `DIR$`
- **THEN** it gets no `pds-function` warning

#### Scenario: Read by a SUB
- **WHEN** a program calls `ShowText CURDIR$`, and `ShowText` only prints its parameter
- **THEN** that `CURDIR$` gets the warning

#### Scenario: Written by PUT
- **WHEN** a program has `PUT #1, , STACK` and then prints `STACK`
- **THEN** both names get the warning

#### Scenario: A string by DEFSTR
- **WHEN** a program has `DEFSTR S` and prints `STACK`
- **THEN** the warning ends with `so it is always ""`

#### Scenario: Set once
- **WHEN** a SUB's only statement is `DIR$ = "once"`
- **THEN** it gets `used-once` and no `pds-function` warning

#### Scenario: Another name
- **WHEN** a program prints `curdir`, with no `$`
- **THEN** it gets no `pds-function` warning

### Requirement: A local variable that is never set
A scalar variable of a SUB or FUNCTION, made by `DIM`, `STATIC` or its first use, that is read but that no
statement sets, by the rule of "A BASIC PDS 7.1 function read as a variable", SHALL get the warning ``` `<name>`
is never set, so it is always <value> ```, once, at its first read. `<value>` is `""` for a string and `0` for a
number. When the main module has a `DIM SHARED` array of the same name, the warning SHALL go on with ``` : is
the shared array `<name>()` meant? ```. It SHALL NOT be given for a parameter, a main-module variable, an
array, a fixed-length string, a variable of a user type, a variable that gets `pds-function`, or a name that
stands in a skipped `$IF` branch. A variable made by its first use and named only once gets `used-once`
instead, and a declared one never named again gets "Unused variable".

#### Scenario: A local where the shared array was meant
- **WHEN** the main module has `DIM SHARED cash(1) AS INTEGER`, and a SUB has `DIM cash AS INTEGER` and
  `IF cash < 222 THEN …`
- **THEN** that `cash` gets ``` `cash` is never set, so it is always 0: is the shared array `cash()` meant? ```

#### Scenario: Set by a FUNCTION
- **WHEN** a SUB has `DIM n AS INTEGER` and `PRINT Bump(n); n`, and the FUNCTION `Bump` assigns its parameter
- **THEN** it gets no `never-set` warning

#### Scenario: A blank record
- **WHEN** a SUB has `DIM blank AS Rec` and `PRINT blank.a`
- **THEN** it gets no `never-set` warning

#### Scenario: Read twice, never declared
- **WHEN** a SUB's only statements are `PRINT total` and `PRINT total + 1`
- **THEN** the first `total` gets the warning, and it gets no `used-once`

### Requirement: A SINGLE that holds a fraction
A number with a fraction written in an expression is a DOUBLE, but the same number held in a SINGLE variable is
not, so a product rounded to an integer can differ: `r% = .3 * 95` stores 28, and `s! = .3: r% = s! * 95`
stores 29. A scalar SINGLE variable that an assignment sets to a number with a fraction that a SINGLE cannot hold
exactly, written out or as a `CONST`, SHALL get the warning ``` `<name>` is a SINGLE set to <number> on line
<n>, which a SINGLE cannot hold exactly: this product, rounded to an integer, can differ from the same product
with <number> written out, which is a DOUBLE; a DOUBLE variable gives the same result as the number ``` at each
place where it is a factor of a product whose value an assignment rounds to an integer. `<number>` is the first
such number assigned to the variable, as written; a `CONST`'s name is followed by its value, `KNOB (0.3)`. `line
<n>` names the file too when the assignment is in another file. A variable that an assignment computes with an
operator (`p! = p! * 2`), or that a `FOR` counts, SHALL NOT get it: its value is no number written out that it
stands for. A literal, a function's value such as `VAL(v$)`, and a copy of another variable are not computed.
Only the variable's own assignments SHALL be looked at: a SUB that computes the parameter it is passed does not
count, so `g! = .3: Scale g!: r% = g! * 95` gets the warning. Only a product under `+`, `-`, unary minus, `_IIF`
and conversions SHALL count, not one inside a function's argument.

#### Scenario: A fraction as a tuning knob
- **WHEN** a program has `rate! = .3` and then `r% = rate! * 95`
- **THEN** that `rate!` gets the warning, naming `.3`

#### Scenario: A fraction a SINGLE holds
- **WHEN** a program has `half! = .5` and then `r% = half! * 95`
- **THEN** it gets no `single-fraction` warning

#### Scenario: A DOUBLE variable
- **WHEN** a program has `d# = .3` and then `r% = d# * 95`
- **THEN** it gets no `single-fraction` warning

#### Scenario: Stored in a SINGLE
- **WHEN** a program has `rate! = .3` and then `f! = rate! * 95`
- **THEN** it gets no `single-fraction` warning

#### Scenario: A computed SINGLE
- **WHEN** a SUB has `p! = .1`, `p! = p! * 2 + .05` and `n% = p! * 30`
- **THEN** it gets no `single-fraction` warning

#### Scenario: Set from a CONST
- **WHEN** a SUB has `CONST KNOB = .3`, `q! = KNOB` and `n% = q! * 30`
- **THEN** that `q!` gets the warning, naming `KNOB (0.3)`

#### Scenario: In an included file
- **WHEN** a program sets the shared SINGLE `rate` to `.3`, and a SUB in a file it includes has `n% = rate * 95`
- **THEN** that `rate` gets the warning, at its place in the included file

### Requirement: Lints table of the settings file
The `[lints]` table of the program's `qb64rust.toml` (the file of spec `compiler/fixes`) SHALL hold `all =
true|false` and `<code> = true|false` for each of the thirteen codes of "Codes". A single code's setting SHALL
override `all`. A lint that is off SHALL give no warning: not with `-w`, and not in the language server. With no
table, every lint SHALL be on. A warning without a code SHALL NOT be affected.

#### Scenario: One lint off
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[lints]` and `unused-label = false`, and `p.bas` has an unused
  label and an unused SUB
- **THEN** `qb64rust -w p.bas` prints the `unused-procedure` warning and no `unused-label` warning

#### Scenario: All off but one
- **WHEN** the table holds `all = false` and `used-once = true`
- **THEN** only `used-once` warnings are given

#### Scenario: No table
- **WHEN** `qb64rust.toml` holds only a `[fixes]` table
- **THEN** the warnings are those of a program with no `qb64rust.toml`

#### Scenario: A warning without a code
- **WHEN** the table holds `all = false` and line 2 of the program is `CONST c = 2 ^ 3 ^ 2`
- **THEN** `qb64rust -w` still prints that line's warning

### Requirement: Command-line lints
`--lints on|off` SHALL set every lint and `--lint <code>=on|off` a single lint, either one any number of times.
The command line SHALL override the file: `--lints` replaces the file's whole set, then each `--lint` overrides
its lint. An unknown code or value SHALL be an error with exit status 1 that names the known codes.

#### Scenario: On again for one run
- **WHEN** `qb64rust.toml` holds `[lints]` and `all = false`, and `qb64rust -w --lints on p.bas` is run
- **THEN** every lint warning of `p.bas` is printed

#### Scenario: One lint off
- **WHEN** `qb64rust -w --lint unreachable=off p.bas` is run on a program with code after a `GOTO`
- **THEN** no `unreachable` warning is printed

#### Scenario: An unknown code
- **WHEN** `qb64rust --lint unused-labl=off p.bas` is run
- **THEN** the exit status is 1 and the message names the thirteen codes

### Requirement: A bad lints table
An unknown code in `[lints]` or a value that is not `true` or `false` SHALL be an error that names the file, the
line and, for a code, the known codes. The program SHALL NOT be built.

#### Scenario: A misspelled code
- **WHEN** `qb64rust.toml` holds `[lints]` and `unused-labl = false`
- **THEN** the compile fails with an error naming `qb64rust.toml`, its line 2 and the thirteen codes, and exit status 1

### Requirement: The warning report counts every lint
`qb64rust --report warnings` SHALL count the warnings of every lint, whatever `[lints]` says: it measures how
often each lint fires.

#### Scenario: A lint turned off
- **WHEN** `qb64rust.toml` beside `p.bas` turns `unused-label` off and `p.bas` has an unused label
- **THEN** `qb64rust --report warnings p.bas` counts one `unused-label` warning

### Requirement: UTF-8 text in a file that declares nothing
A file SHALL get one `undeclared-utf8` warning when all of these hold:

- the file has no `'$ENCODING` line, and no `encoding` key of a settings file covers it (spec
  `compiler/source-encoding`, "A file's encoding");
- the file's bytes are valid UTF-8;
- a string literal or a `DATA` item of the file holds a byte above 127.

The warning SHALL be at the first such string or item. It SHALL say that the file looks like UTF-8 and is read
as code page 437, give the number of bytes the first such character becomes in the program, and name both ways
to declare the file. It SHALL be given for an included file too.

#### Scenario: A new file from an editor
- **WHEN** `p.bas` is saved as UTF-8, declares nothing, and line 2 is `PRINT "─"`
- **THEN** `qb64rust -w p.bas` gives one `undeclared-utf8` warning at line 2, column 7, which says that `─` is
  3 characters in the program and names `'$ENCODING:UTF8` and `encoding` in `qb64rust.toml`

#### Scenario: Only in a comment
- **WHEN** a file that declares nothing holds `' café` in UTF-8 and no byte above 127 in any string
- **THEN** no `undeclared-utf8` warning is given

#### Scenario: A code page 437 file
- **WHEN** a file that declares nothing holds the single byte 0xC4 in a string
- **THEN** no `undeclared-utf8` warning is given, since the file is not valid UTF-8

#### Scenario: Declared code page 437
- **WHEN** a file holds `'$ENCODING:CP437` and UTF-8 bytes in a string, for `_UPRINTSTRING` with encoding 8
- **THEN** no `undeclared-utf8` warning is given
