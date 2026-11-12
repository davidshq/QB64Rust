# VIEW

`VIEW (x1, y1)-(x2, y2)` limits drawing to a rectangle of the screen, the viewport, whose top left becomes point (0,
0). `VIEW` alone draws on the whole screen again.

## Syntax

```text
VIEW [[SCREEN] (x1, y1)-(x2, y2)[, [color][, border]]]
```

## Behavior

After `VIEW (10, 10)-(29, 19)`, `PSET (0, 0)` lights the pixel at (10, 10) of the screen.
<!-- src: language/screen-output "Drawing statements"; tests\reference\examples\view_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `VIEW (1, 2)-(3, 4), 5,`: "this `VIEW` statement matches none of its forms".
  - `VIEW SCREEN`: "this `VIEW` statement matches none of its forms".
  - `VIEW STEP(1, 2)-(3, 4)`: "this `VIEW` statement matches none of its forms".
  <!-- src: language/screen-output "Drawing statements"; verification\v27_x22_view_comma_after_color; verification\v27_x24_view_screen_alone; verification\v27_x35_view_step -->

## Availability

<!-- begin generated: availability -->
Supported. All 17 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:view_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
VIEW (10, 10)-(29, 19)
PSET (0, 0), 7
VIEW
_DEST _CONSOLE
PRINT "PSET (0, 0) in the viewport lit"; POINT(10, 10); "at (10, 10); (0, 0) is"; POINT(0, 0)
END
```

Output:

```text
PSET (0, 0) in the viewport lit 7 at (10, 10); (0, 0) is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`WINDOW`, `VIEW PRINT`, `CLS`.
