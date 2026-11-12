# _NEWIMAGE

`_NEWIMAGE(width, height, mode)` makes a new image and gives its handle, for `_DEST`, `SCREEN` and the other image
statements. Mode 32 is 32-bit color, 256 a 256-color palette, 0 text.

## Syntax

<!-- begin generated: syntax -->
```text
_NEWIMAGE(width&, height&, mode)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `width`, `height`: in pixels, or in characters for a text image.
- `mode`: 32, 256, 0, or a legacy screen mode such as 12 or 13.

## Result

A LONG handle, below -1.

## Behavior

`_NEWIMAGE(320, 200, 32)` makes a 32-bit image of 320 by 200 pixels; its handle is negative. `_NEWIMAGE(80, 25, 0)`
makes a text image of 80 by 25 characters.
<!-- src: language/images "Image handles"; tests\reference\examples\_newimage_1 -->

The image starts with every pixel 0. In a 32-bit image that is `&H00000000`, a transparent black.
<!-- src: tests\reference\examples\_printstring_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `n = _NEWIMAGE(1)`: "`_NEWIMAGE` takes 2 or 3 arguments".
  - `n = _NEWIMAGE(1, 2, 3, 4)`: "`_NEWIMAGE` takes 2 or 3 arguments".
  - `n = _NEWIMAGE("a", 2)`: "argument 1 of `_NEWIMAGE` needs a number".
  <!-- src: language/images "Forms of the image built-ins"; verification\v26_x104_newimage_one; verification\v26_x105_newimage_four; verification\v26_x106_newimage_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_newimage_1 -->
```qb64
img& = _NEWIMAGE(320, 200, 32)
PRINT "A handle is negative:"; img& < -1
PRINT _WIDTH(img&); _HEIGHT(img&); _PIXELSIZE(img&)
t& = _NEWIMAGE(80, 25, 0)
PRINT "A text image of"; _WIDTH(t&); "by"; _HEIGHT(t&); "characters"
END
```

Output:

```text
A handle is negative:-1 
 320  200  4 
A text image of 80 by 25 characters

Press any key to continue
```
<!-- end generated -->

## See also

`_FREEIMAGE`, `_DEST`, `SCREEN`, `_COPYIMAGE`, `_WIDTH`.
