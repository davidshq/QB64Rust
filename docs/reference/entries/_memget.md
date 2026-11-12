# _MEMGET

`_MEMGET(block, offset, type)` reads a value of a type from a block. The statement `_MEMGET block, offset, variable`
reads into a variable.

## Syntax

```text
value = _MEMGET(block, offset, type)
_MEMGET block, offset, variable
```

## Behavior

The same byte reads as -5 as a `_BYTE` and 251 as an `_UNSIGNED _BYTE`: the bytes are read as they are.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\corpus\slice\s90_mem_get_put; tests\reference\examples\_memget_1 -->

The statement reads as many bytes as the variable takes.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\corpus\slice\s90_mem_get_put; tests\reference\examples\_memget_1 -->

`STRING * n` and a user type are types it reads too.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\corpus\slice\s90_mem_get_put -->

## Errors

- A string, `_BIT` or user type where a number is needed is a compile error, as is a user type that holds a
  variable-length `STRING` as the statement's variable.
  <!-- src: language/mem-blocks "Reading and writing through a block" -->

## Availability

<!-- begin generated: availability -->
Supported. All 33 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memget_1 -->
```qb64
DIM m AS _MEM, a AS LONG
m = _MEMNEW(8)
_MEMPUT m, m.OFFSET, -5 AS _BYTE
PRINT "As _BYTE and _UNSIGNED _BYTE:"; _MEMGET(m, m.OFFSET, _BYTE); _MEMGET(m, m.OFFSET, _UNSIGNED _BYTE)
_MEMPUT m, m.OFFSET, -12345 AS LONG
_MEMGET m, m.OFFSET, a
PRINT "The statement into a LONG:"; a
_MEMFREE m
END
```

Output:

```text
As _BYTE and _UNSIGNED _BYTE:-5  251 
The statement into a LONG:-12345 

Press any key to continue
```
<!-- end generated -->

## See also

`_MEMPUT`, `_MEM`, `PEEK`.
