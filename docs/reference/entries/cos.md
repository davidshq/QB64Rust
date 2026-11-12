# COS

`COS` gives the cosine of an angle in radians: `COS(0)` is 1.

## Syntax

<!-- begin generated: syntax -->
```text
COS(radian_angle!)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The angle, in radians.

## Result

A floating-point number from -1 to 1.

## Behavior

`COS(x)` is the cosine of `x` radians: `COS(_PI)` is -1 and `COS(_PI / 3)` is 0.5.
<!-- src: tests\reference\examples\cos_1 -->

The result's type follows the argument's: SINGLE for a SINGLE or an INTEGER, DOUBLE for a DOUBLE, a LONG or an
`_INTEGER64`. So `COS(2%)` prints 7 digits and `COS(2#)` 16.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:cos_1 -->
```qb64
PRINT COS(0)
PRINT CINT(COS(_PI) * 1000)
PRINT CINT(COS(_PI / 3) * 1000)
END
```

Output:

```text
 1 
-1000 
 500 

Press any key to continue
```
<!-- end generated -->

## See also

[`SIN`](sin.md), [`TAN`](tan.md), [`_ACOS`](_asin.md), [`_PI`](_pi.md).
