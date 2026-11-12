# _ERRORMESSAGE$

`_ERRORMESSAGE$` gives the message of the last error, and `_ERRORMESSAGE$(n)` the message of error `n`:
`_ERRORMESSAGE$(53)` is `"File not found"`.

## Syntax

<!-- begin generated: syntax -->
```text
_ERRORMESSAGE$[(errorCode&)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING.

## Behavior

`_ERRORMESSAGE$` is the message of the last error, and `_ERRORMESSAGE$(n)` that of error `n`, `n` rounded half to
even.
<!-- src: language/error-handling "An error's message"; tests\reference\examples\_errormessage-str_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_errormessage-str_1 -->
```qb64
PRINT _ERRORMESSAGE$(5)
PRINT _ERRORMESSAGE$(53)
ON ERROR GOTO handler
ERROR 6
END

handler:
PRINT "Trapped: "; _ERRORMESSAGE$
RESUME NEXT
```

Output:

```text
Illegal function call
File not found
Trapped: Overflow

Press any key to continue
```
<!-- end generated -->

## See also

[`ERR`](err.md), [`ON ERROR`](on-error.md).
