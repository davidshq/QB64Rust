# Arrays and user types

An array holds elements of one type in one or more dimensions, static or dynamic. A `TYPE` block defines a user type
of members, single values or arrays.

The pages of this area: [DIM](../entries/dim.md), [REDIM](../entries/redim.md), [ERASE](../entries/erase.md),
[LBOUND](../entries/lbound.md), [UBOUND](../entries/ubound.md), [OPTION](../entries/option.md),
[TYPE](../entries/type.md), [_ARRAYCOPY](../entries/_arraycopy.md), [COMMON](../entries/common.md),
[$DYNAMIC](../entries/meta-dynamic.md), [$STATIC](../entries/meta-static.md), [SWAP](../entries/swap.md).

## Arrays

`DIM name(bounds)` declares an array; each index of an element is converted to an integer, and one outside its
dimension's bounds raises error 9.
<!-- src: language/arrays-and-types "Static arrays"; language/arrays-and-types "Array elements"; tests\reference\examples\dim_1 -->

An array is dynamic when its first declaration is a `REDIM`, a `DIM` with a bound that is no constant, a `DIM` in a
procedure that is not `STATIC`, or a declaration under `$DYNAMIC`. `REDIM` gives it new bounds, `REDIM _PRESERVE`
keeping the values.
<!-- src: language/arrays-and-types "Dynamic arrays"; language/arrays-and-types "REDIM _PRESERVE"; tests\reference\examples\redim_1 -->

`ERASE` sets a static array's elements to zero or empty and releases a dynamic one.
<!-- src: language/arrays-and-types "ERASE"; tests\reference\examples\erase_1 -->

A name used with an index and declared nowhere makes an implicit array, with bounds from `OPTION BASE` to 10 in each
dimension.
<!-- src: language/arrays-and-types "Implicit arrays"; language/arrays-and-types "OPTION BASE" -->

## User types

A `TYPE` block's members are numbers, fixed-length strings, `STRING`s and earlier user types, each a single value or
an array.
<!-- src: language/arrays-and-types "User types"; language/arrays-and-types "Member arrays"; tests\reference\examples\type_1 -->

A whole user-type value copies all its members, and `target() = source()` copies a whole array, deeply: strings and
dynamic member arrays are copied, not shared.
<!-- src: language/arrays-and-types "Whole user-type values"; language/arrays-and-types "Whole-array assignment" -->

## Copying elements

`_ARRAYCOPY` copies a range of elements from one array into another, as if every element were read before any is
written.
<!-- src: language/arrays-and-types "_ARRAYCOPY"; tests\reference\examples\_arraycopy_1 -->
