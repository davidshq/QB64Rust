# _INCLERRORLINE

`_INCLERRORLINE` gives the line, inside an included file, where the last error was raised, and `_INCLERRORFILE$` the
name of that file.

## Syntax

<!-- begin generated: syntax -->
```text
_INCLERRORFILE$
_INCLERRORLINE
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

`_INCLERRORLINE`: a LONG. `_INCLERRORFILE$`: a STRING.

## Behavior

For an error raised in an included file, `_INCLERRORLINE` is its line in the innermost included file and
`_INCLERRORFILE$` that file's name, without its folder.
<!-- src: language/error-handling "The line and file of an error"; tests\reference\examples\_inclerrorline_1 -->

For an error in the main file they are 0 and `""`.
<!-- src: language/error-handling "The line and file of an error" -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_inclerrorline_1 -->
```qb64
ON ERROR GOTO handler
'$INCLUDE: 'risky.bi'
END

handler:
PRINT "Error"; ERR; "at main line"; _ERRORLINE
PRINT "In "; _INCLERRORFILE$; ", line"; _INCLERRORLINE
RESUME NEXT
```

The file `risky.bi`:

```qb64
' risky.bi: its third line raises an error.
PRINT "in risky.bi"
x = LOG(0)
```

Output:

```text
in risky.bi
Error 5 at main line 3 
In risky.bi, line 3 

Press any key to continue
```
<!-- end generated -->

## See also

[`_ERRORLINE`](_errorline.md), [`$INCLUDE`](meta-include.md), [`ERL`](erl.md).
