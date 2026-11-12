# GOSUB

`GOSUB` jumps to a label and remembers where it was; `RETURN` goes back to the statement after the `GOSUB`. It is
the oldest way to write a subroutine in BASIC.

## Syntax

```text
GOSUB label
...
label:
    statements
RETURN [label]
```

## Behavior

`GOSUB label` continues at the label. `RETURN` continues after the most recent `GOSUB` that has not returned yet.
`RETURN label` forgets that `GOSUB` and continues at the label instead.
<!-- src: language/control-flow "GOSUB and RETURN"; tests\reference\examples\gosub_1 -->

`GOSUB` works in the main module and inside a SUB or FUNCTION, with labels of the same body. A line number may be
the target.
<!-- src: language/control-flow "GOSUB and RETURN"; language/control-flow "Line numbers as jump targets" -->

The program has one stack of pending `GOSUB`s. A procedure left with its own `GOSUB` still pending leaves it on the
stack.
<!-- src: language/control-flow "GOSUB and RETURN" -->

A bare `RETURN` may stand just before `ELSE` in a single-line `IF`: `IF a = 0 THEN RETURN ELSE PRINT "a"`.
<!-- src: language/control-flow "GOSUB and RETURN"; verification\v17_d_return_before_else -->

## Errors

- `RETURN` with no `GOSUB` pending raises run-time error 3. So does a `RETURN` in a
  procedure whose most recent pending `GOSUB` belongs to another body; that `GOSUB` is then taken off the stack.
  <!-- src: language/control-flow "GOSUB and RETURN" -->
- `RETURN label` inside a SUB or FUNCTION is a compile error: "`RETURN` with a label or line number is not allowed
  in a SUB or FUNCTION".
  <!-- src: language/control-flow "GOSUB and RETURN" -->

## Differences

From QB64pe: `RETURN label` with no `GOSUB` pending makes the next `GOSUB` crash the program there. QB64Rust raises
error 3 and goes on normally (`DIVERGENCES-QB64PE.md` D-003).
<!-- src: D-003 -->

From QuickBASIC 4.5: a SUB that ends with its own `GOSUB` pending drops it there. In QB64Rust, as in QB64pe, it
stays on the one stack, so the main module's next `RETURN` takes it (`DIVERGENCES-QB45.md` Q-018).
<!-- src: Q-018 -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:gosub_1 -->
```qb64
x = 3: GOSUB square
x = 5: GOSUB square
PRINT "Back in the main program."
END

square:
PRINT x; "squared is"; x * x
RETURN
```

Output:

```text
 3 squared is 9 
 5 squared is 25 
Back in the main program.

Press any key to continue
```
<!-- end generated -->

## See also

`GOTO`, `ON`, `SUB`.
