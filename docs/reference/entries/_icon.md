# _ICON

`_ICON image, small` sets the window's icon from an image, or with no argument to the icon of the executable.

## Syntax

<!-- begin generated: syntax -->
```text
_ICON [mainImageHandle&[, smallImageHandle&]]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\slice\s66_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: three arguments or a string. For example:
  - `_ICON 1, 2, 3`: "this `_ICON` statement matches none of its forms".
  - `_ICON "a"`: "argument 1 of `_ICON` needs a number".
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x78_icon_three; verification\v26_x79_icon_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 14 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _ICON -->

## See also

`$EXEICON`, `_TITLE`.
