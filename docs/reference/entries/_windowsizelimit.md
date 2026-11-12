# _WINDOWSIZELIMIT

`_WINDOWSIZELIMIT (minW, minH)-(maxW, maxH)` limits the size the user may resize the window to.

## Syntax

<!-- begin generated: syntax -->
```text
_WINDOWSIZELIMIT [(minWidth, minHeight)] [- (maxWidth, maxHeight)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\slice\s66_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: a size without parentheses, or one number in a pair. For example:
  - `_WINDOWSIZELIMIT 320, 240`: "this `_WINDOWSIZELIMIT` statement matches none of its forms".
  - `_WINDOWSIZELIMIT (320)`: "this `_WINDOWSIZELIMIT` statement matches none of its forms".
  - `_WINDOWSIZELIMIT (1, 2)-(3)`: "this `_WINDOWSIZELIMIT` statement matches none of its forms".
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x87_sizelimit_no_parens; verification\v26_x88_sizelimit_one_in_pair; verification\v26_x89_sizelimit_max_one -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _WINDOWSIZELIMIT (320, 240)-(800, 600) -->

## See also

`$RESIZE`, `_RESIZE`.
