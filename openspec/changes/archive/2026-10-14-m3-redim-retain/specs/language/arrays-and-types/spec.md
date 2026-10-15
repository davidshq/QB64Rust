## MODIFIED Requirements

### Requirement: REDIM
`REDIM [SHARED] name(bounds) [AS type]` SHALL declare the dynamic array if it does not exist and SHALL give it new
bounds and new elements (zero or empty) each time it runs. Without a type, `REDIM` of an array declared with `AS`
SHALL keep that type. `REDIM` of a static array SHALL raise error 10 and leave it unchanged. Another number of
dimensions than the array has, another type written with `AS` than the array was declared with, and a change
between `STRING` and `STRING * n` or of n, SHALL be compile errors. A dimension with no element, a lower upper
bound still and a bound that raises SHALL behave as for `DIM` (requirement "Dynamic arrays"), the old elements
being gone. `REDIM name()` and `REDIM name AS type` SHALL declare a scalar, as `DIM name()` does. The words after
`REDIM` SHALL be one of `_PRESERVE` and `_RETAIN`, then `SHARED`; `REDIM SHARED _PRESERVE`, `REDIM SHARED
_RETAIN`, both words together in either order, either word twice, `REDIM _RETAIN` with nothing after it and `DIM
_RETAIN` SHALL be compile errors. `SHARED` SHALL
share the array only in the statement that declares it: on a later `REDIM` of the array it SHALL change nothing,
as in the old compiler. `PRESERVE` and `RETAIN` without their underscore SHALL be names like any other (`REDIM
preserve(3)` declares an array of that name).

#### Scenario: SHARED on a later REDIM
- **WHEN** `REDIM ns(3) AS LONG`, then `REDIM SHARED ns(4) AS LONG: PRINT UBOUND(ns)` run
- **THEN** it prints ` 4 `, and `ns` is not visible in a SUB below

#### Scenario: New bounds, new elements
- **WHEN** `REDIM r(5) AS LONG: r(2) = 7: REDIM r(1 TO 20) AS LONG: PRINT LBOUND(r); UBOUND(r); r(2)` runs
- **THEN** it prints ` 1  20  0 `

#### Scenario: Type kept
- **WHEN** `REDIM x(5) AS LONG: REDIM x(10): x(1) = 100000: PRINT x(1)` runs
- **THEN** it prints ` 100000 `

#### Scenario: REDIM of a static array
- **WHEN** `DIM s(5) AS LONG: s(1) = 3: REDIM s(9) AS LONG` runs under a handler that prints `ERR` and resumes next,
  then `PRINT UBOUND(s); s(1)`
- **THEN** the handler prints 10, then ` 5  3 ` is printed

#### Scenario: Another number of dimensions
- **WHEN** `REDIM m(2, 3): REDIM m(4)` is compiled
- **THEN** it is a compile error

#### Scenario: Another type
- **WHEN** `REDIM x(5) AS LONG: REDIM x(5) AS INTEGER` is compiled
- **THEN** it is a compile error; `REDIM x&(7)` in its place names the same array and `REDIM x!(7)` another one

#### Scenario: REDIM of an array made by DIM
- **WHEN** `n = 3: DIM x(n) AS LONG: REDIM x(7): x(1) = 100000: PRINT UBOUND(x); x(1)` runs
- **THEN** it prints ` 7  100000 `

#### Scenario: The words after REDIM
- **WHEN** `REDIM a(3) AS LONG` is followed by `REDIM SHARED _RETAIN a(5) AS LONG`, by `REDIM _PRESERVE _RETAIN
  a(5) AS LONG`, by `REDIM _RETAIN _PRESERVE a(5) AS LONG` or by `REDIM _RETAIN _RETAIN a(5) AS LONG`, each in a
  program of its own, or a program holds `REDIM _RETAIN` alone or `DIM _RETAIN a(3) AS LONG`
- **THEN** each program is a compile error, and `REDIM _RETAIN SHARED a(5) AS LONG` in the main module is not

## ADDED Requirements

