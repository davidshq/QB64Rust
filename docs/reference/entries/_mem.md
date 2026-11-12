# _MEM

`_MEM` is the type of a memory block, and `_MEM(variable)` gives the block of a variable, an array or an element.
`_MEM(offset, size)` gives a block at an address.

## Syntax

```text
DIM block AS _MEM
block = _MEM(reference)
block = _MEM(offset, size)
```

## Behavior

A `_MEM` has the members `OFFSET`, `SIZE`, `TYPE`, `ELEMENTSIZE` (`_OFFSET` values), `IMAGE` and `SOUND` (LONG). It
may be used wherever a user type may, and a whole value copies.
<!-- src: language/mem-blocks "The _MEM type"; tests\corpus\slice\s88_mem_new_image -->

The block of a LONG has `SIZE` 4, `TYPE` 132 and `ELEMENTSIZE` 4, and its `OFFSET` is `_OFFSET` of the LONG. A
`STRING * 6` has `SIZE` 6 and `TYPE` 512.
<!-- src: language/mem-blocks "The _MEM type"; language/mem-blocks "A block of a reference"; language/mem-blocks "The address of a reference"; tests\reference\examples\_mem_1 -->

The block of an element reaches from it to the array's end; `_MEMELEMENT` gives the element alone. A member, a member
array and a whole array `a()` each have a block.
<!-- src: language/mem-blocks "A block of a reference"; tests\corpus\slice\s89_mem_references -->

In a value of a `TYPE` with a `STRING` member, the member counts 8 bytes.
<!-- src: language/mem-blocks "A block of a reference" -->

`_MEM(_OFFSET(a), 4)` is a block of 4 bytes at that address, of `TYPE` 16384.
<!-- src: language/mem-blocks "A block of an address" -->

A block stops being valid when `_MEMFREE` frees it, when the SUB or FUNCTION whose variable it describes returns, and
when its array is redimensioned or erased.
<!-- src: language/mem-blocks "A block that is no longer valid" -->

## Errors

- A store to a member, such as `m.SIZE = 4`, is the compile error "Cannot set read-only element of _MEM TYPE".
  <!-- src: language/mem-blocks "The _MEM type" -->
- `_MEM` of a variable-length string is the compile error "_MEM cannot reference variable-length strings".
  <!-- src: language/mem-blocks "A block of a reference" -->
- `_MEM(5, 4)`, whose address is not an `_OFFSET` value, is the compile error "Expected _MEM(_OFFSET-value,...)".
  <!-- src: language/mem-blocks "A block of an address" -->
- A negative size, `_MEM(o, -1)`, raises error 301, "Invalid size", which ends the program even under `ON ERROR`.
  <!-- src: language/mem-blocks "A block that is no longer valid"; tests\corpus\slice\s92_mem_error_301 -->

## Differences

`_MEM(p())` of an array parameter whose number of dimensions nothing above it has given takes the array's own
lock. QB64pe read the array's first 8 bytes as the lock there, so its block stopped being valid when element 0
changed.
<!-- src: D-016 -->

The block of a dynamic array that `ERASE` releases stops being valid, as after `REDIM`; QB64pe keeps it valid and
writes into freed memory.
<!-- src: language/mem-blocks "A block that is no longer valid"; D-043 -->

`_MEM(x.v())` of a `_DYNAMIC` member array that is not allocated, as after `ERASE x.v`, raises error 9 and gives a
block for which `_MEMEXISTS` is 0. QB64pe's program crashes there (`DIVERGENCES-QB64PE.md` D-021).
<!-- src: D-021 -->

## Availability

<!-- begin generated: availability -->
Supported. All 29 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mem_1 -->
```qb64
DIM a AS LONG, m AS _MEM
m = _MEM(a)
PRINT "SIZE, TYPE, ELEMENTSIZE:"; m.SIZE; m.TYPE; m.ELEMENTSIZE
PRINT "OFFSET is the address of a:"; m.OFFSET = _OFFSET(a)
DIM s AS STRING * 6
m = _MEM(s)
PRINT "A STRING * 6:"; m.SIZE; m.TYPE
END
```

Output:

```text
SIZE, TYPE, ELEMENTSIZE: 4  132  4 
OFFSET is the address of a:-1 
A STRING * 6: 6  512 

Press any key to continue
```
<!-- end generated -->

## See also

`_MEMELEMENT`, `_MEMGET`, `_MEMPUT`, `_MEMCOPY`, `_MEMNEW`, `_OFFSET`.
