# Handling errors

A run-time error stops the program unless a handler set by `ON ERROR GOTO` takes it. The handler finds the error in
`ERR` and goes on with `RESUME`.

The pages of this area: [ON ERROR](../entries/on-error.md), [RESUME](../entries/resume.md),
[ERROR](../entries/error.md), [ERR](../entries/err.md), [ERL](../entries/erl.md),
[_ERRORLINE](../entries/_errorline.md), [_INCLERRORLINE](../entries/_inclerrorline.md),
[_ERRORMESSAGE$](../entries/_errormessage-str.md), [$ERRORLOCATION](../entries/meta-errorlocation.md),
[TRON](../entries/tron.md).

## A handler

`ON ERROR GOTO label` makes the lines from the label the handler. When a statement raises an error, the rest of it is
skipped and the handler runs with `ERR` set.
<!-- src: language/error-handling "Error handler"; tests\reference\examples\on-error_1 -->

`RESUME` runs the failing statement again, `RESUME NEXT` goes on after it, and `RESUME label` at a label. Each ends
the handler and sets `ERR` back to 0.
<!-- src: language/error-handling "Resume"; tests\reference\examples\resume_1 -->

`ON ERROR GOTO _NEWHANDLER label` keeps the active handler, and `_LASTHANDLER` returns to it.
<!-- src: language/error-handling "The handler chain" -->

## Where an error came from

`ERR` gives the error's number, `_ERRORLINE` the main file's line, and `_INCLERRORLINE` and `_INCLERRORFILE$` the line
and file inside an included file.
<!-- src: language/error-handling "ERROR, ERR and ERL"; language/error-handling "The line and file of an error" -->

`ERROR n` raises error `n` on purpose; an error that libqb treats as critical ends the program even with a handler.
<!-- src: language/error-handling "ERROR, ERR and ERL" -->

## A program ended by an error

A program that an error ends exits with code 1; QB64pe's exits with 0.
<!-- src: language/error-handling "The exit code of a program ended by an error"; D-012 -->
