# _PALETTECOLOR

`_PALETTECOLOR entry, color` sets an entry of an image's palette to a 32-bit color, and `_PALETTECOLOR(entry)` gives
it.

## Syntax

```text
_PALETTECOLOR entry, color~&[, image]
color~& = _PALETTECOLOR(entry[, image])
```

## Behavior

After `_PALETTECOLOR 3, _RGB32(10, 20, 30), img&`, `_PALETTECOLOR(3, img&)` is `&HFF0A141E`.
<!-- src: language/images "Transparency, blending and palettes"; tests\reference\examples\_palettecolor_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_palettecolor_1 -->
```qb64
img& = _NEWIMAGE(20, 20, 256)
_PALETTECOLOR 3, _RGB32(10, 20, 30), img&
PRINT HEX$(_PALETTECOLOR(3, img&))
END
```

Output:

```text
FF0A141E

Press any key to continue
```
<!-- end generated -->

## See also

[`PALETTE`](palette.md), [`_COPYPALETTE`](_copypalette.md), [`_RGB32`](_rgb32.md).
