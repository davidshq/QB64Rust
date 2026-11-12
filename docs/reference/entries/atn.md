# ATN

`ATN` gives the arctangent of a number, the angle in radians whose tangent it is: `4 * ATN(1)` is pi.

## Syntax

<!-- begin generated: syntax -->
```text
ATN(tangent!)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

An angle in radians, from -pi/2 to pi/2.

## Behavior

`ATN(0)` is 0 and `ATN(1)` is a quarter of pi. A very large argument gives close to pi/2: `ATN(1E+300)` prints
`1.570796`.
<!-- src: verification\v20_f_math_edges; tests\reference\examples\atn_1 -->

`_ATAN2(y, x)` gives the angle of a point in all four quarters of the circle.
<!-- src: language/builtin-functions "Supported built-in functions" -->

The result's type follows the argument's: SINGLE for a SINGLE or an INTEGER, DOUBLE for a DOUBLE, a LONG or an
`_INTEGER64`. So `ATN(2%)` prints 7 digits and `ATN(2#)` 16.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:atn_1 -->
```qb64
PRINT ATN(0)
PRINT "4 * ATN(1) is close to pi:"; CINT(4 * ATN(1) * 1000)
END
```

Output:

```text
 0 
4 * ATN(1) is close to pi: 3142 

Press any key to continue
```
<!-- end generated -->

## See also

`_ATAN2`, `TAN`, `_PI`.
