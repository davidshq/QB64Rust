# _SCREENX

`_SCREENX` and `_SCREENY` give the place of the program's window on the desktop, in pixels.

## Syntax

```text
x = _SCREENX
y = _SCREENY
```

## Behavior

`_SCREENX` and `_SCREENY` give the place of the program's window on the desktop, in pixels, as `_SCREENMOVE` sets
it: after `_SCREENMOVE 100, 120` they are 100 and 120.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\sandbox\screenmove_readback; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: `_SCREENX()` or an assignment to it. For example:
  - `_SCREENX = 1`: "names starting with `_` are reserved: `_SCREENX`".
  - `n = _SCREENX()`: "`_SCREENX` with empty parentheses".
  - `n = _SCREENX(1)`: "`_SCREENX` takes no arguments".
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x98_screenx_empty; verification\v26_x111_screenx_assign; verification\v26_x99_screenx_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN x = _SCREENX: y = _SCREENY -->

## See also

[`_SCREENMOVE`](_screenmove.md), [`_DESKTOPWIDTH`](_desktopwidth.md).
