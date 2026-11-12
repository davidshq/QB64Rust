# LOG

`LOG` gives the natural logarithm of a number, the power of e that gives it: `LOG(1)` is 0. `LOG(x) / LOG(10)` is the
base-10 logarithm.

## Syntax

<!-- begin generated: syntax -->
```text
LOG(value)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number above 0.

## Result

A floating-point number.

## Behavior

`LOG(x)` is the power `p` with `EXP(p) = x`: `LOG(EXP(2))` is 2.
<!-- src: tests\reference\examples\log_1 -->

The result's type follows the argument's: SINGLE for a SINGLE or an INTEGER, DOUBLE for a DOUBLE, a LONG or an
`_INTEGER64`. So `SQR(2%)` prints 7 digits and `SQR(2#)` 16.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

## Errors

- An argument of 0 or below raises run-time error 5, and the result is 0.
  <!-- src: language/builtin-functions "Runtime errors of built-ins"; verification\v20_f_math_edges -->
- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:log_1 -->
```qb64
PRINT LOG(1)
PRINT "LOG(EXP(2)) is"; CINT(LOG(EXP(2)) * 1000) / 1000
PRINT "The base-10 logarithm of 1000 is"; CINT(LOG(1000) / LOG(10))
END
```

Output:

```text
 0 
LOG(EXP(2)) is 2 
The base-10 logarithm of 1000 is 3 

Press any key to continue
```
<!-- end generated -->

## See also

`EXP`.
