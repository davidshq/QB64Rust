# _RESIZE

`_RESIZE ON` lets the user resize the window. The function `_RESIZE` tells whether the user has asked for a new size,
and `_RESIZEWIDTH` and `_RESIZEHEIGHT` give it.

## Syntax

```text
_RESIZE [{ON|OFF}][, {_STRETCH|_SMOOTH}]
asked = _RESIZE
w = _RESIZEWIDTH
h = _RESIZEHEIGHT
```

## Behavior

In a program whose window the user has not resized, the functions give what QB64pe's program gives.
<!-- src: language/console-and-window "_RESIZE and the size it asks for"; tests\corpus\screen\c15_resize -->

`_STRETCH` and `_SMOOTH` scale the screen to fit the window.
<!-- src: language/console-and-window "_RESIZE and the size it asks for" -->

## Errors

- `_RESIZE ON ON`, a number, and `_RESIZE` or `_RESIZEWIDTH` with an argument or as a statement are compile errors.
  <!-- src: language/console-and-window "_RESIZE and the size it asks for"; verification\v53_x05_resize_on_on; verification\v53_x06_resize_number; verification\v53_x09_resizewidth_stmt -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _RESIZE ON: r = _RESIZE: w = _RESIZEWIDTH: h = _RESIZEHEIGHT -->

## See also

[`$RESIZE`](meta-resize.md), [`_WINDOWSIZELIMIT`](_windowsizelimit.md).
