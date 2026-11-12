# POINT

`POINT(x, y)` gives the color of a pixel of the source image. `POINT(n)` with one argument gives the graphics cursor's
position.

## Syntax

```text
color = POINT(x, y)
value = POINT(n)
```

## Result

The pixel's color: a palette entry, or a 32-bit color as an `_UNSIGNED LONG`.

## Behavior

In a 32-bit image `POINT` gives the whole color: after `PSET (3, 4), _RGB32(255, 128, 0)` it is `&HFFFF8000`.
<!-- src: language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\point_1 -->

It reads the image that `_SOURCE` names.
<!-- src: language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\_source_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:point_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 32)
_DEST img&: _SOURCE img&
PSET (3, 4), _RGB32(255, 128, 0)
c~& = POINT(3, 4)
_DEST _CONSOLE
PRINT HEX$(c~&); _RED32(c~&); _GREEN32(c~&); _BLUE32(c~&)
END
```

Output:

```text
FFFF8000 255  128  0 

Press any key to continue
```
<!-- end generated -->

## See also

`PSET`, `_SOURCE`, `_RED32`.
