# _COPYPALETTE

`_COPYPALETTE from, to` copies the palette of one image to another.

## Syntax

```text
_COPYPALETTE [source[, destination]]
```

## Behavior

After `_COPYPALETTE a&, b&`, an entry set in `a&` has the same color in `b&`.
<!-- src: language/images "Transparency, blending and palettes"; tests\reference\examples\_copypalette_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_copypalette_1 -->
```qb64
a& = _NEWIMAGE(20, 20, 256)
b& = _NEWIMAGE(20, 20, 256)
_PALETTECOLOR 7, _RGB32(1, 2, 3), a&
_COPYPALETTE a&, b&
PRINT HEX$(_PALETTECOLOR(7, b&))
END
```

Output:

```text
FF010203

Press any key to continue
```
<!-- end generated -->

## See also

`_PALETTECOLOR`, `PALETTE`.
