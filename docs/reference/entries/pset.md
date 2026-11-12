# PSET

`PSET (x, y), color` sets one pixel to a color. `PRESET (x, y)` sets it to the background color when no color is
given.

## Syntax

```text
PSET [STEP](x, y)[, color]
PRESET [STEP](x, y)[, color]
```

## Behavior

`PSET (5, 5), 4` sets the pixel to color 4. Without a color, `PSET` uses the current foreground color and `PRESET` the
background color, 0 by default.
<!-- src: language/screen-output "Drawing statements"; tests\reference\examples\pset_1 -->

`STEP` makes the point relative to the last one drawn.
<!-- src: language/screen-output "Drawing statements" -->

The examples draw on an image made with `_NEWIMAGE` and read its pixels back with `POINT`, so that their output is
text.
<!-- src: tests\reference\examples\pset_1 -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/screen-output "Drawing statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 29 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:pset_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
PSET (5, 5), 4
PRESET (6, 5)
PSET (10, 10), 15: PRESET (10, 10)
_DEST _CONSOLE
PRINT POINT(5, 5); POINT(6, 5); POINT(10, 10); POINT(0, 0)
END
```

Output:

```text
 4  0  0  0 

Press any key to continue
```
<!-- end generated -->

## See also

`POINT`, `LINE`, `COLOR`, `_DEST`.
