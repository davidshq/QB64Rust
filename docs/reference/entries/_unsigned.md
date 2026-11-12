# _UNSIGNED

`_UNSIGNED` before an integer type makes it hold only zero and positive numbers, twice as large: an
`_UNSIGNED INTEGER` holds 0 to 65535. In a suffix it is written `~`.

## Syntax

```text
DIM name AS _UNSIGNED type
name~%%   name~%   name~&   name~&&   name~%&   name~`n
```

## Arguments

- `type`: `_BYTE`, `INTEGER`, `LONG`, `_INTEGER64`, `_OFFSET` or `_BIT` (with or without `* n`).

## Behavior

An unsigned type takes the same bytes as its signed type, and holds 0 to 2 to the power of its bits, less 1:
`_UNSIGNED _BYTE` 0 to 255, `_UNSIGNED INTEGER` 0 to 65535, `_UNSIGNED LONG` 0 to 4294967295 and
`_UNSIGNED _INTEGER64` 0 to 18446744073709551615.
<!-- src: language/numeric-semantics "Numeric types"; verification\v21_a_decls; verification\v24_a_define -->

A negative value stored into an unsigned variable is read as its bits: -1 stored in an `_UNSIGNED INTEGER` reads
as 65535, and in an `_UNSIGNED _INTEGER64` as 18446744073709551615.
<!-- src: language/numeric-semantics "Storing into an integer variable"; verification\v21_a_decls; tests\reference\examples\_unsigned_1 -->

The suffix of each unsigned type is the signed type's suffix after `~`: `~%%`, `~%`, `~&`, `~&&`, `~%&`, and
`` ~` `` or `` ~`n `` for `_BIT`. One name with each suffix is a variable of its own.
<!-- src: language/numeric-semantics "Numeric types"; verification\v21_a_suffixes -->

`_UNSIGNED STRING` is taken as `STRING` in a declaration.
<!-- src: language/numeric-semantics "Numeric types"; verification\v21_a_decls -->

## Errors

These are compile errors:

- `_UNSIGNED` before `SINGLE`, `DOUBLE` or `_FLOAT`: "`SINGLE` cannot be `_UNSIGNED`", with the type's name.
  <!-- src: language/numeric-semantics "Numeric types"; verification\v21_x07_unsigned_single -->
- `AS _UNSIGNED` with no type after it: "`_UNSIGNED` needs a type after it".
  <!-- src: language/numeric-semantics "Numeric types"; verification\v21_x08_unsigned_alone -->

## Availability

<!-- begin generated: availability -->
Supported. All 27 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_unsigned_1 -->
```qb64
DIM u AS _UNSIGNED INTEGER
u = 65535: PRINT "_UNSIGNED INTEGER 65535:"; u
u = -1: PRINT "-1 stored:"; u
DIM s AS INTEGER
s = -1: PRINT "-1 in a signed INTEGER:"; s
v~& = 4294967295: PRINT "With the suffix ~&:"; v~&
END
```

Output:

```text
_UNSIGNED INTEGER 65535: 65535 
-1 stored: 65535 
-1 in a signed INTEGER:-1 
With the suffix ~&: 4294967295 

Press any key to continue
```
<!-- end generated -->

## See also

[`_BYTE`](_byte.md), [`INTEGER`](integer.md), [`LONG`](long.md), [`_INTEGER64`](_integer64.md), [`_BIT`](_bit.md).
