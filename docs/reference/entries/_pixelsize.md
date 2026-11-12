# _PIXELSIZE

`_PIXELSIZE(handle)` gives the bytes a pixel of an image takes: 4 for a 32-bit image, 1 for a palette image, 0 for a
text image.

## Syntax

<!-- begin generated: syntax -->
```text
_PIXELSIZE[(imageHandle&)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

`_PIXELSIZE` of a 32-bit, a 256-color and a text image is 4, 1 and 0.
<!-- src: language/images "Image handles"; tests\reference\examples\_pixelsize_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_pixelsize_1 -->
```qb64
PRINT _PIXELSIZE(_NEWIMAGE(10, 10, 32)); _PIXELSIZE(_NEWIMAGE(10, 10, 256)); _PIXELSIZE(_NEWIMAGE(80, 25, 0))
END
```

Output:

```text
 4  1  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_NEWIMAGE`, `_WIDTH`.
