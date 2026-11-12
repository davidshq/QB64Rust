# _LOGMINLEVEL

`_LOGMINLEVEL` gives the runtime's minimum level of the log: entries below it are not written.

## Syntax

<!-- begin generated: syntax -->
```text
_LOGMINLEVEL
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

The level comes from the `QB64PE_LOG_*` environment variables. The test runner sets no level, and there it is 2.
<!-- src: language/logging "The minimum level"; tests\corpus\slice\s153_handlers_log_checking; tests\reference\examples\_logminlevel_1 -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/logging "The minimum level" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_logminlevel_1 -->
```qb64
PRINT "The minimum level:"; _LOGMINLEVEL
END
```

Output:

```text
The minimum level: 2 

Press any key to continue
```
<!-- end generated -->

## See also

`_LOGINFO`.
