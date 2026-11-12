# _SECH

`_SECH`, `_CSCH` and `_COTH` give the hyperbolic secant, cosecant and cotangent of a number: 1 / `_COSH`, 1 / `_SINH`
and 1 / `_TANH`.

## Syntax

<!-- begin generated: syntax -->
```text
_COTH(value)
_CSCH(value)
_SECH(value)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

A `_FLOAT`.

## Behavior

`_SECH(0)` is 1. `_SECH(1)` is about 0.648 and `_CSCH(1)` about 0.851.
<!-- src: language/builtin-functions "The reciprocal and hyperbolic functions at run time"; tests\reference\examples\_sech_1 -->

`_COTH(x)` is the hyperbolic cotangent of `x`, with no upper limit.
<!-- src: language/builtin-functions "_COTH is the hyperbolic cotangent" -->

## Errors

- An argument outside the domain raises run-time error 5, and the result is 0: `_CSCH(0)`, `_COTH(0)`, and
  `_SECH(88.1)` or `_CSCH(88.1)`, where the hyperbolic functions grow beyond the range.
  <!-- src: language/builtin-functions "The reciprocal and hyperbolic functions at run time"; language/builtin-functions "_COTH is the hyperbolic cotangent" -->
- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Differences

From QB64pe: its `_COTH(x)` computes `2 * EXP(x) - 1`, not the hyperbolic cotangent, and raises error 5 above
44.014845. QB64Rust gives the hyperbolic cotangent (`DIVERGENCES-QB64PE.md` D-031).
<!-- src: D-031 -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sech_1 -->
```qb64
PRINT _SECH(0)
PRINT CINT(_SECH(1) * 1000); CINT(_CSCH(1) * 1000)
END
```

Output:

```text
 1 
 648  851 

Press any key to continue
```
<!-- end generated -->

## See also

`_SINH`, `_SEC`.
