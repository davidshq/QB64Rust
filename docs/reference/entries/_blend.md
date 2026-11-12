# _BLEND

`_BLEND` makes drawing on an image mix with what is there by the alpha, and `_DONTBLEND` makes it replace the pixels.
The function `_BLEND` gives whether blending is on.

## Syntax

```text
_BLEND [handle]
_DONTBLEND [handle]
value = _BLEND(handle)
```

## Behavior

A new 32-bit image blends: `_BLEND(img&)` is -1, 0 after `_DONTBLEND img&`, and -1 again after `_BLEND img&`.
<!-- src: language/images "Transparency, blending and palettes"; tests\reference\examples\_blend_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_BLEND 1, 2`: "this `_BLEND` statement matches none of its forms".
  - `PRINT _BLEND(1, 2)`: "`_BLEND` takes 0 or 1 arguments".
  - `_DONTBLEND 1, 2`: "this `_DONTBLEND` statement matches none of its forms".
  <!-- src: language/images "Forms of the image built-ins"; verification\v31_x93_blend_two; verification\v31_x94_blend_fn_two; verification\v31_x95_dontblend_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_blend_1 -->
```qb64
img& = _NEWIMAGE(4, 4, 32)
PRINT "Blending is on:"; _BLEND(img&)
_DONTBLEND img&
PRINT "After _DONTBLEND:"; _BLEND(img&)
_BLEND img&
PRINT "After _BLEND:"; _BLEND(img&)
END
```

Output:

```text
Blending is on:-1 
After _DONTBLEND: 0 
After _BLEND:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_SETALPHA`, `_RGBA32`.
