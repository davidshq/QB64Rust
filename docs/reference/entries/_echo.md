# _ECHO

`_ECHO text$` writes a line to the console, whatever the destination of `PRINT` is. It is the way a program with a
window writes a log line to its console.

## Syntax

```text
_ECHO text$
```

## Behavior

`_ECHO` writes the string and a line end to the console, and leaves the destination as it was.
<!-- src: language/console-and-window "_ECHO"; tests\reference\examples\_echo_1 -->

## Errors

- `_ECHO` in a program with no `$CONSOLE` or `$CONSOLE:ONLY` is a compile error.
  <!-- src: language/console-and-window "_ECHO" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_echo_1 -->
```qb64
_ECHO "_ECHO writes to the console, whatever the destination."
PRINT "PRINT writes to the destination."
END
```

Output:

```text
_ECHO writes to the console, whatever the destination.
PRINT writes to the destination.

Press any key to continue
```
<!-- end generated -->

## See also

`$CONSOLE`, `PRINT`, `_LOGINFO`.
