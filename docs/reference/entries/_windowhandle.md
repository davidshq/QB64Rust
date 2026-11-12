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

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error: `_WINDOWHANDLE()` or two arguments.
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x108_windowhandle_empty; verification\v26_x109_windowhandle_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN h%& = _WINDOWHANDLE -->

## See also

`_WINDOWHASFOCUS`, `DECLARE LIBRARY`.
