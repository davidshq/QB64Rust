# WINDOW

`WINDOW (x1, y1)-(x2, y2)` gives the screen coordinates of your own, so that drawing uses them: `WINDOW (0, 0)-(100,
100)` makes every image 100 by 100 units.

## Syntax

```text
WINDOW [[SCREEN] (x1, y1)-(x2, y2)]
```

## Behavior

After `WINDOW (0, 0)-(100, 100)` on an image 40 by 30, point (50, 50) is pixel (20, 14), and y grows upward. `WINDOW`
alone goes back to pixels.
<!-- src: language/screen-output "Drawing statements"; tests\reference\examples\window_1 -->

`PMAP` converts between the two.
<!-- src: language/images "_AUTODISPLAY and PMAP" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `WINDOW SCREEN`: "this `WINDOW` statement matches none of its forms".
  - `WINDOW (1, 2)-(3, 4), 5`: "this `WINDOW` statement matches none of its forms".
  <!-- src: language/screen-output "Drawing statements"; verification\v27_x23_window_screen_alone; verification\v27_x47_window_color -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:window_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
WINDOW (0, 0)-(100, 100)
PSET (50, 50), 6
x = PMAP(50, 0): y = PMAP(50, 1)
WINDOW
_DEST _CONSOLE
PRINT "(50, 50) of the WINDOW is pixel"; x; y; "and holds"; POINT(x, y)
END
```

Output:

```text
(50, 50) of the WINDOW is pixel 20  14 and holds 6 

Press any key to continue
```
<!-- end generated -->

## See also

[`PMAP`](pmap.md), [`VIEW`](view.md).
