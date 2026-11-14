# _ARRAYCOPY

`_ARRAYCOPY` copies elements from one array to another of the same element type: a whole array, or a range of it,
to a starting element of the other.

## Syntax

```text
_ARRAYCOPY source() TO destination()
_ARRAYCOPY source(lo TO hi) TO destination(start)
_ARRAYCOPY source() TO destination(start)
_ARRAYCOPY source(lo TO hi) TO destination()
```

With several dimensions, one range or start for each, separated by commas.

## Behavior

The elements are copied as if all were read before any is written, so the two sides may overlap or be one array.
`STRING` elements are the destination's own afterwards.
<!-- src: language/arrays-and-types "_ARRAYCOPY"; tests\reference\examples\_arraycopy_1 -->

Both sides have one element type: a number type, `STRING`, or `STRING * n` of one length. Each side may be an array
of any kind, or a member array.
<!-- src: language/arrays-and-types "_ARRAYCOPY element types and places" -->

## Errors

- Before it copies, `_ARRAYCOPY` raises run-time error 9 and copies nothing when an array has no elements, a range
  runs backwards or lies outside its array, or the elements do not fit in the destination from the start.
  <!-- src: language/arrays-and-types "_ARRAYCOPY at run time" -->
- Arrays of two element types are a compile error: "`_ARRAYCOPY` copies between arrays of one element type, not
  `LONG` and `INTEGER`". So are a side that is no array and a form the six do not cover.
  <!-- src: language/arrays-and-types "_ARRAYCOPY checks" -->

## Differences

From QB64pe: it copies the wrong elements of a `_FLOAT` array, unless the whole array goes to `d()`. QB64Rust copies
`_FLOAT` arrays element by element (`DIVERGENCES-QB64PE.md` D-025).
<!-- src: D-025 -->

## Availability

<!-- begin generated: availability -->
Supported. All 31 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_arraycopy_1 -->
```qb64
DIM src(1 TO 5) AS LONG, dst(0 TO 9) AS LONG
FOR i = 1 TO 5: src(i) = i * 10: NEXT i
_ARRAYCOPY src(2 TO 4) TO dst(0)
PRINT dst(0); dst(1); dst(2); dst(3)
_ARRAYCOPY src() TO dst(5)
PRINT dst(5); dst(9)
END
```

Output:

```text
 20  30  40  0 
 10  50 

Press any key to continue
```
<!-- end generated -->

## See also

[`DIM`](dim.md), [`_MEMCOPY`](_memcopy.md), [`SWAP`](swap.md).
