# _MEMFILL

`_MEMFILL block, offset, bytes, value` fills a number of bytes of a block with a value, repeated.

## Syntax

```text
_MEMFILL block, offset, bytes, value [AS type]
```

## Behavior

`_MEMFILL m, m.OFFSET, m.SIZE, 3 AS INTEGER` sets every element of an INTEGER array to 3.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\reference\examples\_memfill_1 -->

The value is repeated byte for byte, so a count that is not a multiple of its size fills part of the last one.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\reference\examples\_memfill_1 -->

## Errors

- A string where a number is needed is a compile error.
  <!-- src: language/mem-blocks "Reading and writing through a block" -->
- `CALL _MEMFILL(...)` is a compile error, "Command not implemented".
  <!-- src: language/mem-blocks "_MEMPUT and _MEMFILL after CALL" -->

## Availability

<!-- begin generated: availability -->
Supported. All 19 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memfill_1 -->
```qb64
DIM b(10) AS INTEGER, m AS _MEM
m = _MEM(b())
_MEMFILL m, m.OFFSET, m.SIZE, 3 AS INTEGER
PRINT "b(0), b(10):"; b(0); b(10)
_MEMFILL m, m.OFFSET, 2, 255 AS _UNSIGNED _BYTE
PRINT "b(0) after two bytes of 255:"; b(0)
END
```

Output:

```text
b(0), b(10): 3  3 
b(0) after two bytes of 255:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_MEMPUT`](_memput.md), [`_MEMCOPY`](_memcopy.md).
