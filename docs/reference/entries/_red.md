# _RED

`_RED`, `_GREEN`, `_BLUE` and `_ALPHA` give the parts of a color of an image: in a palette image, of a palette entry.

## Syntax

```text
value = _RED(color[, image])
value = _GREEN(color[, image])
value = _BLUE(color[, image])
value = _ALPHA(color[, image])
```

## Result

A number from 0 to 255.

## Behavior

Entry 4 of a 256-color image's default palette is dark red: its parts are 170, 0, 0 and 255.
<!-- src: language/images "The colour functions"; tests\reference\examples\_red_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_red_1 -->
```qb64
img& = _NEWIMAGE(20, 20, 256)
PRINT "Entry 4 of the palette:"; _RED(4, img&); _GREEN(4, img&); _BLUE(4, img&); _ALPHA(4, img&)
END
```

Output:

```text
Entry 4 of the palette: 170  0  0  255 

Press any key to continue
```
<!-- end generated -->

## See also

`_RED32`, `_RGB`, `_PALETTECOLOR`.
