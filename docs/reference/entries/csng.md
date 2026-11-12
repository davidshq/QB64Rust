# CSNG

`CSNG` converts a number to SINGLE precision, about 7 significant digits: `CSNG(2# / 3)` is `.6666667`.

## Syntax

<!-- begin generated: syntax -->
```text
CSNG(expression)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `expression`: a number of any numeric type.

## Result

A SINGLE.

## Behavior

A DOUBLE or `_FLOAT` value is rounded to the nearest SINGLE: `CSNG(2# / 3)` prints `.6666667`.
<!-- src: verification\v20_b_result_types; tests\corpus\slice\s27_math_builtins -->

`CSNG` of an integer prints as a SINGLE, but its value is not narrowed when it is used in arithmetic. With
`l& = 16777217`, `CSNG(l&)` prints `1.677722E+07`, and `CSNG(l&) - 16777216` is 1.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

Large unsigned values convert too: `CSNG` of an `_UNSIGNED LONG` of 4294967295 prints `4.294967E+09`.
<!-- src: verification\v21_d_builtins -->

## Errors

- A value beyond SINGLE's range, such as `CSNG(1D+300)`, raises run-time error 6, "Overflow". After `RESUME NEXT`,
  the value stored is 0.
  <!-- src: verification\v20_f_math_edges -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:csng_1 -->
```qb64
d# = 2 / 3
PRINT "As DOUBLE:"; d#
PRINT "As SINGLE:"; CSNG(d#)
PRINT CSNG(1 / 3) * 3 = 1
END
```

Output:

```text
As DOUBLE: .6666666666666666 
As SINGLE: .6666667 
-1 

Press any key to continue
```
<!-- end generated -->

## See also

`CDBL`, `CINT`, `CLNG`, `_CAST`.
