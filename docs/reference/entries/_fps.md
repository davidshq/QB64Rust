# _FPS

`_FPS n` sets how many times a second the window is redrawn.

## Syntax

```text
_FPS {rate|_AUTO}
```

## Behavior

`_FPS 30` and `_FPS _AUTO` compile to QB64pe's call.
<!-- src: language/console-and-window "The window's display settings" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_FPS "a"`: "`_FPS` needs a number".
  - `_FPS 30, 1`: "this `_FPS` statement matches none of its forms".
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x45_fps_string; verification\v58_x46_fps_two_args -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: _FPS 30 -->

## See also

`_LIMIT`, `_DISPLAY`.
