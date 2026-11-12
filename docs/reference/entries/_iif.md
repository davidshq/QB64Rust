# _IIF

`_IIF(condition, a, b)` gives `a` when the condition is true and `b` when it is false: `_IIF(score >= 60, "pass",
"fail")`. Only the part chosen is evaluated.

## Syntax

<!-- begin generated: syntax -->
```text
_IIF(expression, truePart, falsePart)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `condition`: a number. 0 is false, any other value is true.
- `a`, `b`: two numbers, or two strings.

## Result

The part chosen. Two numbers give the type QB64pe gives the pair; two strings give a STRING.

## Behavior

The condition is evaluated first, then only the part it chooses: a FUNCTION called in the other part does not run.
<!-- src: language/builtin-functions "_IIF evaluates one part"; verification\v30_g_iif_min_max -->

The condition is rounded to a whole number first, so 0.4 and 0.5 are false and 0.6 is true.
<!-- src: language/builtin-functions "_IIF evaluates one part"; tests\reference\examples\_iif_1 -->

The two numbers are converted to one result type, as for `_MIN`: a `_FLOAT` among them gives a `_FLOAT`, else a DOUBLE
a DOUBLE, else a SINGLE a SINGLE. Of two integers the wider wins, and of two of one width the unsigned one: `_IIF(1,
b%%, ub~%%)` with `b%% = -1` gives 255.
<!-- src: language/builtin-functions "The result type of _IIF, _MIN, _MAX and _CLAMP"; verification\v30_g_iif_min_max -->

Two strings may be of different kinds; the result is the one chosen, with its length.
<!-- src: language/builtin-functions "_IIF evaluates one part" -->

## Errors

- A string as the condition, a string beside a number, or another number of arguments than three, is a compile error.
  <!-- src: language/builtin-functions "_IIF evaluates one part" -->
- `_IIF` in a `CONST` is a compile error.
  <!-- src: language/builtin-functions "Not in a CONST" -->

## Availability

<!-- begin generated: availability -->
Supported. All 17 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_iif_1 -->
```qb64
score = 75
PRINT _IIF(score >= 60, "pass", "fail")
lives = 0
PRINT "Lives left:"; _IIF(lives > 0, lives, 0)
PRINT _IIF(0.4, 1, 2); _IIF(0.6, 1, 2)
END
```

Output:

```text
pass
Lives left: 0 
 2  1 

Press any key to continue
```
<!-- end generated -->

## See also

`IF`, `_MIN`, `SELECT CASE`.
