# $RESIZE

`$RESIZE:ON` lets the user resize the program's window; `$RESIZE:STRETCH` and `$RESIZE:SMOOTH` also scale the screen
to fit it.

## Syntax

```text
$RESIZE:{ON|OFF|STRETCH|SMOOTH}
```

## Behavior

Any letter case is accepted. With several, the last in the file counts.
<!-- src: language/console-and-window "$RESIZE" -->

A program reads the size the user asks for with `_RESIZE`, `_RESIZEWIDTH` and `_RESIZEHEIGHT`.
<!-- src: language/console-and-window "_RESIZE and the size it asks for"; tests\corpus\screen\c15_resize -->

## Errors

- Every other form, such as `$RESIZE` alone or `$RESIZE:BOGUS`, is a compile error.
  <!-- src: language/console-and-window "$RESIZE"; verification\v53_x70_meta_resize_bare; verification\v53_x71_meta_resize_bogus -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

<!-- probe: $RESIZE:ON -->

## See also

[`_RESIZE`](_resize.md), [`_FULLSCREEN`](_fullscreen.md).
