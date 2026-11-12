# _ERRORLINE

`_ERRORLINE` gives the line of the main file where the last error was raised, numbered or not.

## Syntax

<!-- begin generated: syntax -->
```text
_ERRORLINE
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

`_ERRORLINE` is the line of the main file where the last error was raised, 0 before any. It keeps its value after
`RESUME`.
<!-- src: language/error-handling "The line and file of an error"; tests\reference\examples\_errorline_1 -->

For an error in an included file it is the main file's line of the `$INCLUDE`. `_INCLERRORLINE` gives the line in the
included file.
<!-- src: language/error-handling "The line and file of an error" -->

A statement continued over several lines with `_` counts as the last of them.
<!-- src: language/error-handling "The line of a joined line" -->

## Errors

- `_ERRORLINE` in a `CONST` is a compile error.
  <!-- src: language/error-handling "The line and file of an error" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_errorline_1 -->
```qb64
ON ERROR GOTO handler
PRINT "line 3"
x = SQR(-1)
PRINT "After RESUME NEXT, _ERRORLINE is still"; _ERRORLINE
END

handler:
PRINT "Error"; ERR; "on line"; _ERRORLINE
RESUME NEXT
```

Output:

```text
line 3
Error 5 on line 4 
After RESUME NEXT, _ERRORLINE is still 4 

Press any key to continue
```
<!-- end generated -->

## See also

`ERL`, `_INCLERRORLINE`, `ERR`, `$ERRORLOCATION`.
