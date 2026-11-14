# Memory blocks

A `_MEM` value describes a block of memory: a variable, an array, an image, a sound or new memory. The program reads
and writes bytes through it, and the runtime checks every access.

The pages of this area: [_MEM](../entries/_mem.md), [_MEMELEMENT](../entries/_memelement.md),
[_MEMNEW](../entries/_memnew.md), [_MEMIMAGE](../entries/_memimage.md), [_MEMSOUND](../entries/_memsound.md),
[_MEMGET](../entries/_memget.md), [_MEMPUT](../entries/_memput.md), [_MEMFILL](../entries/_memfill.md),
[_MEMCOPY](../entries/_memcopy.md), [_MEMFREE](../entries/_memfree.md), [_MEMEXISTS](../entries/_memexists.md),
[_OFFSET](../entries/_offset.md).

## Taking a block

`_MEM(variable)` gives the block of a variable, an element (from it to the array's end), a whole array or a member;
`_MEMELEMENT` the element alone. `_MEMNEW`, `_MEMIMAGE` and `_MEMSOUND` give new memory, an image's pixels and a
sound's samples.
<!-- src: language/mem-blocks "A block of a reference"; language/mem-blocks "New memory, images and sounds"; tests\corpus\slice\s89_mem_references -->

A block's `SIZE`, `TYPE` and `ELEMENTSIZE` describe it: the block of a LONG is 4 bytes of `TYPE` 132.
<!-- src: language/mem-blocks "The _MEM type"; tests\reference\examples\_mem_1 -->

## Reading and writing

`_MEMGET`, `_MEMPUT`, `_MEMFILL` and `_MEMCOPY` read and write bytes as they are, at an address inside the block: the
block's `OFFSET` plus a count.
<!-- src: language/mem-blocks "Reading and writing through a block"; tests\corpus\slice\s90_mem_get_put -->

## When a block stops being valid

A block stops being valid when it is freed, when the procedure whose variable it describes returns, and when its array
is redimensioned or erased. An access through it, or outside its region, is a run-time error from 300 to 313 that ends
the program even under `ON ERROR`.
<!-- src: language/mem-blocks "A block that is no longer valid"; tests\corpus\slice\s98_mem_error_307 -->

Below `$CHECKING:OFF` the reads and writes are not checked.
<!-- src: language/run-time-checks "Memory statements without checks" -->
