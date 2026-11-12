# CLNG

`CLNG` rounds a number to the nearest whole number and gives it as a LONG: `CLNG(70000.5)` is 70000.

## Syntax

<!-- begin generated: syntax -->
```text
CLNG(expression)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `expression`: a number of any numeric type.

## Result

A LONG, from -2147483648 to 2147483647.

## Behavior

The value is rounded to the nearest whole number. A value exactly halfway is rounded to the even number:
`CLNG(70000.5)` is 70000, `CLNG(70001.5)` is 70002, `CLNG(2.5)` is 2 and `CLNG(-2.5)` is -2.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types; verification\v20_f_math_edges -->

A value that rounds into the LONG range is converted, so `CLNG(2147483647.4#)` is 2147483647 and
`CLNG(-2147483648.5#)` is -2147483648.
<!-- src: verification\v20_f_math_edges -->

## Errors

- A value that rounds outside the LONG range, such as `CLNG(2147483647.5#)`, raises run-time error 6, "Overflow".
  After `RESUME NEXT`, the value stored is 0. An `_INTEGER64` argument of 3000000000 raises it too.
  <!-- src: verification\v20_f_math_edges -->
- An `_UNSIGNED LONG` of 4294967295 or an `_UNSIGNED _INTEGER64` of 18446744073709551615 raises error 6 too.
  <!-- src: verification\v21_d_builtins -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:clng_1 -->
```qb64
PRINT CLNG(70000.5); CLNG(70001.5); CLNG(-2.5)
PRINT CLNG(2147483647.4#)
ON ERROR GOTO toobig
n& = CLNG(3000000000#)
END

toobig:
PRINT "Error"; ERR; "(Overflow)"
RESUME NEXT
```

Output:

```text
 70000  70002 -2 
 2147483647 
Error 6 (Overflow)

Press any key to continue
```
<!-- end generated -->

## See also

[`CINT`](cint.md), [`_ROUND`](_round.md), [`INT`](int.md), [`FIX`](fix.md), [`_CAST`](_cast.md).
