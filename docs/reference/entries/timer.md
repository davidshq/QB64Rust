# TIMER

`TIMER` gives the seconds since midnight. `ON TIMER(seconds) GOSUB label` runs a handler every so many seconds, and
`TIMER ON`, `OFF`, `STOP` and `FREE` switch it.

## Syntax

<!-- begin generated: syntax -->
```text
TIMER[(number%)] {ON|OFF|STOP|FREE}
TIMER[(accuracy!)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
ON TIMER([n,] seconds) GOSUB label
ON TIMER([n,] seconds) subname [argument]
```

## Result

The function: a SINGLE, from 0 to below 86400. `TIMER(.001)` rounds to thousandths of a second.

## Behavior

The function's value is the clock's, so a program compares it with limits: `TIMER >= 0 AND TIMER < 86400` is true. A
program that times itself adds 86400 to a difference below 0, for a run across midnight.
<!-- src: language/builtin-functions "Built-ins without parentheses"; tests\reference\examples\timer_1 -->

`ON TIMER(seconds) GOSUB label` sets the handler of timer 0. The handler does not run until `TIMER ON`, and then runs
every `seconds`: twice in 1.5 seconds at 0.6.
<!-- src: language/events "Timer handlers"; language/events "Event switches"; tests\corpus\slice\s134_events_timer; verification\v42_a_timer; tests\reference\examples\timer_1 -->

The handler is a label of the main module, also when `ON TIMER` stands in a procedure, or a SUB. A SUB's argument is
computed when the handler is set up, and converted to the type of its parameter.
<!-- src: language/events "Timer handlers"; tests\corpus\slice\s134_events_timer -->

`TIMER OFF` stops the timer and forgets an event that came. `TIMER STOP` holds an event until the next `TIMER ON`,
which then runs the handler at once.
<!-- src: language/events "Event switches"; tests\corpus\slice\s134_events_timer; tests\upstream\compile_tests\timer\timer_stop -->

An event runs its handler at the end of a statement, but not while an error handler runs or while its own handler is
still running. A timer event ends a `SLEEP`.
<!-- src: language/events "When an event is serviced"; verification\v42_e_service; tests\upstream\compile_tests\timer\sleep_ends_early -->

`TIMER(n) ON`, `ON TIMER(n, seconds)` and `TIMER(n) FREE` act on a timer `n` given by `_FREETIMER`.
<!-- src: language/events "Numbered timers"; tests\corpus\slice\s134_events_timer -->

## Errors

- A timer that `_FREETIMER` did not give raises error 5, as does `TIMER FREE` of timer 0. The number is rounded:
  `TIMER(u + 0.4) ON` is timer `u`.
  <!-- src: language/events "Numbered timers"; verification\v42_b_numbered; tests\corpus\slice\s134_events_timer; tests\reference\examples\_freetimer_1 -->
- A handler QB64pe rejects is a compile error: a label that is not in the main module, a name that is no SUB, a SUB
  with more than one parameter or one not of an integer type, or an argument given to a SUB without a parameter or
  left out for one with it.
  <!-- src: language/events "Rejected handler forms" -->
- A missing `)` before `GOSUB` is a compile error with the note "Expected )".
  <!-- src: language/events "An event handler without its closing parenthesis" -->
- `TIMER(1, 2)`, `TIMER("a")`, `TIMER()`, `TIMER = 1` and `TIMER` as a variable's name are compile errors.
  <!-- src: verification\v22_x124_timer_two_arguments; verification\v22_x126_timer_empty_parens; verification\v22_x127_timer_assign; verification\v22_x128_timer_dim -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:timer_1 -->
```qb64
ON TIMER(0.6) GOSUB tick
_DELAY 0.9
PRINT "Before TIMER ON:"; n
TIMER ON
_DELAY 1.5
TIMER OFF
PRINT "After 1.5 s at 0.6 s:"; n
PRINT "TIMER is within a day:"; TIMER >= 0 AND TIMER < 86400
END
tick:
n = n + 1
RETURN
```

Output:

```text
Before TIMER ON: 0 
After 1.5 s at 0.6 s: 2 
TIMER is within a day:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_FREETIMER`, `ON`, `KEY`, `SLEEP`, `_DELAY`.
