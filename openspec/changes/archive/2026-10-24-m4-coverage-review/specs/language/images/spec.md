# Spec Delta

## MODIFIED Requirements

### Requirement: Image built-ins that no test runs
`_SCREENIMAGE`, `_CLIPBOARDIMAGE` (function and statement), `_PRINTIMAGE`, `_DEPTHBUFFER` and the 3-D form of
`_MAPTRIANGLE` SHALL compile to the call the old compiler writes, and SHALL link the part of libqb each needs.
`_CLIPBOARDIMAGE`, `_PRINTIMAGE` and the 3-D form of `_MAPTRIANGLE` are run by no test, since what they read or
draw is the clipboard's, a printer's or OpenGL's; `_SCREENIMAGE` (the upstream tests) and `_DEPTHBUFFER` (a screen
program) are run, what they read or draw not compared.

#### Scenario: A desktop image
- **WHEN** `i = _SCREENIMAGE` is compiled
- **THEN** it compiles to the call the old compiler writes, and the program is linked with the screen-capture
  part of libqb

#### Scenario: A desktop image takes no corners or all four
- **WHEN** `i = _SCREENIMAGE(1, 2)` is compiled
- **THEN** it is an error that `_SCREENIMAGE` takes 0 or 4 arguments, as the old compiler rejects it

#### Scenario: A depth buffer on a hardware image
- **WHEN** a program makes a hardware image with `_COPYIMAGE(…, 33)` and runs `_DEPTHBUFFER ON`, `_CLEAR` and
  `OFF` on it
- **THEN** it goes on after them, and its screen is the old compiler's
