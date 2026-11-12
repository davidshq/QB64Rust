# _SETBIT

`_SETBIT(n, b)` gives `n` with bit `b` set to 1, `_RESETBIT` with it set to 0 and `_TOGGLEBIT` with it turned over.
`_READBIT(n, b)` tells whether bit `b` is set. Bit 0 is the lowest.

## Syntax

<!-- begin generated: syntax -->
```text
_READBIT(numericalVariable, numericalValue)
_RESETBIT(numericalVariable, numericalValue)
_SETBIT(numericalVariable, numericalValue)
_TOGGLEBIT(numericalVariable, numericalValue)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `n`: an integer.
- `b`: the bit number, 0 for the lowest.

## Result

`_SETBIT`, `_RESETBIT` and `_TOGGLEBIT`: an integer of the type QB64pe gives for `n`'s type. `_READBIT`: -1 when the
bit is set, 0 when it is not.

## Behavior

`_SETBIT(5, 1)` is 7, `_RESETBIT(5, 0)` is 4 and `_TOGGLEBIT(5, 2)` is 1.
<!-- src: language/builtin-functions "Bit functions"; tests\reference\examples\_setbit_1 -->

A bit number outside 0 to 63 is taken modulo 64, its low six bits: `_SETBIT(0, 64)` is 1 and `_SETBIT(0, 65)` is 2. A
bit beyond `n`'s width gives QB64pe's value.
<!-- src: language/builtin-functions "Bit functions" -->

With the fix `bit-number-check` on ([Fixes](../chapters/fixes.md)), a bit number outside 0 to 63 raises error 5
instead.
<!-- src: language/builtin-functions "Fix bit-number-check" -->

`_SETBIT` may not stand in a `CONST`.
<!-- src: language/builtin-functions "Hyperbolic and gradian functions" -->

## Errors

- A string as an argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Differences

From QB64pe: a constant bit number of 64 or more gives a value there that depends on its C++ compiler. QB64Rust takes
it modulo 64, as QB64pe does for a variable (`DIVERGENCES-QB64PE.md` D-053).
<!-- src: D-053; language/builtin-functions "Bit functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_setbit_1 -->
```qb64
flags = 0
flags = _SETBIT(flags, 0)
flags = _SETBIT(flags, 2)
PRINT "flags ="; flags
PRINT "Bit 2 is set:"; _READBIT(flags, 2)
flags = _RESETBIT(flags, 0)
PRINT "After _RESETBIT bit 0:"; flags
PRINT "_TOGGLEBIT(5, 1) ="; _TOGGLEBIT(5, 1)
END
```

Output:

```text
flags = 5 
Bit 2 is set:-1 
After _RESETBIT bit 0: 4 
_TOGGLEBIT(5, 1) = 7 

Press any key to continue
```
<!-- end generated -->

## See also

`_SHL`, `AND`, `OR`, `_BIN$`.
