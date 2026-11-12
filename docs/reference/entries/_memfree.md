# _MEMFREE

`_MEMFREE block` frees a block from `_MEMNEW`, `_MEMIMAGE` or `_MEM`. Every copy of the block stops being valid.

## Syntax

<!-- begin generated: syntax -->
```text
_MEMFREE memoryVariable
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

After `_MEMFREE n`, `_MEMEXISTS` of `n` and of a copy of it are 0.
<!-- src: language/mem-blocks "New memory, images and sounds"; tests\corpus\slice\s88_mem_new_image; tests\reference\examples\_memfree_1 -->

Freeing one block of an image leaves another block of the same image valid.
<!-- src: tests\corpus\slice\s88_mem_new_image -->

`CALL _MEMFREE(m)` is accepted.
<!-- src: tests\corpus\slice\s88_mem_new_image -->

## Errors

- Freeing a block twice raises error 307, "Memory already freed", which ends the program, even under `ON ERROR`.
  <!-- src: language/mem-blocks "A block that is no longer valid"; tests\corpus\slice\s98_mem_error_307 -->

## Differences

A block of `_MEMNEW(0)` freed after another block could free that block's memory again in QB64pe.
<!-- src: language/mem-blocks "A block of no bytes"; D-050 -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memfree_1 -->
```qb64
DIM n AS _MEM, c AS _MEM
n = _MEMNEW(16)
c = n
_MEMFREE n
PRINT "After _MEMFREE, the block and its copy:"; _MEMEXISTS(n); _MEMEXISTS(c)
END
```

Output:

```text
After _MEMFREE, the block and its copy: 0  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MEMNEW`, `_MEMEXISTS`.
