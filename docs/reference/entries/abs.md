# ABS

`ABS` gives the absolute value of a number: the number without its sign. `ABS(-7)` is 7.

## Syntax

<!-- begin generated: syntax -->
```text
ABS(numericalValue)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

A number of the argument's type.

## Behavior

`ABS(n)` is `n` when `n` is 0 or more, and `-n` when it is negative.
<!-- src: tests\reference\examples\abs_1 -->

The result has the argument's type: `ABS(-7%) / 2` is 3.5, computed from an INTEGER.
<!-- src: language/builtin-functions "Result types" -->

The smallest value of a signed integer type has no positive counterpart in the type, and `ABS` gives it back
unchanged: `ABS` of an INTEGER holding -32768 is -32768.
<!-- src: verification\v20_f_math_edges; verification\v20_b_result_types -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:abs_1 -->
```qb64
PRINT ABS(-7); ABS(7); ABS(-2.5)
distance = ABS(3 - 10)
PRINT "The distance from 3 to 10 is"; distance
END
```

Output:

```text
 7  7  2.5 
The distance from 3 to 10 is 7 

Press any key to continue
```
<!-- end generated -->

## See also

[`SGN`](sgn.md), [`INT`](int.md), [`FIX`](fix.md).
