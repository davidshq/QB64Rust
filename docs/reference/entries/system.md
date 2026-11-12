# SYSTEM

`SYSTEM` ends the program at once, with no "Press any key to continue" prompt. `SYSTEM n` gives the exit code `n`.

## Syntax

```text
SYSTEM [exitcode]
```

## Arguments

- `exitcode`: a number, the exit code the program gives the system.

## Behavior

`SYSTEM` ends the program without a prompt and without waiting.
<!-- src: language/console-and-window "The end of a program and errors with a hidden window"; tests\reference\examples\system_1 -->

`SYSTEM n` ends it with the exit code `n`: `PRINT "x": SYSTEM 3` prints `x` and exits with code 3. A pending error is
handled before the code is stored.
<!-- src: language/control-flow "END and SYSTEM with an exit code" -->

## Errors

- A string as the exit code is a compile error: "the exit code of `SYSTEM` must be a number".
  <!-- src: language/control-flow "END and SYSTEM with an exit code" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:system_1 -->
```qb64
PRINT "SYSTEM ends the program at once."
SYSTEM
PRINT "Never printed."
```

Output:

```text
SYSTEM ends the program at once.
```
<!-- end generated -->

## See also

[`END`](end.md), [`STOP`](stop.md), [`_EXIT`](_exit.md).
