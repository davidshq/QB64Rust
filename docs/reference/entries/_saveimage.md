# _SAVEIMAGE

`_SAVEIMAGE name$, handle` writes an image to a file, in the format its extension names: PNG, BMP and others.

## Syntax

```text
_SAVEIMAGE name$[, handle[, requirements$]]
```

## Behavior

An image saved with `_SAVEIMAGE` and loaded again with `_LOADIMAGE` holds the same pixels.
<!-- src: language/images "Loading and saving images"; tests\reference\examples\_saveimage_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SAVEIMAGE`: "this `_SAVEIMAGE` statement matches none of its forms".
  - `_SAVEIMAGE 5`: "argument 1 of `_SAVEIMAGE` needs a string".
  - `_SAVEIMAGE "a", 1, "b", 2`: "this `_SAVEIMAGE` statement matches none of its forms".
  <!-- src: language/images "Forms of the image built-ins"; verification\v31_x85_saveimage_bare; verification\v31_x86_saveimage_number; verification\v31_x87_saveimage_four -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_saveimage_1 -->
```qb64
img& = _NEWIMAGE(8, 8, 32)
_DEST img&: PSET (3, 3), _RGB32(0, 255, 0)
_DEST _CONSOLE
_SAVEIMAGE "_saveimage_1.png", img&
back& = _LOADIMAGE("_saveimage_1.png", 32)
_SOURCE back&
PRINT "Loaded"; _WIDTH(back&); "by"; _HEIGHT(back&); ", pixel (3, 3) is "; HEX$(POINT(3, 3))
KILL "_saveimage_1.png"
END
```

Output:

```text
Loaded 8 by 8 , pixel (3, 3) is FF00FF00

Press any key to continue
```
<!-- end generated -->

## See also

`_LOADIMAGE`.
