# INT

`INT` gives the largest whole number not above a number: `INT(2.7)` is 2 and `INT(-2.7)` is -3. It rounds down, toward
minus infinity.

## Syntax

<!-- begin generated: syntax -->
```text
INT(expression)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

A number of the argument's type, with no fraction.

## Behavior

`INT` rounds down: `INT(-0.5)` is -1 and `INT(1.5)` is 1. `FIX` rounds toward zero instead.
<!-- src: verification\v20_f_math_edges; tests\reference\examples\int_1 -->

The result has the argument's type: `INT` of a SINGLE is a SINGLE, of a DOUBLE a DOUBLE, of an integer the integer
unchanged.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:int_1 -->
```qb64
PRINT INT(2.7); INT(-2.7); INT(-0.5)
PRINT "A die roll from 0.73:"; INT(0.73 * 6) + 1
END
```

Output:

```text
 2 -3 -1 
A die roll from 0.73: 5 

Press any key to continue
```
<!-- end generated -->

## See also

`FIX`, `CINT`, `_ROUND`, `_CEIL`.
