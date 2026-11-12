# _ARCSEC

`_ARCSEC`, `_ARCCSC` and `_ARCCOT` give the inverse secant, cosecant and cotangent, as angles in radians: `_ARCCOT(1)`
is pi/4, 45 degrees.

## Syntax

<!-- begin generated: syntax -->
```text
_ARCCOT(value)
_ARCCSC(value)
_ARCSEC(value)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number. `_ARCSEC` and `_ARCCSC` need a value of 1 or more, or of -1 or less.

## Result

A `_FLOAT`, an angle in radians.

## Behavior

`_ARCSEC(1)` is 0. `_R2D(_ARCCSC(2))` is 30 and `_R2D(_ARCCOT(1))` is 45.
<!-- src: language/builtin-functions "The reciprocal and hyperbolic functions at run time"; verification\v52_b_math; tests\reference\examples\_arcsec_1 -->

## Errors

- An argument between -1 and 1, such as `_ARCSEC(0.5)` or `_ARCCSC(-0.5)`, raises run-time error 5, and the result is
  0.
  <!-- src: language/builtin-functions "The reciprocal and hyperbolic functions at run time" -->
- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_arcsec_1 -->
```qb64
PRINT _ARCSEC(1)
PRINT CINT(_R2D(_ARCCSC(2))); CINT(_R2D(_ARCCOT(1)))
END
```

Output:

```text
 0 
 30  45 

Press any key to continue
```
<!-- end generated -->

## See also

`_SEC`, `_ASIN`, `ATN`.
