# _D2R

`_D2R` turns degrees into radians and `_R2D` radians into degrees. `_D2G`, `_G2D`, `_R2G` and `_G2R` do the same with
gradians, of which a right angle has 100.

## Syntax

<!-- begin generated: syntax -->
```text
_D2G(angleInDegrees!)
_D2R(angleInDegrees!)
_G2D(gradient!)
_G2R(gradient!)
_R2D(angleInRadians!)
_R2G(angleInRadians!)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The angle to convert.

## Result

A floating-point number.

## Behavior

`_D2R(180)` is pi and `_R2D(_PI)` is 180. `_D2G(90)` is 100, `_G2D(200)` is 180, `_R2G(_PI)` is 200 and `_G2R(200)` is
pi.
<!-- src: language/builtin-functions "Hyperbolic and gradian functions"; verification\v22_f_functions; tests\reference\examples\_d2r_1 -->

In a `CONST`, `_D2G(90)` gives 99.99999999900001, the value of QB64pe's constant evaluator, where a statement gives
100.
<!-- src: language/builtin-functions "Hyperbolic and gradian functions"; verification\v58_f05_d2g_const -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 14 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_d2r_1 -->
```qb64
PRINT CINT(_D2R(180) * 1000); _R2D(_PI)
PRINT _D2G(90); _G2D(200)
PRINT CINT(_R2G(_PI)); CINT(_G2R(200) * 1000)
END
```

Output:

```text
 3142  180 
 100  180 
 200  3142 

Press any key to continue
```
<!-- end generated -->

## See also

`SIN`, `COS`, `_PI`, `_ATAN2`.
