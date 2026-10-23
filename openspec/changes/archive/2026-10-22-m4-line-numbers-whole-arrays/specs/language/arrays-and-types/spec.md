# Spec Delta

## ADDED Requirements

### Requirement: A whole array as a value
Where the old compiler takes a whole array `a()` of a declared array in place of a value or a variable, it SHALL
compile and stand for its first element in memory, with no bounds check (design D4, `verification\v54`). The
places are any
expression and the variable of `READ`, `INPUT`, `INPUT #`, `LINE INPUT #`, `SWAP`, `FIELD`, `LSET` and `RSET`, each
taking one item; `LEN(a())` keeps its meaning. Each place is a row of `SOMEDAY.md`'s "QB64pe behaviours to review"
(user, 2026-10-22).

#### Scenario: PRINT a whole array
- **WHEN** `DIM a(2) AS LONG: PRINT a(): PRINT "ok"` runs
- **THEN** it prints ` 0 ` and `ok`, as recorded (`verification\v36_x37`)

#### Scenario: READ into a whole array
- **WHEN** `DIM a(3) AS LONG: READ a(): PRINT a(0); a(3)` runs with `DATA 1,2,3,4`
- **THEN** it prints ` 1  0 `: one item is read, into the first element (`verification\v22_x70`)

#### Scenario: SWAP two whole arrays
- **WHEN** `x(0) = 1: y(0) = 2: SWAP x(), y(): PRINT x(0); y(0)` runs for two `LONG` arrays
- **THEN** it prints ` 2  1 ` (`verification\v22_x101`)

#### Scenario: PRINT USING and _IIF
- **WHEN** `PRINT USING "##"; a()` and `PRINT _IIF(0, a(), b())` run for two `LONG` arrays of zeros
- **THEN** each prints the first element's value, as recorded (`verification\v30_x18`, `v30_x99`)

#### Scenario: A lower bound other than 0, two dimensions
- **WHEN** `DIM a(5 TO 7) AS LONG: a(5) = 55: PRINT a()` runs, and `PRINT b()` for `DIM b(1 TO 2, 1 TO 3)` with
  `b(1, 1) = 11`
- **THEN** it prints ` 55 ` and ` 11 ` (`verification\v54_c_whole_values`)

#### Scenario: In an expression
- **WHEN** `PRINT a() + 1`, `PRINT -a()`, `PRINT (a())`, `PRINT ABS(a())`, `IF a() THEN`, `x = a() + 1` and
  `b(1) = a() * 2` run for a `LONG` array with `a(0) = 4`
- **THEN** each uses 4, as recorded (`verification\v54_x05`, `x06`, `x16`, `x17`, `x28`, `x33`, `x34`)

#### Scenario: READ and INPUT # take one item
- **WHEN** `READ r()` for `DIM r(1 TO 3)` with `DATA 7,8`, then `READ ss()` for an array of `STRING`, and
  `INPUT #1, n()` for `DIM n(2 TO 4)` from a file of the lines `41` and `42`
- **THEN** `r(1)` is 7, `ss(0)` is `8`, `n(2)` is 41, and the other elements stay 0 (`verification\v54_c`)

#### Scenario: An erased array
- **WHEN** `REDIM a(2) AS LONG: ERASE a: PRINT a()`, then `READ a()` and `SWAP a(), b()` of two erased arrays run
  under a handler
- **THEN** it prints ` 0 ` and no error is raised (`verification\v54_d_whole_erased`)

### Requirement: A whole array where the old compiler rejects it
A whole array as a value where the old compiler rejects one SHALL stay a compile error: as the whole value of an
assignment (`x = a()`, `x = (a())`, `a() = b() + 1`), in `SWAP` with an array of another element type, as a
whole member array `t.m()`, as the argument of a scalar parameter, and in `LEN` of variable-length strings.

#### Scenario: Assigned to a scalar
- **WHEN** `x = a()`, `x = (a())` or `x$ = s()` is compiled
- **THEN** it is a compile error, as in requirement "Whole-array assignment checks" (`verification\v54_x29`,
  `x37`)

#### Scenario: SWAP of two types
- **WHEN** `SWAP a(), b()` is compiled for a `LONG` and an `INTEGER` array
- **THEN** it is a compile error (`verification\v54_x10`)

#### Scenario: A member array
- **WHEN** `PRINT t.m()` is compiled for a member array `m`
- **THEN** it is a compile error (`verification\v54_x27`)

## MODIFIED Requirements

### Requirement: Whole-array assignment checks
A whole-array assignment SHALL be a compile error, as in the old compiler, when the element types differ
(kind, width, sign, `STRING * n` length, user type), when the numbers of dimensions differ, and when one side is
`name()` and the other is not a whole array (a scalar, an element, an expression, a literal, a FUNCTION, a constant,
an undeclared name). A scalar or element target with a whole array as its value SHALL be a compile error too.
With a type suffix on the last name of both sides (`b%() = a%()`), the statement SHALL compile as the old
compiler takes it: an assignment of one element to one element (requirement "A whole array as a value").

#### Scenario: Types that differ
- **WHEN** `a() = b()` is compiled for `LONG` and `INTEGER`, `LONG` and `_UNSIGNED LONG`, `SINGLE` and `DOUBLE`,
  `_BIT * 2` and `_BIT * 3`, `_OFFSET` and `_INTEGER64`, `STRING` and `STRING * 4`, two user types of one layout
- **THEN** each is a compile error that names the two element types (the old compiler: "Cannot assign arrays of
  different types")

#### Scenario: Sizes and dimensions
- **WHEN** `a() = b()` is compiled for `STRING * 4` and `STRING * 8`, and for one and two dimensions
- **THEN** each is a compile error (the old compiler: "Cannot assign fixed-length STRING arrays with different
  element sizes", "Cannot assign arrays with different dimensions")

#### Scenario: Not a whole array on one side
- **WHEN** `arrA() = arrB`, `arrA() = arrB(1)`, `arrA = arrB()`, `arrA(1) = arrB()`, `values() = scalarVal`,
  `scalarVal = values()`, `a() = b() + 1` or `a() = 5` is compiled for arrays `arrA`, `arrB`, `values`, `a`, `b` and
  a scalar `scalarVal`
- **THEN** each is a compile error at the side that is not a whole array, or at the statement when neither side
  can be taken alone (the old compiler: "Expected array index after" the name, or "Whole-array assignment requires
  empty parentheses on both sides")

#### Scenario: Suffixes on both names
- **WHEN** `a%(0) = 1: a%(1) = 2: b%(2) = 9: b%() = a%(): PRINT b%(0); b%(1); b%(2)` runs
- **THEN** it prints ` 1  0  9 `: only the first element is copied, as recorded (`verification\v36_x38`)

#### Scenario: Suffixes on both names, types that differ
- **WHEN** `a&() = b%()` is compiled for `DIM a&(2), b%(2)`
- **THEN** it compiles and runs, as recorded (`verification\v36_x24`); the element is converted as an
  assignment converts it
