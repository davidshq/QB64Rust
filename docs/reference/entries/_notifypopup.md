# _NOTIFYPOPUP

`_NOTIFYPOPUP title$, message$, icon$` shows a notification of the system.

## Syntax

<!-- begin generated: syntax -->
```text
_NOTIFYPOPUP [title$][, message$][, iconType$]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

No test opens a dialog, shows a notification, moves the window, or clicks or types on the desktop.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s159_builtins_last -->

## Errors

- A form QB64pe rejects is a compile error: a number, four arguments, or the name as a function.
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v58_x58_notifypopup_number; verification\v58_x104_notifypopup_fn -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _NOTIFYPOPUP "t", "m" -->

## See also

`_MESSAGEBOX`.
