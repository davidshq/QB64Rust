## MODIFIED Requirements

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
