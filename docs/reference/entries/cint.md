# CINT

`CINT` rounds a number to the nearest whole number and gives it as an INTEGER: `CINT(3.7)` is 4.

## Syntax

<!-- begin generated: syntax -->
```text
CINT(expression)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `expression`: a number of any numeric type.

## Result

An INTEGER, from -32768 to 32767.

## Behavior

The value is rounded to the nearest whole number. A value exactly halfway is rounded to the even number:
`CINT(2.5)` is 2, `CINT(3.5)` is 4, `CINT(-0.5)` is 0 and `CINT(-1.5)` is -2.
<!-- src: verification\v01_numeric; verification\v20_f_math_edges; tests\corpus\slice\s27_math_builtins -->

A value that rounds into the INTEGER range is converted, so `CINT(32767.4)` is 32767 and `CINT(-32768.5)` is
-32768.
<!-- src: verification\v20_f_math_edges -->

## Errors

- A value that rounds outside -32768 to 32767, such as `CINT(32767.5)` or `CINT(40000)`, raises run-time error 6,
  "Overflow". After `RESUME NEXT`, the value stored is 0. An INTEGER, LONG or `_INTEGER64` argument out of range
  raises it too.
  <!-- src: verification\v20_f_math_edges; tests\corpus\slice\s27_math_builtins -->
- An `_UNSIGNED INTEGER` argument of 65535 raises error 6 too.
  <!-- src: verification\v21_d_builtins -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:cint_1 -->
```qb64
PRINT CINT(3.7); CINT(-3.7); CINT(2.5); CINT(3.5)
ON ERROR GOTO toobig
x% = 7
x% = CINT(40000)
PRINT "x% is"; x%
END

toobig:
PRINT "Error"; ERR; "(Overflow)"
RESUME NEXT
```

Output:

```text
 4 -4  2  4 
Error 6 (Overflow)
x% is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`CLNG`, `_ROUND`, `INT`, `FIX`, `_CAST`.
