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

- A form QB64pe rejects is a compile error.
  <!-- src: language/screen-output "Drawing statements" -->

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

`LINE`, `PAINT`, `DRAW`.
