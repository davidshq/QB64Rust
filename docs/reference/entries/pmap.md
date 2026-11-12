# PMAP

`PMAP(value, mode)` converts a coordinate between the units of `WINDOW` and pixels.

## Syntax

<!-- begin generated: syntax -->
```text
PMAP(coordinate, function_number%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `value`: the coordinate.
- `mode`: 0 a `WINDOW` x to a pixel, 1 a `WINDOW` y to a pixel, 2 a pixel x to `WINDOW` units, 3 a pixel y to `WINDOW`
  units.

## Result

A SINGLE.

## Behavior

After `WINDOW (0, 0)-(100, 100)` on an image 40 by 30, x from 0 to 100 maps to pixels 0 to 39 and y to pixels 29 to 0.
<!-- src: language/images "_AUTODISPLAY and PMAP"; tests\reference\examples\pmap_1 -->

It works on the destination image.
<!-- src: tests\reference\examples\pmap_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:pmap_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&
WINDOW (0, 0)-(100, 100)
x0 = PMAP(0, 0): x1 = PMAP(100, 0): y0 = PMAP(0, 1): y1 = PMAP(100, 1)
_DEST _CONSOLE
PRINT "x from 0 to 100 maps to pixels"; x0; "to"; x1
PRINT "y from 0 to 100 maps to pixels"; y0; "to"; y1
END
```

Output:

```text
x from 0 to 100 maps to pixels 0 to 39 
y from 0 to 100 maps to pixels 29 to 0 

Press any key to continue
```
<!-- end generated -->

## See also

`WINDOW`, `VIEW`.
