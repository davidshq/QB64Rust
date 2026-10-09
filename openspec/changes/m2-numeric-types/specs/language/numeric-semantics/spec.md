## ADDED Requirements

### Requirement: Numeric types
The numeric types SHALL be INTEGER, LONG, `_INTEGER64`, SINGLE, DOUBLE, `_FLOAT`, `_BYTE`, `_OFFSET`, `_BIT` and
`_BIT * n` (n a number literal from 1 to 64), and the `_UNSIGNED` form of each integer type, with QB64pe's
`AS` names and type suffixes (`%%`, `~%%`, `~%`, `~&`, `~&&`, `%&`, `~%&`, `` ` ``, `` `n ``, `` ~` ``,
`` ~`n ``). Each SHALL hold the values of its width and signedness; `_OFFSET` is 64 bits wide. A name with each
suffix SHALL be a variable of its own. As in the old compiler: `_UNSIGNED` before `STRING` SHALL be ignored in a
declaration (on a parameter it is an error, `language/procedures`);
`_UNSIGNED` before a floating-point type, `AS _UNSIGNED` alone, a `_BIT` width of 0, above 64 or not a number
literal, a suffix and an `AS` clause on one name, and `LEN` of a `_BIT` variable SHALL be compile errors. A
`_BIT` value SHALL be read as an `_INTEGER64` (an `_UNSIGNED _BIT * 64` above 2^63 prints negative, as in the old
compiler). `_MEM` SHALL stay "not supported yet".

#### Scenario: Unsigned byte
- **WHEN** `DIM b AS _UNSIGNED _BYTE: b = 255: PRINT b; LEN(b)` runs
- **THEN** it prints ` 255  1 `

#### Scenario: Suffix names the type
- **WHEN** `x~% = 65535: PRINT x~%` runs
- **THEN** it prints ` 65535 `

#### Scenario: One name, two suffixes
- **WHEN** `n%% = 1: n~%% = 2: PRINT n%%; n~%%` runs
- **THEN** it prints ` 1  2 `

#### Scenario: Unsigned 64-bit bit field read as _INTEGER64
- **WHEN** `DIM u AS _UNSIGNED _BIT * 64: u = -1: PRINT u` runs
- **THEN** it prints `-1 `

#### Scenario: LEN of a bit field
- **WHEN** `DIM t AS _BIT * 4: PRINT LEN(t)` is compiled
- **THEN** it is a compile error

#### Scenario: _MEM
- **WHEN** a program has `DIM m AS _MEM`
- **THEN** the compiler reports it "not supported yet"

