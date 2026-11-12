# _MAPTRIANGLE

`_MAPTRIANGLE` copies a triangle of one image onto a triangle of another, stretching it to fit.

## Syntax

```text
_MAPTRIANGLE (sx1, sy1)-(sx2, sy2)-(sx3, sy3), source TO (dx1, dy1)-(dx2, dy2)-(dx3, dy3)[, destination]
```

## Behavior

The 2-D form maps the triangle of the source onto that of the destination: the inside of the destination's triangle
takes the source's pixels, and the rest is untouched.
<!-- src: language/images "Drawing between images"; tests\reference\examples\_maptriangle_1 -->

The 3-D form compiles to QB64pe's call; no test runs it, since it draws with OpenGL.
<!-- src: language/images "Image built-ins that no test runs" -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/images "Forms of the image built-ins" -->

## Availability

<!-- begin generated: availability -->
Supported. All 16 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_maptriangle_1 -->
```qb64
src& = _NEWIMAGE(10, 10, 32)
_DEST src&: CLS , _RGB32(0, 0, 255)
dst& = _NEWIMAGE(20, 20, 32)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), src& TO(0, 0)-(19, 0)-(0, 19), dst&
_SOURCE dst&
_DEST _CONSOLE
PRINT HEX$(POINT(2, 2)); " "; HEX$(POINT(18, 18))
END
```

Output:

```text
FF0000FF 0

Press any key to continue
```
<!-- end generated -->

## See also

`_PUTIMAGE`.
