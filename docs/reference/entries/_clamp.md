# _CLAMP

`_CLAMP(value, low, high)` holds a value between two bounds: below `low` it gives `low`, above `high` it gives `high`.
`_CLAMP(130, 0, 100)` is 100.

## Syntax

<!-- begin generated: syntax -->
```text
_CLAMP(value, minimum, maximum)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `value`: the number held.
- `low`, `high`: the bounds.

## Result

A number of the type QB64pe gives the three arguments.

## Behavior

`_CLAMP(15, 0, 10)` is 10, `_CLAMP(-5, 0, 10)` is 0 and `_CLAMP(5, 0, 10)` is 5.
<!-- src: language/builtin-functions "_MIN, _MAX and _CLAMP"; tests\reference\examples\_clamp_1 -->

Each argument is evaluated once, in the order `value`, `low`, `high`.
<!-- src: language/builtin-functions "_MIN, _MAX and _CLAMP" -->

## Errors

- A string, a `TYPE` value, or another number of arguments than three, is a compile error.
  <!-- src: language/builtin-functions "_MIN, _MAX and _CLAMP" -->
- `_CLAMP` in a `CONST` is a compile error.
  <!-- src: language/builtin-functions "Not in a CONST" -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_clamp_1 -->
```qb64
PRINT _CLAMP(15, 0, 10); _CLAMP(-5, 0, 10); _CLAMP(5, 0, 10)
health = 130
health = _CLAMP(health, 0, 100)
PRINT "Health is held at"; health
END
```

Output:

```text
 10  0  5 
Health is held at 100 

Press any key to continue
```
<!-- end generated -->

## See also

`_MIN`, `_MAX`.
