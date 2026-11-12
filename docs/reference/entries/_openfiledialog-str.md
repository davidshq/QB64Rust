# _OPENFILEDIALOG$

`_OPENFILEDIALOG$` asks the user for a file to open, `_SAVEFILEDIALOG$` for a file to save, and `_SELECTFOLDERDIALOG$`
for a folder, each in a dialog, and gives the path.

## Syntax

```text
path$ = _OPENFILEDIALOG$([title$][, default$][, patterns$][, description$][, multiple&])
path$ = _SAVEFILEDIALOG$([title$][, default$][, patterns$][, description$])
path$ = _SELECTFOLDERDIALOG$([title$][, default$])
```

## Behavior

No test opens a dialog, shows a notification, moves the window, or clicks or types on the desktop.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s155_console_on_off -->

## Errors

- A form QB64pe rejects is a compile error: a number, the name as a statement, or too many arguments. For example:
  - `f$ = _OPENFILEDIALOG$(1)`: "argument 1 of `_OPENFILEDIALOG$` needs a string".
  - `_OPENFILEDIALOG$ "t"`: "`_OPENFILEDIALOG$` is a function, not a statement".
  - `f$ = _OPENFILEDIALOG$("t", "", "*.bas", "BASIC", 0, 1)`: "`_OPENFILEDIALOG$` takes 0 to 5 arguments".
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v53_x33_openfiledialog_number; verification\v53_x34_openfiledialog_stmt; verification\v53_x35_openfiledialog_six -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN a$ = _OPENFILEDIALOG$("t") + _SAVEFILEDIALOG$("t") + _SELECTFOLDERDIALOG$("t") -->

## See also

`_FILEEXISTS`, `_INPUTBOX$`.
