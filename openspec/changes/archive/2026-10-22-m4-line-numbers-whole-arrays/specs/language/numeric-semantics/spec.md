# Spec Delta

## MODIFIED Requirements

### Requirement: Integer literal typing
An integer literal without a type suffix SHALL have the smallest of INTEGER, LONG and `_INTEGER64` that holds its
value. Two exceptions, as in the old compiler: `-2147483648` SHALL be `_INTEGER64`, not LONG; and a literal
without a minus above 9223372036854775807, up to 18446744073709551615, SHALL be `_UNSIGNED _INTEGER64`, as if
written with `~&&` (`qb64pe.bas` adds that suffix).

A decimal literal with the suffix of an integer type (`%`, `&`, `&&`, `%%`, `~%%`, `~%`, `~&`, `~&&`, and the
`_BIT` suffixes `` ` ``, `` `n ``, `` ~` ``, `` ~`n ``) SHALL be believed to have the suffix's type, in range or not,
but SHALL hold its value as written, as the old compiler emits it (its digits, with a unary minus directly before
it as part of it, `ll` added for a signed type wider than 32 bits, `&&` and `` `n `` with n above 32, and `ull`
for an unsigned one, `~&&` and `` ~`n `` with n above 32), in the type C++ gives that text: an `ull` literal in an
unsigned 64-bit integer (`-1~&&` and `` -1~`40 `` hold 2^64-1); an `ll` literal in a signed 64-bit one (`2147483647&& + 1` is
2147483648, where `2147483647& + 1` wraps to -2147483648); any other in a signed 32-bit one if the value fits,
else a signed 64-bit one; and beyond the signed 64-bit range in an unsigned 64-bit one. The held value
SHALL be converted to the believed type only where the old compiler's generated code casts to it: by `PRINT` and
`STR$`, and by a built-in whose result keeps its argument's type (`ABS`). Everywhere else (a store, an argument, an
operator, a comparison, a `CASE` item, a `FOR` limit, a condition, an array bound) the held value SHALL be used,
converted only to that place's own type: `PRINT 300~%%` and `PRINT (300~%%)` print 44, while `300~%% + 0` is 300,
`l& = 300~%%` stores 300, `-1~& < 0` is true and `4294967295~& + 1` is 4294967296. `HEX$`, `OCT$` and `_BIN$`
SHALL see the held value, with the believed type's width for a negative one (`HEX$(300~%%)` is `12C`,
`HEX$(-1~%%)` `FF`), except a believed 64-bit type, which keeps `builtin-functions`' rule for a 64-bit argument
that is not a place, as the old compiler (`HEX$(-1~&&)` is `""`, `HEX$(-2~&&)` `FFFE`). A `_BIT` type is printed as `_INTEGER64`, so a `_BIT`-suffixed literal is never converted
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

#### Scenario: Literal above the _INTEGER64 range
- **WHEN** a program prints `18446744073709551615`, `18446744073709551614 + 1` and `9223372036854775808 / 2`,
  and stores `18446744073709551615` into a `~&&`, an `&&` and a `DOUBLE` variable
- **THEN** it prints ` 18446744073709551615 `, `-1 ` (the sum takes the type an operation of `~&&` and INTEGER
  has, as `18446744073709551614~&& + 1` does) and ` 4.611686018427388D+18 `, and the variables hold
  18446744073709551615, -1 and 1.844674407370955D+19, as recorded (`verification\v22_b_print`, `v25_signs`,
  `v54_b_unsigned_literals`)

#### Scenario: Literal beyond 64 bits
- **WHEN** a program contains `PRINT 18446744073709551616`
- **THEN** it is a compile error

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
