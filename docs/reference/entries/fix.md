# FIX

`FIX` drops the fraction of a number: `FIX(2.7)` is 2 and `FIX(-2.7)` is -2. It rounds toward zero.

## Syntax

<!-- begin generated: syntax -->
```text
FIX(expression)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

A number of the argument's type, with no fraction.

## Behavior

`FIX` cuts toward zero: `FIX(-0.5)` is 0 and `FIX(-1.5)` is -1. `INT` rounds down instead, so they differ for negative
numbers: `INT(-2.5)` is -3, `FIX(-2.5)` is -2.
<!-- src: verification\v20_f_math_edges; tests\reference\examples\fix_1 -->

The result has the argument's type.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:fix_1 -->
```qb64
PRINT FIX(2.7); FIX(-2.7); FIX(-0.5)
PRINT "INT and FIX of -2.5:"; INT(-2.5); FIX(-2.5)
END
```

Output:

```text
 2 -2  0 
INT and FIX of -2.5:-3 -2 

Press any key to continue
```
<!-- end generated -->

## See also

[`INT`](int.md), [`CINT`](cint.md), [`_ROUND`](_round.md).
