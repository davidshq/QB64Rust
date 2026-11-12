# TAN

`TAN` gives the tangent of an angle in radians: `TAN(_PI / 4)` is 1.

## Syntax

<!-- begin generated: syntax -->
```text
TAN(radian_angle!)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The angle, in radians.

## Result

A floating-point number.

## Behavior

`TAN(x)` is the tangent of `x` radians: `TAN(0)` is 0.
<!-- src: tests\reference\examples\tan_1 -->

Near a right angle the value is very large, with no error: `TAN(_PI / 2)` is about 1.6D+16.
<!-- src: verification\v20_f_math_edges -->

The result's type follows the argument's: SINGLE for a SINGLE or an INTEGER, DOUBLE for a DOUBLE, a LONG or an
`_INTEGER64`. So `SQR(2%)` prints 7 digits and `SQR(2#)` 16.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:tan_1 -->
```qb64
PRINT TAN(0)
PRINT CINT(TAN(_PI / 4) * 1000)
END
```

Output:

```text
 0 
 1000 

Press any key to continue
```
<!-- end generated -->

## See also

`SIN`, `COS`, `ATN`, `_COT`.
