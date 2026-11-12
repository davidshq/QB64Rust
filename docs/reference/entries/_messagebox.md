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

No test opens a dialog yet: a dialog waits for someone to answer it, and the sandbox family, which runs the
built-ins that act on the desktop in the test VM, has no program that answers one.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s155_console_on_off -->

## Errors

- A form QB64pe rejects is a compile error: six arguments, or a number for a text. For example:
  - `r& = _MESSAGEBOX("a", "b", "ok", "info", 1, 2)`: "`_MESSAGEBOX` takes 0 to 5 arguments".
  - `_MESSAGEBOX "a", "b", "ok", "info"`: "this `_MESSAGEBOX` statement matches none of its forms".
  - `_MESSAGEBOX "a", "b", 1`: "argument 3 of `_MESSAGEBOX` needs a string".
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v53_x26_messagebox_fn_six; verification\v53_x28_messagebox_number; verification\v53_x27_messagebox_stmt_four -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _MESSAGEBOX "t", "m", "info" -->

## See also

[`_INPUTBOX$`](_inputbox-str.md), [`_NOTIFYPOPUP`](_notifypopup.md).
