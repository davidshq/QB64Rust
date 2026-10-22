# language/fixed-length-strings Specification

## Purpose
Fixed-length strings (`STRING * n`): how they are declared, what they hold at the start, how assignment fits a
value into them, and how they behave as values, array elements, `TYPE` members and arguments, as measured with
`qb64pe.exe`.

## Requirements

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

### Requirement: A suffixed name of a fixed-length string array
`name$(i)` SHALL be the element of the array of that name declared `AS STRING * n`, as a value and as a place,
and `name$` SHALL be that array where a whole array is named (`UBOUND(name$)`), in the main module and in a
procedure that shares it (`verification\v52_x40`–`x48`; QB64pe `source\utilities\hash.bas` relies on it). It
SHALL NOT make another, implicit array. `DIM name$(n)` beside such an array SHALL be a compile error (`v52_x45`).

#### Scenario: The $ name reads the element
- **WHEN** `REDIM a(1 TO 3) AS STRING * 4: a(2) = "ab": PRINT "["; a$(2); "]"; UBOUND(a$)` runs
- **THEN** it prints `[ab  ] 3`

#### Scenario: In a SUB, the shared array
- **WHEN** `DIM SHARED h(5) AS STRING * 8` is declared, and a SUB runs `h(1) = "x": PRINT LEN(h$(1))`
- **THEN** it prints ` 8 `

#### Scenario: The $ name as a place
- **WHEN** `DIM a(3) AS STRING * 2: a$(1) = "zzz": PRINT "["; a(1); "]"` runs
- **THEN** it prints `[zz]`

#### Scenario: An array of its own beside it
- **WHEN** a program has `DIM a(3) AS STRING * 4` and then `DIM a$(3)`
- **THEN** it is a compile error

### Requirement: A suffixed name of a fixed-length string variable
`name$` SHALL be the variable of that name declared `AS STRING * n`, as a value and as a place, in the main module
and in a procedure that shares it (`verification\v52_x39`, `x43`, `x44`; QB64pe `source\utilities\file.bas`
relies on it for `pathsep$`).

#### Scenario: The $ name reads and sets the variable
- **WHEN** `DIM s AS STRING * 4: s = "ab": PRINT "["; s$; "]": s$ = "x": PRINT "["; s; "]"` runs
- **THEN** it prints `[ab  ]` and `[x   ]`

#### Scenario: In a SUB, the shared variable
- **WHEN** `DIM SHARED pathsep AS STRING * 1` is declared, and a SUB runs `pathsep$ = "\": PRINT LEN(pathsep$)`
- **THEN** it prints ` 1 `, and `pathsep` holds `\` after the call

### Requirement: Declaring the $ name of a fixed-length string variable
`DIM name$`, `REDIM name$` and, in a procedure, `STATIC name$` beside a variable `name` declared `AS STRING * n`
SHALL be accepted and SHALL declare nothing new, whichever of `DIM` and `STATIC` declared `name`: `name$` stays the
fixed-length variable (`verification\v52_x49`, `x69`, `x70`, `x73`, `x74`). `SHARED name$` in a procedure SHALL bind the main module's variable-length `name$`, not the
fixed-length variable (`x71`).

#### Scenario: DIM of the $ name beside it
- **WHEN** `DIM s AS STRING * 4: DIM s$: s = "f": s$ = "v": PRINT "["; s; "]["; s$; "]"` runs
- **THEN** it prints `[v   ][v   ]`

#### Scenario: REDIM of the $ name beside it
- **WHEN** `DIM s AS STRING * 4: REDIM s$: s$ = "abcdefg": PRINT "["; s; "]["; s$; "]"` runs
- **THEN** it prints `[abcd][abcd]`
