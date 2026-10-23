# language/control-flow Specification

## Purpose
How a program's statements are sequenced: `IF`, the loops, `EXIT` from loops, `GOTO`, `GOSUB`/`RETURN` and
labels, with the old compiler's observed behaviour. Every rule is measured with `qb64pe.exe`; the scenarios are
tests in `tests\corpus\slice\` or `tests\frontend\`.

## Requirements

### Requirement: IF
A condition SHALL be true when its value is not zero. Block `IF` SHALL run the first branch whose condition is
true (`IF`, then each `ELSEIF` in order) or else the `ELSE` branch. A single-line `IF` SHALL do the same for its
`THEN` and `ELSE` parts; `IF c GOTO label` SHALL jump when `c` is true. A string condition SHALL be a compile error.

#### Scenario: ELSEIF chain
- **WHEN** a block `IF` tests `x = 1`, `ELSEIF x = 2`, `ELSE`, with `x = 2`
- **THEN** only the `ELSEIF` branch runs

#### Scenario: Single-line IF with ELSE
- **WHEN** `IF 0 THEN PRINT "a" ELSE PRINT "b": PRINT "c"` runs
- **THEN** it prints `b` and `c` (the statements after `ELSE` belong to the `ELSE` part)

### Requirement: FOR loops
`FOR v = a TO b [STEP s]` SHALL evaluate `a`, `b` and `s` once, in that order, assign `a` to `v`, and run the body
while `v` has not passed `b` (above `b` for a step of 0 or more, below it for a negative step). `NEXT` SHALL add
`s` to the current value of `v` (changes made in the body count). The comparison SHALL use a value wider than `v`,
after `v` has been assigned, as in the old compiler: `a`, `b` and `s` converted with rounding to a type wider
than `v` (8-bit `_BYTE` or `_UNSIGNED _BYTE` → INTEGER, 16-bit INTEGER or `_UNSIGNED INTEGER` → LONG, every
32-bit and 64-bit integer type, signed or unsigned, and `_OFFSET` → `_INTEGER64`, SINGLE → DOUBLE, DOUBLE and
`_FLOAT` → `_FLOAT`), and the direction taken from the sign of `s` at the header. `v` SHALL be a numeric scalar
variable; a string, a `_BIT` or `_BIT * n` variable, a constant or an array element SHALL be a compile error, and so
SHALL a string `a`, `b` or `s`.

#### Scenario: Limits rounded to the wider type
- **WHEN** `FOR i% = 1 TO 2.6: PRINT i%;: NEXT: PRINT i%` runs
- **THEN** it prints ` 1  2  3  4 `

#### Scenario: _INTEGER64 variable at its maximum
- **WHEN** `FOR q&& = 9223372036854775800 TO 9223372036854775807 STEP 5` runs a body capped at five passes
- **THEN** the third pass sees `-9223372036854775806` (the value wraps and the loop does not end by itself)

#### Scenario: Limits evaluated once
- **WHEN** `e = 3: FOR m = 1 TO e: e = 10: PRINT m;: NEXT` runs
- **THEN** it prints ` 1  2  3 `

#### Scenario: Body changes the variable
- **WHEN** `FOR k = 1 TO 5: k = k + 1: PRINT k;: NEXT` runs
- **THEN** it prints ` 2  4  6 `

#### Scenario: Loop that does not run
- **WHEN** `FOR m = 3 TO 1: PRINT "never": NEXT: PRINT m` runs
- **THEN** it prints ` 3 ` only

#### Scenario: INTEGER variable passes its range
- **WHEN** `FOR i% = 32760 TO 32767 STEP 4: NEXT: PRINT i%` runs
- **THEN** it prints `-32768` and the loop ends (the wider value 32768 is past the limit)

#### Scenario: SINGLE steps
- **WHEN** `FOR s! = 0 TO 1 STEP 0.1: c = c + 1: NEXT: PRINT s!; c` runs
- **THEN** it prints ` 1  10 `

#### Scenario: Unsigned byte passes its range
- **WHEN** `FOR b~%% = 250 TO 255 STEP 3: PRINT b~%%;: NEXT: PRINT b~%%` runs
- **THEN** it prints ` 250  253  0 ` (the wider value 256 is past the limit; the store wraps to 0)

#### Scenario: Unsigned variable counts down through 0
- **WHEN** `FOR u~% = 2 TO 0 STEP -1: PRINT u~%;: NEXT: PRINT u~%` runs
- **THEN** it prints ` 2  1  0  65535 ` (the wider value -1 is past the limit)

#### Scenario: Bit variable
- **WHEN** `DIM t AS _BIT * 8: FOR t = 1 TO 3: NEXT` is compiled
- **THEN** it is a compile error

### Requirement: NEXT variables
A `NEXT` that names variables SHALL name the variables of the `FOR` blocks it closes, innermost first; any other
variable SHALL be a compile error. A `NEXT` without a variable SHALL close the innermost `FOR`. A variable is a
name plus a type: `NEXT i!` closes `FOR i` (plain `i` being SINGLE), and `NEXT i` does not close `FOR i%`.

#### Scenario: Wrong order
- **WHEN** `FOR i = 1 TO 2: FOR j = 1 TO 2: NEXT i, j` is compiled
- **THEN** it is a compile error

### Requirement: DO and WHILE loops
`DO WHILE c` and `WHILE c` SHALL test before each pass and run the body while `c` is true; `DO UNTIL c` while it is
false; `LOOP WHILE c` and `LOOP UNTIL c` SHALL test after each pass; a `DO … LOOP` without a condition SHALL loop
until left by `EXIT DO` or a jump. A string condition SHALL be a compile error.

#### Scenario: LOOP UNTIL runs at least once
- **WHEN** `n = 5: DO: PRINT n;: n = n + 1: LOOP UNTIL n > 3` runs
- **THEN** it prints ` 5 `

### Requirement: EXIT from loops
`EXIT FOR`, `EXIT DO` and `EXIT WHILE` SHALL continue after the innermost open block of that kind, also from
inside other blocks nested in it.

#### Scenario: EXIT FOR from inside an IF
- **WHEN** `FOR i = 1 TO 9: IF i = 3 THEN EXIT FOR` … `NEXT: PRINT i` runs
- **THEN** it prints ` 3 `

### Requirement: GOTO
`GOTO label` SHALL continue at the label, which SHALL be in the same body (the main module, or the same
procedure). The label MAY stand inside a block.

#### Scenario: Jump into an IF body
- **WHEN** a program jumps with `GOTO inside` to a label inside the `THEN` branch of a block `IF` whose condition
  is false
- **THEN** the statements after the label in that branch run, then the statements after `END IF`

### Requirement: GOSUB and RETURN
`GOSUB label` SHALL continue at the label and remember where it was; `RETURN` SHALL continue after the most
recent `GOSUB` not yet returned from; `RETURN label` SHALL forget it and continue at the label. `GOSUB` SHALL work
in the main module and inside procedures, with labels of the same body; `RETURN label` inside a procedure SHALL be
a compile error. The program SHALL have one stack of pending `GOSUB`s, as the old compiler has: `RETURN` without a
pending `GOSUB` SHALL raise error 3 at run time, and a `RETURN` in a procedure whose most recent pending `GOSUB`
belongs to another body SHALL raise error 3 and remove that `GOSUB` from the stack. A procedure left with its own
`GOSUB` pending SHALL leave it on the stack.

#### Scenario: RETURN in a SUB during a main GOSUB
- **WHEN** main executes `GOSUB g1`, `g1` calls a SUB that executes `RETURN`, the handler executes `RESUME NEXT`,
  and `g1` then executes `RETURN`
- **THEN** both `RETURN`s raise error 3, and execution continues after `g1`'s `RETURN`, as with the old compiler

#### Scenario: GOSUB inside a SUB
- **WHEN** a SUB executes `GOSUB lab`, `lab` (in the SUB) prints `lab in s` and executes `RETURN`, and the SUB then
  prints `s done`
- **THEN** the output is `lab in s`, then `s done`

### Requirement: Labels per body
A label SHALL belong to the body it stands in: the main module or one procedure. Two labels with the same name in
one body SHALL be a compile error; the same name in two bodies SHALL be two labels. A jump to a label of another
body SHALL be a compile error.

#### Scenario: GOTO from a SUB to a main-module label
- **WHEN** a SUB executes `GOTO done` and `done:` stands only in the main module
- **THEN** it is a compile error

### Requirement: SELECT CASE
`SELECT CASE e` SHALL evaluate `e` once (a plain scalar variable is instead read at each test, as in the old
compiler) and run the body of the first `CASE` with an item that matches it, or else the `CASE ELSE` body; an item SHALL be a value (equal), `a TO b` (from `a` to `b` inclusive) or `IS op v` (one of
the six comparisons). An item SHALL be compared with the selector as the old compiler compares them: a float item
against an integer selector is first rounded half to even to an integer (to the selector's type for a 64-bit
selector, else to LONG); an integer item against an integer selector is compared as written, in the type C++ gives
the two (so `CASE -1` matches an `_UNSIGNED LONG` selector holding 4294967295 but not an `_UNSIGNED INTEGER` one
holding 65535); any item against a float selector is converted to the selector's type. Numeric and string selectors
SHALL work; a numeric item for a string selector or the reverse SHALL be a compile error.

#### Scenario: Float item against an integer selector
- **WHEN** `SELECT CASE i%` with `i% = 2` has `CASE 2.4` and `CASE ELSE`
- **THEN** the `CASE 2.4` body runs, as with the old compiler

#### Scenario: Negative item against unsigned selectors
- **WHEN** `SELECT CASE ui` with `ui AS _UNSIGNED INTEGER` holding 65535, and `SELECT CASE ul` with `ul AS
  _UNSIGNED LONG` holding 4294967295, each have `CASE -1` and `CASE ELSE`
- **THEN** the first runs its `CASE ELSE` body and the second its `CASE -1` body, as with the old compiler

#### Scenario: Items of each kind
- **WHEN** `SELECT CASE x` has `CASE 1, 3`, `CASE 4 TO 6`, `CASE IS > 10`, `CASE ELSE`, and runs for `x` = 3, 5, 11
  and 8
- **THEN** the first, second, third and `ELSE` bodies run, in that order

#### Scenario: String selector
- **WHEN** `SELECT CASE s$` has `CASE "a" TO "m"` and `CASE ELSE`, with `s$ = "dog"`
- **THEN** the first body runs

#### Scenario: Selector evaluated once
- **WHEN** the selector is a FUNCTION call that prints a line, and three `CASE` items are tested
- **THEN** the line is printed once

#### Scenario: Variable selector read at each test
- **WHEN** `SELECT CASE v` (a `SHARED` variable `v = 1`) has `CASE f`, where FUNCTION `f` sets `v = 2` and returns
  0, then `CASE 2` and `CASE ELSE`
- **THEN** the `CASE 2` body runs, as with the old compiler

#### Scenario: Recursion between the test and a later CASE
- **WHEN** a recursive FUNCTION computes `SELECT CASE n * 10` and, in a `CASE` item tested before the matching one,
  calls itself with another `n`
- **THEN** the outer `SELECT` tests its later items against the inner call's value, as with the old compiler (one
  static copy per `SELECT`; `DIVERGENCES-QB45.md` Q-002)

#### Scenario: Mismatched item
- **WHEN** `SELECT CASE x` has `CASE "a"` for a numeric `x`
- **THEN** it is a compile error

### Requirement: SELECT EVERYCASE
`SELECT EVERYCASE e` SHALL run the body of every `CASE` with a matching item, in order, and the `CASE ELSE` body
only when none matched.

#### Scenario: Two matching cases
- **WHEN** `SELECT EVERYCASE x` has `CASE IS > 1` and `CASE IS > 2` and `CASE ELSE`, with `x = 5`
- **THEN** the first two bodies run, and not the `ELSE` body

### Requirement: Errors in SELECT CASE
A runtime error raised in the selector or in a `CASE` item SHALL be handled by the IR's pending-error rule, so a
handler that resumes next sees the behaviour the old compiler shows, as measured (`verification\v20_*`).

#### Scenario: Selector raises
- **WHEN** `SELECT CASE ASC("")` with `CASE 0` and `CASE ELSE` runs under a handler that resumes next
- **THEN** the same body runs as with the old compiler (`CASE 0`)

#### Scenario: Item raises
- **WHEN** `SELECT CASE x` with `x = 5` has `CASE ASC("")`, then `CASE ELSE`, under a handler that resumes next
- **THEN** the `CASE ASC("")` body runs, as with the old compiler (a test that raises is taken, as for `IF`)

### Requirement: ON GOTO and ON GOSUB
`ON n GOTO l1, l2, …` SHALL jump to the n-th label, and `ON n GOSUB l1, l2, …` SHALL `GOSUB` to it; `n` SHALL be
converted to an integer as measured (a float rounded half to even). When `n` is 0 or greater than the number of
labels, execution SHALL continue with the next statement; a negative `n` SHALL raise error 5. When computing `n`
raises an error, the jump SHALL use the value left behind, as the old compiler. Labels SHALL be in the same body;
a target MAY be a line number (requirement "Line numbers as jump targets").

#### Scenario: Float value
- **WHEN** `ON 1.5 GOTO a, b` runs
- **THEN** execution continues at `b`

#### Scenario: Second label
- **WHEN** `ON 2 GOTO a, b, c` runs
- **THEN** execution continues at `b`

#### Scenario: Out of range
- **WHEN** `ON 4 GOTO a, b, c` runs, followed by `PRINT "next"`
- **THEN** it prints `next`

#### Scenario: Above 255
- **WHEN** `ON 258 GOTO a, b` runs, followed by `PRINT "next"`
- **THEN** it prints `next`, as with the old compiler (QuickBASIC 4.5 raises error 5: `DIVERGENCES-QB45.md` Q-001)

#### Scenario: Negative value
- **WHEN** `ON -1 GOTO a, b` runs under a handler that prints `ERR`
- **THEN** the handler prints 5

#### Scenario: ON GOSUB returns
- **WHEN** `ON 1 GOSUB g: PRINT "back"` runs and `g` prints `in g` and executes `RETURN`
- **THEN** the output is `in g`, then `back`

### Requirement: A SUB's name and a colon, with no jump to the name
A line that starts with the name of a SUB and a colon SHALL be a call of that SUB when the SUB can be called with
no argument and no jump of the program, in any body, names the word; a statement after the colon SHALL follow the
call. This is the old compiler's rule.

#### Scenario: A call
- **WHEN** a program has the line `topbar:` in its main module, a `SUB topbar` without parameters, and no jump to
  `topbar`
- **THEN** the line calls the SUB, as with the old compiler

#### Scenario: Twice in one body
- **WHEN** the main module has the line `one:` twice, the program has a `SUB one` without parameters and no jump
  to `one`
- **THEN** the SUB is called twice

#### Scenario: A statement after the colon
- **WHEN** the line is `one: PRINT "after"` and `SUB one` prints `sub one`
- **THEN** the output is `sub one`, then `after`

### Requirement: A SUB's name and a colon, with a jump to the name
When a jump anywhere in the program (`GOTO`, `GOSUB`, `RETURN`, `RESTORE`, `RESUME`, `ON ERROR GOTO`, `ON … GOTO`,
`ON … GOSUB`) names the word, every line that is the word and a colon SHALL be a label, with the rules of any
label, and `CALL name` SHALL still call the SUB. The name of a SUB that needs an argument, and the name of a
FUNCTION, SHALL always be a label there.

#### Scenario: A label beside a call
- **WHEN** a SUB has the line `ships:`, later `IF n < 2 GOTO ships` and `CALL ships`, and the program has a `SUB
  ships` without parameters
- **THEN** the `GOTO` continues at the line and the `CALL` calls the SUB

#### Scenario: A jump in another body
- **WHEN** the main module has the line `one:`, a SUB has its own line `one:` and `GOTO one`, and the program has a
  `SUB one` without parameters
- **THEN** the main module's line is a label and calls nothing

#### Scenario: A jump with no such line in its body
- **WHEN** a SUB executes `GOTO one`, `one:` stands only in the main module, and the program has a `SUB one`
- **THEN** it is a compile error (the label is not defined in the SUB)

#### Scenario: Twice in one body, with a jump
- **WHEN** the main module has the line `one:` twice and `GOTO one`, and the program has a `SUB one`
- **THEN** it is a compile error (a duplicate label)

#### Scenario: A SUB with a parameter, and a FUNCTION
- **WHEN** a program has the line `one:` and `SUB one (k AS INTEGER)`, or the line `fone:` and `FUNCTION fone`
- **THEN** the line is a label, with or without a jump to it

### Requirement: EXIT SELECT and EXIT CASE
`EXIT SELECT` SHALL continue after the `END SELECT` of the innermost `SELECT` that has a `CASE` or `CASE ELSE`
open around it, also from inside an `IF` or a loop within that `CASE`, and also in `SELECT EVERYCASE`. `EXIT
CASE` SHALL do the same in a plain `SELECT`, and in `SELECT EVERYCASE` SHALL go on with the next `CASE`. With
no `CASE` open around it either SHALL be a compile error.

#### Scenario: Out of a CASE from inside an IF
- **WHEN** `SELECT CASE 1: CASE 1: PRINT "a": IF 1 THEN EXIT SELECT` … `PRINT "b": CASE ELSE: PRINT "c": END
  SELECT: PRINT "d"` runs
- **THEN** it prints `a` and `d`

#### Scenario: A loop around the SELECT goes on
- **WHEN** a `FOR i = 1 TO 3` loop holds a `SELECT CASE i` whose `CASE 2` runs `EXIT SELECT` before a `PRINT`,
  and the loop prints `i` after `END SELECT`
- **THEN** it prints 1, 2 and 3, and the `PRINT` in `CASE 2` does not run

#### Scenario: From a loop inside the CASE
- **WHEN** a `CASE` body holds a `DO` loop that runs `EXIT SELECT`
- **THEN** execution continues after `END SELECT`

#### Scenario: EVERYCASE
- **WHEN** `SELECT EVERYCASE 5` has `CASE IS > 1` with `EXIT CASE` before a `PRINT`, then `CASE IS > 2` with
  `EXIT SELECT` before a `PRINT`, then `CASE IS > 3` with a `PRINT`
- **THEN** nothing is printed: the first leaves its `CASE`, the second the whole block

#### Scenario: The nearest SELECT
- **WHEN** a `SELECT` inside a `CASE` of another runs `EXIT SELECT`
- **THEN** execution continues after the inner `END SELECT`, inside the outer `CASE`

#### Scenario: No CASE open
- **WHEN** a program has `EXIT SELECT` outside any `SELECT`, in a SUB called from a `CASE`, or between `SELECT
  CASE x` and its first `CASE`
- **THEN** each is a compile error

### Requirement: END and SYSTEM with an exit code
`SYSTEM n` and `END n` SHALL end the program as `SYSTEM` and `END` do, with `n`, converted as the old compiler
converts it (to an `_INTEGER64`, then to the process's exit code as libqb does), as the process's exit code. A
pending error SHALL be handled before `n` is stored, as the old compiler's program does. `n` SHALL be any numeric
expression the old compiler accepts there, and a string a compile error, as with it.

#### Scenario: An exit code
- **WHEN** a console program runs `PRINT "x": SYSTEM 3`
- **THEN** it prints `x` and its exit code is 3

#### Scenario: END with a code
- **WHEN** a console program runs `END 7`
- **THEN** its exit code is what the old compiler's program gives (7, as measured)

#### Scenario: A value out of range
- **WHEN** a program runs `SYSTEM -1` or `SYSTEM 2.7`
- **THEN** its exit code is the old compiler's program's

#### Scenario: A string
- **WHEN** a program has `SYSTEM "a"`
- **THEN** it is a compile error, as with the old compiler

### Requirement: _CONTINUE
`_CONTINUE`, alone on its statement, SHALL go on with the next pass of the innermost open `FOR`, `DO` or `WHILE`
loop of the current body, whatever other blocks lie between. In a `FOR` loop the step SHALL still be added and the
limit tested; in a `DO … LOOP WHILE|UNTIL c` the test SHALL still run; in a `DO WHILE`, `DO UNTIL` or `WHILE` loop
the test at the top SHALL run. Outside such a loop it SHALL be a compile error that is not "not supported yet".

#### Scenario: Skipping in a FOR loop
- **WHEN** `FOR i = 1 TO 4: IF i = 2 THEN _CONTINUE` … `PRINT i;: NEXT` runs
- **THEN** it prints ` 1  3  4 `

#### Scenario: From inside SELECT CASE
- **WHEN** a `FOR` loop's body is a `SELECT CASE` whose `CASE 3, 6: _CONTINUE` skips the rest of the body
- **THEN** the loop goes on with the next value, as with the old compiler (`colors/color_with_dest`)

#### Scenario: LOOP UNTIL still tests
- **WHEN** `n = 0: DO: n = n + 1: IF n < 3 THEN _CONTINUE` … `PRINT n;: LOOP UNTIL n >= 5` runs
- **THEN** it prints ` 3  4  5 ` and ends

#### Scenario: The innermost loop
- **WHEN** a `_CONTINUE` stands in a `WHILE` loop that stands in a `FOR` loop
- **THEN** it goes on with the `WHILE` loop

#### Scenario: Outside a loop
- **WHEN** a program has `_CONTINUE` outside any loop, or in a SUB whose only loop is around its call
- **THEN** the compiler reports an error that is not "not supported yet" (`verification\v47_x01`, `x02`)

#### Scenario: Not alone on its statement
- **WHEN** a program has `_CONTINUE 1` in a loop, or `_CONTINUE = 3`
- **THEN** the compiler reports an error that is not "not supported yet", as the old compiler does (`v47_x03`,
  `x26`)

#### Scenario: In a GOSUB subroutine and as a single-line IF's ELSE
- **WHEN** a loop after a label reached by `GOSUB` holds `IF i = 2 THEN _CONTINUE`, or a loop holds
  `IF i <> 3 THEN PRINT i; ELSE _CONTINUE`
- **THEN** each goes on with the next pass, as with the old compiler (`v47_x24`, `x25`)

### Requirement: STOP
`STOP` SHALL end the program as `SYSTEM` does without a code: no "Press any key" prompt, the files closed, and the
exit code the one set so far. `STOP` followed by an expression SHALL check the expression as a number, as the old
compiler does (a string is a compile error), and SHALL NOT evaluate it at run time. `STOP` SHALL be allowed in the
main module and in a SUB or FUNCTION, and after `THEN` or `ELSE` on one line.

#### Scenario: STOP ends the program
- **WHEN** a console program runs `PRINT "a": STOP: PRINT "b"`
- **THEN** it prints `a` and ends with exit code 0

#### Scenario: STOP in a SUB after THEN
- **WHEN** a SUB runs `IF x = 1 THEN PRINT "e": STOP` with `x = 1`
- **THEN** the program prints `e` and ends

#### Scenario: An expression is not evaluated
- **WHEN** a program runs `STOP f(1)`, where the FUNCTION `f` prints a line
- **THEN** nothing of `f` is printed (`verification\v52_x12`)

#### Scenario: A window program
- **WHEN** a program with a window runs `STOP`
- **THEN** it ends without waiting for a key, as the old compiler's program does (`verification\v52_x14`)

### Requirement: Line numbers
A number at the start of a line SHALL be a line number: a label of the body it stands in, as the old compiler has
it. It MAY have a decimal point or a type suffix, be glued to the statement, be beyond the LONG range, and be
followed by a label, a statement, a colon or nothing. Line numbers and named labels SHALL share one name space per
body, and the order of the numbers SHALL NOT matter.

#### Scenario: Before a statement, a colon, alone
- **WHEN** `v16_m3_basic` runs: `10 PRINT "ten"`, `20 :`, `30` on a line alone, then `GOTO 50` over
  `40 PRINT "forty, skipped"`
- **THEN** it prints `ten` and `fifty`, as recorded

#### Scenario: Forms of the number
- **WHEN** a program jumps to `10.5`, to `10` written `10&` at its line, to a number glued to `PRINT`
  (`10PRINT "glued"`) and to `4294967296`
- **THEN** each jump reaches its line, as recorded (`verification\v16_m3_decimal`, `v16_m3_suffix`,
  `v16_m3_glued`)

#### Scenario: Out of order
- **WHEN** lines numbered `30`, `10` and `20` stand in that order and the program executes `GOSUB 20`
- **THEN** the line numbered 20 runs and `RETURN` comes back, as recorded (`v16_m3_out_of_order`)

#### Scenario: Before a SUB header and before a label
- **WHEN** a line reads `10 SUB s`, or `10 lab: PRINT "after 10 lab:"`
- **THEN** the program compiles and runs as recorded (`v16_m3_sub_header`, `v16_m3_number_then_label`)

### Requirement: The spelling of a line number
A line number SHALL name its line by the old compiler's spelling (`verification\v54`): a whole number without a
suffix by its digits with leading zeros dropped; a number with a decimal point or a suffix by its text as written,
with a suffix other than `!` and `#` left off. A spelling longer than 40 characters SHALL be a compile error, and
so SHALL a number whose text C++ does not take as an integer (where the old compiler's C++ build fails).

#### Scenario: Spellings of one number
- **WHEN** a program has the lines `10` and `010`
- **THEN** it is a compile error naming the duplicate label 10 (`verification\v54_x04`)

#### Scenario: Spellings of two numbers
- **WHEN** a program has the lines `10` and `10.0`, or `1.10` and `1.1`, or `30` and `30#`, and jumps to the
  second of each
- **THEN** it compiles, and each jump reaches the line written as it (`verification\v54_x03`, `x11`, `x12`)

#### Scenario: A number C++ does not take
- **WHEN** a program has the line `08&` (an octal number with the digit 8) or a number of 25 digits
- **THEN** it is a compile error at the line number, where the old compiler's C++ build fails
  (`verification\v54_x40`, `x41`)

### Requirement: Line numbers as jump targets
A line number SHALL be a target wherever a label is: `GOTO`, `GOSUB`, `RETURN n`, `ON n GOTO` and `ON n GOSUB`,
and after `THEN` or `ELSE` of a single-line `IF` (`IF c THEN 100 ELSE 200`, `IF c GOTO 100`). The same number twice
in one body SHALL be the compile error "Duplicate label", and a number no line has SHALL be "Label '…' not
defined", as in the old compiler.

#### Scenario: IF THEN a line number
- **WHEN** `IF c THEN 100` and `IF c THEN 300 ELSE 400` run with `c` true and false
- **THEN** each jumps to the line it names, as recorded (`verification\v17_b_if_then_line`)

#### Scenario: The same number twice
- **WHEN** a program has `10 PRINT "a"` and `10 PRINT "b"`
- **THEN** it is a compile error at the second, naming the duplicate label 10

#### Scenario: A number no line has
- **WHEN** a program has `GOTO 99` and no line 99
- **THEN** it is a compile error naming label 99 as not defined

#### Scenario: A number of another body
- **WHEN** the main module has `GOTO 10` or `ON ERROR GOTO 10` and only a SUB has a line 10
- **THEN** it is a compile error naming label 10 as not defined (`verification\v54_x01`, `x21`)

#### Scenario: RETURN to a number in a SUB
- **WHEN** a SUB has `GOSUB 10` and, at line 10, `RETURN 20`
- **THEN** it is a compile error, as `RETURN label` in a SUB is (`verification\v54_x02`)

#### Scenario: IF GOTO a number
- **WHEN** `IF c GOTO 100` and `IF c = 0 GOTO 200 ELSE 300` run with `c` 1
- **THEN** they reach the lines 100 and 300 (`verification\v54_x13`)

#### Scenario: ON GOSUB with line numbers
- **WHEN** `ON 2 GOSUB 100, 200` runs and the line numbered 200 prints `two` and executes `RETURN`
- **THEN** it prints `two` and continues after the `ON` statement

### Requirement: A jump to a built-in's name
A line that is a built-in statement's name and a colon (`BEEP:`) SHALL be that statement, not a label. A line
that is a built-in function's name and a colon, and a jump to a built-in's name whether a line of that name
stands in the file or not, SHALL be compile errors that are not "not supported yet", as the old compiler's are.

#### Scenario: GOTO a built-in statement's name
- **WHEN** a program has `BEEP: l = 1` and then `GOTO BEEP`, or the same with `CLS` or `SOUND`
- **THEN** each is a compile error that is not "not supported yet" (`verification\v29_x78_beep_goto`,
  `v29_x79_sound_goto`, `v27_x63_goto_statement_name`)

#### Scenario: The other jumps
- **WHEN** a program has `GOTO abs` or `GOTO CLS` with no such line, `ON 1 GOTO BEEP`, `GOSUB BEEP`, `RESTORE
  BEEP` or `ON ERROR GOTO BEEP` beside a line `BEEP: l = 1`
- **THEN** each is a compile error that is not "not supported yet" (the old compiler: "Invalid label!" for `GOTO`
  and `ON … GOTO`, "Invalid label" for the others; `verification\v55_x36`–`x38`, `x42`, `x43`, `x45`)

#### Scenario: A line named like a built-in function
- **WHEN** a program has the line `abs: l = 1` or `LEN: l = 1`
- **THEN** it is a compile error on that line that is not "not supported yet" (the old compiler: "Syntax error";
  `verification\v55_x35`, `x44`)
