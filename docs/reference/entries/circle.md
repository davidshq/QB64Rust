# CIRCLE

`CIRCLE (x, y), radius, color` draws a circle. It can also draw an arc between two angles, and an ellipse with an
aspect.

## Syntax

```text
CIRCLE [STEP](x, y), radius[, [color][, [start][, [end][, aspect]]]]
```

## Behavior

`CIRCLE (20, 15), 10, 2` draws the circle's outline in color 2: the points 10 pixels from the centre are lit and the
centre is not.
<!-- src: language/screen-output "Drawing statements"; tests\reference\examples\circle_1 -->

The start and end angles, in radians, draw an arc, and the aspect makes an ellipse. The pixels are those QB64pe's
program draws.
<!-- src: language/screen-output "Drawing statements" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `CIRCLE (1, 2),`: "this `CIRCLE` statement matches none of its forms".
  - `CIRCLE (1, 2), 3, 4, 5, 6, 7, 8`: "this `CIRCLE` statement matches none of its forms".
  <!-- src: language/screen-output "Drawing statements"; verification\v27_x21_circle_no_radius; verification\v27_x45_circle_seven -->

## Differences

From QuickBASIC 4.5:

- A small circle with the screen's own aspect lights other pixels for some radii: radius 4 in `SCREEN 12` has two
  pixels side by side at each diagonal there and one in QB64Rust, as in QB64pe, so a `PAINT` inside it fills 57
  pixels where QuickBASIC 4.5 fills 61. In `SCREEN 9`, 8 of the radii 1 to 10 differ (`DIVERGENCES-QB45.md` Q-036).
  <!-- src: Q-036 -->
- An arc with an aspect and start and end angles lights other pixels, so a `PAINT` that stays inside a shape there can
  leak out of it: Civil War Strategy's cannon wheel, `CIRCLE (x + 5, y), 29, 1, 4, 1.5, 1.4`, is one (Q-020).
  <!-- src: Q-020 -->

## Availability

<!-- begin generated: availability -->
Supported. All 22 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:circle_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
CIRCLE (20, 15), 10, 2
_DEST _CONSOLE
PRINT "On the circle:"; POINT(30, 15); POINT(20, 5)
PRINT "At the centre:"; POINT(20, 15)
END
```

Output:

```text
On the circle: 2  2 
At the centre: 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`LINE`](line.md), [`PAINT`](paint.md), [`DRAW`](draw.md).
