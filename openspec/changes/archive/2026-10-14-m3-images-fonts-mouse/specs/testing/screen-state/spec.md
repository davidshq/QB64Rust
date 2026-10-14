# Spec Delta

## ADDED Requirements

### Requirement: Mouse steps in the keys file
A keys file SHALL also give the program mouse events, in order with its keys: `mouse <x> <y>` (a move to pixel x,
y of the window), `press <button>` and `release <button>` (1 left, 2 right, 3 middle, at the last position) and
`wheel <n>` (n steps, negative up). Each SHALL reach the queue that `_MOUSEINPUT` reads as the window's event
would.

#### Scenario: A click
- **WHEN** the keys file is `prompt`, `mouse 40 30`, `press 1`, `release 1`, and the program reads every event
  with `_MOUSEINPUT` and prints `_MOUSEX`, `_MOUSEY` and `_MOUSEBUTTON(1)` for each
- **THEN** it prints four events at 40, 30: the move and libqb's copy of it with the movement cleared, both with
  the button up, the press with it down, the release with it up

#### Scenario: A mistake in a mouse step
- **WHEN** a line reads `press 9` or `mouse 10`
- **THEN** the program exits with code 125 and stderr names the file and the line

### Requirement: A screen program for each image, font and mouse built-in
Every built-in of `language/images`, `language/fonts` and `language/mouse` that a test runs SHALL be used, in
code that runs, by at least one screen program of `tests/corpus/slice.list`, recorded with the old compiler.

#### Scenario: A built-in no screen program runs
- **WHEN** `_SETALPHA` is compiled by the new compiler
- **THEN** a screen program recorded with `qb64pe.exe` calls `_SETALPHA` in code that runs and is on `slice.list`
