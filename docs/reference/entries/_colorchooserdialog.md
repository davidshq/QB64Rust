# _COLORCHOOSERDIALOG

`_COLORCHOOSERDIALOG(title$, default&)` lets the user pick a color in a dialog and gives it as a 32-bit color.

## Syntax

<!-- begin generated: syntax -->
```text
_COLORCHOOSERDIALOG&([title$][, defaultRGB&])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

No test opens a dialog yet: a dialog waits for someone to answer it, and the sandbox family, which runs the
built-ins that act on the desktop in the test VM, has no program that answers one.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s159_builtins_last -->

## Errors

- A form QB64pe rejects is a compile error: a string for the color, three arguments, or the name as a statement.
  For example:
  - `x = _COLORCHOOSERDIALOG("t", "u")`: "argument 2 of `_COLORCHOOSERDIALOG` needs a number".
  - `x = _COLORCHOOSERDIALOG(1, 2, 3)`: "`_COLORCHOOSERDIALOG` takes 0 to 2 arguments".
  - `_COLORCHOOSERDIALOG "t"`: "`_COLORCHOOSERDIALOG` is a function, not a statement".
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v58_x60_colorchooser_string_colour; verification\v58_x84_colorchooser_statement; verification\v58_x61_colorchooser_three_args -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN c~& = _COLORCHOOSERDIALOG -->

## See also

[`_RGB32`](_rgb32.md), [`_MESSAGEBOX`](_messagebox.md).
