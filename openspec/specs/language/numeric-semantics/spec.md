# language/numeric-semantics Specification

## Purpose
The numeric rules of the language as QB64Rust implements them: how literals are typed, in which width arithmetic
is computed, what happens on overflow, how floating-point values become integers. The reference is the observed
behaviour of `qb64pe.exe` 4.7.0 (`16f629784e`) in its **default** build; each intentional difference is listed in
`DIVERGENCES.md`. Every scenario is a measured example and is pinned by a test.

## Requirements

### Requirement: Integer literal typing
An integer literal without a type suffix SHALL have the smallest of INTEGER, LONG and `_INTEGER64` that holds its
value. A literal with a suffix (`%`, `&`, `&&`) SHALL have the suffix's type. A unary minus directly before an
integer literal SHALL be part of the literal for this purpose. One exception, as in the old compiler: `-2147483648`
SHALL be `_INTEGER64`, not LONG.

#### Scenario: Small literal is INTEGER
- **WHEN** a program prints `-3`
- **THEN** the value is typed INTEGER and prints as `-3 `

#### Scenario: Literal beyond INTEGER range
- **WHEN** a program prints `40000`
- **THEN** the value is typed LONG and prints as ` 40000 `

#### Scenario: LONG's minimum is `_INTEGER64`
- **WHEN** a program prints `-2147483648`
- **THEN** the value is typed `_INTEGER64` and prints as `-2147483648 `

### Requirement: Float literal typing
A floating-point literal without a suffix SHALL be SINGLE if it has at most 7 significant digits and is in SINGLE
range, DOUBLE if it has at most 16, and `_FLOAT` otherwise; `!`, `#`, `##` and the exponent letters `E`, `D`, `F`
SHALL force SINGLE, DOUBLE and `_FLOAT`. A SINGLE literal's value SHALL be the nearest DOUBLE to its decimal text
(not the nearest SINGLE widened), as in the old compiler.

#### Scenario: Literal printed with its type's format
- **WHEN** a program prints `1.5E2`, `2.5` and `.5`
- **THEN** the output matches the old compiler's (`134_float_literal`)

#### Scenario: SINGLE literal stored in a DOUBLE
- **WHEN** a program executes `d# = 0.1!: PRINT d#`
- **THEN** it prints the DOUBLE 0.1 (` .1 `), not the widened SINGLE

### Requirement: Integer arithmetic width
`+`, `-` and `*` on integer operands SHALL be computed in 32 bits when both operands are at most 32 bits wide, and
in 64 bits when either is `_INTEGER64`. The result SHALL wrap in two's complement on overflow; no error is raised.
The same SHALL hold in constant folding.

#### Scenario: INTEGER sum exceeds INTEGER range
- **WHEN** a program executes `i% = 32767: PRINT i% + 1`
- **THEN** it prints ` 32768 ` (computed in 32 bits)

#### Scenario: LONG overflow wraps
- **WHEN** a program executes `x& = 2147483647: PRINT x& + 1`
- **THEN** it prints `-2147483648 ` (and `x& + 1 > x&` is false once comparisons exist: `v11_wrap_o2`)

#### Scenario: Literal overflow wraps in folding
- **WHEN** a program prints `2147483647 * 2`
- **THEN** it prints `-2 `

#### Scenario: _INTEGER64 overflow wraps
- **WHEN** a program executes `x&& = 9223372036854775807: PRINT x&& + 1`
- **THEN** it prints `-9223372036854775808 `

### Requirement: Division
`/` SHALL produce a floating-point result. When both operands are integers it SHALL be computed in `_FLOAT`
(extended precision) and printed with DOUBLE's 16 significant digits. When an operand is SINGLE or DOUBLE, the
result SHALL have the wider float type of the float operands.

#### Scenario: Integer division prints 16 digits
- **WHEN** a program prints `1 / 3`
- **THEN** it prints ` .3333333333333333 `

#### Scenario: DOUBLE divided by integer
- **WHEN** a program executes `d# = 2.5: PRINT d# / 3`
- **THEN** the output matches the old compiler's (`s04_division`)

### Requirement: Storing into an integer variable
Assigning a floating-point value to an integer variable SHALL round half to even. For an INTEGER (16-bit or
narrower) target, the value SHALL first be narrowed to SINGLE and then rounded. After rounding, or when an integer
value is wider than the target, the value SHALL be truncated to the target width without an error.

