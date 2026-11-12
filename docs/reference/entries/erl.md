# ERL

`ERL` gives the number of the last numbered line the program passed before an error, 0 when it passed none.

## Syntax

<!-- begin generated: syntax -->
```text
ERL
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A number.

## Behavior

`ERL` is the last line number passed before the error, in the order the program ran, in any body and any included
file: after `20 x = LOG(0)` it is 20.
<!-- src: language/error-handling "ERROR, ERR and ERL"; tests\reference\examples\erl_1 -->

A line with no number does not change it, and it keeps its value after `RESUME`.
<!-- src: language/error-handling "ERROR, ERR and ERL" -->

The value is the number as QB64pe writes it: `10.5` gives 10.5 and `50.` gives 50.
<!-- src: language/error-handling "ERROR, ERR and ERL" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:erl_1 -->
```qb64
ON ERROR GOTO handler
10 PRINT "Line 10"
20 x = LOG(0)
30 PRINT "Line 30"
END

handler:
PRINT "Error"; ERR; "after line"; ERL
RESUME NEXT
```

Output:

```text
Line 10
Error 5 after line 20 
Line 30

Press any key to continue
```
<!-- end generated -->

## See also

`ERR`, `_ERRORLINE`, `ON ERROR`.
