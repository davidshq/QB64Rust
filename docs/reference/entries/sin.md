# SIN

`SIN` gives the sine of an angle in radians: `SIN(_PI / 2)` is 1.

## Syntax

<!-- begin generated: syntax -->
```text
SIN(radian_angle!)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The angle, in radians. `_D2R` turns degrees into radians.

## Result

A floating-point number from -1 to 1.

## Behavior

`SIN(x)` is the sine of `x` radians: `SIN(0)` is 0 and `SIN(_PI / 6)` is 0.5.
<!-- src: tests\reference\examples\sin_1 -->

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

<!-- begin generated: example:sin_1 -->
```qb64
PRINT SIN(0)
PRINT CINT(SIN(_PI / 2) * 1000)
PRINT CINT(SIN(_PI / 6) * 1000)
END
```

Output:

```text
 0 
 1000 
 500 

Press any key to continue
```
<!-- end generated -->

## See also

`COS`, `TAN`, `_ASIN`, `_D2R`, `_PI`.
