## Purpose

Fixed-length strings (`STRING * n`): how they are declared, what they hold at the start, how assignment fits a
value into them, and how they behave as values, array elements, `TYPE` members and arguments, as measured with
`qb64pe.exe`.

## ADDED Requirements

### Requirement: Fixed-length string declarations
`DIM name AS STRING * n`, with `n` a number literal or the name of a numeric constant, SHALL declare a string of
exactly `n` bytes, in every storage class a scalar variable may have. As in the old compiler, `n` SHALL be read as a
32-bit integer: a result of 0 or below SHALL be a compile error (QB64pe: "Cannot create a fixed string of length
0", or a failed C++ build for a negative result), any other result is the length (4294967297 gives 1;
`DECISIONS.md`, 2026-10-08); an expression, a float or a negative literal after `*` SHALL be a compile error. The
suffix form `name$n` SHALL declare the same type, for `DIM` and for implicit variables, and SHALL be a variable other
than `name$`. `_UNSIGNED STRING * n` SHALL mean `STRING * n`. Every byte SHALL start as NUL (0), also for a local on
each call (`DIVERGENCES-QB45.md` Q-005).

#### Scenario: Initial bytes
- **WHEN** `DIM f AS STRING * 4: PRINT LEN(f); ASC(f, 1); ASC(f, 4)` runs
- **THEN** it prints ` 4  0  0 `

#### Scenario: Length of zero
- **WHEN** `DIM f AS STRING * 0` is compiled
- **THEN** it is a compile error

#### Scenario: Length read as 32 bits
- **WHEN** `DIM f AS STRING * 4294967297: PRINT LEN(f)` runs
- **THEN** it prints ` 1 `, as with the old compiler

#### Scenario: Suffix form
- **WHEN** `DIM p$3: p$3 = "abcdef": p$ = "x": PRINT p$3; LEN(p$3); p$` runs
- **THEN** it prints `abc 3 x`

### Requirement: Assignment to a fixed-length string
Storing a string into a fixed-length string SHALL copy its first `n` bytes and fill any remaining bytes with
spaces (32); storing `""` gives `n` spaces. Reading the variable SHALL give all `n` bytes, padding included, also
in comparisons and as a `SELECT CASE` selector. `MID$` as a statement SHALL write only inside the `n` bytes.

#### Scenario: Short value padded
- **WHEN** `DIM f AS STRING * 4: f = "ab": PRINT "["; f; "]"; ASC(f, 3)` runs
- **THEN** it prints `[ab  ] 32 `

#### Scenario: Long value cut
- **WHEN** `DIM f AS STRING * 4: f = "abcdef": PRINT f` runs
- **THEN** it prints `abcd`

#### Scenario: Compared with its padding
- **WHEN** `DIM f AS STRING * 4: f = "ab": PRINT f = "ab"; f = "ab  "` runs
- **THEN** it prints ` 0 -1 `

### Requirement: Fixed-length strings in arrays and types
A static array (`DIM a(n) AS STRING * k`) and a `TYPE` member (`m AS STRING * k`) SHALL hold fixed-length strings
with the same rules; each element and member SHALL start as NUL bytes. A member SHALL take `k` bytes of its type's
layout.

#### Scenario: Member in the layout
- **WHEN** `TYPE rec: id AS LONG: nm AS STRING * 6: END TYPE` (as a block) is declared, `DIM r AS rec`, `r.nm =
  "bob"` runs
- **THEN** `PRINT LEN(r); "["; r.nm; "]"` prints ` 10 [bob   ]`

#### Scenario: Array of fixed-length strings
- **WHEN** `DIM a(2) AS STRING * 3: a(1) = "x"` runs
- **THEN** `PRINT ASC(a(0), 1); "["; a(1); "]"` prints ` 0 [x  ]`

### Requirement: Fixed-length strings as arguments
A fixed-length string passed to a built-in function SHALL be a string of its `n` bytes. A fixed-length string
variable, element or member passed to a `STRING` parameter SHALL be passed by reference, also in parentheses: the
procedure sees its `n` bytes, and each store to the parameter reaches it cut and padded to `n`, as the old compiler
does. A parameter declared `STRING * n` (or `name$n`) is in the procedures spec (a `STRING` parameter whose `LEN` is
n).

#### Scenario: Passed to a STRING parameter
- **WHEN** a `STRING * 5` variable holding `"ab"` is passed to `SUB s (t AS STRING)`, which prints `LEN(t)` and
  assigns `t = "longer text"`, and the variable is printed after the call
- **THEN** the SUB prints ` 5 ` and the variable is `longe`

#### Scenario: In parentheses, as a member, as an element
- **WHEN** the same SUB is called as `s (f)`, `s r.nm` and `s a(1)`, each a `STRING * 5`
- **THEN** each is `longe` afterwards

#### Scenario: Built-in argument
- **WHEN** `DIM f AS STRING * 5: f = "ab": PRINT LEN(RTRIM$(f)); UCASE$(f)` runs
- **THEN** it prints ` 2 AB   `
