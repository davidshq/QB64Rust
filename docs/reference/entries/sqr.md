# SQR

`SQR` gives the square root of a number: `SQR(16)` is 4.

## Syntax

<!-- begin generated: syntax -->
```text
SQR(value)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of 0 or more.

## Result

A floating-point number.

## Behavior

`SQR(n)` is the number that, multiplied by itself, gives `n`: `SQR(2)` prints `1.414214`.
<!-- src: tests\reference\examples\sqr_1; verification\v20_b_result_types -->

The result's type follows the argument's: SINGLE for a SINGLE or an INTEGER, DOUBLE for a DOUBLE, a LONG or an
`_INTEGER64`. So `SQR(2%)` prints 7 digits and `SQR(2#)` 16.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

## Errors

- A negative argument raises run-time error 5, and the result is 0.
  <!-- src: language/builtin-functions "Runtime errors of built-ins"; verification\v20_f_math_edges -->
- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:sqr_1 -->
```qb64
PRINT SQR(16); SQR(2)
PRINT "The diagonal of a 3 by 4 rectangle is"; SQR(3 * 3 + 4 * 4)
END
```

Output:

```text
 4  1.414214 
The diagonal of a 3 by 4 rectangle is 5 

Press any key to continue
```
<!-- end generated -->

## See also

`_HYPOT`, `^`, `EXP`.
