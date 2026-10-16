# Spec Delta

## MODIFIED Requirements

### Requirement: Numeric types
The numeric types SHALL be INTEGER, LONG, `_INTEGER64`, SINGLE, DOUBLE, `_FLOAT`, `_BYTE`, `_OFFSET`, `_BIT` and
`_BIT * n` (n a number literal from 1 to 64), and the `_UNSIGNED` form of each integer type, with QB64pe's
`AS` names and type suffixes (`%%`, `~%%`, `~%`, `~&`, `~&&`, `%&`, `~%&`, `` ` ``, `` `n ``, `` ~` ``,
`` ~`n ``). Each SHALL hold the values of its width and signedness; `_OFFSET` is 64 bits wide. A name with each
suffix SHALL be a variable of its own. As in the old compiler: `_UNSIGNED` before `STRING` SHALL be ignored in a
declaration (on a parameter it is an error, `language/procedures`);
`_UNSIGNED` before a floating-point type, `AS _UNSIGNED` alone, a `_BIT` width of 0, above 64 or not a number
literal, a suffix and an `AS` clause on one name, and `LEN` of a `_BIT` variable SHALL be compile errors. A
`_BIT` value SHALL be printed as an `_INTEGER64` (an `_UNSIGNED _BIT * 64` above 2^63 prints negative, as in the old
compiler). `_MEM` is a type of its own (`language/mem-blocks`).

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
- **THEN** it compiles, and `LEN(m)` is 56, as with the old compiler
