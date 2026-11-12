# COMMAND$

`COMMAND$` gives the arguments the program was started with, as one string. `COMMAND$(n)` gives argument `n`, and
`COMMAND$(0)` the program's own name.

## Syntax

```text
args$ = COMMAND$
arg$ = COMMAND$(n)
```

## Result

A STRING.

## Behavior

A program started with no arguments gives "", and `_COMMANDCOUNT` is 0.
<!-- src: language/builtin-functions "Built-ins without parentheses"; verification\v22_d_environ; tests\reference\examples\command-str_1 -->

`COMMAND$` may be written without parentheses.
<!-- src: language/builtin-functions "Built-ins without parentheses" -->

## Errors

- A string argument, two arguments, and `COMMAND$()` are compile errors.
  <!-- src: verification\v22_x167_command_string_arg; verification\v22_x168_command_two; verification\v22_x169_command_empty_parens -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:command-str_1 -->
```qb64
PRINT "The command line: ["; COMMAND$; "]"
PRINT "Arguments:"; _COMMANDCOUNT
END
```

Output:

```text
The command line: []
Arguments: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_COMMANDCOUNT`, `ENVIRON$`.
