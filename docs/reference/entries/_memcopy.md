# _MEMCOPY

`_MEMCOPY` copies a number of bytes from one memory block (`_MEM`) to another, or within one block.

## Syntax

<!-- begin generated: syntax -->
```text
_MEMCOPY sourceBlock, sourceBlock.OFFSET, sourceBlock.SIZE TO destBlock, destBlock.OFFSET
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `sourceBlock`: a `_MEM` variable, element or member: the block to copy from.
- `sourceBlock.OFFSET`: the address of the first byte to copy. It must lie in the source block.
- `sourceBlock.SIZE`: the number of bytes to copy.
- `destBlock`: a `_MEM` variable, element or member: the block to copy to. It may be the source block.
- `destBlock.OFFSET`: the address the first byte is copied to. It must lie in the destination block.

The two addresses are `_OFFSET` values, usually a block's `.OFFSET` plus a count of bytes. The names in the syntax
line are only the usual choice: any expression of a numeric type may stand there.

## Behavior

`_MEMCOPY` copies the bytes as they are, with no conversion. It copies between blocks of any kind: a variable, an
array, an image or memory from `_MEMNEW`.
<!-- src: language/mem-blocks "Reading and writing through a block"; verification\v32_b_get_put -->

Both regions must lie wholly inside their blocks, and both blocks must still be valid. A block stops being valid
when it is freed with `_MEMFREE`, when the SUB or FUNCTION whose variable it describes returns, and when its array
is redimensioned or erased.
<!-- src: language/mem-blocks "A block that is no longer valid" -->

`CALL _MEMCOPY(…)` takes the same arguments in parentheses: `CALL _MEMCOPY(m, 0, 8 TO n, 0)`.
<!-- src: language/builtin-statements "A statement with a template after CALL"; tests\callsite\memcopy_call -->

## Errors

These are compile errors:

- A first or fourth argument that is not a `_MEM` variable, element or member.
  <!-- src: verification\v32_x66_memcopy_to_long; verification\v32_x67_memcopy_from_long -->
- `TO` missing, or the destination's address left out.
  <!-- src: verification\v32_x64_memcopy_no_to; verification\v32_x65_memcopy_to_short -->

These run-time errors end the program with their message, even under `ON ERROR`: a handler does not run.
<!-- src: language/mem-blocks "A block that is no longer valid"; verification\v32_d15_copy_source_range -->

| Error | Message | When |
|---|---|---|
| 301 | Invalid size | The number of bytes is negative |
| 302 | Source memory region out of range | The bytes to copy reach outside the source block |
| 303 | Destination memory region out of range | The bytes to write reach outside the destination block |
| 304 | Source and destination memory regions out of range | Both |
| 305 | Source memory has been freed | The source block was freed |
| 306 | Destination memory has been freed | The destination block was freed |
| 313 | Source and destination memory have been freed | Both |
| 310 | Source memory not initialized | The source `_MEM` was never set |
| 311 | Destination memory not initialized | The destination `_MEM` was never set |
| 312 | Source and destination memory not initialized | Neither was set |
<!-- src: verification\v32_d24_copy_negative; verification\v32_d15_copy_source_range; verification\v32_d16_copy_dest_range; verification\v32_d17_copy_both_range; verification\v32_d18_copy_source_freed; verification\v32_d19_copy_dest_freed; verification\v32_d20_copy_both_freed; verification\v32_d21_copy_source_uninitialised; verification\v32_d22_copy_dest_uninitialised; verification\v32_d23_copy_both_uninitialised -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memcopy_1 -->
```qb64
DIM source(1 TO 5) AS INTEGER, target(1 TO 5) AS INTEGER
FOR i = 1 TO 5: source(i) = i * 10: NEXT
DIM ms AS _MEM, mt AS _MEM
ms = _MEM(source())
mt = _MEM(target())
' Copy elements 2 to 4 (three INTEGERs of 2 bytes each) to the start of target().
_MEMCOPY ms, ms.OFFSET + 2, 3 * 2 TO mt, mt.OFFSET
FOR i = 1 TO 5: PRINT target(i);: NEXT
PRINT
_MEMFREE ms
_MEMFREE mt
END
```

Output:

```text
 20  30  40  0  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MEM`, `_MEMNEW`, `_MEMGET`, `_MEMPUT`, `_MEMFILL`, `_MEMFREE`.
