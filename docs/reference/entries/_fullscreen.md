# _FULLSCREEN

`_FULLSCREEN` shows the program full screen: `_STRETCH`, `_SQUAREPIXELS` or `_OFF`, with `_SMOOTH`. The function
`_FULLSCREEN` gives the mode, and `_SMOOTH` whether it is smoothed.

## Syntax

```text
_FULLSCREEN [{_STRETCH|_SQUAREPIXELS|_OFF}][, _SMOOTH]
mode = _FULLSCREEN
smoothed = _SMOOTH
```

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/builtin-statements "Supported built-in statements"; language/console-and-window "The window's display settings"; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: a number, `_SMOOTH` first, or two modes.
  <!-- src: language/builtin-statements "Supported built-in statements"; verification\v28_x50_fullscreen_number; verification\v28_x52_fullscreen_two_modes -->

## Availability

<!-- begin generated: availability -->
Supported. All 22 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _FULLSCREEN _SQUAREPIXELS, _SMOOTH: m = _FULLSCREEN -->

## See also

`_ALLOWFULLSCREEN`, `_SCALEDWIDTH`.
