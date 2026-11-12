# _INTEGER64

`_INTEGER64` is the type of a 64-bit whole number, from -9223372036854775808 to 9223372036854775807. Its suffix
is `&&`.

## Syntax

```text
DIM name AS _INTEGER64
DIM name AS _UNSIGNED _INTEGER64
name&&
name~&&
```

## Behavior

An `_INTEGER64` takes 8 bytes. `_UNSIGNED _INTEGER64`, suffix `~&&`, holds 0 to 18446744073709551615.
<!-- src: language/numeric-semantics "Numeric types"; verification\v24_a_define; verification\v21_a_decls -->

Arithmetic with an `_INTEGER64` operand is done in 64 bits and wraps on overflow, with no error: the largest value
plus 1 is -9223372036854775808.
<!-- src: language/numeric-semantics "Integer arithmetic width"; tests\reference\examples\_integer64_1 -->

A floating-point value stored into an `_INTEGER64` is rounded half to even.
<!-- src: language/numeric-semantics "Storing into an integer variable" -->

A whole-number literal without a suffix is an `_INTEGER64` when it does not fit a LONG, and so is `-2147483648`.
A literal above 9223372036854775807, up to 18446744073709551615, is an `_UNSIGNED _INTEGER64`. A larger one is a
compile error.
<!-- src: language/numeric-semantics "Integer literal typing" -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_integer64_1 -->
```qb64
DIM total AS _INTEGER64
total = 3000000000
PRINT "total ="; total; "takes"; LEN(total); "bytes"
PRINT "total * 3 ="; total * 3
x&& = 9223372036854775807
PRINT "The largest value plus 1 wraps to"; x&& + 1
END
```

Output:

```text
total = 3000000000 takes 8 bytes
total * 3 = 9000000000 
The largest value plus 1 wraps to-9223372036854775808 

Press any key to continue
```
<!-- end generated -->

## See also

`LONG`, `_UNSIGNED`, `_OFFSET`.
