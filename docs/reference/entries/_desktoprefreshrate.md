# _DESKTOPREFRESHRATE

`_DESKTOPREFRESHRATE` gives the refresh rate of the desktop's display, in hertz.

## Syntax

<!-- begin generated: syntax -->
```text
_DESKTOPREFRESHRATE
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "The window's display settings"; verification\v58_f14_display_window -->

## Errors

- A form QB64pe rejects is a compile error: an argument, or the name as a statement.
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x48_desktoprefreshrate_arg; verification\v58_x94_desktoprefreshrate_statement -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN r = _DESKTOPREFRESHRATE -->

## See also

`_DESKTOPWIDTH`.