### Requirement: Arithmetic with unsigned, narrow and bit operands
An operation on integer operands SHALL be computed in the type C++ gives the old compiler's generated expression
(operands narrower than 32 bits widened to a 32-bit signed integer, then C++'s usual arithmetic conversions on the
operands' storage types), and its result SHALL be printed as the old compiler believes it: `_INTEGER64` unless both
operands are `_UNSIGNED _INTEGER64`. Comparisons SHALL compare in that computed type.

#### Scenario: Narrow operands are widened
- **WHEN** `b~%% = 255: x~% = 65535: PRINT b~%% + 1; x~% * 2` runs
- **THEN** it prints ` 256  131070 `

#### Scenario: Every operator and type pair
- **WHEN** the differential programs for each operator (`testing/differential-tests`) are built and run
- **THEN** every line of their output equals the old compiler's recorded output

### Requirement: _OFFSET operands
When an operand is an `_OFFSET`, `+`, `-`, `\`, `MOD` and the bit-wise operators SHALL compute on integers; `*`
with a floating-point operand and `/` SHALL compute in `_FLOAT` and round half to even to an integer; the result
SHALL be an `_OFFSET`, unsigned unless an `_OFFSET` operand is signed. Comparisons SHALL be as for other integers.
`^` with an `_OFFSET` operand SHALL be a compile error.

#### Scenario: Power of an offset
- **WHEN** `DIM o AS _OFFSET: PRINT o ^ 2` is compiled
- **THEN** it is a compile error

#### Scenario: Division keeps the type
- **WHEN** `DIM o AS _UNSIGNED _OFFSET: o = 7: PRINT o / 2` runs
- **THEN** the output equals the old compiler's (an integer, rounded half to even)

### Requirement: _BIT stores
A store into a `_BIT * n` variable SHALL keep the value's low `n` bits; for a signed `_BIT * n` the result SHALL
then be sign-extended from bit `n - 1`. A `_BIT * n` scalar SHALL have storage of its own (4 bytes for n up to 32, 8
bytes above), so that two scalars never share bytes (`DIVERGENCES.md` D-009). A `_BIT` array SHALL be reported "not
supported yet". In arithmetic and comparisons a `_BIT * n` of up to 32 bits SHALL count as its 32-bit storage
type (signed or unsigned), as in the old compiler.

#### Scenario: Unsigned bit field in arithmetic
- **WHEN** `DIM u AS _UNSIGNED _BIT * 3: u = 7: PRINT u * 1000000000; u > -1` runs
- **THEN** it prints ` 2705032704  0 ` (computed in 32-bit unsigned, as the old compiler)

#### Scenario: Signed bit field
- **WHEN** `DIM b AS _BIT * 3: b = 5: PRINT b` runs
- **THEN** it prints `-3 `

#### Scenario: Unsigned bit field
- **WHEN** `DIM u AS _UNSIGNED _BIT * 3: u = 13: PRINT u` runs
- **THEN** it prints ` 5 `

#### Scenario: Single bit
- **WHEN** `DIM t AS _BIT: t = 1: PRINT t` runs
- **THEN** it prints `-1 `

#### Scenario: Wide bit fields do not overlap
- **WHEN** `DIM a AS _BIT * 33, b AS _BIT * 33: a = 5: b = -1: PRINT a` runs
- **THEN** it prints ` 5 ` (the old compiler prints ` 4294967295 `)

## MODIFIED Requirements

### Requirement: Integer literal typing
An integer literal without a type suffix SHALL have the smallest of INTEGER, LONG and `_INTEGER64` that holds its
value. One exception, as in the old compiler: `-2147483648` SHALL be `_INTEGER64`, not LONG.

A decimal literal with the suffix of an integer type (`%`, `&`, `&&`, `%%`, `~%%`, `~%`, `~&`, `~&&`, and the
`_BIT` suffixes `` ` ``, `` `n ``, `` ~` ``, `` ~`n ``) SHALL be believed to have the suffix's type, in range or not,
but SHALL hold its value as written, as the old compiler emits it (its digits, with a unary minus directly before
it as part of it, `ll` added for `&&` and `ull` for `~&&`), in the type C++ gives that text: a `~&&` literal in an
unsigned 64-bit integer (`-1~&&` holds 2^64-1); an `&&` literal in a signed 64-bit one (`2147483647&& + 1` is
2147483648, where `2147483647& + 1` wraps to -2147483648); any other in a signed 32-bit one if the value fits,
else a signed 64-bit one; and beyond the signed 64-bit range in an unsigned 64-bit one. The held value
SHALL be converted to the believed type only where the old compiler's generated code casts to it: by `PRINT` and
`STR$`, and by a built-in whose result keeps its argument's type (`ABS`). Everywhere else (a store, an argument, an
operator, a comparison, a `CASE` item, a `FOR` limit, a condition, an array bound) the held value SHALL be used,
converted only to that place's own type: `PRINT 300~%%` and `PRINT (300~%%)` print 44, while `300~%% + 0` is 300,
`l& = 300~%%` stores 300, `-1~& < 0` is true and `4294967295~& + 1` is 4294967296. `HEX$`, `OCT$` and `_BIN$`
SHALL see the held value, with the believed type's width for a negative one (`HEX$(300~%%)` is `12C`,
`HEX$(-1~%%)` `FF`), except a believed 64-bit type, which keeps `builtin-functions`' rule for a 64-bit argument
that is not a place, as the old compiler (`HEX$(-1~&&)` is `""`, `HEX$(-2~&&)` `FFFE`). A `_BIT` type is read as `_INTEGER64`, so a `_BIT`-suffixed literal is never converted
(`` PRINT 9`3 `` prints 9). (`DECISIONS.md`, 2026-10-08: as QB64pe; `verification\v21_a_literals`,
`v21_f_literal_uses`.)

