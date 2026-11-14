# _DESKTOPREFRESHRATE

`_DESKTOPREFRESHRATE` gives the refresh rate of the desktop's display, in hertz.

## Syntax

<!-- begin generated: syntax -->
```text
_DESKTOPREFRESHRATE
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

It gives the refresh rate of the desktop's display, in hertz: a number above 0 in the test VM. The value is the
machine's, so no test compares it.
<!-- src: language/console-and-window "The window's display settings"; verification\v58_f14_display_window; tests\sandbox\window_fullscreen -->

## Errors

- A form QB64pe rejects is a compile error: an argument, or the name as a statement. For example:
  - `x = _DESKTOPREFRESHRATE(1)`: "`_DESKTOPREFRESHRATE` takes no arguments".
  - `_DESKTOPREFRESHRATE`: "`_DESKTOPREFRESHRATE` is a function, not a statement".
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x48_desktoprefreshrate_arg; verification\v58_x94_desktoprefreshrate_statement -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN r = _DESKTOPREFRESHRATE -->

## See also

[`_DESKTOPWIDTH`](_desktopwidth.md).
