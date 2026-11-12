# _MEMIMAGE

`_MEMIMAGE(handle)` gives the block of an image's pixels, or of the destination's with no handle.

## Syntax

<!-- begin generated: syntax -->
```text
_MEMIMAGE[(imageHandle&)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A `_MEM`.

## Behavior

The block of a 4 by 2 image of 32 bits is 32 bytes, of `TYPE` 3204 with 4 bytes a pixel. The pixel (1, 0) is at
`OFFSET + 4`, as a color: `FF0A141E` for `_RGB32(10, 20, 30)`.
<!-- src: language/mem-blocks "New memory, images and sounds"; tests\corpus\slice\s88_mem_new_image; tests\reference\examples\_memimage_1 -->

`IMAGE` is the image's handle. A 256-color image has 1 byte a pixel, of `TYPE` 3201.
<!-- src: tests\corpus\slice\s88_mem_new_image -->

The block stops being valid when the image is freed.
<!-- src: tests\corpus\slice\s88_mem_new_image -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `m = _MEMIMAGE()`: "`_MEMIMAGE` with empty parentheses".
  - `m = _MEMIMAGE(1, 2)`: "`_MEMIMAGE` takes 0 or 1 arguments".
  - `m = _MEMIMAGE("a")`: "`_MEMIMAGE` needs a number".
  <!-- src: language/mem-blocks "New memory, images and sounds"; verification\v32_x74_memimage_empty; verification\v32_x75_memimage_two; verification\v32_x76_memimage_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memimage_1 -->
```qb64
img& = _NEWIMAGE(4, 2, 32)
_DEST img&
PSET (1, 0), _RGB32(10, 20, 30)
_DEST _CONSOLE
DIM m AS _MEM
m = _MEMIMAGE(img&)
PRINT "SIZE, TYPE, ELEMENTSIZE:"; m.SIZE; m.TYPE; m.ELEMENTSIZE
PRINT "Pixel (1, 0): "; HEX$(_MEMGET(m, m.OFFSET + 4, _UNSIGNED LONG))
PRINT "IMAGE is the handle:"; m.IMAGE = img&
END
```

Output:

```text
SIZE, TYPE, ELEMENTSIZE: 32  3204  4 
Pixel (1, 0): FF0A141E
IMAGE is the handle:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_MEM`, `_NEWIMAGE`, `_MEMGET`.
