# _MIN

`_MIN` gives the lesser of two numbers and `_MAX` the greater: `_MIN(3, 7)` is 3 and `_MAX(3, 7)` is 7.

## Syntax

<!-- begin generated: syntax -->
```text
_MAX(value1, value2)
_MIN(value1, value2)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `a`, `b`: two numbers of any numeric types.

## Result

A number of the type QB64pe gives the pair.

## Behavior

Each argument is evaluated once, in order, and the two are compared in the result's type.
<!-- src: language/builtin-functions "_MIN, _MAX and _CLAMP"; tests\reference\examples\_min_1 -->

The result type: a `_FLOAT` among them gives a `_FLOAT`, else a DOUBLE a DOUBLE, else a SINGLE a SINGLE. Of two
integers the wider wins, and of two of one width the unsigned one: `_MIN(b%%, ub~%%)` with `b%% = -1` and `ub~%% =
255` gives 255.
<!-- src: language/builtin-functions "The result type of _IIF, _MIN, _MAX and _CLAMP" -->

## Errors

- A string or a whole `TYPE` value, or another number of arguments, is a compile error.
  <!-- src: language/builtin-functions "_MIN, _MAX and _CLAMP" -->
- `_MIN` and `_MAX` in a `CONST` are compile errors.
  <!-- src: language/builtin-functions "Not in a CONST" -->

## Availability

<!-- begin generated: availability -->
Supported. All 22 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_min_1 -->
```qb64
PRINT _MIN(3, 7); _MAX(3, 7)
PRINT _MIN(-2.5, 1); _MAX(-2.5, 1)
best = 0
FOR i = 1 TO 4
    READ v
    best = _MAX(best, v)
NEXT i
PRINT "The highest is"; best
DATA 12, 47, 5, 30
END
```

Output:

```text
 3  7 
-2.5  1 
The highest is 47 

Press any key to continue
```
<!-- end generated -->

## See also

`_CLAMP`, `_IIF`, `ABS`.
