# language/keyboard Specification

## Purpose

The keyboard read beside `INKEY$`: the codes of keys going down and up (`_KEYHIT`), whether a key is held now
(`_KEYDOWN`), and the emptying of the key buffers (`_KEYCLEAR`), as programs built with the old compiler read them.

## Requirements

### Requirement: _KEYHIT
`_KEYHIT` SHALL compile, and at run time SHALL give the next code of libqb's key-hit buffer as a LONG: a key's
code when it goes down, the code negated when it goes up, and 0 when the buffer is empty. The codes and their order
SHALL be those the old compiler's program reads for the same keys.

#### Scenario: A key down and up
- **WHEN** a screen program reads `_KEYHIT` until it is not 0, twice, and the keys file gives `text a`
- **THEN** it reads 97 and then -97, as the old compiler's program does

#### Scenario: Nothing typed
- **WHEN** a program with a window prints `_KEYHIT` before any key is given
- **THEN** it prints ` 0 `

### Requirement: _KEYDOWN
`_KEYDOWN(code)` SHALL compile, and at run time SHALL give -1 while the key of that code is held down and 0
otherwise. A code of 0 or less SHALL raise error 5, as in the old compiler.

#### Scenario: A held key
- **WHEN** a screen program announces a wait, then loops until `_KEYDOWN(100304)` is -1, and the keys file gives
  `prompt`, `hold lshift`, `wait 200`, `lift lshift`
- **THEN** the loop ends, and the program reads `_KEYDOWN(100304)` as 0 once the key is lifted, as the old
  compiler's program does

#### Scenario: A code out of range
- **WHEN** `k& = _KEYDOWN(0)` runs under a handler
- **THEN** error 5 is raised

### Requirement: _KEYCLEAR
`_KEYCLEAR` SHALL compile with no argument and with one. With none it SHALL empty the `INKEY$` buffer, the
`_KEYHIT` buffer and the `INP(&H60)` buffer; with 1, 2 or 3 only that one of them, in that order; any other number
SHALL raise error 5, as in the old compiler.

#### Scenario: The INKEY$ buffer emptied
- **WHEN** a screen program waits until `_KEYHIT` has seen a key, then runs `_KEYCLEAR 1: PRINT LEN(INKEY$)`
- **THEN** it prints ` 0 `, as the old compiler's program does

#### Scenario: A buffer that does not exist
- **WHEN** `_KEYCLEAR 4` runs under a handler
- **THEN** error 5 is raised

#### Scenario: Each form compiles
- **WHEN** a program has `_KEYCLEAR`, `_KEYCLEAR 2` and `CALL _KEYCLEAR(3)`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

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
