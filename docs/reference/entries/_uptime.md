# _UPTIME

`_UPTIME` gives the seconds since the machine started.

## Syntax

<!-- begin generated: syntax -->
```text
_UPTIME
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A number above 0.

## Behavior

Its value is the machine's, so a program compares it: `_UPTIME > 0` is true.
<!-- src: language/builtin-functions "System information functions"; tests\corpus\slice\s159_builtins_last; tests\reference\examples\_uptime_1 -->

## Errors

- An argument, and `_UPTIME` as a statement, are compile errors.
  <!-- src: language/builtin-functions "System information functions"; verification\v58_x67_uptime_arg; verification\v58_x88_uptime_statement -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_uptime_1 -->
```qb64
PRINT "Seconds since the machine started, over 0:"; _UPTIME > 0
END
```

Output:

```text
Seconds since the machine started, over 0:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`TIMER`.
