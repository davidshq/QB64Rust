# _SCREENICON

`_SCREENICON` minimizes the program's window. The function `_SCREENICON` tells whether it is minimized.

## Syntax

```text
_SCREENICON
minimized = _SCREENICON
```

## Behavior

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\slice\s66_window_builtins; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: an argument.
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x97_screenicon_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _SCREENICON: m = _SCREENICON -->

## See also

`_SCREENHIDE`, `_SCREENMOVE`.
