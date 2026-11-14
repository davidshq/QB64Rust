# MKI$

`MKI$`, `MKL$`, `MKS$` and `MKD$` give the bytes of a number as a string: 2 bytes of an INTEGER, 4 of a LONG, 4 of a
SINGLE and 8 of a DOUBLE. They are how a number is written into a record of a random-access file. `CVI`, `CVL`, `CVS`
and `CVD` read it back.

## Syntax

<!-- begin generated: syntax -->
```text
MKD$(doublePrecisionVariableOrLiteral#)
MKI$(integerVariableOrLiteral%)
MKL$(longVariableOrLiteral&)
MKS$(singlePrecisionVariableOrLiteral!)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The number. It is converted to the function's type first, as an assignment converts it: `MKI$(2.5)` holds 2.

## Result

A STRING of 2, 4, 4 or 8 bytes.

## Behavior

The string holds the number's bytes in memory order, the lowest byte first: `MKI$(1)` is the bytes 1 and 0, and
`MKL$(-1)` four bytes of 255.
<!-- src: language/builtin-functions "Supported built-in functions"; verification\v32_c_mk_cv -->

The conversion rounds half to even, as an assignment does: `MKI$(1.5)` and `MKI$(2.5)` both hold 2.
<!-- src: verification\v32_c_mk_cv -->

The matching `CV` function gives the number back: `CVI(MKI$(300))` is 300 and `CVD(MKD$(2.25))` is 2.25.
<!-- src: language/builtin-functions "Supported built-in functions"; tests\reference\examples\mki-str_1 -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 15 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:mki-str_1 -->
```qb64
record$ = MKI$(300) + MKL$(-2) + MKS$(1.5) + MKD$(2.25)
PRINT "The record holds"; LEN(record$); "bytes"
PRINT CVI(MID$(record$, 1, 2)); CVL(MID$(record$, 3, 4))
PRINT CVS(MID$(record$, 7, 4)); CVD(MID$(record$, 11, 8))
END
```

Output:

```text
The record holds 18 bytes
 300 -2 
 1.5  2.25 

Press any key to continue
```
<!-- end generated -->

## See also

[`CVI`](cvi.md), [`_MK$`](_mk-str.md), [`MKSMBF$`](mksmbf-str.md), [`FIELD`](field.md).
