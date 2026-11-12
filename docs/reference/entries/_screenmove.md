# _SCREENMOVE

`_SCREENMOVE x, y` moves the program's window to a place on the desktop, and `_SCREENMOVE _MIDDLE` centers it.

## Syntax

<!-- begin generated: syntax -->
```text
_SCREENMOVE x, y or _SCREENMOVE _Middle
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

No test opens a dialog, shows a notification, moves the window, or clicks or types on the desktop.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s155_console_on_off -->

## Errors

- A form QB64pe rejects is a compile error: one number, a string or no argument.
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v53_x40_screenmove_one; verification\v53_x43_screenmove_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _SCREENMOVE _MIDDLE -->

## See also

`_SCREENX`.
