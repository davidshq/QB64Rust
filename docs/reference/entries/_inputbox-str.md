# _INPUTBOX$

`_INPUTBOX$(title$, message$, default$)` asks for a line of text in a dialog and gives it.

## Syntax

<!-- begin generated: syntax -->
```text
_INPUTBOX$([title$][, message$][, defaultInput$])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

No test opens a dialog, shows a notification, moves the window, or clicks or types on the desktop.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s159_builtins_last -->

## Errors

- A form QB64pe rejects is a compile error: a number, four arguments, or the name as a statement. For example:
  - `x$ = _INPUTBOX$(1)`: "argument 1 of `_INPUTBOX$` needs a string".
  - `x$ = _INPUTBOX$("a", "b", "c", "d")`: "`_INPUTBOX$` takes 0 to 3 arguments".
  - `_INPUTBOX$ "t"`: "`_INPUTBOX$` is a function, not a statement".
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v58_x62_inputbox_number; verification\v58_x83_inputbox_statement; verification\v58_x63_inputbox_four_args -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN a$ = _INPUTBOX$("t") -->

## See also

`_MESSAGEBOX`, `INPUT`.
