# _PI

`_PI` gives the number pi, 3.141592653589793. `_PI(n)` gives pi times `n`.

## Syntax

<!-- begin generated: syntax -->
```text
_PI[(multiplier)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `multiplier`, optional: what pi is multiplied by.

## Result

A DOUBLE.

## Behavior

`_PI` without parentheses is pi. `_PI(2)` is two pi, and `_PI(0)` is 0.
<!-- src: language/builtin-functions "Built-ins without parentheses"; verification\v20_f_math_edges; tests\reference\examples\_pi_1 -->

It prints with DOUBLE's 16 digits: `PRINT _PI` prints `3.141592653589793`.
<!-- src: verification\v20_b_result_types -->

`_PI` may stand in a `CONST`.
<!-- src: language/constants "CONST values" -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_pi_1 -->
```qb64
PRINT _PI
PRINT _PI(2)
r = 2
PRINT "The area of a circle of radius 2 is"; CINT(_PI * r * r * 100) / 100
END
```

Output:

```text
 3.141592653589793 
 6.283185307179586 
The area of a circle of radius 2 is 12.57 

Press any key to continue
```
<!-- end generated -->

## See also

`SIN`, `COS`, `_D2R`, `ATN`.
