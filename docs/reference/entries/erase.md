# ERASE

`ERASE` clears arrays. A static array keeps its bounds and has every element set back to 0 or empty; a dynamic
array loses its elements until a `DIM` or `REDIM` gives it bounds again.

## Syntax

```text
ERASE name [, name]...
```

## Behavior

`ERASE` of a static array sets every element to 0, an empty string, or NUL bytes for a fixed-length string, and
keeps the bounds.
<!-- src: language/arrays-and-types "ERASE"; tests\reference\examples\erase_1 -->

`ERASE` of a dynamic array releases its elements. Until a `DIM` or `REDIM` gives it bounds again, using an element,
`LBOUND` or `UBOUND` of it raises error 9. `ERASE` of a dynamic array that has no elements does nothing.
<!-- src: language/arrays-and-types "ERASE"; language/arrays-and-types "Dynamic arrays"; tests\reference\examples\erase_1 -->

A static array of a user type has every numeric member set to 0 and every string member emptied.
<!-- src: language/arrays-and-types "ERASE" -->

`ERASE` of a member array sets its elements to 0 or empty. A variable with the array's name does not change.
<!-- src: language/arrays-and-types "Member arrays"; language/arrays-and-types "ERASE" -->

## Errors

- A name that is no declared array, such as a variable, an element or `name()`: "`x` is no declared array
  (`ERASE`)".
  <!-- src: language/arrays-and-types "ERASE" -->

## Differences

From QB64pe:

- A whole array with no storage, such as one that `ERASE` released, used whole (`PRINT a()`, `READ a()`, `SWAP`,
  `GET`) takes 8 bytes that every such array shares there, with no error, so a value read into one shows up in the
  others. QB64Rust raises error 9 and reads and writes nothing (`DIVERGENCES-QB64PE.md` D-042).
  <!-- src: D-042 -->
- `ERASE a(f(1)).d` of a `_DYNAMIC` member array calls `f` twice there. QB64Rust calls it once, as QuickBASIC 4.5
  evaluates an index once (D-055).
  <!-- src: D-055 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:erase_1 -->
```qb64
DIM fixed(3) AS LONG
fixed(2) = 9
ERASE fixed
PRINT "A static array keeps its bounds:"; UBOUND(fixed); "and is cleared:"; fixed(2)
REDIM grow(5) AS LONG
ERASE grow
ON ERROR GOTO noarray
PRINT UBOUND(grow)
REDIM grow(2) AS LONG
PRINT "Given bounds again:"; UBOUND(grow)
END

noarray:
PRINT "Error"; ERR; ": the dynamic array has no elements after ERASE"
RESUME NEXT
```

Output:

```text
A static array keeps its bounds: 3 and is cleared: 0 
Error 9 : the dynamic array has no elements after ERASE
Given bounds again: 2 

Press any key to continue
```
<!-- end generated -->

## See also

[`DIM`](dim.md), [`REDIM`](redim.md), [`CLEAR`](clear.md).