### Requirement: REDIM _RETAIN
`REDIM _RETAIN` SHALL give the array new bounds and SHALL keep each element whose indexes lie within both the old
and the new bounds of every dimension, at the same indexes; every other element SHALL be zero or empty. In all
else it SHALL behave as `REDIM _PRESERVE`, for every kind of element and every kind of array, and on a member
array it SHALL change nothing.

#### Scenario: One dimension moved
- **WHEN** `REDIM a(1 TO 5) AS LONG` holds 10, 20, 30, 40, 50, then `REDIM _RETAIN a(3 TO 7) AS LONG` runs and
  `a(3)` to `a(7)` are printed
- **THEN** it prints ` 30  40  50  0  0 `

#### Scenario: Grown and shrunk
- **WHEN** that array is then given `REDIM _RETAIN a(0 TO 9) AS LONG`, and after it `REDIM _RETAIN a(4 TO 5) AS
  LONG`
- **THEN** `a(0)` to `a(9)` are 0, 0, 0, 30, 40, 50, 0, 0, 0, 0, then `a(4)` and `a(5)` are 40 and 50

#### Scenario: Two dimensions keep coordinates
- **WHEN** `REDIM m(1 TO 3, 1 TO 4) AS INTEGER` holds `10 * i + j` at `m(i, j)`, then `REDIM _RETAIN m(2 TO 5, 0
  TO 3) AS INTEGER` runs
- **THEN** `m(2, 1)` to `m(2, 3)` are 21, 22, 23, `m(3, 1)` to `m(3, 3)` are 31, 32, 33, and the other elements
  are 0

#### Scenario: Ranges that do not meet
- **WHEN** an array `a(4 TO 5) AS LONG` holding 40 and 50 is given `REDIM _RETAIN a(100 TO 102) AS LONG`
- **THEN** its three elements are 0

#### Scenario: Strings, fixed-length strings and bits
- **WHEN** `s(1 TO 4) AS STRING` holding `A` to `D` is given `REDIM _RETAIN s(3 TO 6) AS STRING`, `x(1 TO 3) AS
  STRING * 3` holding `aaa`, `bbb`, `ccc` is given `REDIM _RETAIN x(2 TO 5) AS STRING * 3`, and `p(0 TO 9) AS
  _UNSIGNED _BIT * 5` holding `3 * i` is given `REDIM _RETAIN p(5 TO 14) AS _UNSIGNED _BIT * 5`
- **THEN** `s(3)` to `s(6)` are `C`, `D` and two empty strings, `x(2)` to `x(5)` are `bbb`, `ccc` and two strings
  of three NUL bytes, and `p(5)` to `p(14)` are 15, 18, 21, 24, 27 and five zeros

#### Scenario: User types
- **WHEN** an array of a user type with a `STRING` member, a `STRING * 3` member and a `LONG` is given
  `REDIM _RETAIN` with bounds that move it by one, and another of a user type with a member array is given
  `REDIM _RETAIN` with a second dimension moved by one
- **THEN** the elements at kept indexes have every member as before, member arrays included, and the others are
  zero or empty

#### Scenario: Not allocated
- **WHEN** `REDIM _RETAIN f(2 TO 4) AS LONG` is the first statement of `f`, and later, after `ERASE f`, `REDIM
  _RETAIN f(1 TO 5) AS LONG` runs
- **THEN** both give an array of zeros with those bounds

#### Scenario: Static array
- **WHEN** `DIM st(5) AS LONG: st(1) = 3: REDIM _RETAIN st(9) AS LONG` runs under a handler that prints `ERR` and
  resumes next, then `PRINT UBOUND(st); st(1)`
- **THEN** the handler prints 10, then ` 5  3 ` is printed

#### Scenario: In a procedure
- **WHEN** a SUB with a parameter `p() AS LONG` does `REDIM _RETAIN p(0 TO 5) AS LONG` and is called for an array
  `w(1 TO 3)` holding 1, 2, 3
- **THEN** `w(0)` to `w(5)` are 0, 1, 2, 3, 0, 0 after the call

#### Scenario: Member array
- **WHEN** a type has `v(1 TO 3) AS LONG`, `DIM x AS` that type`: x.v(2) = 5: REDIM _RETAIN x.v(1 TO 3) AS LONG:
  PRINT x.v(2)` runs
- **THEN** it prints ` 5 `
