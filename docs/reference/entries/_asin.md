# _ASIN

`_ASIN` gives the arcsine of a number, the angle in radians whose sine it is, and `_ACOS` the arccosine. `_ASIN(0.5)`
is pi/6, 30 degrees.

## Syntax

<!-- begin generated: syntax -->
```text
_ACOS(cosine_value!)
_ASIN(sine_value!)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number from -1 to 1.

## Result

An angle in radians: from -pi/2 to pi/2 for `_ASIN`, from 0 to pi for `_ACOS`.

## Behavior

`_ASIN(0)` is 0 and `_ACOS(1)` is 0. `_R2D(_ASIN(0.5))` is 30 and `_R2D(_ACOS(0.5))` is 60.
<!-- src: verification\v22_f_functions; tests\reference\examples\_asin_1 -->

A SINGLE argument is computed as a SINGLE and gives 7 digits; a DOUBLE or an integer gives a DOUBLE's 16.
<!-- src: language/builtin-functions "Supported built-in functions"; verification\v22_f_functions -->

An argument outside -1 to 1 gives NaN, printed `-nan(ind)`, with no error: `_ACOS(2)`, `_ASIN(-2)`.
<!-- src: verification\v22_f_functions -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_asin_1 -->
```qb64
PRINT _ASIN(0); _ACOS(1)
PRINT CINT(_R2D(_ASIN(0.5))); CINT(_R2D(_ACOS(0.5)))
END
```

Output:

```text
 0  0 
 30  60 

Press any key to continue
```
<!-- end generated -->

## See also

[`SIN`](sin.md), [`COS`](cos.md), [`ATN`](atn.md), [`_ATAN2`](_atan2.md), [`_ARCSEC`](_arcsec.md).
