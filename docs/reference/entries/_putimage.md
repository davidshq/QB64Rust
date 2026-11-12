# _PUTIMAGE

`_PUTIMAGE` copies a rectangle of one image onto another, scaled to fit the rectangle given.

## Syntax

```text
_PUTIMAGE [[STEP](dx1, dy1)[-[STEP](dx2, dy2)]][, source][, destination][, [STEP](sx1, sy1)[-[STEP](sx2, sy2)]][, _SMOOTH]
```

## Behavior

`_PUTIMAGE (10, 10), src&, dst&` copies `src&` with its top left at (10, 10). `_PUTIMAGE (0, 0)-(7, 7), src&, dst&`
stretches it to fill that rectangle.
<!-- src: language/images "Drawing between images"; tests\reference\examples\_putimage_1 -->

Where the source or destination is left out, `_SOURCE` and `_DEST` are used. The result is scaled, mirrored or clipped
as libqb does.
<!-- src: language/images "Drawing between images" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_PUTIMAGE (1, 2)-`: "this `_PUTIMAGE` statement matches none of its forms".
  - `_PUTIMAGE , , , , , 1`: "this `_PUTIMAGE` statement matches none of its forms".
  - `_PUTIMAGE (0, 0)-(9, 9), img _SMOOTH`: "this `_PUTIMAGE` statement matches none of its forms".
  <!-- src: language/images "Forms of the image built-ins"; verification\v31_x08_putimage_dash; verification\v31_x09_putimage_too_many; verification\v31_x10_putimage_smooth_no_comma -->

## Availability

<!-- begin generated: availability -->
Supported. All 23 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_putimage_1 -->
```qb64
src& = _NEWIMAGE(4, 4, 32)
_DEST src&: LINE (0, 0)-(3, 3), _RGB32(255, 0, 0), BF
dst& = _NEWIMAGE(20, 20, 32)
_PUTIMAGE (10, 10), src&, dst&
_PUTIMAGE (0, 0)-(7, 7), src&, dst&
_SOURCE dst&
_DEST _CONSOLE
PRINT HEX$(POINT(11, 11)); " "; HEX$(POINT(7, 7)); " "; HEX$(POINT(15, 15))
END
```

Output:

```text
FFFF0000 FFFF0000 0

Press any key to continue
```
<!-- end generated -->

## See also

`_MAPTRIANGLE`, `_NEWIMAGE`, `_COPYIMAGE`.
