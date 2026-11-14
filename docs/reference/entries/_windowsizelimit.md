# _WINDOWSIZELIMIT

`_WINDOWSIZELIMIT (minW, minH)-(maxW, maxH)` limits the size the user may resize the window to.

## Syntax

<!-- begin generated: syntax -->
```text
_WINDOWSIZELIMIT [(minWidth, minHeight)] [- (maxWidth, maxHeight)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

The sizes are those of the window's inside, without its frame, in pixels. In `SCREEN 0` they are in characters:
`(40, 10)-(80, 25)` limits the window to 320 × 160 up to 640 × 400 pixels with the 8 × 16 font.
`_WINDOWSIZELIMIT (w, h)` sets only the smallest size and `_WINDOWSIZELIMIT -(w, h)` only the largest; the other
stays as it was. `_WINDOWSIZELIMIT` with no arguments removes both.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\sandbox\window_size_limit; tests\corpus\slice\s66_window_builtins -->

This runs only in the sandbox family, in the test VM. It reads the limits the window gives Windows for a resize
(`WM_GETMINMAXINFO`); no test drags the window's edge.
<!-- src: tests\sandbox\window_size_limit -->

## Errors

- A form QB64pe rejects is a compile error: a size without parentheses, or one number in a pair. For example:
  - `_WINDOWSIZELIMIT 320, 240`: "this `_WINDOWSIZELIMIT` statement matches none of its forms".
  - `_WINDOWSIZELIMIT (320)`: "this `_WINDOWSIZELIMIT` statement matches none of its forms".
  - `_WINDOWSIZELIMIT (1, 2)-(3)`: "this `_WINDOWSIZELIMIT` statement matches none of its forms".
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x87_sizelimit_no_parens; verification\v26_x88_sizelimit_one_in_pair; verification\v26_x89_sizelimit_max_one -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _WINDOWSIZELIMIT (320, 240)-(800, 600) -->

## See also

[`$RESIZE`](meta-resize.md), [`_RESIZE`](_resize.md).
