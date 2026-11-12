# _SHELLHIDE

`_SHELLHIDE(command$)` runs a command of the system's shell with no window, waits for it, and gives its exit code.

## Syntax

<!-- begin generated: syntax -->
```text
_SHELLHIDE(commandToRun$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An `_INTEGER64`.

## Behavior

`_SHELLHIDE("cmd /c exit 3")` is 3.
<!-- src: language/builtin-functions "The system's folders, commands and build"; tests\corpus\slice\s156_system_builtins -->

The examples run no command, since a command may show a window on the user's desktop.
<!-- src: language/builtin-functions "The system's folders, commands and build" -->

## Errors

- A number, no argument, or `_SHELLHIDE` as a statement is a compile error.
  <!-- src: language/builtin-functions "The system's folders, commands and build"; verification\v53_x53_shellhide_number; verification\v53_x54_shellhide_bare; verification\v53_x55_shellhide_stmt -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN r&& = _SHELLHIDE("cmd /c exit 0") -->

## See also

`SHELL`.
