# _COMMANDCOUNT

`_COMMANDCOUNT` gives the number of arguments the program was started with, for `COMMAND$(n)`.

## Syntax

<!-- begin generated: syntax -->
```text
_COMMANDCOUNT
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

With no arguments it is 0; `COMMAND$(0)`, the program's name, is not counted.
<!-- src: language/builtin-functions "The system's folders, commands and build"; tests\corpus\slice\s156_system_builtins; tests\reference\examples\_commandcount_1 -->

## Errors

- An argument is a compile error.
  <!-- src: language/builtin-functions "The system's folders, commands and build"; verification\v53_x56_commandcount_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_commandcount_1 -->
```qb64
PRINT "Arguments:"; _COMMANDCOUNT
PRINT "COMMAND$(0) names the program:"; LEN(COMMAND$(0)) > 0
END
```

Output:

```text
Arguments: 0 
COMMAND$(0) names the program:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`COMMAND$`](command-str.md).
