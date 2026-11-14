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

The function `_FULLSCREEN` gives the mode in use: 0 in a window, 1 after `_FULLSCREEN _STRETCH`, 2 after
`_FULLSCREEN _SQUAREPIXELS`, and 0 again after `_FULLSCREEN _OFF`. `_SMOOTH` is -1 while the `, _SMOOTH` of the
last `_FULLSCREEN` is in effect, and 0 otherwise.
<!-- src: language/builtin-statements "Supported built-in statements"; language/console-and-window "The window's display settings"; tests\sandbox\window_fullscreen -->

It changes the real display, so it runs with its effect read back only in the sandbox family, in the test VM.
<!-- src: tests\sandbox\window_fullscreen; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: a number, `_SMOOTH` first, or two modes. For example:
  - `_FULLSCREEN 5`: "this `_FULLSCREEN` statement matches none of its forms".
  - `_FULLSCREEN _SMOOTH`: "this `_FULLSCREEN` statement matches none of its forms".
  - `_FULLSCREEN _OFF _STRETCH`: "this `_FULLSCREEN` statement matches none of its forms".
  <!-- src: language/builtin-statements "Supported built-in statements"; verification\v28_x50_fullscreen_number; verification\v28_x52_fullscreen_two_modes; verification\v28_x51_fullscreen_smooth_first -->

## Availability

<!-- begin generated: availability -->
Supported. All 22 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _FULLSCREEN _SQUAREPIXELS, _SMOOTH: m = _FULLSCREEN -->

## See also

[`_ALLOWFULLSCREEN`](_allowfullscreen.md), [`_SCALEDWIDTH`](_scaledwidth.md).
