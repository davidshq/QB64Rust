# SGN

`SGN` gives the sign of a number: -1 for a negative number, 0 for zero and 1 for a positive one.

## Syntax

<!-- begin generated: syntax -->
```text
SGN(value)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

An integer: -1, 0 or 1.

## Behavior

`SGN(-12)` is -1, `SGN(0)` is 0 and `SGN(4.5)` is 1. A negative zero gives 0.
<!-- src: verification\v20_f_math_edges; tests\reference\examples\sgn_1 -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:sgn_1 -->
```qb64
PRINT SGN(-12); SGN(0); SGN(4.5)
END
```

Output:

```text
-1  0  1 

Press any key to continue
```
<!-- end generated -->

## See also

`ABS`.
