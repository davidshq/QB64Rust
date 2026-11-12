# _RGB

`_RGB(red, green, blue)` gives the color an image uses for those parts: in a 32-bit image the color itself, in a
palette image the nearest entry. `_RGBA` takes an alpha too.

## Syntax

```text
color = _RGB(red, green, blue[, image])
color = _RGBA(red, green, blue, alpha[, image])
```

## Result

An `_UNSIGNED LONG`.

## Behavior

In a 32-bit image `_RGB(255, 0, 0)` is `&HFFFF0000`; in a 256-color image it is the number of the palette entry
nearest to red, 40.
<!-- src: language/images "The colour functions"; tests\reference\examples\_rgb_1 -->

The image is the destination, or the one named.
<!-- src: language/images "The colour functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_rgb_1 -->
```qb64
img& = _NEWIMAGE(20, 20, 256)
PRINT "In a 256-color image, _RGB(255, 0, 0) is entry"; _RGB(255, 0, 0, img&)
t& = _NEWIMAGE(20, 20, 32)
PRINT "In a 32-bit image it is &H"; HEX$(_RGB(255, 0, 0, t&))
PRINT HEX$(_RGBA(0, 0, 255, 128, t&))
END
```

Output:

```text
In a 256-color image, _RGB(255, 0, 0) is entry 40 
In a 32-bit image it is &HFFFF0000
800000FF

Press any key to continue
```
<!-- end generated -->

## See also

`_RGB32`, `_RED`, `PALETTE`.
