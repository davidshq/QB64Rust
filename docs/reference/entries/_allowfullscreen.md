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

With no `_ALLOWFULLSCREEN`, Alt+Enter switches a window to `_STRETCH`, and Alt+Enter again to `_STRETCH` with
`_SMOOTH`: the function `_FULLSCREEN` gives 1, then 1 with `_SMOOTH` -1. After `_ALLOWFULLSCREEN _SQUAREPIXELS`,
Alt+Enter switches to `_SQUAREPIXELS` (2). After `_ALLOWFULLSCREEN _OFF`, Alt+Enter changes nothing.
<!-- src: language/console-and-window "The window's display settings"; tests\sandbox\window_allowfullscreen; verification\v58_s05_display_window -->

This runs only in the sandbox family, in the test VM: the program presses Alt+Enter itself, with Windows'
`keybd_event`, after it clicks its own window.
<!-- src: tests\sandbox\window_allowfullscreen -->

## Errors

- A form QB64pe rejects is a compile error: a number, or a comma alone. For example:
  - `_ALLOWFULLSCREEN 1`: "this `_ALLOWFULLSCREEN` statement matches none of its forms".
  - `_ALLOWFULLSCREEN _STRETCH, _STRETCH`: "this `_ALLOWFULLSCREEN` statement matches none of its forms".
  - `_ALLOWFULLSCREEN ,`: "this `_ALLOWFULLSCREEN` statement matches none of its forms".
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x42_allowfullscreen_number; verification\v58_x89_allowfullscreen_comma_only; verification\v58_x43_allowfullscreen_stretch_twice -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _ALLOWFULLSCREEN _STRETCH, _SMOOTH -->

## See also

[`_FULLSCREEN`](_fullscreen.md).
