# _MEMEXISTS

`_MEMEXISTS(block)` tells whether a block is still valid.

## Syntax

<!-- begin generated: syntax -->
```text
_MEMEXISTS(memBlock)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

-1 or 0, a LONG.

## Behavior

A new block gives -1, one freed 0, and a `_MEM` never set 0.
<!-- src: language/mem-blocks "New memory, images and sounds"; tests\reference\examples\_memexists_1 -->

The block of an image stops being valid when the image is freed.
<!-- src: tests\corpus\slice\s88_mem_new_image -->

A block stops being valid when `_MEMFREE` frees it, when the SUB or FUNCTION whose variable it describes returns, and
when its array is redimensioned or erased.
<!-- src: language/mem-blocks "A block that is no longer valid" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `PRINT _MEMEXISTS(_MEMNEW(4))`: "`_MEMEXISTS` needs a `_MEM` variable, element or member here".
  - `PRINT _MEMEXISTS(k)`: "`_MEMEXISTS` needs a `_MEM` variable, element or member here".
  - `PRINT _MEMEXISTS`: "`_MEMEXISTS` takes 1 argument".
  <!-- src: language/mem-blocks "New memory, images and sounds"; verification\v32_x03_memexists_of_result; verification\v32_x72_memexists_long; verification\v32_x73_memexists_bare -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memexists_1 -->
```qb64
DIM n AS _MEM, never AS _MEM
n = _MEMNEW(8)
PRINT "New:"; _MEMEXISTS(n)
_MEMFREE n
PRINT "Freed:"; _MEMEXISTS(n)
PRINT "Never set:"; _MEMEXISTS(never)
END
```

Output:

```text
New:-1 
Freed: 0 
Never set: 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_MEMFREE`](_memfree.md), [`_MEMNEW`](_memnew.md).
