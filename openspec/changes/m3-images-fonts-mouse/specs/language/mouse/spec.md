# Spec Delta

## Purpose

Reading the mouse in a program with a window: the queue of mouse events `_MOUSEINPUT` reads, the position, the
buttons, the wheel and the movement, whether the cursor is hidden or disabled, and `_MOUSEMOVE`, as the old
compiler's program behaves.

## ADDED Requirements

### Requirement: Forms of the mouse built-ins
`_MOUSEINPUT`, `_MOUSEX`, `_MOUSEY`, `_MOUSEBUTTON`, `_MOUSEWHEEL`, `_MOUSEMOVEMENTX`, `_MOUSEMOVEMENTY`,
`_MOUSEHIDDEN`, `_MOUSEDISABLED` and `_MOUSEMOVE` SHALL compile in each form the old compiler accepts, with the
call it writes and the result type it gives; a form it rejects SHALL be a compile error that is not "not
supported yet".

#### Scenario: Bare and with an argument
- **WHEN** `WHILE _MOUSEINPUT: WEND`, `PRINT _MOUSEX; _MOUSEY; _MOUSEBUTTON(1); _MOUSEWHEEL; _MOUSEWHEEL(1)` and
  `_MOUSEMOVE 10, 20` are compiled
- **THEN** each compiles to the call the old compiler writes, and `PRINT` shows each value as the old compiler's
  program does

#### Scenario: A button without its number
- **WHEN** `PRINT _MOUSEBUTTON` is compiled
- **THEN** it is the old compiler's compile error, not "not supported yet"

### Requirement: The mouse queue
A program SHALL read mouse events in the order they came with `_MOUSEINPUT`, which gives true while it takes an
event and false when the queue is empty; `_MOUSEX`, `_MOUSEY`, `_MOUSEBUTTON` and `_MOUSEWHEEL` SHALL give the
event last taken, as the old compiler's program does.

#### Scenario: A click read from the queue
- **WHEN** a screen program waits for its mouse steps, then reads every event with `_MOUSEINPUT` and prints, for
  each, `_MOUSEX`, `_MOUSEY` and `_MOUSEBUTTON(1)`, and the keys file moves the mouse, presses and releases the
  left button
- **THEN** the state of its screen equals the old compiler's

#### Scenario: The wheel
- **WHEN** the keys file turns the wheel one step down and the program prints `_MOUSEWHEEL` for each event it
  takes
- **THEN** the state of its screen equals the old compiler's

### Requirement: Position in text and graphics modes
`_MOUSEX` and `_MOUSEY` SHALL give a column and a row in a text mode and a pixel in a graphics mode, kept inside
the screen, as libqb computes them from the position of the event.

#### Scenario: Text mode
- **WHEN** a `SCREEN 0` program and a `SCREEN 12` program each read a mouse event at the same window pixel
- **THEN** each prints what the old compiler's program prints: a column and row, and a pixel

### Requirement: Mouse built-ins that no test runs
`_MOUSEMOVE`, which moves the machine's cursor, and `_MOUSESHOW`, `_MOUSEHIDE` and `_MOUSECURSOR`, whose effect is
the shape of the cursor, SHALL compile to the call the old compiler writes; no test runs them.

#### Scenario: Moving the cursor
- **WHEN** `_MOUSEMOVE 100, 50` is compiled
- **THEN** it compiles to the call the old compiler writes, and a slice program holds it as code that is not
  reached
