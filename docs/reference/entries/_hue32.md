# _HUE32

`_HUE32`, `_SATURATION32` and `_BRIGHTNESS32` give the hue, in degrees, and the saturation and brightness, in percent,
of a 32-bit color.

## Syntax

<!-- begin generated: syntax -->
```text
_BRIGHTNESS32(rgbaColor&)
_HUE32(rgbaColor&)
_SATURATION32(rgbaColor&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A DOUBLE.

## Behavior

Of pure blue they give 240, 100 and 100.
<!-- src: language/images "The colour functions"; tests\reference\examples\_hue32_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_hue32_1 -->
```qb64
c~& = _RGB32(0, 0, 255)
PRINT CINT(_HUE32(c~&)); CINT(_SATURATION32(c~&)); CINT(_BRIGHTNESS32(c~&))
END
```

Output:

```text
 240  100  100 

Press any key to continue
```
<!-- end generated -->

## See also

`_HSB32`, `_RED32`.
