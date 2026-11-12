# _ASINH

`_ASINH`, `_ACOSH` and `_ATANH` give the inverse hyperbolic sine, cosine and tangent: `_ASINH(_SINH(2))` is 2.

## Syntax

<!-- begin generated: syntax -->
```text
_ACOSH(value)
_ASINH(value)
_ATANH(value)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number. `_ACOSH` needs 1 or more, and `_ATANH` a value between -1 and 1.

## Result

A floating-point number.

## Behavior

`_ASINH(0)`, `_ACOSH(1)` and `_ATANH(0)` are 0. Each undoes its hyperbolic function.
<!-- src: language/builtin-functions "Hyperbolic and gradian functions"; verification\v58_f01_hyperbolic; tests\reference\examples\_asinh_1 -->

An argument outside the domain raises no error: `_ACOSH(0)` and `_ATANH(2)` give NaN, `_ATANH(1)` infinity and
`_ATANH(-1)` minus infinity.
<!-- src: language/builtin-functions "Hyperbolic and gradian functions" -->

A SINGLE argument is computed as a SINGLE and gives 7 digits; a DOUBLE or an integer gives a DOUBLE's 16.
<!-- src: verification\v58_f01_hyperbolic -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->
- These functions in a `CONST` are a compile error.
  <!-- src: language/builtin-functions "Hyperbolic and gradian functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_asinh_1 -->
```qb64
PRINT _ASINH(0); _ACOSH(1); _ATANH(0)
PRINT CINT(_ASINH(_SINH(2)) * 1000)
END
```

Output:

```text
 0  0  0 
 2000 

Press any key to continue
```
<!-- end generated -->

## See also

`_SINH`, `LOG`.
