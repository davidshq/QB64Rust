# _DISPLAYORDER

`_DISPLAYORDER` sets the order in which the window's layers are drawn: `_SOFTWARE` (the screen), `_HARDWARE`,
`_HARDWARE1` and `_GLRENDER`, in any order.

## Syntax

<!-- begin generated: syntax -->
```text
_DISPLAYORDER {_SOFTWARE|_HARDWARE|_HARDWARE1|_GLRENDER} (any combination/order)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A layer left out is not drawn.
<!-- src: language/console-and-window "The window's display settings"; verification\v58_s05_display_window -->

A window's state is outside what the tests compare, so no test checks what it shows; it compiles to QB64pe's call.
<!-- src: language/console-and-window "The window's display settings"; tests\corpus\screen\d19_blink_display -->

## Errors

- `_DISPLAYORDER` with no word, a word twice, a trailing comma, or used as a function, is a compile error.
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x40_displayorder_bare; verification\v58_x41_displayorder_twice; verification\v58_x90_displayorder_trailing_comma; verification\v58_x101_displayorder_fn -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _DISPLAYORDER _HARDWARE, _SOFTWARE -->

## See also

[`_GLRENDER`](_glrender.md), [`_DISPLAY`](_display.md).
