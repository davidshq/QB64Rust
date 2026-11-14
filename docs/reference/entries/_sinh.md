# _SINH

`_SINH`, `_COSH` and `_TANH` give the hyperbolic sine, cosine and tangent of a number.

## Syntax

<!-- begin generated: syntax -->
```text
_COSH(value)
_SINH(value)
_TANH(value)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

A floating-point number.

## Behavior

`_SINH(0)` is 0, `_COSH(0)` is 1 and `_TANH(0)` is 0. `_TANH` stays between -1 and 1.
<!-- src: verification\v22_f_functions; tests\reference\examples\_sinh_1 -->

A SINGLE argument is computed as a SINGLE and gives 7 digits; a DOUBLE or an integer gives a DOUBLE's 16.
<!-- src: language/builtin-functions "Supported built-in functions"; verification\v22_f_functions -->

A very large argument gives infinity, printed `inf`, with no error: `_SINH(1000)` and `_COSH(1000)`. `_TANH(1000)` is
1.
<!-- src: verification\v22_f_functions -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_sinh_1 -->
```qb64
PRINT _SINH(0); _COSH(0); _TANH(0)
PRINT CINT(_COSH(1) * 1000); CINT(_TANH(1) * 1000)
END
```

Output:

```text
 0  1  0 
 1543  762 

Press any key to continue
```
<!-- end generated -->

## See also

[`_ASINH`](_asinh.md), [`_SECH`](_sech.md), [`EXP`](exp.md).
