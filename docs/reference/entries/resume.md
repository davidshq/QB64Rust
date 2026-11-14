# RESUME

`RESUME` ends an error handler. `RESUME` runs the failed statement again, `RESUME NEXT` goes on after it, and `RESUME
label` goes on at a label.

## Syntax

```text
RESUME
RESUME 0
RESUME NEXT
RESUME label
```

## Behavior

`RESUME` and `RESUME 0` run the statement that raised the error again, from its start: a handler that fixes the cause
makes the statement succeed.
<!-- src: language/error-handling "Resume"; tests\reference\examples\resume_1 -->

`RESUME NEXT` goes on with the statement after the one that raised the error. `RESUME label` goes on at the label.
<!-- src: language/error-handling "Resume" -->

Each form ends the handler and sets `ERR` back to 0. `ERL` and `_ERRORLINE` keep their values.
<!-- src: language/error-handling "Resume"; language/error-handling "ERROR, ERR and ERL" -->

After an error in a block header, `RESUME NEXT` goes on inside the `THEN` branch of an `IF`, inside the body of a
`WHILE` or `DO WHILE`, after the loop for a `LOOP UNTIL`, and at the start of the body for a `FOR`.
<!-- src: language/error-handling "Errors in block headers" -->

With the fix `resume-after-block` on ([Fixes](../chapters/fixes.md)), `RESUME NEXT` after an error in the condition
of a `WHILE`, `DO WHILE`, `DO UNTIL`, `LOOP WHILE`, `LOOP UNTIL`, `IF` or `ELSEIF` goes on after the whole block:
after its `WEND`, `LOOP` or `END IF`, or after the line of a one-line `IF`. `RESUME` still evaluates the condition
again.
<!-- src: language/error-handling "Fix resume-after-block" -->

In a SUB, `RESUME label` takes a label of the SUB, and the other forms leave the SUB as `EXIT SUB` does.
<!-- src: language/error-handling "RESUME inside a procedure"; language/error-handling "RESUME leaves a SUB through its exit" -->

## Errors

- `RESUME` while no error is being handled raises run-time error 20.
  <!-- src: language/error-handling "Resume" -->
- `RESUME` in a FUNCTION is a compile error: "`RESUME` cannot stand in a FUNCTION: QB64pe's C++ for it does not
  build".
  <!-- src: language/error-handling "RESUME inside a procedure" -->
- With the fix `resume-in-procedure` on, any `RESUME` in a SUB or FUNCTION is a compile error, as in QuickBASIC 4.5:
  "`RESUME` belongs in an error handler of the main module, not in a SUB or FUNCTION (the fix
  `resume-in-procedure`)".
  <!-- src: language/error-handling "Fix resume-in-procedure" -->

## Differences

From QB64pe: a `RESUME` that leaves a SUB skips its exit there, so its strings are never freed. QB64Rust leaves it as
`EXIT SUB` does (`DIVERGENCES-QB64PE.md` D-045).
<!-- src: D-045 -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:resume_1 -->
```qb64
ON ERROR GOTO retry
d = -16
PRINT "The square root of"; d; "is"; SQR(d)
PRINT "Done."
END

retry:
PRINT "Error"; ERR; ": using 16 instead."
d = 16
RESUME
```

Output:

```text
The square root of-16 isError 5 : using 16 instead.
The square root of 16 is 4 
Done.

Press any key to continue
```
<!-- end generated -->

## See also

[`ON ERROR`](on-error.md), [`ERR`](err.md), [`ERL`](erl.md).
