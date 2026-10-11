# Spec Delta

## Purpose

The default type of a name that carries neither a type suffix nor an `AS` clause: SINGLE, or what `DEFINT`,
`DEFLNG`, `DEFSNG`, `DEFDBL`, `DEFSTR` and `_DEFINE` set for its first letter, with the old compiler's observed
behaviour (`verification\v24_a_*`).

## ADDED Requirements

### Requirement: Default type by first letter
`DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL` and `DEFSTR`, each followed by letters and letter ranges (`A-Z`, `i`,
`a-c, x`), SHALL set the default type of names beginning with those letters to INTEGER, LONG, SINGLE, DOUBLE and
`STRING`. Letters SHALL be compared without regard to case. A name with a type suffix or declared with `AS` SHALL
keep that type. Without any such statement the default type SHALL be SINGLE.

#### Scenario: DEFINT A-Z
- **WHEN** `DEFINT A-Z: x = 2.5: PRINT x; x / 2` runs
- **THEN** it prints ` 2  1 ` (`x` is an INTEGER; the division of an INTEGER prints as SINGLE)

#### Scenario: A range and single letters
- **WHEN** `DEFLNG a-c, x` is followed by `b = 100000: d = 1.5: x = 70000: PRINT b; d; x`
- **THEN** it prints ` 100000  1.5  70000 `

#### Scenario: Suffix and AS win
- **WHEN** `DEFINT A-Z` is followed by `DIM d AS DOUBLE: d = 1.5: s! = 2.5: n$ = "q": PRINT d; s!; n$`
- **THEN** it prints ` 1.5  2.5 q`

#### Scenario: DEFSTR
- **WHEN** `DEFSTR S: s = "text": PRINT s + "!"` runs
- **THEN** it prints `text!`

### Requirement: Default types apply from the statement on
A default-type statement SHALL apply to names first met after it in the file and SHALL NOT change a variable that
exists already; a later statement for the same letter SHALL replace the earlier one. In which procedures a
statement of the main module applies, and where a statement inside a procedure stops applying, SHALL be as the old
compiler does (`verification\v24_a_*`).

#### Scenario: Before and after
- **WHEN** `a = 1.5: DEFINT A-Z: b = 1.5: PRINT a; b` runs
- **THEN** it prints ` 1.5  2 `

#### Scenario: A later statement replaces an earlier one
- **WHEN** `DEFINT A-Z: DEFDBL D: i = 1.5: d = 1.5: PRINT i; d` runs
- **THEN** it prints ` 2  1.5 `

#### Scenario: Procedure after the statement
- **WHEN** `DEFINT A-Z` stands above `SUB s: v = 2.5: PRINT v: END SUB` and the main module calls `s`
- **THEN** the output equals the old compiler's

### Requirement: What a default type gives its type to
The default type SHALL be the type of every name that may be written without a suffix and has no `AS` clause: a
variable used or declared with `DIM`, `STATIC`, `SHARED`, `COMMON` or `REDIM`, an array, a parameter, and the
result of a FUNCTION. A FUNCTION whose result is typed by a default SHALL be callable with and without the
matching suffix.

#### Scenario: Array and parameter
- **WHEN** `DEFINT A-Z` is followed by `DIM t(3): t(1) = 2.5: PRINT t(1)` and by `SUB p (n): PRINT n / 2: END SUB`
  called as `p 5`
- **THEN** it prints ` 2 ` and ` 2.5 `

#### Scenario: FUNCTION result
- **WHEN** `DEFLNG F` stands above `FUNCTION f (x): f = x * 100000: END FUNCTION` and the main module prints
  `f(3)` and `f&(3)`
- **THEN** it prints ` 300000 ` twice

### Requirement: _DEFINE
`_DEFINE letters AS type` SHALL set the default type of those letters to any numeric type or `STRING`, by the same
rules as the `DEFxxx` statements.

#### Scenario: Unsigned default
- **WHEN** `_DEFINE u AS _UNSIGNED _BYTE: u = -1: PRINT u` runs
- **THEN** it prints ` 255 `

#### Scenario: A type that cannot be a default
- **WHEN** `_DEFINE a AS` is followed by a type the old compiler rejects there (a user type, `STRING * n`,
  `_BIT * n`: `verification\v24_x*`)
- **THEN** it is a compile error where the old compiler gives one
