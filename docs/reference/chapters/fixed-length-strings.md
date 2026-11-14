# Fixed-length strings

`STRING * n` is a string of exactly `n` bytes: a store cuts or pads with spaces, and it reads back with all its bytes.

The pages of this area: [STRING](../entries/string.md), [DIM](../entries/dim.md), [TYPE](../entries/type.md),
[LSET](../entries/lset.md).

## Storing and reading

A store copies the first `n` bytes and pads the rest with spaces; reading gives all `n` bytes, padding included, also
in comparisons.
<!-- src: language/fixed-length-strings "Assignment to a fixed-length string"; tests\reference\examples\string_1 -->

An element of an array or a member of a `TYPE` starts as NUL bytes, not spaces.
<!-- src: language/fixed-length-strings "Fixed-length strings in arrays and types" -->

The length is a literal or a numeric constant, read as a 32-bit integer; 0 or below is a compile error.
<!-- src: language/fixed-length-strings "Fixed-length string declarations" -->

## Passing one

To a `STRING` parameter it is passed by reference, also in parentheses: each store reaches it cut and padded to `n`.
<!-- src: language/fixed-length-strings "Fixed-length strings as arguments" -->
