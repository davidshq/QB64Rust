# _SCREENMOVE

`_SCREENMOVE x, y` moves the program's window to a place on the desktop, and `_SCREENMOVE _MIDDLE` centers it.

## Syntax

<!-- begin generated: syntax -->
```text
_SCREENMOVE x, y
_SCREENMOVE _MIDDLE
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`_SCREENMOVE x, y` moves the window to that place of the desktop, in pixels, as `_SCREENX` and `_SCREENY` give
it: after `_SCREENMOVE 100, 120` they are 100 and 120.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\sandbox\screenmove_readback -->

`_SCREENMOVE _MIDDLE` centers the window on the desktop: across within a pixel, and down up to 40 pixels above the
middle, as the taskbar takes part of the desktop's height.
<!-- src: tests\sandbox\screenmove_readback -->

It moves a real window, so it runs only in the sandbox family, in the test VM, not on the host.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

## Errors

- A form QB64pe rejects is a compile error: one number, a string or no argument. For example:
  - `_SCREENMOVE _MIDDLE, 10, 20`: "this `_SCREENMOVE` statement matches none of its forms".
  - `_SCREENMOVE 10`: "this `_SCREENMOVE` statement matches none of its forms".
  - `x = _SCREENMOVE`: "names starting with `_` are reserved: `_SCREENMOVE`".
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v53_x40_screenmove_one; verification\v53_x43_screenmove_string; verification\v53_x39_screenmove_middle_xy; verification\v53_x42_screenmove_fn -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _SCREENMOVE _MIDDLE -->

## See also

[`_SCREENX`](_screenx.md).
