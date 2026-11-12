# LONG

`LONG` is the type of a 32-bit whole number, from -2147483648 to 2147483647. Its suffix is `&`.

## Syntax

```text
DIM name AS LONG
DIM name AS _UNSIGNED LONG
name&
name~&
```

## Behavior

A LONG takes 4 bytes. `_UNSIGNED LONG`, suffix `~&`, holds 0 to 4294967295.
<!-- src: language/numeric-semantics "Numeric types"; verification\v24_a_define; verification\v21_a_decls -->

Arithmetic on LONGs is done in 32 bits and wraps on overflow, with no error: with `x& = 2147483647`, `x& + 1` is
-2147483648.
<!-- src: language/numeric-semantics "Integer arithmetic width"; tests\reference\examples\long_1 -->

A floating-point value stored into a LONG is rounded half to even. A value outside the range is cut to its low 32
bits, with no error.
<!-- src: language/numeric-semantics "Storing into an integer variable" -->

A whole-number literal without a suffix is a LONG when it does not fit an INTEGER but fits a LONG: `40000` is one.
`-2147483648` is an `_INTEGER64`, not a LONG.
<!-- src: language/numeric-semantics "Integer literal typing" -->

## Differences

From QuickBASIC 4.5: it raises error 6, "Overflow", when LONG arithmetic leaves the LONG range. QB64Rust wraps, as
QB64pe does (`DIVERGENCES-QB45.md` Q-004).
<!-- src: Q-004 -->

## Availability

<!-- begin generated: availability -->
Supported. All 19 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:long_1 -->
```qb64
DIM population AS LONG
population = 2000000
PRINT "population ="; population; "takes"; LEN(population); "bytes"
big& = 2147483647
PRINT "2147483647 + 1 wraps to"; big& + 1
END
```

Output:

```text
population = 2000000 takes 4 bytes
2147483647 + 1 wraps to-2147483648 

Press any key to continue
```
<!-- end generated -->

## See also

`INTEGER`, `_INTEGER64`, `_UNSIGNED`, `CLNG`, `DEFLNG`.
