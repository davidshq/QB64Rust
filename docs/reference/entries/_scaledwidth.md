# _SCALEDWIDTH

`_SCALEDWIDTH` and `_SCALEDHEIGHT` give the size of the screen as the window shows it, after any scaling.

## Syntax

```text
w = _SCALEDWIDTH
h = _SCALEDHEIGHT
```

## Behavior

They give the size in pixels at which the screen is shown. In a window at a display scale of 100%, that is the
screen's own size: 320 and 200 for `SCREEN _NEWIMAGE(320, 200, 32)`, also after a return from full screen.
<!-- src: language/console-and-window "The window's display settings"; verification\v58_f14_display_window; tests\sandbox\window_fullscreen; tests\corpus\screen\d19_blink_display -->

## Errors

- A form QB64pe rejects is a compile error: an argument, or the name as a statement. For example:
  - `x = _SCALEDWIDTH(1)`: "`_SCALEDWIDTH` takes no arguments".
  - `_SCALEDWIDTH`: "`_SCALEDWIDTH` is a function, not a statement".
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x37_scaledwidth_arg; verification\v58_x85_scaledwidth_statement -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN w = _SCALEDWIDTH: h = _SCALEDHEIGHT -->

## See also

[`_RESIZE`](_resize.md), [`_FULLSCREEN`](_fullscreen.md).
