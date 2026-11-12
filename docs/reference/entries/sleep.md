# SLEEP

`SLEEP seconds` waits that many seconds, or until a key is pressed or an event comes. `SLEEP` with no time waits for a
key.

## Syntax

<!-- begin generated: syntax -->
```text
SLEEP seconds%
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A key ends `SLEEP` only when it comes while `SLEEP` waits. A key pressed before stays in the `INKEY$` buffer.
<!-- src: language/screen-output "Pacing statements"; tests\corpus\screen\d15_pacing -->

A timer event ends it: `SLEEP 10` with a timer of 0.5 second ends after about 0.5 second, and the handler has run.
<!-- src: language/events "When an event is serviced"; verification\v42_e_service; tests\upstream\compile_tests\timer\sleep_ends_early -->

How long it waits depends on the clock and the keyboard, so no test checks the time.
<!-- src: language/screen-output "Pacing statements"; tests\corpus\screen\d15_pacing -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `SLEEP 1, 2`: "this `SLEEP` statement matches none of its forms".
  <!-- src: language/builtin-statements "Supported built-in statements"; verification\v27_x28_sleep_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN SLEEP 1 -->

## See also

`_DELAY`, `_LIMIT`, `TIMER`, `INKEY$`.
