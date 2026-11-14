# ERR

`ERR` gives the number of the error being handled, as 5 for "Illegal function call" or 53 for "File not found".

## Syntax

<!-- begin generated: syntax -->
```text
ERR
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

In a handler `ERR` is the number of the error. Outside a handler, and after `RESUME`, it is 0.
<!-- src: language/error-handling "ERROR, ERR and ERL"; tests\reference\examples\err_1 -->

## Errors

- `ERR` with an argument is a compile error: "`ERR` takes no arguments".
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:err_1 -->
```qb64
ON ERROR GOTO handler
x = SQR(-1)
PRINT "ERR after RESUME NEXT is"; ERR
ERROR 53
END

handler:
PRINT "ERR is"; ERR
RESUME NEXT
```

Output:

```text
ERR is 5 
ERR after RESUME NEXT is 0 
ERR is 53 

Press any key to continue
```
<!-- end generated -->

## See also

[`ERL`](erl.md), [`_ERRORMESSAGE$`](_errormessage-str.md), [`ON ERROR`](on-error.md), [`ERROR`](error.md).
