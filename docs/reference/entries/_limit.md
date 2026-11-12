# _LIMIT

`_LIMIT rate` holds a loop to at most `rate` rounds a second: it waits for what is left of the round's share of the
second.

## Syntax

<!-- begin generated: syntax -->
```text
_LIMIT framesPerSecond!
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

Ten rounds of a loop with `_LIMIT 20` take about half a second.
<!-- src: language/screen-output "Pacing statements"; tests\reference\examples\_limit_1 -->

A screen program paced by `_LIMIT` leaves the screen QB64pe's program leaves.
<!-- src: language/screen-output "Pacing statements"; tests\corpus\screen\d15_pacing -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/screen-output "Pacing statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_limit_1 -->
```qb64
t# = TIMER(.001)
FOR i = 1 TO 10
    _LIMIT 20
NEXT
PRINT "Ten rounds at 20 a second took over 0.3 seconds:"; Taken#(t#) > .3
END

' The seconds since t#, also across midnight.
FUNCTION Taken# (t#)
    e# = TIMER(.001) - t#
    IF e# < 0 THEN e# = e# + 86400
    Taken# = e#
END FUNCTION
```

Output:

```text
Ten rounds at 20 a second took over 0.3 seconds:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_DELAY`, `_DISPLAY`, `SLEEP`.
