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
than `v` (`_BYTE` → INTEGER, INTEGER → LONG, LONG → `_INTEGER64`, `_INTEGER64` unchanged, SINGLE → DOUBLE, DOUBLE
and `_FLOAT` → `_FLOAT`), and the direction taken from the sign of `s` at the header. `v` SHALL be a numeric scalar
variable; a string, a constant or an array element SHALL be a compile error, and so SHALL a string `a`, `b` or `s`.

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
the six comparisons). Each item SHALL be converted to the selector's type before it is compared, as the old
compiler does (a float item against an integer selector is rounded half to even). Numeric and string selectors
SHALL work; a numeric item for a string selector or the reverse SHALL be a compile error.

#### Scenario: Float item against an integer selector
- **WHEN** `SELECT CASE i%` with `i% = 2` has `CASE 2.4` and `CASE ELSE`
- **THEN** the `CASE 2.4` body runs, as with the old compiler

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
line numbers as targets SHALL be reported "not supported yet".

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
