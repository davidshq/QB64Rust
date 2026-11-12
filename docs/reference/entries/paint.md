# PAINT

`PAINT (x, y), color, border` fills an area with a color, from a point outward until it meets the border color.

## Syntax

```text
PAINT [STEP](x, y)[, [color][, [border][, background$]]]
```

## Behavior

After a box is drawn in color 9, `PAINT (10, 10), 14, 9` fills its inside with 14 and leaves the border and the
outside alone.
<!-- src: language/screen-output "Drawing statements"; tests\reference\examples\paint_1 -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/screen-output "Drawing statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 28 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:paint_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
LINE (5, 5)-(25, 20), 9, B
PAINT (10, 10), 14, 9
_DEST _CONSOLE
PRINT "Inside:"; POINT(10, 10); "border:"; POINT(5, 5); "outside:"; POINT(30, 25)
END
```

Output:

```text
Inside: 14 border: 9 outside: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`LINE`, `CIRCLE`, `DRAW`.
