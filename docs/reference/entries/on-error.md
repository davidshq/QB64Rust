# ON ERROR

`ON ERROR GOTO label` names an error handler: when a statement raises a run-time error, the program goes on at the
label instead of stopping. `ON ERROR GOTO 0` turns the handler off.

## Syntax

```text
ON ERROR GOTO label
ON ERROR GOTO 0
ON ERROR GOTO _NEWHANDLER label
ON ERROR GOTO _LASTHANDLER
```

## Behavior

When a statement raises an error and a handler is active and not already running, the rest of the statement is skipped
and the handler runs, with `ERR` giving the error's number. The handler ends with `RESUME`.
<!-- src: language/error-handling "Error handler"; tests\reference\examples\on-error_1 -->

The label is one of the main module. `ON ERROR GOTO` in a SUB names a main-module label and sets the same handler,
which stays active after the SUB returns. An error raised inside a SUB is handled there too, and `RESUME NEXT` goes on
inside the SUB.
<!-- src: language/error-handling "Error handler"; language/error-handling "Errors raised inside procedures" -->

Without a handler, an error stops the program with QB64pe's message. Critical errors, such as running out of stack
space, are never trapped.
<!-- src: language/error-handling "Error handler" -->

`ON ERROR GOTO _NEWHANDLER label` keeps the active handler and makes `label` the active one. `ON ERROR GOTO
_LASTHANDLER` goes back to the handler kept last. `ON ERROR GOTO 0` forgets them all.
<!-- src: language/error-handling "The handler chain" -->

A line number may stand where a label does: `ON ERROR GOTO 100`.
<!-- src: language/error-handling "Line numbers in error handling" -->

## Errors

- A label that does not exist, or one that stands only in a procedure: "label `nowhere` is not defined in the main
  module".
  <!-- src: language/error-handling "Errors raised inside procedures" -->
- `ON LOCAL ERROR` is BASIC PDS 7.1's: "`ON LOCAL ERROR` is BASIC PDS 7.1's, and QB64 does not have it: use `ON ERROR
  GOTO` with the handler in the main module, and `ON ERROR GOTO 0` before the SUB ends".
  <!-- src: language/error-handling "Error handler" -->
- `ON ERROR GOTO _LASTHANDLER label`: "No label allowed after _LASTHANDLER". `ON ERROR GOTO _NEWHANDLER 0`: "Zero not
  allowed after _NEWHANDLER".
  <!-- src: language/error-handling "The handler chain's errors" -->

## Differences

From QB64pe: an integer division by zero is a critical error there, which no handler catches. QB64Rust raises error
11, which a handler catches, as QuickBASIC 4.5 does (`DIVERGENCES-QB64PE.md` D-052).
<!-- src: D-052; Q-003 -->

A program that ends because of an error exits with code 1; QB64pe's exits with 0 (`DIVERGENCES-QB64PE.md` D-012).
<!-- src: D-012; language/error-handling "The exit code of a program ended by an error" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:on-error_1 -->
```qb64
ON ERROR GOTO handler
PRINT "Opening a file that does not exist..."
OPEN "no such file.txt" FOR INPUT AS #1
PRINT "Going on after the error."
ON ERROR GOTO 0
END

handler:
PRINT "Error"; ERR; "was trapped."
RESUME NEXT
```

Output:

```text
Opening a file that does not exist...
Error 53 was trapped.
Going on after the error.

Press any key to continue
```
<!-- end generated -->

## See also

[`RESUME`](resume.md), [`ERR`](err.md), [`ERL`](erl.md), [`ERROR`](error.md), [`_ERRORLINE`](_errorline.md).