#### Scenario: Half to even
- **WHEN** a program assigns `2.5` and `3.5` to an INTEGER variable and prints it
- **THEN** it prints ` 2 ` and ` 4 `

#### Scenario: INTEGER target rounds the SINGLE value
- **WHEN** a program executes `d# = 2.5000001: x% = d#: l& = d#`
- **THEN** `x%` is 2 and `l&` is 3

#### Scenario: Out-of-range store truncates
- **WHEN** a program executes `x% = 70000: PRINT x%`
- **THEN** it prints ` 4464 `

### Requirement: Printing numbers
`PRINT` of a numeric item SHALL output `STR$` of the value in the type it was computed in, followed by one space.
`STR$` formatting itself is libqb's and is not respecified here.

#### Scenario: Items separated by semicolons
- **WHEN** a program executes `PRINT "["; 5; -3; 1.5; "]"`
- **THEN** it prints `[ 5 -3  1.5 ]`

### Requirement: Comparisons
`=`, `<>`, `<`, `>`, `<=`, `>=` SHALL give -1 when true and 0 when false, typed LONG. Two strings SHALL compare
byte by byte; a string with a number SHALL be a compile error. When both operands are floating point, both SHALL
be compared at the narrower of their two float types (so a SINGLE variable equals the literal it was set from).

#### Scenario: Numbers and strings
- **WHEN** `PRINT 3 > 2; 2 > 3; "a" < "b"` runs
- **THEN** it prints `-1  0 -1 `

#### Scenario: SINGLE against a literal
- **WHEN** `a! = 2.1: PRINT a! = 2.1; 2.1 = a!` runs
- **THEN** it prints `-1 -1 `

### Requirement: Logical operators
`NOT`, `AND`, `OR`, `XOR`, `EQV` and `IMP` SHALL work bit by bit on integers, in 32 bits for INTEGER and LONG
operands and in 64 bits with an `_INTEGER64`; a floating-point operand SHALL first be rounded half to even to a
64-bit integer. `_ANDALSO` and `_ORELSE` SHALL give -1 or 0 and SHALL NOT evaluate their right operand when the
left one decides the result. `_NEGATE` SHALL give -1 for 0 and 0 otherwise. The operands of `_ANDALSO`, `_ORELSE`
and `_NEGATE` SHALL be rounded like those of the other logical operators. A string operand SHALL be a compile
error. An `IMP` whose left operand is an `IMP` SHALL be "not supported yet" (the old compiler computes `a IMP b IMP
c` as `a OR b OR c`).

#### Scenario: Short-circuit operands are rounded
- **WHEN** `PRINT 0.4 _ANDALSO 1; _NEGATE 0.4` runs
- **THEN** it prints ` 0 -1 `

#### Scenario: Float operands are rounded
- **WHEN** `PRINT 1.5 AND 3; 2.5 OR 0; NOT 1.5; 5 XOR 3; 5 EQV 3; 5 IMP 3` runs
- **THEN** it prints ` 2  2 -3  6 -7 -5 `

#### Scenario: Short-circuit operators
- **WHEN** `PRINT 1 _ANDALSO 2; 0 _ORELSE 0; _NEGATE 0; _NEGATE 5` runs
- **THEN** it prints `-1  0 -1  0 `

### Requirement: Integer division and MOD
`\` and `MOD` SHALL round floating-point operands half to even to integers first, then divide truncating toward
zero; `MOD` SHALL give the remainder with the sign of the dividend. A divisor of 0 SHALL raise error 11, which is
fatal (not trappable). The smallest LONG or `_INTEGER64` divided by -1 is left unspecified for now: the old
compiler's program crashes (`study/00` §6, "Fix").

#### Scenario: Signs and rounding
- **WHEN** `PRINT 7 \ 2; -7 \ 2; 7.5 \ 2; -7 MOD 3; 7.5 MOD 2` runs
- **THEN** it prints ` 3 -3  4 -1  0 `

### Requirement: Power
`^` SHALL be left-associative at run time and computed in extended precision; its result SHALL be typed as the old
compiler types it (an INTEGER operand counts as SINGLE, a LONG as DOUBLE, an `_INTEGER64` as `_FLOAT`; the widest
wins). A negative base with a non-integer exponent SHALL raise error 5.

#### Scenario: Associativity and typing
- **WHEN** `PRINT 2 ^ 3 ^ 2; 2 ^ 0.5` runs
- **THEN** it prints ` 64  1.414214 `
