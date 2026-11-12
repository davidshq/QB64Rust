# _CV

`_CV` reads a number of any numeric type from the bytes of a string: `_CV(LONG, s$)` reads 4 bytes as a LONG. It is
the general form of `CVI` and `CVL`, and reads back what `_MK$` wrote.

## Syntax

<!-- begin generated: syntax -->
```text
_CV(numericalType, MKstringValue$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The type: a numeric type, written as in `DIM`.
- A string of at least as many bytes as the type has.

## Result

A value of the type named.

## Behavior

The first bytes of the string are read as the type, the lowest byte first. The byte 200 is 200 as an
`_UNSIGNED _BYTE` and -56 as a `_BYTE`.
<!-- src: verification\v32_c_mk_cv; tests\reference\examples\_cv_1 -->

## Errors

- A string shorter than the type raises run-time error 5.
  <!-- src: verification\v32_c_mk_cv -->

These are compile errors:

- A type that is not numeric: "`_CV` takes a numeric type".
  <!-- src: verification\v32_x100_cv_string -->
- A name that is no type: "the first argument of `_CV` is a type name".
  <!-- src: verification\v32_x111_cv_unknown -->
- A number as the second argument: "`_CV` needs a string".
  <!-- src: verification\v32_x105_cv_number -->

## Availability

<!-- begin generated: availability -->
Supported. All 17 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_cv_1 -->
```qb64
s$ = _MK$(LONG, 123456)
PRINT _CV(LONG, s$)
PRINT _CV(_UNSIGNED _BYTE, CHR$(200)); _CV(_BYTE, CHR$(200))
END
```

Output:

```text
 123456 
 200 -56 

Press any key to continue
```
<!-- end generated -->

## See also

[`_MK$`](_mk-str.md), [`CVI`](cvi.md), [`_MEMGET`](_memget.md).
