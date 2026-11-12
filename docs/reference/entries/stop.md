# STOP

`STOP` ends the program at once, as `SYSTEM` does: with no "Press any key to continue" prompt.

## Syntax

```text
STOP [expression]
```

## Behavior

`STOP` ends the program, closes its files, and gives the exit code set so far. It prints no prompt and does not
wait for a key, also in a program with a window.
<!-- src: language/control-flow "STOP"; tests\reference\examples\stop_1; verification\v52_x14_stop_window -->

`STOP` may stand in the main module, in a SUB or FUNCTION, and after `THEN` or `ELSE` on one line.
<!-- src: language/control-flow "STOP" -->

An expression after `STOP` is checked to be a number but is not evaluated: in `STOP f(1)`, the FUNCTION `f` is not
called.
<!-- src: language/control-flow "STOP"; verification\v52_x12_stop_function_arg -->

## Errors

- A string after `STOP` is a compile error: "the expression after `STOP` must be a number".
  <!-- src: language/control-flow "STOP" -->

## Differences

From QB64pe: `STOP` in `SUB _GL` makes the program hang there. QB64Rust ends it with libqb's critical error
"END/SYSTEM called within SUB _GL's scope", as `SYSTEM` does (`DIVERGENCES-QB64PE.md` D-036).
<!-- src: D-036 -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:stop_1 -->
```qb64
PRINT "Before STOP."
STOP
PRINT "Never printed."
```

Output:

```text
Before STOP.
```
<!-- end generated -->

## See also

`SYSTEM`, `END`.
