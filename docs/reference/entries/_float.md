# _FLOAT

`_FLOAT` is the type of an extended-precision floating-point number, more precise than DOUBLE. Its suffix is `##`.

## Syntax

```text
DIM name AS _FLOAT
name##
```

## Behavior

`LEN` of a `_FLOAT` variable is 32.
<!-- src: verification\v24_a_define; tests\reference\examples\_float_1 -->

`PRINT` shows a `_FLOAT` with DOUBLE's 16 significant digits: `1 / 3` stored in one prints `.3333333333333333`.
<!-- src: tests\reference\examples\_float_1 -->

A literal with more than 16 significant digits is a `_FLOAT`, as is one with the suffix `##` or the exponent letter
`F`.
<!-- src: language/numeric-semantics "Float literal typing" -->

`/` of two integers is computed in `_FLOAT`.
<!-- src: language/numeric-semantics "Division" -->

`_UNSIGNED _FLOAT` is a compile error: "`_FLOAT` cannot be `_UNSIGNED`".
<!-- src: language/numeric-semantics "Numeric types" -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_float_1 -->
```qb64
DIM f AS _FLOAT
f = 2.5
PRINT "f ="; f; "takes"; LEN(f); "bytes"
f = 1 / 3: PRINT "1 / 3 stored:"; f
g## = 0.1: PRINT "With the suffix ##:"; g##
END
```

Output:

```text
f = 2.5 takes 32 bytes
1 / 3 stored: .3333333333333333 
With the suffix ##: .1 

Press any key to continue
```
<!-- end generated -->

## See also

[`DOUBLE`](double.md), [`SINGLE`](single.md), [`CDBL`](cdbl.md).
