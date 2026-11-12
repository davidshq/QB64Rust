# END

`END` ends the program. In a console program it first prints "Press any key to continue" and waits for a key.
`END` also closes blocks (`END IF`, `END SUB`, `END SELECT` …), which have their own pages.

## Syntax

```text
END [exitcode]
```

## Arguments

- `exitcode`: a number, the exit code the program gives the system.

## Behavior

`END` ends the program. In a program with a console it prints an empty line and "Press any key to continue", and
waits for a key when its input is a console. When its input is a file, a pipe or the null device, it prints the
line and ends at once. Running off the program's last line does the same.
<!-- src: language/console-and-window "The end of a program and errors with a hidden window"; tests\reference\examples\end_1 -->

`END n` ends the program with the exit code `n`. A pending error is handled first.
<!-- src: language/control-flow "END and SYSTEM with an exit code" -->

`END` may stand in a SUB or FUNCTION.
<!-- src: language/control-flow "RUN leaves the active procedures first" -->

## Errors

- A string as the exit code is a compile error.
  <!-- src: language/control-flow "END and SYSTEM with an exit code" -->

## Differences

From QB64pe: when the program's input is a file, a pipe or the null device, `END` waits there for a key that never
comes. QB64Rust prints the line and ends (`DIVERGENCES-QB64PE.md` D-048).
<!-- src: D-048 -->

## Availability

<!-- begin generated: availability -->
Supported. All 348 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:end_1 -->
```qb64
PRINT "The last line of the program."
END
PRINT "Never printed."
```

Output:

```text
The last line of the program.

Press any key to continue
```
<!-- end generated -->

## See also

`SYSTEM`, `STOP`, `_EXIT`.
