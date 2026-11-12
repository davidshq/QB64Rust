# _MEMNEW

`_MEMNEW(bytes)` gives a block of new memory, which stays until `_MEMFREE` frees it.

## Syntax

<!-- begin generated: syntax -->
```text
_MEMNEW(byteSize)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A `_MEM`.

## Behavior

`_MEMNEW(16)` is a block of 16 bytes, of `TYPE` 16384.
<!-- src: language/mem-blocks "New memory, images and sounds"; tests\corpus\slice\s88_mem_new_image; tests\reference\examples\_memnew_1 -->

`_MEMNEW(0)` is a valid block of size 0 that owns no memory, and its `_MEMFREE` frees nothing.
<!-- src: language/mem-blocks "A block of no bytes"; tests\corpus\slice\s88_mem_new_image -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/mem-blocks "New memory, images and sounds" -->

## Differences

`_MEMFREE` of a block of `_MEMNEW(0)` taken after another block was freed could free that other memory again in
QB64pe.
<!-- src: language/mem-blocks "A block of no bytes"; D-050 -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memnew_1 -->
```qb64
DIM n AS _MEM
n = _MEMNEW(16)
PRINT "Exists, SIZE, TYPE:"; _MEMEXISTS(n); n.SIZE; n.TYPE
_MEMFREE n
END
```

Output:

```text
Exists, SIZE, TYPE:-1  16  16384 

Press any key to continue
```
<!-- end generated -->

## See also

`_MEMFREE`, `_MEMEXISTS`, `_MEM`.
