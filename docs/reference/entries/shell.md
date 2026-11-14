# SHELL

`SHELL command$` runs a command of the system's shell and waits for it. `SHELL(command$)` also gives the command's
exit code.

## Syntax

<!-- begin generated: syntax -->
```text
SHELL [_DONTWAIT] [_HIDE] commandToRun$
SHELL(commandToRun$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`_HIDE` runs the command with no console window shown, and `_DONTWAIT` returns at once, in either order. `SHELL` alone
starts the shell.
<!-- src: language/builtin-statements "SHELL and ENVIRON"; verification\v22_d_shell -->

A test runs no `SHELL`, since its console window would show on the user's desktop.
<!-- src: language/builtin-statements "SHELL and ENVIRON" -->

## Errors

- A command that is no string, two arguments, and `_HIDE` twice are compile errors.
  <!-- src: language/builtin-statements "SHELL and ENVIRON"; verification\v22_x159_shell_number; verification\v22_x160_shell_two_args; verification\v22_x161_shell_hide_twice -->
- `CALL SHELL(c$)`, which QB64pe accepts, may be reported "not supported yet".
  <!-- src: language/builtin-statements "SHELL and ENVIRON" -->

## Availability

<!-- begin generated: availability -->
Supported. All 21 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN SHELL _HIDE "dir" -->

## See also

[`_SHELLHIDE`](_shellhide.md), [`ENVIRON`](environ.md), [`CHAIN`](chain.md).
