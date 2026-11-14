# EXP

`EXP` gives e, about 2.718282, raised to a power: `EXP(1)` is e and `EXP(0)` is 1.

## Syntax

<!-- begin generated: syntax -->
```text
EXP(numericExpression)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The power.

## Result

A floating-point number.

## Behavior

`EXP(x)` is e to the power `x`. `LOG` undoes it: `LOG(EXP(2))` is 2.
<!-- src: tests\reference\examples\exp_1; tests\reference\examples\log_1 -->

The result's type follows the argument's: SINGLE for a SINGLE or an INTEGER, DOUBLE for a DOUBLE, a LONG or an
`_INTEGER64`. So `EXP(2%)` prints 7 digits and `EXP(2#)` 16.
<!-- src: language/builtin-functions "Result types"; verification\v20_b_result_types -->

## Errors

- A result too large for the type raises run-time error 6, Overflow: `EXP(1000)` for a DOUBLE, and `EXP(100!)` for a
  SINGLE. After `RESUME NEXT` the value is 0. A very negative power gives 0 with no error.
  <!-- src: verification\v20_f_math_edges -->
- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:exp_1 -->
```qb64
PRINT EXP(0)
PRINT "e to 7 digits:"; EXP(1)
END
```

Output:

```text
 1 
e to 7 digits: 2.718282 

Press any key to continue
```
<!-- end generated -->

## See also

[`LOG`](log.md), `^`.
