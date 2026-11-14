# LBOUND

`LBOUND` gives the lowest index of a dimension of an array: for `DIM board(1 TO 8)`, `LBOUND(board)` is 1.
`UBOUND` gives the highest.

## Syntax

<!-- begin generated: syntax -->
```text
LBOUND(arrayName[, dimension%])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `array`: an array's name, without parentheses, an array parameter or a member array.
- `dimension`: which dimension, 1 for the first. It is 1 when left out.

## Result

An `_INTEGER64`.

## Behavior

`LBOUND(a)` gives the lower bound of the first dimension, and `LBOUND(a, 2)` that of the second. A dimension
declared without `TO` starts at 0, or at 1 under `OPTION BASE 1`.
<!-- src: language/arrays-and-types "LBOUND and UBOUND"; tests\reference\examples\lbound_1 -->

A dimension that is not a whole number is rounded half to even.
<!-- src: language/arrays-and-types "LBOUND and UBOUND" -->

A member array may be written with or without empty parentheses, `LBOUND(x(0).n)`, and the indexes of the place it
is part of are not evaluated.
<!-- src: language/arrays-and-types "LBOUND and UBOUND"; language/arrays-and-types "Member arrays" -->

A name that is no array makes an implicit array of 0 to 10, so `LBOUND(q)` is 0.
<!-- src: language/arrays-and-types "LBOUND and UBOUND" -->

## Errors

- A dimension outside 1 to the array's number of dimensions raises run-time error 9. So does a dynamic array with
  no elements.
  <!-- src: language/arrays-and-types "LBOUND and UBOUND" -->
- An array variable written with parentheses, `LBOUND(a())`, is a compile error: "`LBOUND` takes the array's name
  without `()`".
  <!-- src: language/arrays-and-types "LBOUND and UBOUND" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:lbound_1 -->
```qb64
DIM board(1 TO 8, 0 TO 7)
PRINT "Rows"; LBOUND(board); "to"; UBOUND(board)
PRINT "Columns"; LBOUND(board, 2); "to"; UBOUND(board, 2)
DIM plain(5)
PRINT "A dimension without TO starts at"; LBOUND(plain)
END
```

Output:

```text
Rows 1 to 8 
Columns 0 to 7 
A dimension without TO starts at 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`UBOUND`](ubound.md), [`DIM`](dim.md), [`OPTION`](option.md).
