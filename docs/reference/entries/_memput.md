# _MEMPUT

`_MEMPUT block, offset, value` writes a value's bytes into a block, at an address inside it.

## Syntax

```text
_MEMPUT block, offset, value [AS type]
```

## Behavior

`_MEMPUT m, m.OFFSET, 7 AS LONG` into the block of a LONG sets it to 7.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\corpus\slice\s90_mem_get_put; tests\reference\examples\_memput_1 -->

With `AS type` the value is converted to that type first, as an assignment converts it: `2.7 AS LONG` writes 3, `300
AS _UNSIGNED _BYTE` writes 44.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\corpus\slice\s90_mem_get_put -->

Without it, a variable writes its own bytes, and a string its characters.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\corpus\slice\s90_mem_get_put; tests\reference\examples\_memput_1 -->

## Errors

- A string where a number is needed, such as `"ab" AS LONG`, is the compile error "Illegal string-number conversion".
  A user type that holds a variable-length `STRING` is "UDT must have fixed size".
  <!-- src: language/mem-blocks "Reading and writing through a block" -->
- `CALL _MEMPUT(...)` is a compile error, "Command not implemented".
  <!-- src: language/mem-blocks "_MEMPUT and _MEMFILL after CALL" -->

## Availability

<!-- begin generated: availability -->
Supported. All 26 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memput_1 -->
```qb64
DIM a AS LONG, m AS _MEM
m = _MEM(a)
_MEMPUT m, m.OFFSET, 7 AS LONG
PRINT "a is"; a
DIM n AS _MEM
n = _MEMNEW(8)
_MEMPUT n, n.OFFSET, "abcdefgh"
PRINT _MEMGET(n, n.OFFSET + 2, STRING * 3)
_MEMFREE n
END
```

Output:

```text
a is 7 
cde

Press any key to continue
```
<!-- end generated -->

## See also

`_MEMGET`, `_MEMFILL`, `_MEMCOPY`, `_MEM`.
