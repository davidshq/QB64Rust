# DOUBLE

`DOUBLE` is the type of a double-precision floating-point number, with about 16 significant digits. Its suffix is
`#`.

## Syntax

```text
DIM name AS DOUBLE
name#
```

## Behavior

A DOUBLE takes 8 bytes. `PRINT` shows up to 16 significant digits: `1 / 3` stored in a DOUBLE prints
`.3333333333333333`.
<!-- src: verification\v24_a_define; tests\reference\examples\double_1 -->

A literal with 8 to 16 significant digits, or one outside SINGLE's range with at most 16, is a DOUBLE, as is one with the suffix `#` or the exponent letter `D`.
<!-- src: language/numeric-semantics "Float literal typing" -->

`/` of two integers is computed in `_FLOAT` and printed with DOUBLE's 16 digits: `PRINT 1 / 3` prints
`.3333333333333333`. With a SINGLE or DOUBLE operand, the result has the wider of their types.
<!-- src: language/numeric-semantics "Division" -->

A DOUBLE compared with a SINGLE is compared at SINGLE's precision.
<!-- src: language/numeric-semantics "Comparisons"; verification\v17_g_ops -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:double_1 -->
```qb64
DIM distance AS DOUBLE
distance = 1 / 3
PRINT "distance ="; distance; "takes"; LEN(distance); "bytes"
third! = 1 / 3
PRINT "1 / 3 as SINGLE:"; third!; "and as DOUBLE:"; distance
END
```

Output:

```text
distance = .3333333333333333 takes 8 bytes
1 / 3 as SINGLE: .3333333 and as DOUBLE: .3333333333333333 

Press any key to continue
```
<!-- end generated -->

## See also

[`SINGLE`](single.md), [`_FLOAT`](_float.md), [`CDBL`](cdbl.md), [`DEFDBL`](defint.md).
