# RUN

`RUN` starts the program again from its first line, with every variable cleared. `RUN label` starts it at a label of
the main module, and `RUN file$` starts another program.

## Syntax

```text
RUN
RUN label
RUN linenumber
RUN file$
```

## Behavior

Before starting again, `RUN` resets the screen, the colors, the palette, `RND`'s seed, the key traps and the
keyboard buffer, and clears every variable as `CLEAR` does. A `COMMON SHARED` variable and a SUB's `STATIC` variable
are cleared too.
<!-- src: language/control-flow "RUN"; verification\v57_r01_restart; verification\v57_r18_common_shared; verification\v57_r19_static_in_sub -->

`RUN label` and `RUN linenumber` go on at that label or line number of the main module. `RUN` works the same from a
SUB or FUNCTION, which it leaves first.
<!-- src: language/control-flow "RUN"; language/control-flow "RUN leaves the active procedures first"; verification\v57_r04_label_sub; verification\v57_r06_restart_sub -->

`RUN file$` starts the program `file$` names and ends this one. When the file cannot be started it raises error 53,
and the program goes on after the `RUN`.
<!-- src: language/control-flow "RUN of a file"; verification\v57_r08_missing_file -->

A name after `RUN` that is a string constant, or a string variable under `DEFSTR`, is a file name. A numeric
constant there is a label name.
<!-- src: language/control-flow "RUN of a file"; verification\v57_r20_string_const -->

In a `$CONSOLE:ONLY` program, the output after a `RUN` still goes to the console.
<!-- src: language/control-flow "RUN in a console-only program" -->

## Errors

These are compile errors:

- A label that does not exist, or a label of a SUB: "label `nowhere` is not defined in the main module".
  <!-- src: language/control-flow "RUN"; verification\v57_r12_no_label; verification\v57_r13_label_in_sub -->
- A numeric variable or an expression, as in `RUN n` or `RUN 1 + 2`.
  <!-- src: language/control-flow "RUN"; verification\v57_r14_numeric_variable; verification\v57_r15_expression -->
- With the fix `run-in-procedure` on ([Fixes](../chapters/fixes.md)), `RUN` in a SUB or FUNCTION, as in QuickBASIC
  4.5: "`RUN` cannot stand in a SUB or FUNCTION (the fix `run-in-procedure`): set a flag there and run `RUN` from the
  main module".
  <!-- src: language/control-flow "Fix run-in-procedure" -->

## Differences

From QB64pe:

- `RUN` from a SUB starts the program again inside the SUB there, so the stack grows with each `RUN`, and after
  about 2,400 it ends with "Out of stack space". QB64Rust leaves the active procedures first
  (`DIVERGENCES-QB64PE.md` D-046).
  <!-- src: D-046; verification\v62_g1_run_sub_many -->
- In a `$CONSOLE:ONLY` program, a `RUN` with a target goes on in a window there. QB64Rust stays on the console (D-046).
  <!-- src: D-046 -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: n = n + 1 : IF n > 5 THEN RUN -->

## See also

[`CLEAR`](clear.md), [`CHAIN`](chain.md), [`END`](end.md), [`SHELL`](shell.md).
