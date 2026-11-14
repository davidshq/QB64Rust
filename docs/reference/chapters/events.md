# Events

A program names a handler for a timer, a key or a joystick button, switches the event on, and the handler runs between
statements when the event happens.

The pages of this area: [TIMER](../entries/timer.md), [_FREETIMER](../entries/_freetimer.md),
[KEY](../entries/key.md), [STRIG](../entries/strig.md), [SLEEP](../entries/sleep.md), [_DELAY](../entries/_delay.md),
[_LIMIT](../entries/_limit.md), [COM](../entries/com.md), [PEN](../entries/pen.md), [UEVENT](../entries/uevent.md).

## Handlers

`ON TIMER(seconds) GOSUB label`, `ON KEY(n) GOSUB label` and `ON STRIG(n) GOSUB label` set a handler: a label of the
main module, also from inside a procedure, or a SUB with one integer parameter, whose argument is computed when the
handler is set.
<!-- src: language/events "Timer handlers"; language/events "Key handlers"; language/events "Joystick-button handlers"; tests\corpus\slice\s134_events_timer -->

`_FREETIMER` gives the number of a further timer, for `ON TIMER(n, seconds)`, and `TIMER(n) FREE` releases it.
<!-- src: language/events "Numbered timers"; tests\reference\examples\_freetimer_1 -->

## Switches

A handler runs only after its event is switched `ON`. `OFF` stops the event and forgets one that came; `STOP` holds
one until the next `ON`, which then runs the handler at once.
<!-- src: language/events "Event switches"; tests\corpus\slice\s134_events_timer; tests\reference\examples\timer_1 -->

An event is serviced at the end of a statement, not while an error handler runs or while its own handler still runs. A
timer event ends a `SLEEP`.
<!-- src: language/events "When an event is serviced"; verification\v42_e_service -->

## Events QB64pe does not have

`ON PEN`, `ON COM` and `ON UEVENT`, and their switches, are compile errors, as in QB64pe; `ON PLAY(n) GOSUB` is `ON
number GOSUB`.
<!-- src: language/events "Events the old compiler does not have"; language/sound "The PLAY function" -->
