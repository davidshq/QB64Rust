# _SCALEDWIDTH

`_SCALEDWIDTH` and `_SCALEDHEIGHT` give the size of the screen as the window shows it, after any scaling.

## Syntax

```text
w = _SCALEDWIDTH
h = _SCALEDHEIGHT
```

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "The window's display settings"; verification\v58_f14_display_window; tests\corpus\screen\d19_blink_display -->

## Errors

- A form QB64pe rejects is a compile error: an argument, or the name as a statement.
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x37_scaledwidth_arg; verification\v58_x85_scaledwidth_statement -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN w = _SCALEDWIDTH: h = _SCALEDHEIGHT -->

## See also

`_RESIZE`, `_FULLSCREEN`.
