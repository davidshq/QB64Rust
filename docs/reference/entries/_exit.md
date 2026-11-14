# _EXIT

`_EXIT` tells whether the user asked to close the program's window or pressed Ctrl+Break. From its first call the
program no longer ends at such a request, and decides itself.

## Syntax

<!-- begin generated: syntax -->
```text
_EXIT
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG: 0 when no request came.

## Behavior

With no request it gives 0.
<!-- src: language/console-and-window "_EXIT"; tests\corpus\slice\s155_console_on_off; tests\reference\examples\_exit_1 -->

After the first call, closing the window or Ctrl+Break gives QB64pe's value for the request instead of ending the
program.
<!-- src: language/console-and-window "_EXIT" -->

## Errors

- `_EXIT` as a statement, and with an argument, are compile errors.
  <!-- src: language/console-and-window "_EXIT"; verification\v53_x62_exit_stmt; verification\v53_x63_exit_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_exit_1 -->
```qb64
PRINT "No request to close:"; _EXIT
END
```

Output:

```text
No request to close: 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`SYSTEM`](system.md), [`END`](end.md).
