# _CAST

`_CAST` converts a number to the numeric type it names, as C does: an integer type keeps only the low bits, and a
floating-point value is cut toward zero, not rounded. It raises no error.

## Syntax

<!-- begin generated: syntax -->
```text
_CAST(numericalType, numericalValue)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `numericalType`: the type, written as in `DIM`: `_BYTE`, `INTEGER`, `LONG`, `_INTEGER64`, each with or without
  `_UNSIGNED`, `_OFFSET`, `_UNSIGNED _OFFSET`, `SINGLE`, `DOUBLE` or `_FLOAT`.
- `numericalValue`: a number of any numeric type.

## Result

A value of the type named.

## Behavior

An integer value converted to a narrower integer type keeps its low bits: `_CAST(_BYTE, 255)` is -1,
`_CAST(_UNSIGNED _BYTE, -1)` is 255 and `_CAST(INTEGER, 90000)` is 24464.
<!-- src: language/builtin-functions "_CAST"; verification\v49_a_cast -->

A floating-point value converted to an integer type is cut toward zero: `_CAST(INTEGER, 2.5)` is 2,
`_CAST(INTEGER, -1.5)` is -1 and `_CAST(LONG, 0.9999)` is 0. `CINT` and `CLNG` round instead.
<!-- src: language/builtin-functions "_CAST"; verification\v49_a_cast -->

The result has the type named, which shows when it is printed: `HEX$(_CAST(_BYTE, -1))` is `FF`,
`HEX$(_CAST(INTEGER, -1))` is `FFFF` and `_CAST(SINGLE, 1 / 3)` prints `.3333333`. In arithmetic it is widened as
any value of its type: `_CAST(_BYTE, 100) + _CAST(_BYTE, 100)` is 200.
<!-- src: language/builtin-functions "_CAST"; verification\v49_a_cast -->

A floating-point value outside an integer type's range, NaN and infinity give what x86-64's conversion gives: for
LONG and the narrower types, -2147483648 or its low bits. `_CAST(LONG, 3E9)` is -2147483648, for a constant as for
a variable.
<!-- src: language/builtin-functions "_CAST from a float out of range"; D-047; verification\v62_h1_cast_variables -->

## Errors

These are compile errors:

- A type that is not numeric, or `_BIT`: "`_CAST` takes a numeric type", "`_CAST` takes no `_BIT` type".
  <!-- src: language/builtin-functions "_CAST"; verification\v49_x01_cast_string; verification\v49_x03_cast_bit -->
- A user type or another name as the type: "the first argument of `_CAST` is a numeric type name".
  <!-- src: language/builtin-functions "_CAST"; verification\v49_x05_cast_user_type; verification\v49_x06_cast_unknown_type -->
- A string as the value: "`_CAST` needs a number".
  <!-- src: language/builtin-functions "_CAST"; verification\v49_x07_cast_string_value -->
- Not one type and one value, as in `_CAST(LONG)`, `_CAST(LONG, 1, 2)`, `_CAST(1, LONG)` or `_CAST` alone:
  "`_CAST` takes a type and 1 argument".
  <!-- src: language/builtin-functions "_CAST"; verification\v49_x09_cast_one_arg; verification\v49_x10_cast_three_args; verification\v49_x14_cast_value_first; verification\v49_x15_cast_bare -->
- `_CAST` in a `CONST`: "`_CAST` cannot be used in a `CONST`".
  <!-- src: language/builtin-functions "_CAST"; verification\v49_x11_cast_in_const -->

## Differences

From QB64pe: a floating-point constant outside the type's range, such as `_CAST(LONG, 3E9)`, gives a value that
depends on how QB64pe was built. QB64Rust always gives what the conversion of a variable gives
(`DIVERGENCES-QB64PE.md` D-047).
<!-- src: D-047; verification\v49_x18_cast_out_of_range -->

## Availability

<!-- begin generated: availability -->
Supported. All 14 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_cast_1 -->
```qb64
PRINT _CAST(_BYTE, 255); _CAST(_UNSIGNED _BYTE, -1)
PRINT _CAST(INTEGER, 2.9); _CAST(INTEGER, -2.9)
PRINT HEX$(_CAST(INTEGER, -1))
PRINT _CAST(SINGLE, 1 / 3); _CAST(DOUBLE, 1 / 3)
END
```

Output:

```text
-1  255 
 2 -2 
FFFF
 .3333333  .3333333333333333 

Press any key to continue
```
<!-- end generated -->

## See also

`CINT`, `CLNG`, `CSNG`, `CDBL`, `_MK$`.
