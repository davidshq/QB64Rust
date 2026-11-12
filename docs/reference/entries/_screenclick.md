# _SCREENCLICK

`_SCREENCLICK x, y` clicks the mouse at a place on the desktop.

## Syntax

<!-- begin generated: syntax -->
```text
_SCREENCLICK x, y[, button%]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`_SCREENCLICK x, y` clicks the left mouse button at that place of the desktop, in pixels. A program that clicks
its own window at `_SCREENX + 100, _SCREENY + 80` sees `_MOUSEBUTTON(1)` go down and then up through `_MOUSEINPUT`,
at 100, 80 within 2 pixels.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\sandbox\screenclick_mousebutton -->

Under a display scale other than 100%, QB64pe's click lands at the scale times its place; qb64rust's lands at the
place.
<!-- src: D-056 -->

It clicks with the real mouse, so it runs only in the sandbox family, in the test VM, not on the host.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

## Errors

- A form QB64pe rejects is a compile error: one number, a string or four arguments. For example:
  - `_SCREENCLICK 10`: "this `_SCREENCLICK` statement matches none of its forms".
  - `_SCREENCLICK "a", 1`: "argument 1 of `_SCREENCLICK` needs a number".
  - `_SCREENCLICK 1, 2, 3, 4`: "this `_SCREENCLICK` statement matches none of its forms".
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v58_x53_screenclick_one_arg; verification\v58_x55_screenclick_four_args; verification\v58_x54_screenclick_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _SCREENCLICK 10, 20 -->

## See also

[`_SCREENPRINT`](_screenprint.md), [`_MOUSEMOVE`](_mousemove.md).
