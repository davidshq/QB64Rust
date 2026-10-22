# Spec Delta

## ADDED Requirements

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

## MODIFIED Requirements

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
