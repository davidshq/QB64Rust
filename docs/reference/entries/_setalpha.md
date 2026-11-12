# _SETALPHA

`_SETALPHA alpha, color, handle` sets the alpha, the opacity from 0 to 255, of the pixels of an image, or of those of
one color.

## Syntax

```text
_SETALPHA alpha[, [color][ TO color2]][, handle]
```

## Behavior

`_SETALPHA 128, , img&` makes every pixel half transparent: a pixel of `&HFF0A141E` becomes `&H800A141E`.
<!-- src: language/images "Transparency, blending and palettes"; tests\reference\examples\_setalpha_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_SETALPHA "a"`: "argument 1 of `_SETALPHA` needs a number".
  - `_SETALPHA`: "this `_SETALPHA` statement matches none of its forms".
  - `_SETALPHA 128, TO 9`: "this `_SETALPHA` statement matches none of its forms".
  <!-- src: language/images "Forms of the image built-ins"; verification\v31_x106_setalpha_string; verification\v31_x49_setalpha_bare; verification\v31_x50_setalpha_to_only -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_setalpha_1 -->
```qb64
img& = _NEWIMAGE(4, 4, 32)
_DEST img&: CLS , _RGB32(10, 20, 30)
_SETALPHA 128, , img&
_SOURCE img&
_DEST _CONSOLE
PRINT HEX$(POINT(1, 1))
END
```

Output:

```text
800A141E

Press any key to continue
```
<!-- end generated -->

## See also

`_CLEARCOLOR`, `_BLEND`, `_RGBA32`.
