# SINGLE

`SINGLE` is the type of a single-precision floating-point number, with about 7 significant digits. Its suffix is
`!`. A name with no suffix and no `AS` is a SINGLE unless a `DEFINT` or a similar statement says otherwise.

## Syntax

```text
DIM name AS SINGLE
name!
```

## Behavior

A SINGLE takes 4 bytes. `PRINT` shows up to 7 significant digits: `1 / 3` stored in a SINGLE prints `.3333333`.
<!-- src: verification\v24_a_define; tests\reference\examples\single_1 -->

A name without a suffix or an `AS` clause is a SINGLE when no default-type statement covers its first letter.
<!-- src: language/default-types "Default type by first letter"; verification\v24_a_letters -->

A literal with a decimal point and at most 7 significant digits is a SINGLE, as is one with the suffix `!` or the
exponent letter `E`: `2.5`, `1.5E2`. Its value is the nearest DOUBLE to its digits, so `d# = 0.1!` stores the
DOUBLE 0.1.
<!-- src: language/numeric-semantics "Float literal typing" -->

Two floating-point values of different types are compared at the narrower type, so a SINGLE equals the literal it
was set from: with `a! = 2.1`, `a! = 2.1` is true.
<!-- src: language/numeric-semantics "Comparisons" -->

`_UNSIGNED SINGLE` is a compile error: "`SINGLE` cannot be `_UNSIGNED`".
<!-- src: language/numeric-semantics "Numeric types"; verification\v21_x07_unsigned_single -->

## Differences

From QuickBASIC 4.5:

- A SINGLE constant times a LONG, as `.01 * s&`, is a DOUBLE there, the SINGLE widened first. In QB64Rust, as in
  QB64pe, it is a SINGLE: with `s& = 69`, `WRITE #` gives `.6899999845772982` there and `.69` here. Read back by
  `INPUT #` into an INTEGER, `s& = 150` gives 1 there and 2 here (`DIVERGENCES-QB45.md` Q-021).
  <!-- src: Q-021 -->
- A SINGLE constant times an INTEGER, stored into an INTEGER, is rounded from its exact value there. QB64Rust, as
  QB64pe, rounds it to a SINGLE first: `x = .7 * x` with `x = 165` gives 115 there and 116 here. A comparison
  differs the same way: `c >= .2 * a` with `c = 15` and `a = 75` is false there and true here (Q-022).
  <!-- src: Q-022 -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:single_1 -->
```qb64
DIM price AS SINGLE
price = 19.99
PRINT "price ="; price; "takes"; LEN(price); "bytes"
PRINT "1 / 3 stored in a SINGLE:";
third! = 1 / 3: PRINT third!
END
```

Output:

```text
price = 19.99 takes 4 bytes
1 / 3 stored in a SINGLE: .3333333 

Press any key to continue
```
<!-- end generated -->

## See also

`DOUBLE`, `_FLOAT`, `CSNG`, `DEFSNG`.
