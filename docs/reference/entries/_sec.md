# _SEC

`_SEC`, `_CSC` and `_COT` give the secant, cosecant and cotangent of an angle in radians: 1 / `COS`, 1 / `SIN` and 1 /
`TAN`.

## Syntax

<!-- begin generated: syntax -->
```text
_COT(value)
_CSC(value)
_SEC(value)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The angle, in radians.

## Result

A `_FLOAT`.

## Behavior

`_SEC(0)` is 1, `_CSC(_PI / 2)` is 1 and `_COT(_PI / 4)` is 1.
<!-- src: verification\v22_f_functions; tests\reference\examples\_sec_1 -->

## Errors

- An angle where the value has no limit, such as `_COT(0)`, raises run-time error 5, and the result is 0.
  <!-- src: verification\v22_f_functions; language/builtin-functions "Supported built-in functions" -->
- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sec_1 -->
```qb64
PRINT _SEC(0); _CSC(_PI / 2)
PRINT CINT(_COT(_PI / 4) * 1000)
END
```

Output:

```text
 1  1 
 1000 

Press any key to continue
```
<!-- end generated -->

## See also

[`SIN`](sin.md), [`COS`](cos.md), [`TAN`](tan.md), [`_ARCSEC`](_arcsec.md), [`_SECH`](_sech.md).
