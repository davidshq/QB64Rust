# CDBL

`CDBL` converts a number to DOUBLE precision, about 16 significant digits.

## Syntax

<!-- begin generated: syntax -->
```text
CDBL(expression)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `expression`: a number of any numeric type.

## Result

A DOUBLE.

## Behavior

A SINGLE value keeps its exact binary value, so its error shows in the extra digits: with `s! = 0.1`, `CDBL(s!)`
prints `.1000000014901161`.
<!-- src: tests\reference\examples\cdbl_1 -->

An integer converts exactly: `CDBL` of an `_UNSIGNED LONG` of 4294967295 prints `4294967295`.
<!-- src: verification\v21_d_builtins -->

`CDBL(2 / 3)` and `CDBL(2! / 3)` both print `.6666666666666666`.
<!-- src: verification\v20_b_result_types -->

## Errors

- An `_FLOAT` value beyond DOUBLE's range raises run-time error 6, "Overflow". After `RESUME NEXT`, the value
  stored is 0.
  <!-- src: verification\v20_f_math_edges -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:cdbl_1 -->
```qb64
s! = 0.1
PRINT "As SINGLE:"; s!
PRINT "As DOUBLE:"; CDBL(s!)
END
```

Output:

```text
As SINGLE: .1 
As DOUBLE: .1000000014901161 

Press any key to continue
```
<!-- end generated -->

## See also

[`CSNG`](csng.md), [`CINT`](cint.md), [`CLNG`](clng.md), [`_CAST`](_cast.md).
