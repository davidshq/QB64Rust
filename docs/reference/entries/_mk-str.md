# _MK$

`_MK$` gives the bytes of a number of any numeric type as a string: `_MK$(_INTEGER64, -9)` is 8 bytes. It is the
general form of `MKI$` and `MKL$`. `_CV` reads the bytes back.

## Syntax

<!-- begin generated: syntax -->
```text
_MK$(numericalType, numericalValue)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The type: a numeric type, written as in `DIM`, such as `_UNSIGNED _BYTE`, `LONG`, `_FLOAT` or `_BIT`.
- The value, converted to that type first.

## Result

A STRING of the type's size in bytes: 1 for a `_BYTE` and for a `_BIT`, 8 for an `_INTEGER64` or `_OFFSET`, 32 for a
`_FLOAT`.
<!-- src: verification\v32_c_mk_cv -->

## Behavior

The string holds the value's bytes in memory order, the lowest byte first: `_MK$(_UNSIGNED INTEGER, 65535)` is two
bytes of 255. A value outside the type's range is cut to its low bits: `_MK$(_UNSIGNED _BYTE, 256)` is one byte of 0.
<!-- src: language/builtin-functions "Supported built-in functions"; verification\v32_c_mk_cv -->

`_CV(type, _MK$(type, x))` gives `x` back.
<!-- src: language/builtin-functions "Supported built-in functions"; tests\reference\examples\_mk-str_1 -->

## Errors

- A type that is not numeric is a compile error.
  <!-- src: verification\v32_x100_cv_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 18 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mk-str_1 -->
```qb64
b$ = _MK$(_UNSIGNED _BYTE, 255)
PRINT LEN(b$); ASC(b$)
q$ = _MK$(_INTEGER64, -9)
PRINT LEN(q$); _CV(_INTEGER64, q$)
END
```

Output:

```text
 1  255 
 8 -9 

Press any key to continue
```
<!-- end generated -->

## See also

`_CV`, `MKI$`, `_MEMGET`.
