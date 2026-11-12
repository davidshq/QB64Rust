# _BIT

`_BIT` is the type of a whole number of a chosen number of bits, from 1 to 64. A plain `_BIT` is one bit: it holds
0 or -1.

## Syntax

```text
DIM name AS _BIT
DIM name AS _BIT * n
DIM name AS _UNSIGNED _BIT * n
name`
name`n
name~`n
```

## Arguments

- `n`: the number of bits, a number literal from 1 to 64.

## Behavior

A store keeps the value's low `n` bits. A signed `_BIT * n` then reads its top bit as the sign: with
`DIM b AS _BIT * 3`, `b = 5` holds -3. An `_UNSIGNED _BIT * n` keeps the bits as they are: 13 stored in an
`_UNSIGNED _BIT * 3` holds 5.
<!-- src: language/numeric-semantics "_BIT stores"; verification\v21_a_decls; tests\reference\examples\_bit_1 -->

A plain `_BIT` holds 0 or -1: 1 stored in it reads as -1, and 2 reads as 0. An `_UNSIGNED _BIT` holds 0 or 1.
<!-- src: language/numeric-semantics "_BIT stores"; verification\v21_a_decls -->

Each `_BIT` variable has storage of its own: 4 bytes up to 32 bits, 8 bytes above. In arithmetic, a `_BIT * n` of
up to 32 bits counts as a 32-bit integer, signed or unsigned as it is declared.
<!-- src: language/numeric-semantics "_BIT stores" -->

A `_BIT` value prints as an `_INTEGER64`, so an `_UNSIGNED _BIT * 64` above 2^63 prints as a negative number.
<!-- src: language/numeric-semantics "Numeric types"; verification\v21_a_decls -->

An array may be of a `_BIT` type of up to 63 bits, signed or unsigned: `DIM a(3) AS _BIT`.
<!-- src: language/arrays-and-types "Static arrays"; verification\v21_x27_bit_array_64 -->

## Errors

These are compile errors:

- A width of 0 or above 64: "a `_BIT` type takes 1 to 64 bits".
  <!-- src: language/numeric-semantics "Numeric types"; verification\v21_x01_bit_65; verification\v21_x02_bit_0 -->
- A constant as the width, as in `_BIT * n`: "a number of bits is expected after `_BIT *`".
  <!-- src: language/numeric-semantics "Numeric types"; verification\v21_x03_bit_const_width -->
- An array of `_BIT * 64`: "an array cannot be of a `_BIT` type of more than 63 bits (`_BIT * 64`)".
  <!-- src: verification\v21_x27_bit_array_64 -->
- `LEN` of a `_BIT` variable: "`LEN` of a `_BIT` variable".
  <!-- src: language/numeric-semantics "Numeric types"; verification\v21_x18_len_bit -->

## Differences

From QB64pe: a `_BIT * n` of more than 32 bits gets 4 bytes there, so a store into it overwrites the `_BIT`
variable declared just before it. QB64Rust gives each its own storage (`DIVERGENCES-QB64PE.md` D-009).
<!-- src: D-009; language/numeric-semantics "_BIT stores" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: DIM a(3) AS _BIT : a(1) = 1 -->

## Example

<!-- begin generated: example:_bit_1 -->
```qb64
DIM flag AS _BIT
flag = 1
PRINT "A _BIT set to 1 holds"; flag
DIM small AS _BIT * 3
small = 3: PRINT "3 in _BIT * 3:"; small
small = 5: PRINT "5 in _BIT * 3:"; small
DIM u AS _UNSIGNED _BIT * 3
u = 13: PRINT "13 in _UNSIGNED _BIT * 3:"; u
END
```

Output:

```text
A _BIT set to 1 holds-1 
3 in _BIT * 3: 3 
5 in _BIT * 3:-3 
13 in _UNSIGNED _BIT * 3: 5 

Press any key to continue
```
<!-- end generated -->

## See also

`_BYTE`, `_UNSIGNED`, `_SETBIT`, `_READBIT`.
