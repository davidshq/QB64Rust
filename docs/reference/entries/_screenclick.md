# _SCREENCLICK

`_SCREENCLICK x, y` clicks the mouse at a place on the desktop.

## Syntax

<!-- begin generated: syntax -->
```text
_SCREENCLICK x, y[, button%]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

No test opens a dialog, shows a notification, moves the window, or clicks or types on the desktop.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

Under a display scale other than 100%, QB64pe's click lands at the scale times its place; qb64rust's lands at the
place.
<!-- src: D-056 -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s159_builtins_last -->

## Errors

- A form QB64pe rejects is a compile error: one number, a string or four arguments.
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v58_x53_screenclick_one_arg; verification\v58_x55_screenclick_four_args -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _SCREENCLICK 10, 20 -->

## See also

`_SCREENPRINT`, `_MOUSEMOVE`.
