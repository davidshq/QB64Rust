# Spec Delta

## MODIFIED Requirements

### Requirement: Mouse built-ins that no test runs
`_MOUSEMOVE`, which moves the machine's cursor, and `_MOUSESHOW`, `_MOUSEHIDE` and `_MOUSECURSOR`, whose effect is
the shape of the cursor, SHALL compile to the call the old compiler writes. No test runs `_MOUSEMOVE`; the other
three are run in a screen program, their effect (the cursor, outside the screen's state) not compared.

#### Scenario: Moving the cursor
- **WHEN** `_MOUSEMOVE 100, 50` is compiled
- **THEN** it compiles to the call the old compiler writes, and a slice program holds it as code that is not
  reached

#### Scenario: The cursor's shape changed in a window
- **WHEN** a screen program runs `_MOUSEHIDE`, `_MOUSESHOW` and `_MOUSECURSOR 2`
- **THEN** it goes on after them, and its screen is the old compiler's
