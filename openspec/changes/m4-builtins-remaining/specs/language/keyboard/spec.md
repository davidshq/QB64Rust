# Spec Delta

## ADDED Requirements

### Requirement: The lock keys
`_CAPSLOCK`, `_NUMLOCK` and `_SCROLLLOCK` SHALL compile as functions, giving the key's state as libqb gives it,
and as statements in every form the old compiler accepts (`ON`, `OFF`, `_TOGGLE`), each with its runtime call,
and the forms it rejects SHALL be its errors (measured, `verification\v58`). The statements change the machine's
keyboard state, so no test runs them; the call-site check holds them to the old compiler's calls.

#### Scenario: The state as a value
- **WHEN** a program prints `_CAPSLOCK; _NUMLOCK; _SCROLLLOCK` and each value's type
- **THEN** each is 0 or -1 and of the old compiler's type, as its program gives them on the same machine

#### Scenario: The statements are compiled
- **WHEN** a program has `_CAPSLOCK ON`, `_NUMLOCK OFF` and `_SCROLLLOCK _TOGGLE`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: A rejected form
- **WHEN** a program has a form of the statement that `v58` records the old compiler rejecting, such as
  `_CAPSLOCK 1`
- **THEN** the compile fails with an error whose note is the old compiler's message
