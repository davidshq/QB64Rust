# Spec Delta

## Purpose

Event trapping: a program names a handler for a timer or a key, switches the event on, off or holds it, and the
handler runs between statements when the event happens, as programs built with the old compiler do.

## ADDED Requirements

### Requirement: Timer handlers
`ON TIMER([n,] seconds) GOSUB label` and `ON TIMER([n,] seconds) name [argument]` SHALL set the handler of timer
`n` (0 when left out) to a label or a SUB, with `seconds` a `DOUBLE` and `n` converted as the old compiler converts
it, also from inside a procedure. A timer that is not allocated SHALL be error 5 at run time.

#### Scenario: A timer that fires twice
- **WHEN** a `$CONSOLE:ONLY` program runs `ON TIMER(2) GOSUB h`, `TIMER ON`, `_DELAY 5`, `SYSTEM`, with `h:` printing
  a line and returning
- **THEN** it prints the line twice, as the old compiler's build does (`timer/timer_first_run`)

#### Scenario: A handler SUB with its argument
- **WHEN** `ON TIMER(1) s 7` is compiled for `SUB s (n AS INTEGER)` and the timer fires
- **THEN** `s` is called with the value 7, evaluated when the handler was set up

#### Scenario: Set up inside a SUB
- **WHEN** `ON TIMER(1) GOSUB h` stands in a SUB and `h` is a label of the main module
- **THEN** it compiles, and the handler runs the main module's `h` up to its `RETURN`

#### Scenario: A timer that was never allocated
- **WHEN** a program runs `ON TIMER(5, 1) GOSUB h` without a `_FREETIMER` that returned 5
- **THEN** it raises error 5, which `ON ERROR` can trap

### Requirement: Numbered timers
`_FREETIMER` SHALL return the number of a newly allocated timer, and `TIMER(n) FREE` SHALL release it so that a later
`_FREETIMER` may return it again. `TIMER FREE` (timer 0) and the release of a timer not allocated SHALL be error 5.

#### Scenario: Two timers at once
- **WHEN** a program allocates `t = _FREETIMER`, sets `ON TIMER(t, 0.5) GOSUB a` and `ON TIMER(2) GOSUB b`, switches
  both on and waits
- **THEN** both handlers run, each at its own interval

#### Scenario: Releasing timer 0
- **WHEN** a program runs `TIMER FREE`
- **THEN** it raises error 5

### Requirement: Event switches
`TIMER[(n)] ON|OFF|STOP|FREE` and `KEY(k) ON|OFF|STOP` SHALL switch an event as the old compiler's build does:
`ON` lets it be serviced, `OFF` stops it and forgets one that happened, `STOP` holds one that happens until the next
`ON`; `KEY(0)` switches every key. A handler set up while its event is off SHALL not run until `ON`.

#### Scenario: Held while stopped
- **WHEN** a program runs `ON TIMER(2) GOSUB h`, `TIMER ON`, `TIMER STOP`, `SLEEP 3`, `TIMER ON`, `TIMER OFF`, `SYSTEM`
- **THEN** `h` runs once, at the second `TIMER ON` (`timer/timer_stop`)

#### Scenario: Off until switched on
- **WHEN** a program runs `ON TIMER(2) GOSUB h`, `_DELAY 3`, `TIMER ON`, `_DELAY 5`, `SYSTEM`
- **THEN** `h` runs twice, counted from `TIMER ON` (`timer/timer_delayed_run`)

### Requirement: When an event is serviced
An event SHALL be serviced at the end of a statement, as the old compiler's build does: not while an error handler
runs, not while the handler of the same event is still running, and a timer event SHALL end a `SLEEP`.

#### Scenario: SLEEP ended by a timer
- **WHEN** a program runs `ON TIMER(2) GOSUB h`, `TIMER ON`, `SLEEP 10`, `SYSTEM`
- **THEN** `h` runs once and the program ends after about two seconds (`timer/sleep_ends_early`)

### Requirement: Key handlers
`ON KEY(k) GOSUB label` and `ON KEY(k) name [argument]` SHALL set the handler of key `k` (1–10 F1–F10, 11–14 the
arrows, 15–25 user keys, 30 and 31 F11 and F12), with `k` converted as the old compiler converts it; a `k` outside
1–31 SHALL be error 5 at run time. A trapped key SHALL run its handler when pressed in the program's window.

#### Scenario: A function key in a window
- **WHEN** a screen program sets `ON KEY(1) GOSUB h`, `KEY(1) ON` and waits, and its keys file presses `f1`
- **THEN** `h` runs once and the screen states equal those of the old compiler's build

#### Scenario: A key number outside the range
- **WHEN** a program runs `ON KEY(40) GOSUB h`
- **THEN** it raises error 5

### Requirement: The KEY statements of their own
`KEY ON`, `KEY OFF` and `KEY LIST` SHALL show, hide and list the soft keys, and `KEY n, text$` SHALL set soft key
`n`, or with `CHR$(flags) + CHR$(scancode)` define user key `n` (15–25) for `ON KEY`, each as the old compiler's
build does. Anything else after `KEY ON`, `KEY OFF` or `KEY LIST` SHALL be a compile error.

#### Scenario: A user key
- **WHEN** a program runs `KEY 15, CHR$(0) + CHR$(30)`, `ON KEY(15) GOSUB h` and `KEY(15) ON`, and the A key is
  pressed in its window
- **THEN** `h` runs, as with the old compiler's build

#### Scenario: KEY ON is not a switch
- **WHEN** `KEY ON` and `KEY(0) ON` are compiled
- **THEN** the first is the soft-key line's call and the second switches every key's event, as the old compiler
  writes them

#### Scenario: KEY ON with more after it
- **WHEN** a program has `KEY ON 1`
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: Rejected handler forms
A handler the old compiler rejects SHALL be a compile error that is not "not supported yet": a label that is not
one of the main module, a name that is no SUB, a SUB with more than one parameter, an argument given to a SUB
without one or left out for a SUB with one, a parameter that is not of an integer type (an array included), and a
switch form outside the table's (`KEY(1) FREE`, `TIMER(1, 2) ON`).

#### Scenario: A label only in a SUB
- **WHEN** `ON TIMER(1) GOSUB h` is compiled and `h` is a label only inside a SUB
- **THEN** the compiler reports that the label is not defined

#### Scenario: The SUB's parameter
- **WHEN** a program has `ON TIMER(1) s` for `SUB s (n AS LONG)`, `ON TIMER(1) s 5` for `SUB s` or for `SUB s (n AS
  SINGLE)`, or `ON TIMER(1) f` for a FUNCTION `f`
- **THEN** each is a compile error that is not "not supported yet", and `ON TIMER(1) s 5` for `SUB s (n AS
  _UNSIGNED INTEGER)` compiles

### Requirement: Events the old compiler does not have
`PEN`, `COM` and `UEVENT` used as events (`ON PEN GOSUB h`, `ON COM(1) GOSUB h`, `PEN ON`, `UEVENT ON`) SHALL be
compile errors that are not "not supported yet", as they are to the old compiler. `ON STRIG` and the `STRIG` switch
SHALL stay marked "not supported yet" (device input is not built), and so SHALL a line number as a handler target.

#### Scenario: PEN as an event
- **WHEN** a program has `ON PEN GOSUB h` or `PEN ON`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: STRIG left marked
- **WHEN** a program has `ON STRIG(0) GOSUB h` or `STRIG(0) ON`
- **THEN** the compiler reports "not supported yet", naming `STRIG`
