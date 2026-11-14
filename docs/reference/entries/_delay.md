# _DELAY

`_DELAY seconds` waits that many seconds, with no key or event ending the wait early.

## Syntax

<!-- begin generated: syntax -->
```text
_DELAY seconds!
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`_DELAY 0.5` waits about half a second.
<!-- src: language/screen-output "Pacing statements"; tests\reference\examples\_delay_1 -->

Timer events that come during the wait run their handlers: a timer of 0.6 second runs its handler twice in `_DELAY
1.5`.
<!-- src: language/events "Event switches"; tests\corpus\slice\s134_events_timer; tests\reference\examples\timer_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_DELAY t`: "`_DELAY` needs a number".
  <!-- src: language/screen-output "Pacing statements"; verification\v27_x49_delay_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_delay_1 -->
```qb64
t# = TIMER(.001)
_DELAY 0.5
PRINT "_DELAY 0.5 waited over 0.4 seconds:"; Taken#(t#) > .4
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
_DELAY 0.5 waited over 0.4 seconds:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`SLEEP`](sleep.md), [`_LIMIT`](_limit.md), [`TIMER`](timer.md).
