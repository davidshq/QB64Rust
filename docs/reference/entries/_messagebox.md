# _MESSAGEBOX

`_MESSAGEBOX title$, message$, icon$` shows a message in a dialog. The function also takes the buttons and gives the
one pressed.

## Syntax

<!-- begin generated: syntax -->
```text
_MESSAGEBOX [title$][, message$][, iconType$]
_MESSAGEBOX&([title$][, message$][, dialogType$][, iconType$][, defaultButton&])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

No test opens a dialog, shows a notification, moves the window, or clicks or types on the desktop.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s155_console_on_off -->

## Errors

- A form QB64pe rejects is a compile error: six arguments, or a number for a text.
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v53_x26_messagebox_fn_six; verification\v53_x28_messagebox_number -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _MESSAGEBOX "t", "m", "info" -->

## See also

`_INPUTBOX$`, `_NOTIFYPOPUP`.