An `&H`, `&O` or `&B` literal with an integer suffix SHALL hold its bits read with the type's signedness when they
fit the type's width (`&HFF%%` is -1 everywhere); an unsigned one wider than its type SHALL hold its whole value
(`&H1FF~%%` prints 255, `&H1FF~%% + 0` is 511), and a signed one wider than its type SHALL be a compile error
("Overflow", as the old compiler; `verification\v21_f_radix`, `v21_x37`–`x41`). A literal beyond 64 bits SHALL be a
compile error (the old compiler's C++ build fails). `%&` and `~%&` after a number, and a suffix of an integer type
on a number with a decimal point or an exponent, SHALL be compile errors, as in the old compiler.

#### Scenario: Small literal is INTEGER
- **WHEN** a program prints `-3`
- **THEN** the value is typed INTEGER and prints as `-3 `

#### Scenario: Literal beyond INTEGER range
- **WHEN** a program prints `40000`
- **THEN** the value is typed LONG and prints as ` 40000 `

#### Scenario: LONG's minimum is `_INTEGER64`
- **WHEN** a program prints `-2147483648`
- **THEN** the value is typed `_INTEGER64` and prints as `-2147483648 `

#### Scenario: Unsigned literal
- **WHEN** a program prints `4294967295~&` and `255~%%`
- **THEN** it prints ` 4294967295 ` and ` 255 `

#### Scenario: Hexadecimal literal read with the type's signedness
- **WHEN** a program prints `&HFF%%` and `&HFF%% + 0`
- **THEN** it prints `-1 ` both times

#### Scenario: Suffixed literal out of range
- **WHEN** a program prints `300~%%; -1~&; 40000%; 300~%% + 0; -1~& + 0` and then runs `l& = 300~%%: PRINT l&`
- **THEN** it prints ` 44  4294967295 -25536  300 -1 ` and ` 300 `

#### Scenario: Suffixed literal held as written in an operation
- **WHEN** a program prints `4294967295~& + 1; -1~&& + 0; -1~&& > 0; 255~%% = -1%%`
- **THEN** it prints ` 4294967296 -1 -1  0 `

#### Scenario: Bit literal
- **WHEN** a program prints ``1`; 9`3; 9`3 + 0; HEX$(-1`5)``
- **THEN** it prints ` 1  9  9 FF`

#### Scenario: Wide radix literal
- **WHEN** a program prints `&H1FF~%%; &H1FF~%% + 0`
- **THEN** it prints ` 255  511 `

#### Scenario: Signed radix literal wider than its type
- **WHEN** a program contains `PRINT &H1FF%%`
- **THEN** it is a compile error

