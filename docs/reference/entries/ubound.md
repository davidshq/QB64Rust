# UBOUND

`UBOUND` gives the highest index of a dimension of an array: for `DIM names(1 TO 3)`, `UBOUND(names)` is 3. It is
the usual limit of a `FOR` loop over the array.

## Syntax

<!-- begin generated: syntax -->
```text
UBOUND(arrayName[, dimension%])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `array`: an array's name, without parentheses, an array parameter or a member array.
- `dimension`: which dimension, 1 for the first. It is 1 when left out.

## Result

An `_INTEGER64`: `UBOUND(b) / 7` prints with DOUBLE's digits.

## Behavior

`UBOUND(a)` gives the upper bound of the first dimension, and `UBOUND(a, 2)` that of the second: for
`DIM b(-2 TO 3, 5)`, `UBOUND(b)` is 3 and `UBOUND(b, 2)` is 5.
<!-- src: language/arrays-and-types "LBOUND and UBOUND"; tests\reference\examples\ubound_1 -->

A member array may be written with or without empty parentheses, and the indexes of the place it is part of are not
evaluated.
<!-- src: language/arrays-and-types "LBOUND and UBOUND"; language/arrays-and-types "Member arrays" -->

A name that is no array makes an implicit array of 0 to 10, so `UBOUND(q)` is 10.
<!-- src: language/arrays-and-types "LBOUND and UBOUND" -->

## Errors

- A dimension outside 1 to the array's number of dimensions raises run-time error 9. So does a dynamic array with
  no elements, such as one after `ERASE`.
  <!-- src: language/arrays-and-types "LBOUND and UBOUND"; tests\reference\examples\erase_1 -->
- An array variable written with parentheses, `UBOUND(a())`, is a compile error.
  <!-- src: language/arrays-and-types "LBOUND and UBOUND" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:ubound_1 -->
```qb64
DIM names(1 TO 3) AS STRING
names(1) = "Ney": names(2) = "Murat": names(3) = "Soult"
FOR i = LBOUND(names) TO UBOUND(names)
    PRINT i; names(i)
NEXT i
END
```

Output:

```text
 1 Ney
 2 Murat
 3 Soult

Press any key to continue
```
<!-- end generated -->

## See also

`LBOUND`, `DIM`, `REDIM`.
