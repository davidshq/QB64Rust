# _FREETIMER

`_FREETIMER` sets aside a new timer and gives its number, for `ON TIMER(n, seconds)` and `TIMER(n) ON`. Each timer
runs its handler at its own interval.

## Syntax

<!-- begin generated: syntax -->
```text
_FREETIMER
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG: 1 for the first timer, 2 for the second.

## Behavior

Two timers set aside one after the other are 1 and 2. After `TIMER(1) FREE`, the next `_FREETIMER` gives 1 again.
<!-- src: language/events "Numbered timers"; verification\v42_b_numbered; tests\reference\examples\_freetimer_1 -->

Two timers at 0.6 and 1 second run their handlers twice and once in 1.5 seconds.
<!-- src: language/events "Numbered timers"; tests\corpus\slice\s134_events_timer -->

## Errors

- `TIMER FREE`, of timer 0, raises error 5, as do `ON TIMER` and `TIMER ON` of a timer released.
  <!-- src: language/events "Numbered timers"; verification\v42_b_numbered; tests\reference\examples\_freetimer_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_freetimer_1 -->
```qb64
ON ERROR GOTO trap
t = _FREETIMER
u = _FREETIMER
PRINT "Two timers:"; t; u
TIMER(t) FREE
PRINT "After TIMER(t) FREE:"; _FREETIMER
PRINT "TIMER FREE:";
TIMER FREE
PRINT
END
trap:
PRINT " error"; ERR;
RESUME NEXT
```

Output:

```text
Two timers: 1  2 
After TIMER(t) FREE: 1 
TIMER FREE: error 5 

Press any key to continue
```
<!-- end generated -->

## See also

`TIMER`.
