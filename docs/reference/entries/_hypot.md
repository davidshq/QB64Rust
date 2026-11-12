# _HYPOT

`_HYPOT(x, y)` gives the length of the hypotenuse of a right triangle with sides x and y, `SQR(x * x + y * y)`:
`_HYPOT(3, 4)` is 5.

## Syntax

<!-- begin generated: syntax -->
```text
_HYPOT(x, y)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `x`, `y`: the two sides, or the two distances along the axes.

## Result

A DOUBLE.

## Behavior

`_HYPOT(x, y)` is the distance from the origin to the point (x, y). The distance between two points is `_HYPOT(x2 -
x1, y2 - y1)`.
<!-- src: verification\v20_f_math_edges; tests\reference\examples\_hypot_1 -->

## Errors

- A string as an argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_hypot_1 -->
```qb64
PRINT _HYPOT(3, 4)
x1 = 1: y1 = 2: x2 = 7: y2 = 10
PRINT "The distance between the two points is"; _HYPOT(x2 - x1, y2 - y1)
END
```

Output:

```text
 5 
The distance between the two points is 10 

Press any key to continue
```
<!-- end generated -->

## See also

`SQR`, `_ATAN2`.
