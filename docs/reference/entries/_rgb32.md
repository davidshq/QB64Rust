# _RGB32

`_RGB32(red, green, blue)` makes a 32-bit color, and `_RGBA32` one with an alpha: `_RGB32(255, 128, 0)` is orange,
`&HFFFF8000`.

## Syntax

```text
color~& = _RGB32(red, green, blue)
color~& = _RGBA32(red, green, blue, alpha)
color~& = _RGB32(gray)
```

## Result

An `_UNSIGNED LONG`.

## Behavior

The color is `&HAARRGGBB`: `_RGB32` gives an alpha of 255, `_RGBA32` the one given. `_RGB32(n)` with one argument is
the gray of `n`.
<!-- src: language/images "The colour functions"; tests\reference\examples\_rgb32_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 15 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_rgb32_1 -->
```qb64
c~& = _RGB32(255, 128, 0)
PRINT HEX$(c~&)
PRINT HEX$(_RGBA32(255, 128, 0, 64))
PRINT HEX$(_RGB32(5))
END
```

Output:

```text
FFFF8000
40FF8000
FF050505

Press any key to continue
```
<!-- end generated -->

## See also

`_RED32`, `_RGB`, `_HSB32`, `POINT`.
