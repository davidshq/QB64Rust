# ERROR

`ERROR n` raises run-time error `n`, as if a statement had failed. A program uses it to test a handler, or to raise
errors of its own above the ones BASIC uses.

## Syntax

```text
ERROR number
```

## Behavior

`ERROR n` raises error `n`: a handler runs with `ERR` equal to `n`. A number that is not whole is rounded half to
even.
<!-- src: language/error-handling "ERROR, ERR and ERL"; tests\reference\examples\error_1 -->

A number of 0 or less raises error 5 instead. A critical error number, such as 256, ends the program even with a
handler.
<!-- src: language/error-handling "ERROR, ERR and ERL" -->

## Differences

From QB64pe: `ERROR 11` is a critical error there, which no handler catches. QB64Rust raises a trappable error 11
(`DIVERGENCES-QB64PE.md` D-052).
<!-- src: D-052 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:error_1 -->
```qb64
ON ERROR GOTO handler
ERROR 5
ERROR 200
PRINT "Done."
END

handler:
PRINT "Error"; ERR; "raised by ERROR"
RESUME NEXT
```

Output:

```text
Error 5 raised by ERROR
Error 200 raised by ERROR
Done.

Press any key to continue
```
<!-- end generated -->

## See also

`ON ERROR`, `ERR`.
