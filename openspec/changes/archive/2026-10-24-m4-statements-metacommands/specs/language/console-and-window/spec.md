# Spec Delta

## ADDED Requirements

### Requirement: The function _SCREENHIDE
`_SCREENHIDE` as a value SHALL be compiled with the runtime call the old compiler writes for it
(`func__screenhide`), and SHALL give what that call gives: -1 when the window is hidden, 0 when it is not and in a
`$CONSOLE:ONLY` program (measured, `verification\v57_h01`–`h04`). The statement `_SCREENHIDE` SHALL stay as it
is.

#### Scenario: Compiled beside the window statements
- **WHEN** a program has `n = _SCREENHIDE` after its `SYSTEM`, beside the other window statements after `CALL`
- **THEN** it compiles and prints `ok` (`verification\v26_x74_window_accepted`), and the call-site check finds
  the call equal to the old compiler's

#### Scenario: After $SCREENHIDE
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` prints `_SCREENHIDE`
- **THEN** it prints `-1` (`verification\v57_h01`)

#### Scenario: After the statement and _SCREENSHOW
- **WHEN** a program with `$CONSOLE` runs the statement `_SCREENHIDE`, prints `_SCREENHIDE`, runs `_SCREENSHOW` and
  prints it again
- **THEN** it prints `-1`, then `0` (`verification\v57_h03`)

### Requirement: INPUT into a place in parentheses
`INPUT (a)` and `LINE INPUT (a$)` SHALL read into the variable inside the parentheses, as the old compiler's
program does (measured, `verification\v57_p06`, `p07`).

#### Scenario: INPUT
- **WHEN** `INPUT (a): PRINT a` runs with the input `4`
- **THEN** it prints the prompt `? ` and ` 4 `, as the old compiler's program (`verification\v57_p06`)
