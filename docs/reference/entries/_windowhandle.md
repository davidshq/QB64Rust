# _WINDOWHANDLE

`_WINDOWHANDLE` gives the system's handle of the program's window, for calls to the system's libraries.

## Syntax

<!-- begin generated: syntax -->
```text
_WINDOWHANDLE[(handleType&)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

It gives the Windows handle of the program's window, for the calls of a `DECLARE LIBRARY` block: Windows' own
`IsWindow` of it is true, and `GetWindowTextA` of it reads the window's title.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\sandbox\window_title_handle; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: `_WINDOWHANDLE()` or two arguments. For example:
  - `n = _WINDOWHANDLE()`: "`_WINDOWHANDLE` with empty parentheses".
  - `n = _WINDOWHANDLE(1, 2)`: "`_WINDOWHANDLE` takes 0 or 1 arguments".
  - `n = _WINDOWHANDLE("a")`: "`_WINDOWHANDLE` needs a number".
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x108_windowhandle_empty; verification\v26_x109_windowhandle_two; verification\v26_x110_windowhandle_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN h%& = _WINDOWHANDLE -->

## See also

[`_WINDOWHASFOCUS`](_windowhasfocus.md), [`DECLARE LIBRARY`](declare-library.md).
