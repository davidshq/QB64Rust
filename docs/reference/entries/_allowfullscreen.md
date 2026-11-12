# _ALLOWFULLSCREEN

`_ALLOWFULLSCREEN` sets which full-screen modes Alt+Enter may switch to.

## Syntax

<!-- begin generated: syntax -->
```text
_ALLOWFULLSCREEN [{_STRETCH|_SQUAREPIXELS|_OFF|_ALL|OFF}][,{_SMOOTH|_OFF|_ALL|OFF}]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "The window's display settings"; verification\v58_s05_display_window; tests\corpus\screen\d19_blink_display -->

## Errors

- A form QB64pe rejects is a compile error: a number, or a comma alone.
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x42_allowfullscreen_number; verification\v58_x89_allowfullscreen_comma_only -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _ALLOWFULLSCREEN _STRETCH, _SMOOTH -->

## See also

`_FULLSCREEN`.
