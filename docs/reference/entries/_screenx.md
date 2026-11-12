# _SCREENX

`_SCREENX` and `_SCREENY` give the place of the program's window on the desktop, in pixels.

## Syntax

```text
x = _SCREENX
y = _SCREENY
```

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: `_SCREENX()` or an assignment to it.
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x98_screenx_empty; verification\v26_x111_screenx_assign -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN x = _SCREENX: y = _SCREENY -->

## See also

`_SCREENMOVE`, `_DESKTOPWIDTH`.
