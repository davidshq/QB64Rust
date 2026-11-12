# _SCREENPRINT

`_SCREENPRINT text$` types text into the window that has the focus, as if from the keyboard.

## Syntax

<!-- begin generated: syntax -->
```text
_SCREENPRINT text$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\corpus\slice\s159_builtins_last -->

## Errors

- A form QB64pe rejects is a compile error: no text or a number.
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v58_x56_screenprint_bare; verification\v58_x57_screenprint_number -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _SCREENPRINT "abc" -->

## See also

`_SCREENCLICK`.
