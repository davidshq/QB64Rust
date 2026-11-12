# _HSB32

`_HSB32(hue, saturation, brightness)` makes a 32-bit color from a hue in degrees and a saturation and brightness in
percent. `_HSBA32` takes an alpha too.

## Syntax

```text
color~& = _HSB32(hue, saturation, brightness)
color~& = _HSBA32(hue, saturation, brightness, alpha)
```

## Result

An `_UNSIGNED LONG`.

## Behavior

`_HSB32(120, 100, 100)` is pure green, 0, 255, 0.
<!-- src: language/images "The colour functions"; tests\reference\examples\_hsb32_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_hsb32_1 -->
```qb64
c~& = _HSB32(120, 100, 100)
PRINT _RED32(c~&); _GREEN32(c~&); _BLUE32(c~&)
PRINT HEX$(_HSBA32(0, 100, 100, 50))
END
```

Output:

```text
 0  255  0 
80FF0000

Press any key to continue
```
<!-- end generated -->

## See also

[`_HUE32`](_hue32.md), [`_RGB32`](_rgb32.md).
