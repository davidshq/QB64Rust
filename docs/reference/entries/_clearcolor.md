# _CLEARCOLOR

`_CLEARCOLOR color, handle` makes one color of an image transparent, so that `_PUTIMAGE` does not copy it. The
function gives that color.

## Syntax

```text
_CLEARCOLOR {color|_NONE}[, handle]
color = _CLEARCOLOR(handle)
```

## Behavior

After `_CLEARCOLOR 0, img&`, `_CLEARCOLOR(img&)` is 0.
<!-- src: language/images "Transparency, blending and palettes"; tests\reference\examples\_clearcolor_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `PRINT _CLEARCOLOR(1, 2)`: "`_CLEARCOLOR` takes 0 or 1 arguments".
  <!-- src: language/images "Forms of the image built-ins"; verification\v31_x78_clearcolor_fn_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 14 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_clearcolor_1 -->
```qb64
img& = _NEWIMAGE(10, 10, 256)
_CLEARCOLOR 0, img&
PRINT "The clear color is"; _CLEARCOLOR(img&)
END
```

Output:

```text
The clear color is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_SETALPHA`, `_PUTIMAGE`.
