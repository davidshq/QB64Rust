# _SCREENICON

`_SCREENICON` minimizes the program's window. The function `_SCREENICON` tells whether it is minimized.

## Syntax

```text
_SCREENICON
minimized = _SCREENICON
```

## Behavior

`_SCREENICON` minimizes the window: Windows' `IsIconic` of it is then true, and the function `_SCREENICON` gives
-1 (0 before). `_SCREENSHOW` does not restore a minimized window: it stays minimized.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\sandbox\window_hide_icon; tests\corpus\screen\w01_window_builtins -->

This runs only in the sandbox family, in the test VM. In about one run in three, a program that has minimized its
window did not end within 20 seconds, in QB64pe's build as in QB64Rust's; its output was complete.
<!-- src: tests\sandbox\README.md -->

## Errors

- A form QB64pe rejects is a compile error: an argument. For example:
  - `_SCREENICON 1`: "`_SCREENICON` takes 0 arguments".
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x97_screenicon_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _SCREENICON: m = _SCREENICON -->

## See also

[`_SCREENHIDE`](_screenhide.md), [`_SCREENMOVE`](_screenmove.md).
