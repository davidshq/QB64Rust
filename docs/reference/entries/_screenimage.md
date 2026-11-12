# _SCREENIMAGE

`_SCREENIMAGE` gives a new image of the desktop, or of a rectangle of it.

## Syntax

```text
handle& = _SCREENIMAGE[(column1, row1, column2, row2)]
```

## Behavior

The upstream tests run it; what it reads is the desktop's, so it is not compared.
<!-- src: language/images "Image built-ins that no test runs"; tests\upstream\compile_tests\screenimage\test -->

## Errors

- Two arguments, and `_SCREENIMAGE()`, are compile errors.
  <!-- src: language/images "Image built-ins that no test runs"; verification\v31_x81_screenimage_two; verification\v31_x82_screenimage_empty -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN h& = _SCREENIMAGE -->

## See also

`_CLIPBOARDIMAGE`, `_NEWIMAGE`.
