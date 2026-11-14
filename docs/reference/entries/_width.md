# _WIDTH

`_WIDTH(handle)` gives the width of an image, and `_HEIGHT(handle)` its height: in pixels, or in characters for a text
image.

## Syntax

<!-- begin generated: syntax -->
```text
_HEIGHT(imageHandle&)
_WIDTH(imageHandle&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

`_WIDTH(img&)` and `_HEIGHT(img&)` of an image of 64 by 48 are 64 and 48.
<!-- src: language/images "Image handles"; tests\reference\examples\_width_1 -->

With no handle, in a `$CONSOLE:ONLY` program, they give the console's size, also while `_DEST` is an image: 120 by 30
in the example's run.
<!-- src: tests\reference\examples\_width_1 -->

## Errors

- A handle of no image raises QB64pe's run-time error.
  <!-- src: language/images "Image handles" -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_width_1 -->
```qb64
img& = _NEWIMAGE(64, 48, 256)
PRINT _WIDTH(img&); _HEIGHT(img&)
_DEST img&
w = _WIDTH: h = _HEIGHT
_DEST _CONSOLE
PRINT "Of the destination:"; w; h
END
```

Output:

```text
 64  48 
Of the destination: 120  30 

Press any key to continue
```
<!-- end generated -->

## See also

[`_NEWIMAGE`](_newimage.md), [`_PIXELSIZE`](_pixelsize.md).
