# REDIM

`REDIM` gives a dynamic array new bounds while the program runs: `REDIM scores(1 TO n) AS INTEGER`. Plain `REDIM`
clears the elements; `REDIM _PRESERVE` keeps them in order, and `REDIM _RETAIN` keeps them at their indexes.

## Syntax

```text
REDIM [_PRESERVE | _RETAIN] [SHARED] name(bounds) [AS type] [, name(bounds) [AS type]]...
```

## Behavior

`REDIM` declares the array if it does not exist yet, as a dynamic array. Each time it runs it gives the array the
new bounds and new elements, 0 or empty.
<!-- src: language/arrays-and-types "REDIM"; tests\reference\examples\redim_1 -->

Without `AS`, `REDIM` of an array declared with `AS` keeps that type: after `REDIM x(5) AS LONG`, `REDIM x(10)` is
still a LONG array.
<!-- src: language/arrays-and-types "REDIM" -->

`REDIM _PRESERVE` keeps as many element values as the smaller of the old and new sizes, by their position in
memory, the first dimension fastest. With two dimensions, values keep their positions, not their indexes.
<!-- src: language/arrays-and-types "REDIM _PRESERVE"; tests\reference\examples\redim_1 -->

`REDIM _RETAIN` keeps each element whose indexes lie within both the old and the new bounds, at the same indexes.
Every other element is 0 or empty: after `REDIM _RETAIN a(3 TO 7)` of an array 1 to 5, `a(3)` to `a(5)` keep their
values.
<!-- src: language/arrays-and-types "REDIM _RETAIN"; tests\reference\examples\redim_1 -->

An array that is not allocated, after `ERASE` or before its first `REDIM`, is given its bounds by `_PRESERVE` and
`_RETAIN` as by a plain `REDIM`.
<!-- src: language/arrays-and-types "REDIM _PRESERVE"; language/arrays-and-types "REDIM _RETAIN" -->

`SHARED` shares the array only in the statement that declares it. On a later `REDIM` of the array it changes
nothing.
<!-- src: language/arrays-and-types "REDIM" -->

A dimension with no element raises error 5, as `DIM` does, also with the fix `dim-empty-error`
([Fixes](../chapters/fixes.md)). `REDIM a()` with empty parentheses declares the variable `a`, not an array.
<!-- src: language/arrays-and-types "REDIM"; language/arrays-and-types "Dynamic arrays" -->

## Errors

- `REDIM` of a static array raises run-time error 10 and leaves it unchanged. Under `'$DYNAMIC`, or with a bound
  that is not a constant, `DIM` makes a dynamic array, which `REDIM` can change.
  <!-- src: language/arrays-and-types "REDIM"; language/arrays-and-types "Dynamic arrays"; tests\reference\examples\meta-static_1 -->

These are compile errors:

- Another number of dimensions than the array has: "`m` has 2 dimensions, not 1".
  <!-- src: language/arrays-and-types "REDIM" -->
- Another type after `AS` than the array was declared with: "name already in use: `x`".
  <!-- src: language/arrays-and-types "REDIM" -->
- `_PRESERVE` and `_RETAIN` together, either twice, `SHARED` before them, or `REDIM _RETAIN` with no array.
  <!-- src: language/arrays-and-types "REDIM" -->

## Differences

From QuickBASIC 4.5:

- `REDIM a()` with empty parentheses is a syntax error there (`DIVERGENCES-QB45.md` Q-032).
  <!-- src: Q-032 -->
- `REDIM d(3)` without `AS` after `REDIM d(2) AS DOUBLE` is "AS clause required" there. QB64Rust keeps the type, as
  QB64pe does (Q-034).
  <!-- src: Q-034 -->

From QB64pe:

- A FUNCTION in a bound that itself redims or erases the same array, as `REDIM z(2 TO shrinkz)`, makes QB64pe's
  program end with no output, as memory is freed twice. QB64Rust evaluates the bounds first and then replaces the
  array as the FUNCTION left it (`DIVERGENCES-QB64PE.md` D-018).
  <!-- src: D-018 -->
- A dimension of fewer than no element, as `REDIM r(2 TO 0, 3)`, raises error 5 there and then ends the program with
  "Out of memory", past any `ON ERROR` handler. QB64Rust raises error 5 and allocates nothing, so a handler can go on
  (D-040).
  <!-- src: D-040 -->
- A `_DYNAMIC` member array given more elements than memory holds succeeds there with a block too small. QB64Rust
  ends the program with "Out of memory", as for any array that is too large (D-020).
  <!-- src: D-020 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:redim_1 -->
```qb64
REDIM scores(1 TO 3) AS INTEGER
scores(1) = 90: scores(2) = 80: scores(3) = 70
REDIM _PRESERVE scores(1 TO 5) AS INTEGER
PRINT "After _PRESERVE:"; scores(1); scores(3); scores(5)
REDIM _RETAIN scores(3 TO 6) AS INTEGER
PRINT "After _RETAIN to 3 TO 6:"; scores(3); scores(4); scores(6)
REDIM scores(1 TO 2) AS INTEGER
PRINT "After a plain REDIM:"; scores(1); UBOUND(scores)
END
```

Output:

```text
After _PRESERVE: 90  70  0 
After _RETAIN to 3 TO 6: 70  0  0 
After a plain REDIM: 0  2 

Press any key to continue
```
<!-- end generated -->

## See also

`DIM`, `ERASE`, `$DYNAMIC`, `LBOUND`, `UBOUND`.
