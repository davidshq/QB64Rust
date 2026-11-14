# _ATAN2

`_ATAN2(y, x)` gives the angle in radians of the point (x, y) from the origin, from -pi to pi. Unlike `ATN(y / x)`, it
knows which quarter of the circle the point is in.

## Syntax

<!-- begin generated: syntax -->
```text
_ATAN2(y, x)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `y`: the vertical distance.
- `x`: the horizontal distance.

## Result

A DOUBLE, an angle from -pi to pi.

## Behavior

`_ATAN2(0, 1)` is 0, `_ATAN2(1, 1)` is a quarter of pi and `_ATAN2(0, -1)` is pi. A point below the x axis gives a
negative angle.
<!-- src: verification\v20_f_math_edges; tests\reference\examples\_atan2_1 -->

`_ATAN2(0, 0)` is 0, with no error.
<!-- src: verification\v20_f_math_edges -->

## Errors

- A string as an argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_atan2_1 -->
```qb64
PRINT _ATAN2(0, 1)
PRINT CINT(_ATAN2(1, 1) * 1000)
PRINT CINT(_ATAN2(0, -1) * 1000)
PRINT CINT(_R2D(_ATAN2(-1, 0)))
END
```

Output:

```text
 0 
 785 
 3142 
-90 

Press any key to continue
```
<!-- end generated -->

## See also

[`ATN`](atn.md), [`_HYPOT`](_hypot.md), [`_R2D`](_d2r.md).