### Requirement: Storing into an integer variable
Assigning a floating-point value to an integer variable SHALL round half to even. For a target of 16 bits or
fewer (INTEGER, `_BYTE` and their unsigned forms), the value SHALL first be narrowed to SINGLE and then rounded;
for every wider target and for every `_BIT * n`, whatever its width, it SHALL be rounded from `_FLOAT` to a 64-bit
integer (above the `_INTEGER64` range, to the unsigned 64-bit value), as the old compiler does. After rounding, or when an integer value is wider than the
target, the value SHALL be truncated to the target width (and interpreted with the target's signedness) without
an error.

#### Scenario: Half to even
- **WHEN** a program assigns `2.5` and `3.5` to an INTEGER variable and prints it
- **THEN** it prints ` 2 ` and ` 4 `

#### Scenario: INTEGER target rounds the SINGLE value
- **WHEN** a program executes `d# = 2.5000001: x% = d#: l& = d#`
- **THEN** `x%` is 2 and `l&` is 3

#### Scenario: Unsigned and bit targets round from their own path
- **WHEN** a program executes `d# = 2.5000001: u~% = d#: b16 = d#` with `b16 AS _BIT * 16`
- **THEN** `u~%` is 2 (from SINGLE) and `b16` is 3 (from `_FLOAT`)

#### Scenario: Out-of-range store truncates
- **WHEN** a program executes `x% = 70000: PRINT x%`
- **THEN** it prints ` 4464 `

#### Scenario: Negative into unsigned
- **WHEN** a program executes `x~% = -1: u~&& = -1: PRINT x~%; u~&&`
- **THEN** it prints ` 65535  18446744073709551615 `

#### Scenario: Signed byte wraps
- **WHEN** a program executes `b%% = 200: PRINT b%%`
- **THEN** it prints `-56 `

#### Scenario: Every conversion
- **WHEN** the differential store programs (every type stored into every type) are built and run
- **THEN** every line of their output equals the old compiler's recorded output

### Requirement: Logical operators
`NOT`, `AND`, `OR`, `XOR`, `EQV` and `IMP` SHALL work bit by bit on integers, in 32 bits for INTEGER and LONG
operands and in 64 bits with an `_INTEGER64`; a floating-point operand SHALL first be rounded half to even to a
64-bit integer. `_ANDALSO` and `_ORELSE` SHALL give -1 or 0 and SHALL NOT evaluate their right operand when the
left one decides the result. `_NEGATE` SHALL give -1 for 0 and 0 otherwise. The operands of `_ANDALSO`, `_ORELSE`
and `_NEGATE` SHALL be rounded like those of the other logical operators. A string operand SHALL be a compile
error. `IMP`, like every other binary operator, SHALL be left-associative: `a IMP b IMP c` SHALL compute
`(a IMP b) IMP c` (the old compiler computes `a OR b OR c`, `DIVERGENCES.md` D-005).

#### Scenario: Short-circuit operands are rounded
- **WHEN** `PRINT 0.4 _ANDALSO 1; _NEGATE 0.4` runs
- **THEN** it prints ` 0 -1 `

#### Scenario: Float operands are rounded
- **WHEN** `PRINT 1.5 AND 3; 2.5 OR 0; NOT 1.5; 5 XOR 3; 5 EQV 3; 5 IMP 3` runs
- **THEN** it prints ` 2  2 -3  6 -7 -5 `

#### Scenario: Short-circuit operators
- **WHEN** `PRINT 1 _ANDALSO 2; 0 _ORELSE 0; _NEGATE 0; _NEGATE 5` runs
- **THEN** it prints `-1  0 -1  0 `

#### Scenario: IMP chain
- **WHEN** `PRINT 5 IMP 3 IMP 0` runs
- **THEN** it prints ` 4 ` (`5 IMP 3` is -5, `-5 IMP 0` is 4; the old compiler prints ` 7 `)

### Requirement: Integer division and MOD
`\` and `MOD` SHALL round floating-point operands half to even to integers first, then divide truncating toward
zero; `MOD` SHALL give the remainder with the sign of the dividend. A divisor of 0 SHALL raise error 11, which is
fatal (not trappable; `DIVERGENCES-QB45.md` Q-003). The smallest value of a signed type divided by -1 SHALL raise
error 6 (Overflow, trappable) with `\` and SHALL give 0 with `MOD` (the old compiler's program crashes,
`DIVERGENCES.md` D-006).

#### Scenario: Signs and rounding
- **WHEN** `PRINT 7 \ 2; -7 \ 2; 7.5 \ 2; -7 MOD 3; 7.5 MOD 2` runs
- **THEN** it prints ` 3 -3  4 -1  0 `

#### Scenario: Smallest LONG divided by -1
- **WHEN** `x& = -2147483648: m& = -1` and, under an `ON ERROR` handler that prints `ERR` and resumes next,
  `PRINT x& \ m&` and then `PRINT x& MOD m&` run
- **THEN** the handler prints 6 for the first, and the second prints ` 0 `
